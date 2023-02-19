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

pub async fn handler(
    repo: &impl CreateIdentityRepo,
    jwt: &Jwt,
    event: CreatedEvent,
    code: String,
) -> Result<Output> {
    repo.create_identity(&event).await?;
    let token = jwt.encode(&event.stream_id, &event.role)?;
    Ok(Output { token })
}
