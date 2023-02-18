use crate::{
    domains::{
        identities::{self, IdentityDomain},
        jobs::JobsDomain,
    },
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

    let prisma_client = PrismaClient::_builder().with_url(db_uri).build().await?;

    let repo = Repo {
        prisma_client: Arc::new(prisma_client),
    };

    tracing::info!("DB pool created.");

    let identities_domain = IdentityDomain::init(repo.clone()).await;

    let jobs_router = JobsDomain::init();

    let router = Router::new();
    let router = router.nest("/identities", identities::new_routes(&identities_domain));
    let router = router.nest("/jobs", jobs_router);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("Server listening on {}.", addr);

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
