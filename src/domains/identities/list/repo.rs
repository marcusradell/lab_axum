use crate::domains::identities::{create::Event, repo::Repo};
use crate::result::Result;
use async_trait::async_trait;
use sqlx::types::Json;
use sqlx::FromRow;

#[async_trait]
pub trait ListRepo {
    async fn list(&self) -> Result<Vec<Event>>;
}

#[derive(FromRow)]
struct Row {
    data: Json<Event>,
}

#[async_trait]
impl ListRepo for Repo {
    async fn list(&self) -> Result<Vec<Event>> {
        let rows = sqlx::query_as!(
            Row,
            r#"select data  as "data: Json<Event>"  from identities.events"#
        )
        .fetch_all(&self.db)
        .await?;

        let result: Vec<Event> = rows
            .iter()
            .map(|r| Event {
                id: r.data.id.to_string(),
                email: r.data.email.to_string(),
                role: r.data.role.clone(),
            })
            .collect();

        Ok(result)
    }
}
