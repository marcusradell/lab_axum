use crate::{
    domains::{identities::IdentityDomain, jobs::JobsDomain},
    io::repo::Repo,
};
use axum::Router;
use dotenvy::dotenv;
use io::env::expect_env;
use prisma::PrismaClient;
use std::{error::Error, net::SocketAddr, sync::Arc};

mod domains;
mod io;
mod prisma;
mod result;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    tracing_subscriber::fmt().init();

    tracing::info!("Starting server...");

    let db_uri = expect_env("DATABASE_URL");

    let prismaClient = PrismaClient::_builder().with_url(db_uri).build().await?;

    let repo = Repo {
        prismaClient: Arc::new(prismaClient),
    };

    tracing::info!("DB pool created.");

    let router = Router::new();

    let identities_router = IdentityDomain::init(repo.clone()).await;
    let router = router.nest("/identities", identities_router);

    let jobs_router = JobsDomain::init();
    let router = router.nest("/jobs", jobs_router);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("Server listening on {}.", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
