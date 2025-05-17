use axum::{debug_handler, extract::Query, response::IntoResponse, Json};
use http::StatusCode;
use saes_shared::db::{bills, hails, supplements};
use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder};
use serde::Serialize;

use crate::{
    config::loader::get_config,
    modules::api::utils::{
        functions::get_fridays,
        queries::BaseListQuery,
        types_statuses::{get_statuses, get_types},
    },
    DB_CLIENT,
};

#[derive(Debug, Serialize)]
pub struct ListReturn {
    pub id: i32,
    pub r#type: i8,
    pub img_1: i32,
    pub img_2: Option<i32>,
    pub desc: Option<String>,
    pub handled_by: Option<i32>,
    pub price: Option<i32>,
}

#[debug_handler]
pub async fn base_list_get(
    quer: Query<BaseListQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let friday = get_fridays();
    let config = get_config().await;
    let db = DB_CLIENT.get().unwrap();
    let statuses = get_statuses();
    let types = get_types();
    if quer.tipus.starts_with("potlek") {
        let supp_ret = supplements::Entity::find()
            .filter(supplements::Column::Owner.eq(quer.driver.clone()))
            .filter(supplements::Column::Date.gt(friday.before_last_friday))
            .filter(supplements::Column::Status.eq(statuses.accepted.id))
            .filter(
                supplements::Column::Faction.eq(config
                    .factions
                    .get(&quer.faction)
                    .unwrap()
                    .settings
                    .id),
            )
            .filter(supplements::Column::Date.lt(friday.last_friday))
            .filter(supplements::Column::Type.eq(if quer.tipus == "potlek_de" {
                1
            } else if quer.tipus == "potlek_ej" {
                2
            } else {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Ilyen pótlékot nem ismerek!".to_string(),
                ));
            }))
            .order_by(supplements::Column::Date, Order::Desc)
            .all(db)
            .await
            .expect("[ERROR] List lekérés sikertelen");
        let ret: Vec<ListReturn> = supp_ret
            .iter()
            .map(|item| -> ListReturn {
                ListReturn {
                    id: item.id,
                    img_1: item.image,
                    img_2: None,
                    price: None,
                    desc: item.reason.clone(),
                    handled_by: item.handled_by.clone(),
                    r#type: types.supplements.id,
                }
            })
            .collect();
        return Ok(Json(ret));
    } else if quer.tipus == "leintes".to_string() {
        let hails_ret = hails::Entity::find()
            .filter(hails::Column::Owner.eq(quer.driver.clone()))
            .filter(hails::Column::Date.gt(friday.before_last_friday))
            .filter(
                hails::Column::Faction.eq(config.factions.get(&quer.faction).unwrap().settings.id),
            )
            .filter(hails::Column::Date.lt(friday.last_friday))
            .filter(hails::Column::Status.eq(statuses.accepted.id))
            .order_by(hails::Column::Date, Order::Desc)
            .all(db)
            .await
            .expect("[ERROR] List lekérés sikertelen");
        let ret = hails_ret
            .iter()
            .map(|item| -> ListReturn {
                ListReturn {
                    id: item.id,
                    img_1: item.image_1,
                    img_2: Some(item.image_2),
                    price: None,
                    desc: item.reason.clone(),
                    handled_by: item.handled_by.clone(),
                    r#type: types.hails.id,
                }
            })
            .collect();
        return Ok(Json(ret));
    } else if quer.tipus == "szamla".to_string() {
        let bills_ret = bills::Entity::find()
            .filter(bills::Column::Driver.eq(quer.driver.clone()))
            .filter(bills::Column::Date.gt(friday.before_last_friday))
            .filter(
                bills::Column::TargetFaction.eq(config
                    .factions
                    .get(&quer.faction)
                    .unwrap()
                    .settings
                    .id),
            )
            .filter(bills::Column::Date.lt(friday.last_friday))
            .filter(bills::Column::Status.eq(statuses.accepted.id))
            .order_by(bills::Column::Date, Order::Desc)
            .all(db)
            .await
            .expect("[ERROR] List lekérés sikertelen");

        let ret = bills_ret
            .iter()
            .map(|item| -> ListReturn {
                ListReturn {
                    id: item.id,
                    img_1: item.image,
                    r#type: types.bills.id,
                    desc: item.reason.clone(),
                    handled_by: item.handled_by.clone(),
                    img_2: None,
                    price: item.price,
                }
            })
            .collect();
        return Ok(Json(ret));
    } else {
        return Err((
            StatusCode::NOT_FOUND,
            "Ilyen típus nem található!".to_string(),
        ));
    }
}
