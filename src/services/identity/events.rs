use serde::{Deserialize, Serialize};

use super::role::Role;

pub const _CREATED_EVENT: &str = "identity/created";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedEvent {
    pub id: String,
    pub email: String,
    pub role: Role,
}
