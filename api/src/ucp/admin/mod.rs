use std::collections::HashMap;

use axum::{
    debug_handler, middleware,
    routing::{get, post},
    Json, Router,
};
use http::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::{
    utils::{api::get_api_envs, middle::admin_auth},
    WEB_CLIENT,
};

use super::{faction, shift};

mod base;
mod home;
mod items;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserArr {
    name: String,
}

pub type UserArrs = HashMap<String, UserArr>;

pub fn routes() -> Router {
    Router::new()
        .nest("/shift", shift::routes())
        .nest("/faction", faction::routes())
        .route("/", get(base::admin_home))
        .route("/getusernames", get(admin_getusernames))
        .route("/items/get", get(items::admin_items_get))
        .route("/items/post", post(items::admin_items_post))
        .route("/items/home", get(home::admin_home_stat))
        .layer(middleware::from_fn(admin_auth))
}

#[debug_handler]
pub async fn admin_getusernames(h: HeaderMap) -> Json<UserArrs> {
    let apis = get_api_envs().await;
    let ids_str = h.get("ids").unwrap().to_str().unwrap();
    let ids: Vec<i32> = serde_json::from_str(ids_str).unwrap();
    let req = WEB_CLIENT
        .get(format!(
            "{}/saes/user/arr?ids={}",
            apis.fms,
            ids.iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(",")
        ))
        .header("authkey", apis.fms_key)
        .send()
        .await
        .unwrap();
    let res = req.json().await.unwrap();
    Json(res)
}
