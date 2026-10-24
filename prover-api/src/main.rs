use log::{error, info};
use std::sync::Arc;

mod config;
mod db;
mod handlers;
mod models;
mod repositories;
mod routers;

use config::Config;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let settings = Config::from_env().unwrap_or_else(|error| {
        error!("failed to load application configuration: {error}");
        std::process::exit(1);
    });

    if settings.database_url.trim().is_empty() {
        error!("database configuration error: DATABASE_URL is empty");
        std::process::exit(1);
    }

    let client = db::connect_and_check(&settings.database_url)
        .await
        .unwrap_or_else(|error| {
            error!("database initialization failed: {error}");
            std::process::exit(1);
        });
    let client = Arc::new(client);

    repositories::GithubUserRepository::new(&client)
        .create_table()
        .await
        .unwrap_or_else(|error| {
            error!("failed to initialize GitHub users table: {error}");
            std::process::exit(1);
        });

    let server_port = settings.server_port;
    info!("Server port configured: {server_port}");

    let app = routers::create_router(settings.clone(), client);

    let addr = format!("127.0.0.1:{}", settings.server_port);
    info!("Server listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|error| {
            error!("failed to bind to address {addr}: {error}");
            std::process::exit(1);
        });

    axum::serve(listener, app).await.unwrap_or_else(|error| {
        error!("server error: {error}");
        std::process::exit(1);
    });
}
