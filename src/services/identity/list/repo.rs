use crate::result::Result;
use crate::{io::repo::Repo, services::identity::events::CreatedEvent};
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait ListRepo {
    async fn list(&self) -> Result<Vec<CreatedEvent>>;
}

#[async_trait]
impl ListRepo for Repo {
    async fn list(&self) -> Result<Vec<CreatedEvent>> {
        let rows = self
            .prisma_client
            .identity_event()
            .find_many(vec![])
            .exec()
            .await?;

        let json: Vec<Value> = rows.iter().map(|r| r.data.clone()).collect();

        let deserialized = json
            .iter()
            .map(|value| serde_json::from_value::<CreatedEvent>(value.clone()))
            .collect::<std::result::Result<Vec<CreatedEvent>, serde_json::Error>>()?;

        Ok(deserialized)
    }
}
