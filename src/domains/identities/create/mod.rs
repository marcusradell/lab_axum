use self::repo::CreateRepo;
use crate::{io::jwt::Jwt, result::Result};
use serde::{Deserialize, Serialize};

use super::events::CreatedEvent;

mod repo;

#[derive(Deserialize)]
pub struct Input {
    pub email: String,
}

#[derive(Serialize)]
pub struct Output {
    pub token: String,
}

pub async fn handler(repo: &impl CreateRepo, jwt: &Jwt, event: CreatedEvent) -> Result<Output> {
    repo.create(&event).await?;
    let token = jwt.encode(&event.id, &event.role)?;
    Ok(Output { token })
}
