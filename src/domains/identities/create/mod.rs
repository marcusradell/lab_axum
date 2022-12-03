use self::repo::CreateRepo;
use serde::{Deserialize, Serialize};

mod repo;

#[derive(Deserialize)]
pub struct CreateArgs {
    pub email: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateData {
    pub _id: String,
    pub _email: String,
}

pub async fn create(repo: &impl CreateRepo, data: CreateData) {
    repo.create(data).await
}
