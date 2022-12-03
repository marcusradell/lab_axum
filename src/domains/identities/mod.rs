use std::sync::Arc;

use axum::{routing::post, Json, Router};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct IdentityData {
    pub _id: String,
    pub _email: String,
}

#[derive(Clone)]
struct Repo {
    _db: Vec<IdentityData>,
}

impl Repo {
    fn new() -> Self {
        Self { _db: vec![] }
    }

    async fn create(&self, data: IdentityData) {
        println!("Identity created! (Fake)");
        dbg!(data);
    }
}

#[derive(Clone)]
pub struct IdentityDomain {
    repo: Repo,
}

impl IdentityDomain {
    pub fn new() -> Self {
        Self { repo: Repo::new() }
    }

    pub fn init_routes(&self, router: Router) -> Router {
        let shared_self = Arc::new(self.clone());

        router.route(
            "/identities/create",
            post({
                let shared_state = Arc::clone(&shared_self);
                move |body| create_user(body, shared_state)
            }),
        )
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
