use self::repo::CreateRepo;
use crate::result::Result;
use serde::Deserialize;

use super::events::CreatedEvent;

mod repo;

#[derive(Deserialize)]
pub struct Input {
    pub email: String,
}

pub async fn handler(repo: &impl CreateRepo, event: CreatedEvent) -> Result<()> {
    repo.create(event).await?;
    Ok(())
}
