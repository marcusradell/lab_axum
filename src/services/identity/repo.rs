use crate::result::Result;
use crate::{io::repo::Repo, prisma::identity_verification::email};
use async_trait::async_trait;

#[async_trait]
pub trait GetVerificationCodeRepo {
    async fn get_verification_code(&self, email: &str) -> Result<Option<String>>;
}

#[async_trait]
impl GetVerificationCodeRepo for Repo {
    async fn get_verification_code(&self, email: &str) -> Result<Option<String>> {
        let row = self
            .prisma_client
            .identity_verification()
            .find_first(vec![email::equals(email.to_string())])
            .exec()
            .await?;

        Ok(row.map(|r| r.code))
    }
}
