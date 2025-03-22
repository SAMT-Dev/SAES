use axum::{extract::Query, response::IntoResponse, Extension, Json};
use serde::Deserialize;

use crate::{
    config::{
        editor::write_config,
        loader::get_config,
        structs::{FactionConfig, ShiftAccess},
    },
    utils::middle::Driver,
};

pub async fn get_shift_access(ext: Extension<Driver>) -> impl IntoResponse {
    let conf = get_config().await;
    let shift = conf
        .factions
        .get(&ext.faction.unwrap())
        .unwrap()
        .shift_access
        .clone();
    Json(shift)
}

#[derive(Debug, Deserialize)]
pub struct PostQuery {
    pub new_access: ShiftAccess,
}

pub async fn post_shift_access(
    ext: Extension<Driver>,
    query: Query<PostQuery>,
) -> impl IntoResponse {
    let conf = get_config().await;
    let mut new_config = conf.clone();
    new_config.factions.insert(
        ext.faction.unwrap(),
        FactionConfig {
            shift_access: query.new_access.clone(),
            ..new_config
                .factions
                .get(&ext.faction.unwrap())
                .unwrap()
                .clone()
        },
    );
    write_config(&new_config).await;
    Json(new_config)
}
