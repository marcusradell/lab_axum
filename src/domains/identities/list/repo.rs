use crate::domains::identities::repo::Repo;
use async_trait::async_trait;

#[async_trait]
pub trait ListRepo {
    async fn list(&self);
}

#[async_trait]
impl ListRepo for Repo {
    async fn list(&self) {
        let result = sqlx::query!(r#"select data from identities.events"#)
            .fetch_all(&self.db)
            .await
            .expect("Failed to create identity.");

        println!("{result:?}");
    }
}
