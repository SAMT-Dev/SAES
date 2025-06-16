use axum::{Extension, Json, debug_handler, response::IntoResponse};
use http::StatusCode;
use saes_shared::{
    db::logs::{self},
    structs::user::Driver,
};
use sea_orm::{EntityTrait, Order, QueryOrder};

use crate::{DB_CLIENT, modules::api::utils::structs::Logs};

#[debug_handler]
pub async fn sys_get_logs(ext: Extension<Driver>) -> impl IntoResponse {
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
