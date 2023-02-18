use crate::{
    io::repo::Repo,
    services::identity::{self},
};
use axum::Router;
use dotenvy::dotenv;
use io::env::expect_env;
use prisma::PrismaClient;
use std::{error::Error, net::SocketAddr, sync::Arc};

mod io;
mod prisma;
mod result;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    tracing_subscriber::fmt().init();

    tracing::info!("Starting server...");

    let db_uri = expect_env("DATABASE_URL");

    let prisma_client = PrismaClient::_builder().with_url(db_uri).build().await?;

    let repo = Repo {
        prisma_client: Arc::new(prisma_client),
    };

    tracing::info!("DB pool created.");

    let identity_service = identity::Service::init(repo.clone()).await;

    let router = Router::new();
    let router = router.nest("/identity", identity::new_routes(&identity_service));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("Server listening on {}.", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
