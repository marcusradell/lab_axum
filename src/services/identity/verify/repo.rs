use crate::io::repo::Repo;
use crate::result::Result;
use crate::services::identity::events::CreatedEvent;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[async_trait]
pub trait CreateIdentityRepo {
    async fn create_identity(&self, created_event: &CreatedEvent) -> Result<()>;
}

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
impl CreateIdentityRepo for Repo {
    async fn create_identity(&self, created_event: &CreatedEvent) -> Result<()> {
        self.prisma_client
            .identity_event()
            .create(
                created_event.stream_id.clone(),
                created_event.version.try_into()?,
                created_event.event_type.clone(),
                serde_json::to_value(created_event)?,
                created_event.cid.to_string(),
                created_event.inserted_at.into(),
                vec![],
            )
            .exec()
            .await?;

        Ok(())
    }
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
