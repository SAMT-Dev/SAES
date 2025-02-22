use axum::{middleware, routing::get, Router};

use crate::{shorts, utils::middle::key_auth};

pub fn routes() -> Router {
    Router::new()
        .route("/shorts", get(shorts::base_get_shorts))
        .layer(middleware::from_fn(key_auth))
}
