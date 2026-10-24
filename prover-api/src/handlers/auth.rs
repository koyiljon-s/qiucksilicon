use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use serde::Deserialize;

use crate::services::{
    AuthService, AuthServiceError,
    session::{SESSION_COOKIE_NAME, SessionServiceError},
};

#[derive(Debug, Deserialize)]
pub struct GithubCallbackQuery {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

#[derive(Debug)]
pub struct AuthError {
    status: StatusCode,
    message: String,
}

impl AuthError {
    fn from_service(error: AuthServiceError) -> Self {
        log::error!("GitHub authentication service error: {error:?}");

        let (status, message) = match error {
            AuthServiceError::AuthorizationFailed(message) => (StatusCode::BAD_REQUEST, message),
            AuthServiceError::MissingCode => (
                StatusCode::BAD_REQUEST,
                "missing GitHub authorization code".to_owned(),
            ),
            AuthServiceError::MissingState => {
                (StatusCode::BAD_REQUEST, "missing OAuth state".to_owned())
            }
            AuthServiceError::InvalidState => (
                StatusCode::BAD_REQUEST,
                "invalid or expired OAuth state".to_owned(),
            ),
            AuthServiceError::IncompleteConfiguration
            | AuthServiceError::InvalidConfiguration
            | AuthServiceError::HttpClient => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "GitHub OAuth configuration or client setup failed".to_owned(),
            ),
            AuthServiceError::TokenExchange
            | AuthServiceError::GithubRequest
            | AuthServiceError::InvalidGithubUser => {
                (StatusCode::BAD_GATEWAY, "GitHub request failed".to_owned())
            }
            AuthServiceError::Persistence(message) => {
                log::error!("GitHub user persistence error: {message}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to save GitHub user".to_owned(),
                )
            }
            AuthServiceError::Session(error) => {
                match error {
                    SessionServiceError::Database(message) => {
                        log::error!("Session database error: {message}");
                    }
                    SessionServiceError::InvalidUserId => {
                        log::error!("Session user ID could not be represented");
                    }
                }
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "session operation failed".to_owned(),
                )
            }
        };

        Self { status, message }
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

pub async fn github_login(State(service): State<AuthService>) -> Result<Redirect, AuthError> {
    let url = service
        .authorization_url()
        .await
        .map_err(AuthError::from_service)?;

    Ok(Redirect::temporary(&url))
}

pub async fn github_callback(
    State(service): State<AuthService>,
    Query(query): Query<GithubCallbackQuery>,
) -> Result<(CookieJar, Redirect), AuthError> {
    let user = service
        .complete_callback(
            query.code,
            query.state,
            query.error,
            query.error_description,
        )
        .await
        .map_err(AuthError::from_service)?;

    let session_token = service
        .create_session(user.id)
        .await
        .map_err(AuthError::from_service)?;

    let cookie = Cookie::build((SESSION_COOKIE_NAME, session_token))
        .http_only(true)
        .secure(service.secure_cookie())
        .same_site(SameSite::Lax)
        .path("/")
        .build();

    Ok((CookieJar::new().add(cookie), Redirect::temporary("/")))
}

pub async fn logout(
    State(service): State<AuthService>,
    jar: CookieJar,
) -> Result<(CookieJar, Redirect), AuthError> {
    if let Some(cookie) = jar.get(SESSION_COOKIE_NAME) {
        service
            .revoke_session(cookie.value())
            .await
            .map_err(AuthError::from_service)?;
    }

    let removal_cookie = Cookie::build((SESSION_COOKIE_NAME, "")).path("/").build();

    Ok((jar.remove(removal_cookie), Redirect::temporary("/")))
}
