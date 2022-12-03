use axum::Router;
use std::net::SocketAddr;

use crate::domains::identities::IdentityDomain;

mod domains;

#[tokio::main]
async fn main() {
    let router = Router::new();

    let (router, _identities) = IdentityDomain::new_with_routes(router);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
