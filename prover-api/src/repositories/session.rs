use tokio_postgres::Client;

const CREATE_TABLE_SQL: &str = r#"
    CREATE TABLE IF NOT EXISTS sessions (
        token_hash TEXT PRIMARY KEY,
        github_id BIGINT NOT NULL REFERENCES github_users(github_id),
        expires_at TIMESTAMPTZ NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        revoked_at TIMESTAMPTZ
    )
"#;

pub struct SessionRepository<'a> {
    client: &'a Client,
}

impl<'a> SessionRepository<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn create_table(&self) -> Result<(), tokio_postgres::Error> {
        self.client.batch_execute(CREATE_TABLE_SQL).await
    }

    pub async fn create(
        &self,
        token_hash: &str,
        github_id: i64,
        ttl_seconds: i64,
    ) -> Result<(), tokio_postgres::Error> {
        self.client
            .execute(
                "INSERT INTO sessions (token_hash, github_id, expires_at) VALUES ($1, $2, NOW() + ($3::double precision * INTERVAL '1 second'))",
                &[&token_hash, &github_id, &(ttl_seconds as f64)],
            )
            .await?;
        Ok(())
    }

    pub async fn find_github_id(
        &self,
        token_hash: &str,
    ) -> Result<Option<i64>, tokio_postgres::Error> {
        let row = self
            .client
            .query_opt(
                "SELECT github_id FROM sessions WHERE token_hash = $1 AND revoked_at IS NULL AND expires_at > NOW()",
                &[&token_hash],
            )
            .await?;

        row.map(|row| row.try_get("github_id")).transpose()
    }

    pub async fn revoke(&self, token_hash: &str) -> Result<(), tokio_postgres::Error> {
        self.client
            .execute(
                "UPDATE sessions SET revoked_at = NOW() WHERE token_hash = $1 AND revoked_at IS NULL",
                &[&token_hash],
            )
            .await?;
        Ok(())
    }
}
