use axum::{
    routing::get,
    Router,
    Json,
};
use axum_prometheus::PrometheusMetricLayer;
use serde::Serialize;

pub fn create_router() -> Router {

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    Router::new()
        .route("/", get(welcome_handler))
        .route("/health", get(health_handler))
        .route("/metrics", get(move || async move { metric_handle.render() }))
        .layer(prometheus_layer)
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