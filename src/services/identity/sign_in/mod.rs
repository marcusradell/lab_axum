use self::repo::CreateVerificationRepo;
use crate::result::Result;
use chrono::Utc;
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;
use uuid::Uuid;

mod repo;

#[derive(Deserialize)]
pub struct Input {
    pub email: String,
}

fn generate_verification_code() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()
}

pub async fn handler(repo: &impl CreateVerificationRepo, email: String) -> Result<()> {
    let code = generate_verification_code();
    repo.create_verification(Uuid::new_v4(), email, code.to_string(), Utc::now())
        .await?;

    Ok(())
}
