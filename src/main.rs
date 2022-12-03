use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::{net::SocketAddr, sync::Arc};

#[derive(Debug)]
struct IdentityData {
    id: String,
    email: String,
}

struct Repo {
    db: Vec<IdentityData>,
}

impl Repo {
    fn new() -> Self {
        Self { db: vec![] }
    }

    async fn create(&self, data: IdentityData) {
        // self.db.insert(0, data)
        println!("Identity created! (Fake)");
        dbg!(data);
    }
}

struct IdentityDomain {
    repo: Repo,
}

impl IdentityDomain {
    fn new() -> Self {
        Self { repo: Repo::new() }
    }

    async fn create(&self, data: IdentityData) {
        self.repo.create(data).await
    }
}

#[tokio::main]
async fn main() {
    let mut shared_state = Arc::new(IdentityDomain::new());

    let app = Router::new()
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
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_user(Path(user_id): Path<String>, state: Arc<IdentityDomain>) {}

async fn create_user(Json(payload): Json<CreateUserPayload>, state: Arc<IdentityDomain>) {
    state
        .create(IdentityData {
            id: "Some UUID".to_string(),
            email: payload.email,
        })
        .await
}

#[derive(Deserialize)]
struct CreateUserPayload {
    email: String,
}
