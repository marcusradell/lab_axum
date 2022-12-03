use axum::{
    extract::Path,
    routing::{get, post},
    Router,
};
use std::{net::SocketAddr, sync::Arc};

use crate::domains::identities::{create_user, IdentityDomain};

mod domains;

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(IdentityDomain::new());

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

async fn get_user(Path(_user_id): Path<String>, _state: Arc<IdentityDomain>) {}
