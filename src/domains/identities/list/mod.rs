use crate::result::Result;

use self::repo::ListRepo;

use super::events::CreatedEvent;

mod repo;

pub async fn handler(repo: &impl ListRepo) -> Result<Vec<CreatedEvent>> {
    repo.list().await
}
