use std::convert::TryFrom;

use tokio_postgres::Client;

use crate::models::GithubUser;

const CREATE_TABLE_SQL: &str = r#"
    CREATE TABLE IF NOT EXISTS github_users (
        github_id BIGINT PRIMARY KEY,
        login TEXT NOT NULL,
        email TEXT,
        avatar_url TEXT,
        created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
        updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    )
"#;

const UPSERT_SQL: &str = r#"
    INSERT INTO github_users (github_id, login, email, avatar_url)
    VALUES ($1, $2, $3, $4)
    ON CONFLICT (github_id) DO UPDATE SET
        login = EXCLUDED.login,
        email = EXCLUDED.email,
        avatar_url = EXCLUDED.avatar_url,
        updated_at = NOW()
    RETURNING github_id, login, email, avatar_url
"#;

const FIND_BY_ID_SQL: &str = r#"
    SELECT github_id, login, email, avatar_url
    FROM github_users
    WHERE github_id = $1
"#;

pub struct GithubUserRepository<'a> {
    client: &'a Client,
}

impl<'a> GithubUserRepository<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn create_table(&self) -> Result<(), tokio_postgres::Error> {
        self.client.batch_execute(CREATE_TABLE_SQL).await
    }

    pub async fn upsert(
        &self,
        user: &GithubUser,
    ) -> Result<GithubUser, Box<dyn std::error::Error + Send + Sync>> {
        let github_id = i64::try_from(user.id)?;

        let row = self
            .client
            .query_one(
                UPSERT_SQL,
                &[&github_id, &user.login, &user.email, &user.avatar_url],
            )
            .await?;

        row_to_user(&row)
    }

    pub async fn find_by_github_id(
        &self,
        github_id: u64,
    ) -> Result<Option<GithubUser>, Box<dyn std::error::Error + Send + Sync>> {
        let github_id = i64::try_from(github_id)?;
        let row = self.client.query_opt(FIND_BY_ID_SQL, &[&github_id]).await?;

        row.map(|row| row_to_user(&row)).transpose()
    }
}

fn row_to_user(
    row: &tokio_postgres::Row,
) -> Result<GithubUser, Box<dyn std::error::Error + Send + Sync>> {
    let github_id: i64 = row.try_get("github_id")?;

    Ok(GithubUser {
        id: u64::try_from(github_id)?,
        login: row.try_get("login")?,
        email: row.try_get("email")?,
        avatar_url: row.try_get("avatar_url")?,
    })
}
