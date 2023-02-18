use self::repo::ListRepo;
use super::events::CreatedEvent;
use crate::result::Result;

mod repo;

pub async fn handler(repo: &impl ListRepo) -> Result<Vec<CreatedEvent>> {
    repo.list().await
}
