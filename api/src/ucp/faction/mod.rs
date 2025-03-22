use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::utils::middle::faction_auth;

mod base;
mod config;
mod logs;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(base::fm_home))
        .route("/get_by_id", get(base::get_images_by_id))
        .route("/config/get", get(config::get_shift_access))
        .route("/config/post", post(config::post_shift_access))
        .nest("/logs", logs::get_routes())
        .layer(middleware::from_fn(faction_auth))
}
