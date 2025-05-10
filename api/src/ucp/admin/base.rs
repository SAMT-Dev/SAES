use std::collections::HashMap;

use axum::{debug_handler, extract::Request, response::IntoResponse, Json};
use saes_shared::structs::user::Driver;
use serde::Serialize;

use crate::config::loader::get_config;

#[debug_handler]
pub async fn admin_home(mut request: Request) -> Json<Driver> {
    let exts: Option<&Driver> = request.extensions_mut().get();
    Json(exts.unwrap().clone())
}

#[derive(Debug, Serialize)]
struct GetFactionsRet {
    name: String,
    id: i8,
    perm_name: String,
}

#[debug_handler]
pub async fn get_factions() -> impl IntoResponse {
    let config = get_config().await;
    let mut ret = HashMap::new();
    config.factions.iter().for_each(|f| {
        ret.insert(
            f.0.to_owned(),
            GetFactionsRet {
                name: f.1.settings.display.clone(),
                perm_name: f.1.settings.perm_name.clone(),
                id: f.1.settings.id,
            },
        );
    });
    Json(ret)
}
