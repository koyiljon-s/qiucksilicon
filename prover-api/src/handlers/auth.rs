use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use serde::Deserialize;

use crate::services::{AuthService, AuthServiceError};

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
            AuthServiceError::Persistence => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "failed to save GitHub user".to_owned(),
            ),
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
) -> Result<Json<crate::models::GithubUser>, AuthError> {
    let user = service
        .complete_callback(
            query.code,
            query.state,
            query.error,
            query.error_description,
        )
        .await
        .map_err(AuthError::from_service)?;

    Ok(Json(user))
}
