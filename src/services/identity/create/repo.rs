use crate::io::repo::Repo;
use crate::result::Result;
use crate::services::identity::events::CreatedEvent;
use async_trait::async_trait;
use chrono::{DateTime, FixedOffset, Utc};
use uuid::Uuid;

#[async_trait]
pub trait CreateRepo {
    async fn create(&self, event: &CreatedEvent) -> Result<()>;
}

#[async_trait]
impl CreateRepo for Repo {
    async fn create(&self, event: &CreatedEvent) -> Result<()> {
        let id = Uuid::new_v4();
        let cid = Uuid::new_v4();
        let inserted_at: DateTime<FixedOffset> = Utc::now().into();
        // let event = Json(event);
        let version = 0;

        self.prisma_client
            .identity_event()
            .create(
                id.to_string(),
                version,
                "identity/create".to_string(),
                serde_json::to_value(event)?,
                cid.to_string(),
                inserted_at,
                vec![],
            )
            .exec()
            .await?;

        Ok(())
    }
}
