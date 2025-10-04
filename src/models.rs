use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct EventType {
    type_: String,
    actor: Actor,
    repo: Repo,
    created_at: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Actor {
    login: String,
    url: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repo {
    name: String,
    url: String
}
