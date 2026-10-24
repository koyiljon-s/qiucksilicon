use log::{error, info};
mod config;
use config::Config;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let settings = match Config::from_env() {
        Ok(settings) => settings,
        Err(error) => {
            error!("failed to load application configuration: {error}");
            std::process::exit(1);
        }
    };

    if settings.database_url.trim().is_empty() {
        error!("database configuration error: DATABASE_URL is empty");
        std::process::exit(1);
    }

    info!("database configuration loaded successfully");

    let (client, connection) =
        match tokio_postgres::connect(&settings.database_url, tokio_postgres::NoTls).await {
            Ok(connection) => connection,
            Err(error) => {
                error!("failed to connect to database: {error}");
                std::process::exit(1);
            }
        };

    tokio::spawn(async move {
        if let Err(error) = connection.await {
            error!("database connection error: {error}");
        }
    });

    if let Err(error) = client.simple_query("SELECT 1").await {
        error!("database health check failed: {error}");
        std::process::exit(1);
    }

    info!("database connection and health check succeeded");

    let server_port = settings.server_port;
    info!("server port configured: {server_port}");
}
