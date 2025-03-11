use axum::{
    debug_handler,
    extract::{Query, Request},
    response::IntoResponse,
    Json,
};
use http::StatusCode;
use saes_shared::db::{bills, hails, supplements};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::{
    utils::{middle::Driver, structs::SMGetItemsFull, types_statuses::get_types},
    DB_CLIENT,
};

#[debug_handler]
pub async fn fm_home(mut request: Request) -> Json<Driver> {
    let exts: Option<&Driver> = request.extensions_mut().get();
    Json(exts.unwrap().clone())
}

#[derive(Debug, Deserialize)]
pub struct GetImageQuery {
    pub id: i32,
    pub r#type: i8,
}

#[debug_handler]
pub async fn get_images_by_id(query: Query<GetImageQuery>) -> impl IntoResponse {
    let types = get_types();
    let db = DB_CLIENT.get().unwrap();
    if query.r#type == types.supplements.id {
        let ret = supplements::Entity::find()
            .filter(supplements::Column::Id.eq(query.id))
            .one(db)
            .await
            .unwrap();
        if ret.is_some() {
            let ret = ret.unwrap();
            let final_ret = SMGetItemsFull {
                id: ret.id,
                img_1: ret.image,
                img_2: None,
                r#type: ret.r#type,
                price: None,
                date: ret.date,
                faction: ret.faction,
                handled_by: ret.handled_by,
                reason: ret.reason,
                target_faction: None,
                driver: None,
                owner: ret.owner,
                status: ret.status,
                item_type: types.supplements.id,
            };
            return Ok(Json(final_ret));
        }
        return Err(StatusCode::NOT_FOUND);
    }
    if query.r#type == types.hails.id {
        let ret = hails::Entity::find()
            .filter(hails::Column::Id.eq(query.id))
            .one(db)
            .await
            .unwrap();
        if ret.is_some() {
            let ret = ret.unwrap();
            let final_ret = SMGetItemsFull {
                id: ret.id,
                img_1: ret.image_1,
                img_2: Some(ret.image_2),
                date: ret.date,
                faction: ret.faction,
                handled_by: ret.handled_by,
                price: None,
                r#type: None,
                target_faction: None,
                driver: None,
                reason: ret.reason,
                owner: ret.owner,
                status: ret.status,
                item_type: types.hails.id,
            };
            return Ok(Json(final_ret));
        }
        return Err(StatusCode::NOT_FOUND);
    }
    if query.r#type == types.bills.id {
        let ret = bills::Entity::find()
            .filter(bills::Column::Id.eq(query.id))
            .one(db)
            .await
            .unwrap();
        if ret.is_some() {
            let ret = ret.unwrap();
            let final_ret = SMGetItemsFull {
                id: ret.id,
                img_1: ret.image,
                img_2: None,
                date: ret.date,
                faction: ret.faction,
                price: ret.price,
                r#type: None,
                handled_by: ret.handled_by,
                driver: ret.driver,
                target_faction: ret.target_faction,
                reason: ret.reason,
                owner: ret.owner,
                status: ret.status,
                item_type: types.bills.id,
            };
            return Ok(Json(final_ret));
        }
        return Err(StatusCode::NOT_FOUND);
    }
    return Err(StatusCode::BAD_REQUEST);
}
