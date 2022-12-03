use async_trait::async_trait;
use chrono::Utc;
use sqlx::types::Json;
use uuid::Uuid;

use crate::domains::identities::repo::Repo;

use super::CreateData;

#[async_trait]
pub trait CreateRepo {
    async fn create(&self, data: CreateData);
}

pub const CREATED: &str = "IDENTITIES/CREATED";

#[async_trait]
impl CreateRepo for Repo {
    async fn create(&self, data: CreateData) {
        let id = Uuid::new_v4();
        let cid = Uuid::new_v4();
        let inserted_at = Utc::now();
        let data = Json(data);

        sqlx::query!(
            r#"
    insert into identities.events
    (stream_id, version, event_type, data, cid, inserted_at) VALUES
    ( $1, $2, $3, $4, $5, $6 )
    returning sequence_num
            "#,
            id,
            1,
            CREATED,
            data as _,
            cid,
            inserted_at
        )
        .fetch_one(&self.db)
        .await
        .expect("Failed to create identity.");
    }
}
