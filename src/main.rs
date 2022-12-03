use axum::{
    extract::Path,
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::{net::SocketAddr, sync::Arc};

#[derive(Debug)]
struct AppState {}

impl AppState {
    fn create(&self) {
        dbg!(self);
    }
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(AppState {});

    let app = Router::new()
        .route(
            "/users",
            post({
                let shared_state = Arc::clone(&shared_state);
                move |body| create_user(body, shared_state)
            }),
        )
        .route(
            "/users/:id",
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

async fn get_user(Path(user_id): Path<String>, state: Arc<AppState>) {}

async fn create_user(Json(payload): Json<CreateUserPayload>, state: Arc<AppState>) {
    state.create()
}

#[derive(Deserialize)]
struct CreateUserPayload {}
