use axum::{
    debug_handler,
    extract::{Query, Request},
    response::IntoResponse,
    Extension, Json,
};
use chrono::NaiveDateTime;
use http::StatusCode;
use saes_shared::db::{bills, hails, supplements};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;

use crate::{
    utils::{
        factions::get_faction_id, functions::get_fridays, middle::Driver, queries::SMStatQuery,
        types_statuses::get_statuses,
    },
    DB_CLIENT,
};

#[debug_handler]
pub async fn sm_home(mut request: Request) -> Json<Driver> {
    let exts: Option<&Driver> = request.extensions_mut().get();
    Json(exts.unwrap().clone())
}

#[derive(Debug, Serialize)]
struct Date {
    next: NaiveDateTime,
    prev: NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct StatDBAll {
    potlekok: Vec<supplements::Model>,
    leintesek: Vec<hails::Model>,
    szamlak: Vec<bills::Model>,
}

#[derive(Debug, Serialize)]
pub struct StatReturn {
    stats: StatDBAll,
    date: Date,
}

#[debug_handler]
pub async fn sm_stat(
    ext: Extension<Driver>,
    quer: Query<SMStatQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let statuses = get_statuses();
    if quer.week == "current".to_string() {
        let friday = get_fridays();
        let db = DB_CLIENT.get().unwrap();
        let statreturn_supp = supplements::Entity::find()
            .filter(supplements::Column::Status.eq(statuses.accepted.id))
            .filter(supplements::Column::Date.gt(friday.last_friday))
            .filter(supplements::Column::Date.lt(friday.next_friday))
            .filter(supplements::Column::Faction.eq(get_faction_id(ext.faction.unwrap())))
            .all(db)
            .await
            .expect("[ERROR] Statisztika lekérés sikertelen");
        let statreturn_hails = hails::Entity::find()
            .filter(hails::Column::Status.eq(statuses.accepted.id))
            .filter(hails::Column::Date.gt(friday.last_friday))
            .filter(hails::Column::Date.lt(friday.next_friday))
            .filter(hails::Column::Faction.eq(get_faction_id(ext.faction.unwrap())))
            .all(db)
            .await
            .expect("[ERROR] Statisztika lekérés sikertelen");
        let statreturn_bills = bills::Entity::find()
            .filter(bills::Column::Status.eq(statuses.accepted.id))
            .filter(bills::Column::Date.gt(friday.last_friday))
            .filter(bills::Column::Date.lt(friday.next_friday))
            .filter(bills::Column::TargetFaction.eq(get_faction_id(ext.faction.unwrap())))
            .all(db)
            .await
            .expect("[ERROR] Statisztika lekérés sikertelen");
        let mut potlekok = vec![];
        let mut leintesek = vec![];
        let mut szamlak = vec![];
        for item in statreturn_supp.iter() {
            potlekok.push(item.clone())
        }
        for item in statreturn_hails.iter() {
            leintesek.push(item.clone())
        }
        for item in statreturn_bills.iter() {
            szamlak.push(item.clone())
        }
        Ok(Json(StatReturn {
            stats: StatDBAll {
                potlekok,
                leintesek,
                szamlak,
            },
            date: Date {
                next: friday.next_friday,
                prev: friday.last_friday,
            },
        }))
    } else if quer.week == "previous" {
        let friday = get_fridays();
        let db = DB_CLIENT.get().unwrap();
        let statreturn_supp = supplements::Entity::find()
            .filter(supplements::Column::Status.eq(statuses.accepted.id))
            .filter(supplements::Column::Date.gt(friday.before_last_friday))
            .filter(supplements::Column::Date.lt(friday.last_friday))
            .filter(supplements::Column::Faction.eq(get_faction_id(ext.faction.unwrap())))
            .all(db)
            .await
            .expect("[ERROR] Statisztika lekérés sikertelen");
        let statreturn_hails = hails::Entity::find()
            .filter(hails::Column::Status.eq(statuses.accepted.id))
            .filter(hails::Column::Date.gt(friday.before_last_friday))
            .filter(hails::Column::Date.lt(friday.last_friday))
            .filter(hails::Column::Faction.eq(get_faction_id(ext.faction.unwrap())))
            .all(db)
            .await
            .expect("[ERROR] Statisztika lekérés sikertelen");
        let statreturn_bills = bills::Entity::find()
            .filter(bills::Column::Status.eq(statuses.accepted.id))
            .filter(bills::Column::Date.gt(friday.before_last_friday))
            .filter(bills::Column::Date.lt(friday.last_friday))
            .filter(bills::Column::TargetFaction.eq(get_faction_id(ext.faction.unwrap())))
            .all(db)
            .await
            .expect("[ERROR] Statisztika lekérés sikertelen");
        let mut potlekok = vec![];
        let mut leintesek = vec![];
        let mut szamlak = vec![];
        for item in statreturn_supp.iter() {
            potlekok.push(item.clone())
        }
        for item in statreturn_hails.iter() {
            leintesek.push(item.clone())
        }
        for item in statreturn_bills.iter() {
            szamlak.push(item.clone())
        }
        Ok(Json(StatReturn {
            stats: StatDBAll {
                potlekok,
                leintesek,
                szamlak,
            },
            date: Date {
                next: friday.last_friday,
                prev: friday.before_last_friday,
            },
        }))
    } else {
        return Err((StatusCode::BAD_REQUEST, "noweek".to_string()));
    }
}
