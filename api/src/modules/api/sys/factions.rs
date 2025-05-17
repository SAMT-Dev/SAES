use std::collections::HashMap;

use axum::{debug_handler, response::IntoResponse, Json};
use http::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{config::loader::get_config, modules::api::utils::api::get_api_envs, WEB_CLIENT};

#[derive(Debug, Deserialize)]
struct GetCompanies {
    archived: bool,
    name: String,
    shortname: String,
}

#[derive(Debug, Serialize)]
struct GetFactionRet {
    name: String,
    shortname: String,
    archived: bool,
    managed: bool,
    icon: Option<i32>,
}

#[debug_handler]
pub async fn get_all_factions() -> Result<impl IntoResponse, (StatusCode, String)> {
    let envs = get_api_envs().await;
    let get = WEB_CLIENT
        .get(format!("{}/saes/company?type=ALL", envs.fms))
        .header("authkey", envs.fms_key)
        .send()
        .await;
    if get.is_err() {
        return Err((StatusCode::BAD_GATEWAY, get.unwrap_err().to_string()));
    }
    let res: HashMap<String, GetCompanies> = get.unwrap().json().await.unwrap();
    let config = get_config().await;
    let mut ret = HashMap::new();
    for (k, v) in res.iter() {
        let cfact = config
            .factions
            .iter()
            .find(|f| &f.1.settings.id.to_string() == k);
        ret.insert(
            k.to_owned(),
            GetFactionRet {
                archived: v.archived,
                icon: if cfact.is_some() {
                    Some(cfact.unwrap().1.settings.icon_id)
                } else {
                    None
                },
                managed: cfact.is_some(),
                name: v.name.clone(),
                shortname: v.shortname.clone(),
            },
        );
    }
    Ok(Json(ret))
}
