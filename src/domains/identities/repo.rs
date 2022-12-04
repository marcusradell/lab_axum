use super::events::CreatedEvent;
use sqlx::{types::Json, PgPool};

pub struct Row {
    pub data: Json<CreatedEvent>,
}

#[derive(Clone)]
pub struct Repo {
    // TODO: Replace with SQLx
    pub db: PgPool,
}

impl Repo {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}
