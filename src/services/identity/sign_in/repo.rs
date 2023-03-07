use crate::io::repo::Repo;
use crate::result::Result;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[async_trait]
pub trait CreateVerificationRepo {
    async fn create_verification(
        &self,
        id: Uuid,
        email: String,
        code: String,
        inserted_at: DateTime<Utc>,
    ) -> Result<()>;
}

#[async_trait]
impl CreateVerificationRepo for Repo {
    async fn create_verification(
        &self,
        id: Uuid,
        email: String,
        code: String,
        inserted_at: DateTime<Utc>,
    ) -> Result<()> {
        self.prisma_client
            .identity_verification()
            .create(id.to_string(), email, code, inserted_at.into(), vec![])
            .exec()
            .await?;

        Ok(())
    }
}
