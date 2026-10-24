use axum::{Router, routing::get};

use crate::{
    handlers::auth::{github_callback, github_login},
    services::AuthService,
};

pub fn auth_routes() -> Router<AuthService> {
    Router::new()
        .route("/auth/github", get(github_login))
        .route("/auth/github/callback", get(github_callback))
}
