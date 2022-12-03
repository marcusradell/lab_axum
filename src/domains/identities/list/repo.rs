use std::error::Error;

use crate::domains::identities::{create::Event, repo::Repo};
use async_trait::async_trait;
use sqlx::types::Json;

#[async_trait]
pub trait ListRepo {
    async fn list(&self) -> Result<Vec<Event>, Box<dyn Error>>;
}

struct Row {
    data: Json<Event>,
}

#[async_trait]
impl ListRepo for Repo {
    async fn list(&self) -> Result<Vec<Event>, Box<dyn Error>> {
        let rows: Vec<Row> = sqlx::query_as!(
            Row,
            r#"select data  as "data: Json<Event>"  from identities.events"#
        )
        .fetch_all(&self.db)
        .await
        .expect("Failed to create identity.");

        let result: Vec<Event> = rows
            .iter()
            .map(|r| Event {
                id: r.data.id.to_string(),
                email: r.data.email.to_string(),
            })
            .collect();

        Ok(result)
    }
}
