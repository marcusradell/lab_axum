use axum::{routing::post, Json, Router};
use create::create;
use std::sync::Arc;
use uuid::Uuid;

use self::create::{CreateArgs, CreateData};

mod create;

#[derive(Clone)]
pub struct Repo {
    // TODO: Replace with SQLx
    _db: (),
}

impl Repo {
    fn new() -> Self {
        Self { _db: () }
    }

    async fn create(&self, data: CreateData) {
        println!("Identity created! (Fake)");
        dbg!(data);
    }
}

#[derive(Clone)]
pub struct IdentityDomain {
    repo: Repo,
}

impl IdentityDomain {
    pub fn new_with_routes() -> (Router, Self) {
        let router = Router::new();
        let me = Self { repo: Repo::new() };
        let router = me.init_routes(router);

        (router, me)
    }

    fn init_routes(&self, router: Router) -> Router {
        let shared_self = Arc::new(self.clone());

        router.route(
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
    }
}
