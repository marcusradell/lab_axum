use sqlx::{types::Json, PgPool};

use super::create::Event;

pub struct Row {
    pub data: Json<Event>,
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
