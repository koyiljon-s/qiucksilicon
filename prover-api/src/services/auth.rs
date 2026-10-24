use std::{collections::HashMap, sync::Arc};

use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
    basic::BasicClient, reqwest,
};
use tokio::sync::Mutex;

use crate::{
    config::Config,
    models::GithubUser,
    repositories::GithubUserRepository,
    services::session::{SessionService, SessionServiceError},
};

const GITHUB_AUTHORIZE_URL: &str = "https://github.com/login/oauth/authorize";
const GITHUB_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const GITHUB_USER_URL: &str = "https://api.github.com/user";

type GithubClient =
    BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>;

#[derive(Clone)]
pub struct AuthService {
    config: Config,
    db: Arc<tokio_postgres::Client>,
    sessions: SessionService,
    pending_pkce: Arc<Mutex<HashMap<String, String>>>,
}

impl AuthService {
    pub fn new(config: Config, db: Arc<tokio_postgres::Client>) -> Self {
        let sessions = SessionService::new(db.clone(), config.cookie_secure);
        Self {
            config,
            db,
            sessions,
            pending_pkce: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn authorization_url(&self) -> Result<String, AuthServiceError> {
        let client = self.github_client()?;
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (authorization_url, csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("read:user".to_owned()))
            .add_scope(Scope::new("user:email".to_owned()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        self.pending_pkce.lock().await.insert(
            csrf_token.secret().to_owned(),
            pkce_verifier.secret().to_owned(),
        );

        Ok(authorization_url.to_string())
    }

    pub async fn create_session(&self, github_id: u64) -> Result<String, AuthServiceError> {
        self.sessions
            .create(github_id)
            .await
            .map_err(AuthServiceError::Session)
    }

    #[allow(dead_code)]
    pub async fn authenticate_session(&self, token: &str) -> Result<Option<u64>, AuthServiceError> {
        self.sessions
            .authenticate(token)
            .await
            .map_err(AuthServiceError::Session)
    }

    pub async fn revoke_session(&self, token: &str) -> Result<(), AuthServiceError> {
        self.sessions
            .revoke(token)
            .await
            .map_err(AuthServiceError::Session)
    }

    pub fn secure_cookie(&self) -> bool {
        self.sessions.secure_cookie()
    }

    pub async fn complete_callback(
        &self,
        code: Option<String>,
        state: Option<String>,
        error: Option<String>,
        error_description: Option<String>,
    ) -> Result<GithubUser, AuthServiceError> {
        if let Some(error) = error {
            let description = error_description.unwrap_or_default();
            let message = if description.is_empty() {
                format!("GitHub OAuth authorization failed: {error}")
            } else {
                format!("GitHub OAuth authorization failed: {error}: {description}")
            };
            return Err(AuthServiceError::AuthorizationFailed(message));
        }

        let code = code.ok_or(AuthServiceError::MissingCode)?;
        let state = state.ok_or(AuthServiceError::MissingState)?;
        let verifier = self
            .pending_pkce
            .lock()
            .await
            .remove(&state)
            .ok_or(AuthServiceError::InvalidState)?;

        let client = self.github_client()?;
        let http_client = reqwest::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|_| AuthServiceError::HttpClient)?;

        let token = client
            .exchange_code(AuthorizationCode::new(code))
            .set_pkce_verifier(PkceCodeVerifier::new(verifier))
            .request_async(&http_client)
            .await
            .map_err(|_| AuthServiceError::TokenExchange)?;

        let response = http_client
            .get(GITHUB_USER_URL)
            .bearer_auth(token.access_token().secret())
            .header(reqwest::header::USER_AGENT, "prover-api")
            .send()
            .await
            .map_err(|_| AuthServiceError::GithubRequest)?;

        if !response.status().is_success() {
            return Err(AuthServiceError::GithubRequest);
        }

        let user = response
            .json::<GithubUser>()
            .await
            .map_err(|_| AuthServiceError::InvalidGithubUser)?;

        GithubUserRepository::new(&self.db)
            .upsert(&user)
            .await
            .map_err(|error| AuthServiceError::Persistence(error.to_string()))
    }

    fn github_client(&self) -> Result<GithubClient, AuthServiceError> {
        if self.config.github_client_id.trim().is_empty()
            || self.config.github_client_secret.trim().is_empty()
            || self.config.github_redirect_uri.trim().is_empty()
        {
            return Err(AuthServiceError::IncompleteConfiguration);
        }

        let client = BasicClient::new(ClientId::new(self.config.github_client_id.clone()))
            .set_client_secret(ClientSecret::new(self.config.github_client_secret.clone()))
            .set_auth_uri(
                AuthUrl::new(GITHUB_AUTHORIZE_URL.to_owned())
                    .map_err(|_| AuthServiceError::InvalidConfiguration)?,
            )
            .set_token_uri(
                TokenUrl::new(GITHUB_TOKEN_URL.to_owned())
                    .map_err(|_| AuthServiceError::InvalidConfiguration)?,
            )
            .set_redirect_uri(
                RedirectUrl::new(self.config.github_redirect_uri.clone())
                    .map_err(|_| AuthServiceError::InvalidConfiguration)?,
            );

        Ok(client)
    }
}

#[derive(Debug)]
pub enum AuthServiceError {
    AuthorizationFailed(String),
    MissingCode,
    MissingState,
    InvalidState,
    IncompleteConfiguration,
    InvalidConfiguration,
    HttpClient,
    TokenExchange,
    GithubRequest,
    InvalidGithubUser,
    Persistence(String),
    Session(SessionServiceError),
}
