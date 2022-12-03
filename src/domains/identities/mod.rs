use self::repo::Repo;
use axum::{
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
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
    pub fn init(db: PgPool) -> Router {
        let router = Router::new();
        let me = Self {
            repo: Repo::new(db),
        };

        me.add_routes(router)
    }

    fn add_routes(&self, router: Router) -> Router {
        let shared_self = Arc::new(self.clone());

        router
            .route(
                "/create",
                post({
                    let shared_self = Arc::clone(&shared_self);

                    |Json(input): Json<create::Input>| async move {
                        create::handler(
                            &shared_self.repo,
                            create::Event {
                                id: Uuid::new_v4().to_string(),
                                email: input.email,
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

                    || async move {
                        Json(json!(list::handler(&shared_self.repo)
                            .await
                            .expect("Failed to list.")))
                    }
                }),
            )
    }
}
