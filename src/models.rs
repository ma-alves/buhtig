use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Actor {
    pub id: u64,
    pub login: String,
    pub display_login: String,
    pub gravatar_id: String,
    pub url: String,
    pub avatar_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub actor: Actor,
    pub repo: Repo,
    pub payload: serde_json::Value,
    pub public: bool,
    pub created_at: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repo {
    pub id: u64,
    pub name: String,
    pub url: String
}
