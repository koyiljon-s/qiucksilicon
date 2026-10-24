use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GithubUser {
    pub id: u64,
    pub login: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
}
