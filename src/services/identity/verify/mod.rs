use std::io::{Error, ErrorKind};

use self::repo::CreateIdentityRepo;
use super::events::CreatedEvent;
use super::repo::GetVerificationCodeRepo;
use crate::{io::jwt::Jwt, result::Result};
use serde::{Deserialize, Serialize};

mod repo;

#[derive(Deserialize)]
pub struct Input {
    pub email: String,
    pub code: String,
}

#[derive(Serialize, Debug)]
pub struct Output {
    pub token: String,
}

pub async fn handler(
    repo: &(impl CreateIdentityRepo + GetVerificationCodeRepo),
    jwt: &Jwt,
    event: CreatedEvent,
    code: String,
) -> Result<Output> {
    let correct_code = repo
        .get_verification_code(&event.email)
        .await?
        .ok_or(Box::new(Error::new(
            ErrorKind::InvalidData,
            "Missing verification code.",
        )))?;

    if code != correct_code {
        return Err(Box::new(Error::new(ErrorKind::InvalidInput, "Wrong code")));
    }

    repo.create_identity(&event).await?;
    let token = jwt.encode(&event.stream_id, &event.role)?;

    Ok(Output { token })
}
