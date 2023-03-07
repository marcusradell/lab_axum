use self::repo::CreateVerificationRepo;
use super::repo::GetVerificationCodeRepo;
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

pub async fn handler(
    repo: &(impl CreateVerificationRepo + GetVerificationCodeRepo),
    email: String,
) -> Result<()> {
    let preexisting_code = repo.get_verification_code(&email).await?;

    let code = match preexisting_code {
        Some(code) => code,
        None => {
            let code = generate_verification_code();

            repo.create_verification(Uuid::new_v4(), email.clone(), code.to_string(), Utc::now())
                .await?;

            code
        }
    };

    println!("Use verification code {code} for email {email}.");

    Ok(())
}
