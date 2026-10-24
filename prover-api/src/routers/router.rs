use axum::{Json, Router, routing::get};
use axum_prometheus::PrometheusMetricLayer;
use serde::Serialize;
use std::sync::Arc;

use crate::{config::Config, handlers::auth::AuthState, routers::auth::auth_routes};

pub fn create_router(config: Config, db: Arc<tokio_postgres::Client>) -> Router {
    let auth_state = AuthState::new(config, db);

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    Router::new()
        .route("/", get(welcome_handler))
        .route("/health", get(health_handler))
        .route(
            "/metrics",
            get(move || async move { metric_handle.render() }),
        )
        .merge(auth_routes())
        .layer(prometheus_layer)
        .with_state(auth_state)
}

#[derive(Serialize)]
struct WelcomeResponse {
    message: String,
}

#[derive(Serialize)]
struct HealthResponse {
    message: String,
    status: String,
}

async fn welcome_handler() -> Json<WelcomeResponse> {
    let response = WelcomeResponse {
        message: "Welcome to the Prover API!".to_string(),
    };
    Json(response)
}

async fn health_handler() -> Json<HealthResponse> {
    let response = HealthResponse {
        status: "healthy".to_string(),
        message: "The API is running smoothly!".to_string(),
    };
    Json(response)
}
