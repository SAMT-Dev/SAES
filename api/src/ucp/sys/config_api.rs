use axum::{debug_handler, extract::Query, response::IntoResponse, Json};
use http::StatusCode;
use saes_shared::structs::factions::Factions;
use serde::Deserialize;

use crate::config::{
    editor::write_config,
    loader::get_config,
    structs::{FactionConfig, GlobalConfig},
};

#[debug_handler]
pub async fn sys_get_configs() -> Result<impl IntoResponse, (StatusCode, String)> {
    let config = get_config().await;
    let conf = config.clone();
    return Ok(Json(conf));
}

#[debug_handler]
pub async fn sys_post_global_config(
    Json(c): Json<GlobalConfig>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let conf = get_config().await;
    let mut new_config = conf.clone();
    if c.announcement != conf.global.announcement {
        new_config.global.announcement = c.announcement;
    }

    if c.maintenance != conf.global.maintenance {
        new_config.global.maintenance = c.maintenance;
    }
    drop(conf);

    write_config(&new_config).await;
    Ok(Json(new_config.global))
}

#[derive(Debug, Deserialize)]
pub struct FactionQuery {
    pub faction: Factions,
}

#[debug_handler]
pub async fn sys_post_faction_config(
    q: Query<FactionQuery>,
    Json(c): Json<FactionConfig>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let conf = get_config().await;
    let mut new_config = conf.clone();
    new_config.factions.insert(q.faction, c);
    write_config(&new_config).await;
    Ok(Json(new_config.factions[&q.faction].clone()))
}
