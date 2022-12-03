use async_trait::async_trait;

use crate::domains::identities::repo::Repo;

use super::CreateData;

#[async_trait]
pub trait CreateRepo {
    async fn create(&self, data: CreateData);
}

#[async_trait]
impl CreateRepo for Repo {
    async fn create(&self, data: CreateData) {
        println!("Identity created! (Fake)");
        dbg!(data);
    }
}
