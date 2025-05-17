use std::collections::HashMap;

use axum::{debug_handler, response::IntoResponse, Extension, Json};
use http::{HeaderMap, StatusCode};
use saes_shared::structs::{permissions::get_perm, user::Driver};
use serde::{Deserialize, Serialize};

use crate::{config::loader::get_config, modules::api::utils::api::get_api_envs, WEB_CLIENT};

#[derive(Debug, Serialize)]
struct HomeRet {
    driver: Driver,
    info: Option<FactionInfo>,
}

#[derive(Debug, Serialize)]
struct FactionInfo {
    display: String,
    icon_id: i32,
    perm_name: String,
    primary: String,
    secondary: String,
    tertiary: String,
}

#[debug_handler]
pub async fn ucp_home(ext: Extension<Driver>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let config = get_config().await;
    let faction = if ext.faction.is_some() {
        Some(config.factions.get(&ext.faction.clone().unwrap()).unwrap())
    } else {
        None
    };
    Ok(Json(HomeRet {
        driver: Driver {
            driverid: ext.driverid,
            access: ext.access.clone(),
            faction: ext.faction.clone(),
            admin: ext.admin,
            factions: ext.factions.clone(),
            perms: ext.perms.clone(),
            name: ext.name.clone(),
            site_access: ext.site_access.clone(),
        },
        info: if faction.is_some() {
            Some(FactionInfo {
                display: faction.unwrap().settings.display.clone(),
                icon_id: faction.unwrap().settings.icon_id,
                perm_name: faction.unwrap().settings.perm_name.clone(),
                primary: faction.unwrap().settings.color.primary.clone(),
                secondary: faction.unwrap().settings.color.secondary.clone(),
                tertiary: faction.unwrap().settings.color.tertiary.clone(),
            })
        } else {
            None
        },
    }))
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

#[derive(Debug, Serialize, Deserialize)]
pub struct PubFactionReturn {
    pub icon_id: i32,
    pub name: String,
    pub perm_name: String,
    pub primary: String,
    pub secondary: String,
    pub tertiary: String,
}

#[debug_handler]
pub async fn ucp_getpubfactions(ext: Extension<Driver>) -> Json<HashMap<String, PubFactionReturn>> {
    let config = get_config().await;
    let mut factions = HashMap::new();
    for (key, faction) in config.factions.iter() {
        if ext.perms.contains(&get_perm(
            saes_shared::structs::permissions::Permissions::SaesUcp(
                faction.settings.perm_name.clone(),
            ),
        )) || ext.admin
        {
            factions.insert(
                key.to_owned(),
                PubFactionReturn {
                    icon_id: faction.settings.icon_id,
                    name: faction.settings.display.clone(),
                    perm_name: faction.settings.perm_name.clone(),
                    primary: faction.settings.color.primary.clone(),
                    secondary: faction.settings.color.secondary.clone(),
                    tertiary: faction.settings.color.tertiary.clone(),
                },
            );
        }
    }
    Json(factions)
}
