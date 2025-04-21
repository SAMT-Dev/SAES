use axum::{response::IntoResponse, Extension, Json};
use http::StatusCode;

use saes_shared::{
    db::{bills, hails, supplements},
    structs::factions::get_faction_id,
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Serialize;

use crate::{
    utils::{middle::Driver, types_statuses::get_statuses},
    DB_CLIENT,
};

#[derive(Debug, Serialize)]
pub struct SMStat {
    feltoltve: i32,
    elfogadva: i32,
    elutasitva: i32,
}
#[derive(Debug, Serialize)]
pub struct SMStatReturn {
    potlek: Option<SMStat>,
    leintes: Option<SMStat>,
    szamla: Option<SMStat>,
}

pub async fn admin_home_stat(
    ext: Extension<Driver>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db = DB_CLIENT.get().unwrap();
    let statreturn_supp = supplements::Entity::find()
        .filter(supplements::Column::Faction.eq(get_faction_id(ext.faction.unwrap())))
        .all(db)
        .await
        .expect("[ERROR] Statisztika lekérés sikertelen");
    let statreturn_hails = hails::Entity::find()
        .filter(hails::Column::Faction.eq(get_faction_id(ext.faction.unwrap())))
        .all(db)
        .await
        .expect("[ERROR] Statisztika lekérés sikertelen");
    let statreturn_bills = bills::Entity::find()
        .filter(bills::Column::Faction.eq(get_faction_id(ext.faction.unwrap())))
        .all(db)
        .await
        .expect("[ERROR] Statisztika lekérés sikertelen");
    let mut potlekok = [0, 0, 0];
    let mut leintesek = [0, 0, 0];
    let mut szamlak = [0, 0, 0];
    let statuses = get_statuses();
    for item in statreturn_supp.iter() {
        if item.status == statuses.accepted.id {
            potlekok[1] += 1
        }
        if item.status == statuses.rejected.id {
            potlekok[2] += 1
        }
        if item.status == statuses.uploaded.id {
            potlekok[0] += 1
        }
    }
    for item in statreturn_hails.iter() {
        if item.status == statuses.accepted.id {
            leintesek[1] += 1
        }
        if item.status == statuses.rejected.id {
            leintesek[2] += 1
        }
        if item.status == statuses.uploaded.id {
            leintesek[0] += 1
        }
    }
    for item in statreturn_bills.iter() {
        if item.status == statuses.accepted.id {
            szamlak[1] += 1
        }
        if item.status == statuses.rejected.id {
            szamlak[2] += 1
        }
        if item.status == statuses.uploaded.id {
            szamlak[0] += 1
        }
    }
    Ok(Json(SMStatReturn {
        potlek: Some(SMStat {
            elfogadva: potlekok[1],
            elutasitva: potlekok[2],
            feltoltve: potlekok[0],
        }),
        leintes: Some(SMStat {
            elfogadva: leintesek[1],
            elutasitva: leintesek[2],
            feltoltve: leintesek[0],
        }),
        szamla: Some(SMStat {
            elfogadva: szamlak[1],
            elutasitva: szamlak[2],
            feltoltve: szamlak[0],
        }),
    }))
}
