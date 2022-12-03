use self::{
    create::{CreateArgs, CreateData},
    repo::Repo,
};
use axum::{
    routing::{get, post},
    Json, Router,
};
use create::create;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

mod create;
mod list;
mod repo;

#[derive(Clone)]
pub struct IdentityDomain {
    repo: Repo,
}

impl IdentityDomain {
    pub fn new_with_routes(db: PgPool) -> (Router, Self) {
        let router = Router::new();
        let me = Self {
            repo: Repo::new(db),
        };
        let router = me.add_routes(router);

        (router, me)
    }

    fn add_routes(&self, router: Router) -> Router {
        let shared_self = Arc::new(self.clone());

        router
            .route(
                "/create",
                post({
                    let shared_self = Arc::clone(&shared_self);

                    |Json(args): Json<CreateArgs>| async move {
                        create(
                            &shared_self.repo,
                            CreateData {
                                _id: Uuid::new_v4().to_string(),
                                _email: args.email,
                            },
                        )
                        .await
                    }
                }),
            )
            .route(
                "/list",
                get({
                    let shared_self = Arc::clone(&shared_self);

                    || async move { list::handler(&shared_self.repo).await }
                }),
            )
    }
}
