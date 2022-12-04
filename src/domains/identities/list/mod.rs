use crate::result::Result;

use self::repo::ListRepo;

use super::create;

mod repo;

pub async fn handler(repo: &impl ListRepo) -> Result<Vec<create::Event>> {
    repo.list().await
}
