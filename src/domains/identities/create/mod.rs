use self::repo::CreateRepo;
use serde::{Deserialize, Serialize};

mod repo;

pub const CREATED: &str = "IDENTITIES/CREATED";

#[derive(Deserialize)]
pub struct Input {
    pub email: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Event {
    pub id: String,
    pub email: String,
}

pub async fn handler(repo: &impl CreateRepo, event: Event) {
    repo.create(event).await
}
