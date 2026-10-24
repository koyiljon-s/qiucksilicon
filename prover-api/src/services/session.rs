use std::{convert::TryFrom, sync::Arc};

use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::repositories::SessionRepository;

pub const SESSION_COOKIE_NAME: &str = "session_id";
pub const SESSION_TTL_SECONDS: i64 = 60 * 60 * 24 * 7;

#[derive(Clone)]
pub struct SessionService {
    db: Arc<tokio_postgres::Client>,
    secure_cookie: bool,
}

impl SessionService {
    pub fn new(db: Arc<tokio_postgres::Client>, secure_cookie: bool) -> Self {
        Self { db, secure_cookie }
    }

    pub fn secure_cookie(&self) -> bool {
        self.secure_cookie
    }

    pub async fn create(&self, github_id: u64) -> Result<String, SessionServiceError> {
        let github_id = i64::try_from(github_id).map_err(|_| SessionServiceError::InvalidUserId)?;
        let token = Uuid::new_v4().to_string();
        let token_hash = hash_token(&token);

        SessionRepository::new(&self.db)
            .create(&token_hash, github_id, SESSION_TTL_SECONDS)
            .await
            .map_err(|error| SessionServiceError::Database(error.to_string()))?;

        Ok(token)
    }

    #[allow(dead_code)]
    pub async fn authenticate(&self, token: &str) -> Result<Option<u64>, SessionServiceError> {
        let github_id = SessionRepository::new(&self.db)
            .find_github_id(&hash_token(token))
            .await
            .map_err(|error| SessionServiceError::Database(error.to_string()))?;

        github_id
            .map(|id| u64::try_from(id).map_err(|_| SessionServiceError::InvalidUserId))
            .transpose()
    }

    pub async fn revoke(&self, token: &str) -> Result<(), SessionServiceError> {
        SessionRepository::new(&self.db)
            .revoke(&hash_token(token))
            .await
            .map_err(|error| SessionServiceError::Database(error.to_string()))
    }
}

fn hash_token(token: &str) -> String {
    Sha256::digest(token.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[derive(Debug)]
pub enum SessionServiceError {
    Database(String),
    InvalidUserId,
}
