use std::collections::HashMap;

use axum::{debug_handler, extract::Request, Json};
use http::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::{
    utils::{api::get_api_envs, middle::Driver},
    WEB_CLIENT,
};

#[debug_handler]
pub async fn ucp_home(mut request: Request) -> Json<Driver> {
    let exts: Option<&Driver> = request.extensions_mut().get();
    Json(exts.unwrap().clone())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserArr {
    name: String,
}

pub type UserArrs = HashMap<String, UserArr>;

#[debug_handler]
pub async fn ucp_getusernames(h: HeaderMap) -> Json<UserArrs> {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct UserId {
    userid: i32,
}

#[debug_handler]
pub async fn ucp_getuserid(h: HeaderMap) -> Json<UserId> {
    let apis = get_api_envs().await;
    let username = h.get("username").unwrap().to_str().unwrap();
    let req = WEB_CLIENT
        .get(format!(
            "{}/saes/user/getid?username={}",
            apis.fms, username
        ))
        .header("authkey", apis.fms_key)
        .send()
        .await
        .unwrap();
    let res = req.json().await.unwrap();
    Json(res)
}
