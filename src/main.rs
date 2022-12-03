use crate::domains::identities::IdentityDomain;
use axum::Router;
use dotenv::dotenv;
use ensure_env::ensure_env;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

mod domains;
mod ensure_env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_uri = ensure_env("DATABASE_URL");

    let db = PgPoolOptions::new()
        .connect(&db_uri)
        .await
        .expect("Failed to create database connection pool.");

    let router = Router::new();

    let (identities_router, _identities) = IdentityDomain::new_with_routes(db.clone());
    let router = router.nest("/identities", identities_router);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
