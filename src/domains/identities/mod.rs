use std::sync::Arc;

use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
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
        // self.db.insert(0, data)
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
        let shared_state = Arc::new((*self).clone());

        router
            .route(
                "/identities/create",
                post({
                    let shared_state = Arc::clone(&shared_state);
                    move |body| create_user(body, shared_state)
                }),
            )
            .route(
                // TODO: use query params instead of path.
                "/identities/get/:id",
                get({
                    let shared_state = Arc::clone(&shared_state);
                    move |path| get_user(path, shared_state)
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

pub async fn get_user(Path(_user_id): Path<String>, _state: Arc<IdentityDomain>) {}
