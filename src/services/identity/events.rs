use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::role::Role;

pub const CREATED_EVENT: &str = "identity/created";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedEvent {
    pub id: String,
    pub email: String,
    pub role: Role,
    pub cid: String,
    pub r#type: String,
    pub version: u32,
}

impl CreatedEvent {
    pub fn new(id: Uuid, email: String, role: Role, cid: Uuid) -> Self {
        Self {
            id: id.to_string(),
            email,
            role,
            cid: cid.to_string(),
            r#type: CREATED_EVENT.to_string(),
            version: 0,
        }
    }
}
