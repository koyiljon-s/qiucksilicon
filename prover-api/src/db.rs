// src/db.rs
use log::{error, info};
use tokio_postgres::Client;

pub async fn connect_and_check(database_url: &str) -> Result<Client, Box<dyn std::error::Error>> {
    info!("Connecting to database...");
    
    let (client, connection) = tokio_postgres::connect(database_url, tokio_postgres::NoTls).await?;

    tokio::spawn(async move {
        if let Err(error) = connection.await {
            error!("database connection error: {error}");
        }
    });

    client.simple_query("SELECT 1").await?;
    info!("Database connection and health check succeeded");

    Ok(client)
}