use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Role {
    Member,
    Owner,
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::Member => "MEMBER".to_string(),
            Role::Owner => "OWNER".to_string(),
        }
    }
}
