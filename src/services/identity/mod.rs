use self::{events::CreatedEvent, sign_in::Output};
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
    response::ErrorResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
pub use role::Role;
use serde_json::json;
use uuid::Uuid;

pub mod events;
mod list;
mod role;
mod sign_in;
mod verify;

#[derive(Clone)]
pub struct Service {
    repo: Repo,
    jwt: Jwt,
}

impl Service {
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

    async fn ensure_owner(&self, email: String) -> Result<()> {
        let identity_repo_is_empty = list::handler(&self.repo).await?.is_empty();

        if identity_repo_is_empty {
            sign_in::handler(
                &self.repo,
                &self.jwt,
                events::CreatedEvent::new(
                    Uuid::new_v4(),
                    email,
                    Role::Owner,
                    Utc::now(),
                    Uuid::new_v4(),
                ),
            )
            .await?;
        }

        Ok(())
    }

    pub async fn sign_in(&self, created_event: CreatedEvent) -> Result<sign_in::Output> {
        sign_in::handler(&self.repo, &self.jwt, created_event).await
    }

    pub async fn verify(
        &self,
        created_event: CreatedEvent,
        code: String,
    ) -> Result<verify::Output> {
        verify::handler(&self.repo, &self.jwt, created_event, code).await
    }

    pub async fn list(&self) -> Result<Vec<CreatedEvent>> {
        list::handler(&self.repo).await
    }
}

pub fn new_routes(service: &Service) -> Router {
    let router: Router = Router::new();

    router
        .route(
            "/sign_in",
            post({
                let service = service.clone();

                |Json(input): Json<sign_in::Input>| async move {
                    let output = service
                        .sign_in(events::CreatedEvent::new(
                            Uuid::new_v4(),
                            input.email,
                            Role::Owner,
                            Utc::now(),
                            Uuid::new_v4(),
                        ))
                        .await
                        .map_err(|e| {
                            tracing::error!(e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?;

                    Ok::<Json<Output>, ErrorResponse>(Json(output))
                }
            }),
        )
        .route(
            "/verify",
            post({
                let service = service.clone();

                |Json(input): Json<verify::Input>| async move {
                    let output = service
                        .verify(
                            events::CreatedEvent::new(
                                Uuid::new_v4(),
                                input.email,
                                Role::Owner,
                                Utc::now(),
                                Uuid::new_v4(),
                            ),
                            input.code,
                        )
                        .await
                        .map_err(|e| {
                            tracing::error!(e);
                            StatusCode::INTERNAL_SERVER_ERROR
                        })?;

                    Ok::<Json<verify::Output>, ErrorResponse>(Json(output))
                }
            }),
        )
        .route(
            "/list",
            get({
                let service = service.clone();

                || async move { Json(service.list().await.expect("Failed to list identities.")) }
            }),
        )
}
