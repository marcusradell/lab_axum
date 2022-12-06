use axum::{routing::get, Json, Router};
use serde_json::json;

use crate::io::jwt::Jwt;

pub struct JobsDomain {}

impl JobsDomain {
    pub fn init() -> Router {
        Router::new().route("/list", get(|| async { Json(json!(vec![1])) }))
    }
}
