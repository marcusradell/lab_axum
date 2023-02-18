use crate::io::repo::Repo;
use crate::result::Result;
use crate::services::identity::events::CreatedEvent;
use async_trait::async_trait;
use chrono::{DateTime, FixedOffset, Utc};

#[async_trait]
pub trait CreateRepo {
    async fn sign_in(&self, created_event: &CreatedEvent) -> Result<()>;
}

#[async_trait]
impl CreateRepo for Repo {
    async fn sign_in(&self, created_event: &CreatedEvent) -> Result<()> {
        let inserted_at: DateTime<FixedOffset> = Utc::now().into();
        let version = 0;

        self.prisma_client
            .identity_event()
            .create(
                created_event.id.to_string(),
                version,
                "identity/create".to_string(),
                serde_json::to_value(created_event)?,
                created_event.cid.to_string(),
                inserted_at,
                vec![],
            )
            .exec()
            .await?;

        Ok(())
    }
}
