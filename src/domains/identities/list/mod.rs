use std::error::Error;

use self::repo::ListRepo;

use super::create::Event;

mod repo;

pub async fn handler(repo: &impl ListRepo) -> Result<Vec<Event>, Box<dyn Error>> {
    repo.list().await
}
