use crate::domains::identities::IdentityDomain;
use axum::Router;
use dotenvy::dotenv;
use expect_env::expect_env;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

mod domains;
mod expect_env;
mod result;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().init();

    tracing::info!("Starting server...");

    let db_uri = expect_env("DATABASE_URL");

    let db = PgPoolOptions::new()
        .connect(&db_uri)
        .await
        .expect("Failed to create DB pool.");

    tracing::info!("DB pool created.");

    let router = Router::new();

    let (identities_router, identities) = IdentityDomain::init(db.clone());

    identities
        .ensure_owner(expect_env("OWNER"))
        .await
        .expect("Failed to ensure owner identity.");

    let router = router.nest("/identities", identities_router);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("Server listening on {}.", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
