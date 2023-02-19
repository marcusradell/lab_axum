use std::error::Error;

use crate::prisma::identity_event;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::role::Role;

pub const CREATED_EVENT: &str = "identity/created";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedEvent {
    pub stream_id: String,
    pub email: String,
    pub role: Role,
    pub cid: String,
    pub event_type: String,
    pub version: u32,
    pub inserted_at: DateTime<Utc>,
}

impl CreatedEvent {
    pub fn new(id: Uuid, email: String, role: Role, inserted_at: DateTime<Utc>, cid: Uuid) -> Self {
        Self {
            stream_id: id.to_string(),
            email,
            role,
            cid: cid.to_string(),
            event_type: CREATED_EVENT.to_string(),
            version: 0,
            inserted_at,
        }
    }
}

impl TryFrom<identity_event::Data> for CreatedEvent {
    type Error = Box<dyn Error>;

    fn try_from(value: identity_event::Data) -> Result<Self, Self::Error> {
        Ok(serde_json::from_value::<CreatedEvent>(value.data)?)
    }
}
