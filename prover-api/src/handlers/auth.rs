use std::{collections::HashMap, sync::Arc};

use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet,
    PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenResponse, TokenUrl,
    basic::BasicClient,
};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::{config::Config, models::GithubUser};

const GITHUB_AUTHORIZE_URL: &str = "https://github.com/login/oauth/authorize";
const GITHUB_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const GITHUB_USER_URL: &str = "https://api.github.com/user";

type GithubClient =
    BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>;

#[derive(Clone)]
pub struct AuthState {
    pub config: Config,
    pending_pkce: Arc<Mutex<HashMap<String, String>>>,
}

impl AuthState {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            pending_pkce: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn save_pkce_verifier(&self, state: String, verifier: String) {
        self.pending_pkce.lock().await.insert(state, verifier);
    }

    async fn take_pkce_verifier(&self, state: &str) -> Option<String> {
        self.pending_pkce.lock().await.remove(state)
    }
}

#[derive(Debug)]
pub struct AuthError {
    status: StatusCode,
    message: String,
}

impl AuthError {
    fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            message: message.into(),
        }
    }

    fn internal(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.into(),
        }
    }

    fn upstream(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_GATEWAY,
            message: message.into(),
        }
    }
}

#[derive(serde::Serialize)]
struct ErrorBody {
    error: String,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(ErrorBody {
                error: self.message,
            }),
        )
            .into_response()
    }
}

#[derive(Debug, Deserialize)]
pub struct GithubCallbackQuery {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

fn github_client(config: &Config) -> Result<GithubClient, AuthError> {
    if config.github_client_id.trim().is_empty()
        || config.github_client_secret.trim().is_empty()
        || config.github_redirect_uri.trim().is_empty()
    {
        return Err(AuthError::internal(
            "GitHub OAuth configuration is incomplete",
        ));
    }

    let client = BasicClient::new(ClientId::new(config.github_client_id.clone()))
        .set_client_secret(ClientSecret::new(config.github_client_secret.clone()))
        .set_auth_uri(
            AuthUrl::new(GITHUB_AUTHORIZE_URL.to_owned())
                .map_err(|_| AuthError::internal("invalid GitHub authorization URL"))?,
        )
        .set_token_uri(
            TokenUrl::new(GITHUB_TOKEN_URL.to_owned())
                .map_err(|_| AuthError::internal("invalid GitHub token URL"))?,
        )
        .set_redirect_uri(
            RedirectUrl::new(config.github_redirect_uri.clone())
                .map_err(|_| AuthError::internal("invalid GitHub redirect URI"))?,
        );

    Ok(client)
}

pub async fn github_login(State(state): State<AuthState>) -> Result<Redirect, AuthError> {
    let client = github_client(&state.config)?;
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (authorization_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("read:user".to_owned()))
        .add_scope(Scope::new("user:email".to_owned()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    state
        .save_pkce_verifier(
            csrf_token.secret().to_owned(),
            pkce_verifier.secret().to_owned(),
        )
        .await;

    Ok(Redirect::temporary(authorization_url.as_str()))
}

pub async fn github_callback(
    State(state): State<AuthState>,
    Query(query): Query<GithubCallbackQuery>,
) -> Result<Json<GithubUser>, AuthError> {
    if let Some(error) = query.error {
        let description = query.error_description.unwrap_or_default();
        let message = if description.is_empty() {
            format!("GitHub OAuth authorization failed: {error}")
        } else {
            format!("GitHub OAuth authorization failed: {error}: {description}")
        };
        return Err(AuthError::bad_request(message));
    }

    let code = query
        .code
        .ok_or_else(|| AuthError::bad_request("missing GitHub authorization code"))?;
    let csrf_state = query
        .state
        .ok_or_else(|| AuthError::bad_request("missing OAuth state"))?;
    let verifier = state
        .take_pkce_verifier(&csrf_state)
        .await
        .ok_or_else(|| AuthError::bad_request("invalid or expired OAuth state"))?;

    let client = github_client(&state.config)?;
    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|_| AuthError::internal("failed to create OAuth HTTP client"))?;

    let token = client
        .exchange_code(AuthorizationCode::new(code))
        .set_pkce_verifier(PkceCodeVerifier::new(verifier))
        .request_async(&http_client)
        .await
        .map_err(|_| AuthError::upstream("GitHub token exchange failed"))?;

    let response = http_client
        .get(GITHUB_USER_URL)
        .bearer_auth(token.access_token().secret())
        .header(reqwest::header::USER_AGENT, "prover-api")
        .send()
        .await
        .map_err(|_| AuthError::upstream("GitHub user request failed"))?;

    if !response.status().is_success() {
        return Err(AuthError::upstream("GitHub user request was rejected"));
    }

    let user = response
        .json::<GithubUser>()
        .await
        .map_err(|_| AuthError::upstream("invalid GitHub user response"))?;

    Ok(Json(user))
}
