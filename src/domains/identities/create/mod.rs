use serde::Deserialize;

use super::Repo;

#[derive(Deserialize)]
pub struct CreateArgs {
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct CreateData {
    pub _id: String,
    pub _email: String,
}

pub async fn create(repo: &Repo, data: CreateData) {
    repo.create(data).await
}
