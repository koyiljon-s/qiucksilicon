use axum::{Router, routing::get};

use crate::handlers::auth::{AuthState, github_callback, github_login};

pub fn auth_routes() -> Router<AuthState> {
    Router::new()
        .route("/auth/github", get(github_login))
        .route("/auth/github/callback", get(github_callback))
}
