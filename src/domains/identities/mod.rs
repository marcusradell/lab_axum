use self::repo::Repo;
use crate::{
    io::{env, jwt::Jwt},
    result::Result,
};
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
pub use role::Role;
use serde_json::json;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

mod create;
mod events;
mod list;
mod repo;
mod role;

#[derive(Clone)]
pub struct IdentityDomain {
    repo: Repo,
    jwt: Jwt,
}

impl IdentityDomain {
    pub fn init(db: PgPool) -> (Router, Self) {
        let router = Router::new();
        let me = Self {
            repo: Repo::new(db),
            jwt: Jwt::new(&env::expect_env("JWT_SECRET")),
        };

        (me.add_routes(router), me)
    }

    pub async fn ensure_owner(&self, email: String) -> Result<()> {
        let identities_repo_is_empty = list::handler(&self.repo).await?.is_empty();

        if identities_repo_is_empty {
            create::handler(
                &self.repo,
                &self.jwt,
                events::CreatedEvent {
                    id: Uuid::new_v4().to_string(),
                    email,
                    role: Role::Owner,
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
                "/create_member",
                post({
                    let shared_self = Arc::clone(&shared_self);

                    |Json(input): Json<create::Input>| async move {
                        let output = create::handler(
                            &shared_self.repo,
                            &shared_self.jwt,
                            events::CreatedEvent {
                                id: Uuid::new_v4().to_string(),
                                email: input.email,
                                role: Role::Member,
                            },
                        )
                        .await
                        .map_err(|e| {
                            tracing::error!(e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })
                        .expect("Failed to create identity.");

                        Json(json!(output))
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
                            .expect("Failed to list identities.")))
                    }
                }),
            )
    }
}
