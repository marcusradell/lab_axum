use std::sync::Arc;

use axum::{routing::post, Json, Router};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct CreateData {
    pub _id: String,
    pub _email: String,
}

#[derive(Clone)]
struct Repo {
    // TODO: Replace with SQLx
    _db: Vec<CreateData>,
}

impl Repo {
    fn new() -> Self {
        Self { _db: vec![] }
    }

    async fn create(&self, data: CreateData) {
        println!("Identity created! (Fake)");
        dbg!(data);
    }
}

#[derive(Clone)]
pub struct IdentityDomain {
    repo: Repo,
}

impl IdentityDomain {
    pub fn new_with_routes() -> (Router, Self) {
        let router = Router::new();
        let me = Self { repo: Repo::new() };
        let router = me.init_routes(router);

        (router, me)
    }

    fn init_routes(&self, router: Router) -> Router {
        let shared_self = Arc::new(self.clone());

        router.route(
            "/create",
            post({
                let shared_self = Arc::clone(&shared_self);
                |Json(payload): Json<CreateArgs>| async move {
                    shared_self
                        .create(CreateData {
                            _id: Uuid::new_v4().to_string(),
                            _email: payload.email,
                        })
                        .await
                }
            }),
        )
    }

    pub async fn create(&self, data: CreateData) {
        self.repo.create(data).await
    }
}

#[derive(Deserialize)]
pub struct CreateArgs {
    email: String,
}
