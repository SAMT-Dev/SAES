use axum::{middleware, routing::get, Router};

use crate::modules::api::utils::middle::shift_auth;

mod base;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(base::sm_home))
        .route("/stat", get(base::sm_stat))
        .layer(middleware::from_fn(shift_auth))
}
