use axum::{debug_handler, response::IntoResponse, routing::get, Extension, Json, Router};
use chrono::{DateTime, Utc};
use http::StatusCode;
use saes_shared::{db::logs, structs::factions::get_faction_id};
use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder};
use serde::Serialize;

use crate::{utils::middle::Driver, DB_CLIENT};

#[derive(Debug, Serialize)]
pub struct Logs {
    owner: i32,
    item_id: Option<i32>,
    item_type: Option<i8>,
    action: String,
    faction: Option<i8>,
    message: Option<String>,
    date: DateTime<Utc>,
}

pub fn get_routes() -> Router {
    Router::new()
        .route("/get", get(fm_get_logs))
        .route("/get_all", get(fm_get_all_logs))
}

#[debug_handler]
pub async fn fm_get_logs(ext: Extension<Driver>) -> impl IntoResponse {
    let db = DB_CLIENT.get().unwrap();
    let logs = logs::Entity::find()
        .filter(logs::Column::Faction.eq(get_faction_id(ext.faction.unwrap())))
        .order_by(logs::Column::Date, Order::Desc)
        .all(db)
        .await
        .unwrap();
    let logs: Vec<Logs> = logs
        .iter()
        .map(|log| -> Logs {
            Logs {
                owner: log.owner.clone(),
                item_id: log.item_id,
                item_type: log.item_type,
                action: log.action.clone(),
                faction: log.faction.clone(),
                message: log.message.clone(),
                date: log.date.clone(),
            }
        })
        .collect();
    return Json(logs);
}

#[debug_handler]
pub async fn fm_get_all_logs(ext: Extension<Driver>) -> impl IntoResponse {
    if ext.admin {
        let db = DB_CLIENT.get().unwrap();
        let logs = logs::Entity::find()
            .order_by(logs::Column::Date, Order::Desc)
            .all(db)
            .await
            .unwrap();
        let logs: Vec<Logs> = logs
            .iter()
            .map(|log| -> Logs {
                Logs {
                    owner: log.owner.clone(),
                    item_id: log.item_id,
                    item_type: log.item_type,
                    action: log.action.clone(),
                    faction: log.faction,
                    message: log.message.clone(),
                    date: log.date.clone(),
                }
            })
            .collect();
        return Ok(Json(logs));
    }
    return Err((
        StatusCode::FORBIDDEN,
        "This is only accessible to sysadmins!",
    ));
}
