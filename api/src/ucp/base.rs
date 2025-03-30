use std::collections::HashMap;

use axum::{debug_handler, extract::Request, Json};
use http::HeaderMap;
use saes_shared::db::{legacy_names, prelude::LegacyNames};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    utils::{api::get_api_envs, middle::Driver},
    DB_CLIENT, WEB_CLIENT,
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

#[debug_handler]
pub async fn ucp_getlegacyusernames(h: HeaderMap) -> Json<UserArrs> {
    let ids_str = h.get("ids").unwrap().to_str().unwrap();
    let ids: Vec<i32> = serde_json::from_str(ids_str).unwrap();
    let db = DB_CLIENT.get().unwrap();
    let users = LegacyNames::find()
        .filter(legacy_names::Column::Id.is_in(ids))
        .all(db)
        .await
        .expect("Lekérés sikertelen");
    let mut arrs: UserArrs = HashMap::new();
    for user in users.iter() {
        arrs.insert(
            user.id.to_string(),
            UserArr {
                name: user.name.clone(),
            },
        );
    }
    info!("{:?}", arrs);
    Json(arrs)
}
