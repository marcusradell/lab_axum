use self::repo::CreateVerificationRepo;
use crate::{io::jwt::Jwt, result::Result};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

mod repo;

#[derive(Deserialize)]
pub struct Input {
    pub email: String,
}

pub async fn handler(repo: &impl CreateVerificationRepo, jwt: &Jwt, email: String) -> Result<()> {
    let code = "abc123";
    repo.create_verification(Uuid::new_v4(), email, code.to_string(), Utc::now())
        .await?;

    Ok(())
}
