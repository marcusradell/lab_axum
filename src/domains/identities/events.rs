use serde::{Deserialize, Serialize};

use super::role::Role;

pub const CREATED_EVENT: &str = "IDENTITIES/CREATED";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatedEvent {
    pub id: String,
    pub email: String,
    pub role: Role,
}
