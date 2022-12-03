use std::sync::Arc;

use axum::Json;
use serde::Deserialize;

#[derive(Debug)]
pub struct IdentityData {
    pub _id: String,
    pub _email: String,
}

struct Repo {
    _db: Vec<IdentityData>,
}

impl Repo {
    fn new() -> Self {
        Self { _db: vec![] }
    }

    async fn create(&self, data: IdentityData) {
        // self.db.insert(0, data)
        println!("Identity created! (Fake)");
        dbg!(data);
    }
}

pub struct IdentityDomain {
    repo: Repo,
}

impl IdentityDomain {
    pub fn new() -> Self {
        Self { repo: Repo::new() }
    }

    pub async fn create(&self, data: IdentityData) {
        self.repo.create(data).await
    }
}

pub async fn create_user(Json(payload): Json<CreateUserPayload>, state: Arc<IdentityDomain>) {
    state
        .create(IdentityData {
            _id: "Some UUID".to_string(),
            _email: payload.email,
        })
        .await
}

#[derive(Deserialize)]
pub struct CreateUserPayload {
    email: String,
}
