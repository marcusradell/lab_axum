use self::repo::CreateRepo;
use crate::result::Result;
use serde::{Deserialize, Serialize};

use super::role::Role;

mod repo;

pub const EVENT: &str = "IDENTITIES/CREATED";

#[derive(Deserialize)]
pub struct Input {
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub email: String,
    pub role: Role,
}

pub async fn handler(repo: &impl CreateRepo, event: Event) -> Result<()> {
    repo.create(event).await?;
    Ok(())
}
