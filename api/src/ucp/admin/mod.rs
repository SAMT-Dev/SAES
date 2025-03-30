use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::utils::middle::admin_auth;

use super::{faction, shift};

mod base;
mod home;
mod items;

pub fn routes() -> Router {
    Router::new()
        .nest("/shift", shift::routes())
        .nest("/faction", faction::routes())
        .route("/", get(base::admin_home))
        .route("/items/get", get(items::admin_items_get))
        .route("/items/post", post(items::admin_items_post))
        .route("/items/home", get(home::admin_home_stat))
        .layer(middleware::from_fn(admin_auth))
}
