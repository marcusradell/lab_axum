use crate::domains::identities::{
    events::CreatedEvent,
    repo::{Repo, Row},
};
use crate::result::Result;
use async_trait::async_trait;
use sqlx::types::Json;

#[async_trait]
pub trait ListRepo {
    async fn list(&self) -> Result<Vec<CreatedEvent>>;
}

#[async_trait]
impl ListRepo for Repo {
    async fn list(&self) -> Result<Vec<CreatedEvent>> {
        let rows: Vec<Row> = sqlx::query_as!(
            Row,
            r#"select data  as "data: Json<CreatedEvent>"  from identities.events"#
        )
        .fetch_all(&self.db)
        .await?;

        let result: Vec<CreatedEvent> = rows
            .iter()
            .map(|r| CreatedEvent {
                id: r.data.id.to_string(),
                email: r.data.email.to_string(),
                role: r.data.role.clone(),
            })
            .collect();

        Ok(result)
    }
}
