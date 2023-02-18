use crate::prisma::PrismaClient;
use std::sync::Arc;

#[derive(Clone)]
pub struct Repo {
    pub prisma_client: Arc<PrismaClient>,
}
