use crate::io::repo::Repo;
use crate::result::Result;
use crate::services::identity::events::CreatedEvent;
use async_trait::async_trait;

#[async_trait]
pub trait CreateRepo {
    async fn sign_in(&self, created_event: &CreatedEvent) -> Result<()>;
}

#[async_trait]
impl CreateRepo for Repo {
    async fn sign_in(&self, created_event: &CreatedEvent) -> Result<()> {
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
