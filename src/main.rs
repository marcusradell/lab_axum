use crate::domains::{identities::IdentityDomain, jobs::JobsDomain};
use axum::Router;
use dotenvy::dotenv;
use io::env::expect_env;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

mod domains;
mod io;
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

    let (identities_router) = IdentityDomain::init(db.clone()).await;
    let router = router.nest("/identities", identities_router);

    let jobs_router = JobsDomain::init();
    let router = router.nest("/jobs", jobs_router);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("Server listening on {}.", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
