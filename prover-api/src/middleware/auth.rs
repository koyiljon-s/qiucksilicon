use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum_extra::extract::CookieJar;

use crate::{services::AuthService, services::session::SESSION_COOKIE_NAME};

#[allow(dead_code)]
pub async fn require_auth(
    State(service): State<AuthService>,
    jar: CookieJar,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = jar
        .get(SESSION_COOKIE_NAME)
        .map(|cookie| cookie.value().to_owned())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let github_id = service
        .authenticate_session(&token)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    request.extensions_mut().insert(github_id);
    Ok(next.run(request).await)
}
