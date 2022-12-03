use sqlx::PgPool;

#[derive(Clone)]
pub struct Repo {
    // TODO: Replace with SQLx
    db: PgPool,
}

impl Repo {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}
