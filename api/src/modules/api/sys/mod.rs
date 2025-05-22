use axum::{
    debug_handler,
    extract::Request,
    middleware,
    routing::{get, post},
    Json, Router,
};
use saes_shared::structs::user::Driver;

use crate::modules::api::utils::middle::{sysadmin_auth, ucp_auth};

mod config_api;
mod factions;
mod image;

#[debug_handler]
pub async fn sys_home(mut request: Request) -> Json<Driver> {
    let exts: Option<&Driver> = request.extensions_mut().get();
    Json(exts.unwrap().clone())
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(sys_home))
        .route("/config/get", get(config_api::sys_get_configs))
        .route(
            "/config/global-post",
            post(config_api::sys_post_global_config),
        )
        .route(
            "/config/faction-post",
            post(config_api::sys_post_faction_config),
        )
        .route(
            "/factions/change_image",
            post(image::sys_change_faction_image),
        )
        .route("/getfactions", get(factions::get_all_factions))
        .layer(middleware::from_fn(sysadmin_auth))
        .layer(middleware::from_fn(ucp_auth))
}
