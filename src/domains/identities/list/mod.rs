use self::repo::ListRepo;

mod repo;

pub async fn handler(repo: &impl ListRepo) {
    repo.list().await
}
