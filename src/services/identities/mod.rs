use crate::{
    io::{
        env::{self, expect_env},
        jwt::Jwt,
        repo::Repo,
    },
    result::Result,
};
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
pub use role::Role;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

mod create;
mod events;
mod list;
mod role;

#[derive(Clone)]
pub struct IdentityDomain {
    repo: Repo,
    jwt: Jwt,
}

impl IdentityDomain {
    pub async fn init(repo: Repo) -> Self {
        let me = Self {
            repo,
            jwt: Jwt::new(&env::expect_env("JWT_SECRET")),
        };

        me.ensure_owner(expect_env("OWNER"))
            .await
            .expect("Failed to ensure owner identity.");

        me
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
}

pub fn new_routes(identity_domain: &IdentityDomain) -> Router {
    let identity_domain = Arc::new(identity_domain.clone());

    let router: Router = Router::new();

    router
        .route(
            "/create_member",
            post({
                let shared_self = Arc::clone(&identity_domain);

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
                let shared_self = Arc::clone(&identity_domain);

                || async move {
                    Json(json!(list::handler(&shared_self.repo)
                        .await
                        .expect("Failed to list identities.")))
                }
            }),
        )
}
