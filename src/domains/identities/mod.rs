use self::repo::Repo;
use crate::result::Result;
use axum::{
    http::StatusCode,
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
    pub fn init(db: PgPool) -> (Router, Self) {
        let router = Router::new();
        let me = Self {
            repo: Repo::new(db),
        };

        (me.add_routes(router), me)
    }

    pub async fn ensure_owner(&self, email: String) -> Result<()> {
        let has_identities = list::handler(&self.repo).await?.is_empty();

        if !has_identities {
            create::handler(
                &self.repo,
                create::Event {
                    id: Uuid::new_v4().to_string(),
                    email,
                },
            )
            .await?;
        }

        Ok(())
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
                        .map_err(|e| {
                            tracing::error!(e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })
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
