use std::io::{Error, ErrorKind};

use self::repo::CreateIdentityRepo;
use super::events::CreatedEvent;
use crate::{io::jwt::Jwt, result::Result};
use serde::{Deserialize, Serialize};

mod repo;

#[derive(Deserialize)]
pub struct Input {
    pub email: String,
    pub code: String,
}

#[derive(Serialize)]
pub struct Output {
    pub token: String,
}

const CORRECT_CODE: &str = "abc123";

pub async fn handler(
    repo: &impl CreateIdentityRepo,
    jwt: &Jwt,
    event: CreatedEvent,
    code: String,
) -> Result<Output> {
    // TODO: implement DB lookup to find the right code.
    let correct_code = CORRECT_CODE;

    if code != correct_code {
        return Err(Box::new(Error::new(ErrorKind::InvalidInput, "Wrong code")));
    }

    repo.create_identity(&event).await?;
    let token = jwt.encode(&event.stream_id, &event.role)?;
    Ok(Output { token })
}
