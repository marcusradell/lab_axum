use std::error::Error;

use crate::io::repo::Repo;
use crate::prisma::identity_verification::email;
use crate::result::Result;
use crate::services::identity::events::CreatedEvent;
use async_trait::async_trait;

#[async_trait]
pub trait GetVerificationCodeRepo {
    async fn get_verification_code(&self, email: &str) -> Result<Option<String>>;
}

#[async_trait]
impl GetVerificationCodeRepo for Repo {
    async fn get_verification_code(&self, email: &str) -> Result<Option<String>> {
        let row = self
            .prisma_client
            .identity_verification()
            .find_first(vec![email::equals(email.to_string())])
            .exec()
            .await?;

        Ok::<_, Box<dyn Error>>(row.map(|r| r.code))
    }
}

#[async_trait]
pub trait CreateIdentityRepo {
    async fn create_identity(&self, created_event: &CreatedEvent) -> Result<()>;
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
