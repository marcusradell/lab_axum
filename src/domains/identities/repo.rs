use sqlx::PgPool;

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
