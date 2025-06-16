use axum::{Extension, Json, Router, debug_handler, response::IntoResponse, routing::get};
use saes_shared::{db::logs, structs::user::Driver};
use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder};

use crate::{DB_CLIENT, config::loader::get_config, modules::api::utils::structs::Logs};

pub fn get_routes() -> Router {
    Router::new().route("/get", get(fm_get_logs))
}

#[debug_handler]
pub async fn fm_get_logs(ext: Extension<Driver>) -> impl IntoResponse {
    let db = DB_CLIENT.get().unwrap();
    let config = get_config().await;
    let logs = logs::Entity::find()
        .filter(
            logs::Column::Faction.eq(config
                .factions
                .get(&ext.faction.clone().unwrap())
                .unwrap()
                .settings
                .id),
        )
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
