use axum::{debug_handler, extract::Request, Json};
use http::StatusCode;
use saes_shared::db::{bills, hails, supplements};
use saes_shared::structs::user::Driver;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

use crate::config::loader::get_config;
use crate::utils::api::get_api_envs;
use crate::utils::functions::get_fridays;
use crate::utils::types_statuses::get_statuses;
use crate::{DB_CLIENT, WEB_CLIENT};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DriverRecord {
    pub driver: String,
    pub count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Callz {
    pub app: Option<u32>,
    pub leintes: usize,
    pub price: isize,
    pub potlek: Potlek,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ApmsCalls {
    pub uploaded: usize,
    pub accepted: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Potlek {
    de: usize,
    du: usize,
}

#[debug_handler]
pub async fn ucp_calls(mut request: Request) -> Result<Json<Callz>, (StatusCode, String)> {
    let exts: Option<&Driver> = request.extensions_mut().get();
    if exts.unwrap().faction.is_some() {
        let db = DB_CLIENT.get().unwrap();
        let config = get_config().await;
        let statuses = get_statuses();
        let envs = get_api_envs().await;
        let calls = WEB_CLIENT
            .get(format!(
                "{}/api/log/status/current",
                if exts.unwrap().faction.clone().unwrap().eq("TOW")
                    || (exts.unwrap().faction.clone().unwrap().eq("UNI")
                        && exts.unwrap().factions.is_some()
                        && exts.unwrap().factions.clone().unwrap().shiftname
                            == "Autómentés".to_string())
                {
                    envs.sckkapp_tow
                } else {
                    envs.sckkapp_taxi
                }
            ))
            .send()
            .await;
        let fridays = get_fridays();
        let dbreturn_supp = supplements::Entity::find()
            .filter(supplements::Column::Owner.eq(exts.unwrap().driverid))
            .filter(supplements::Column::Status.eq(statuses.accepted.id))
            .filter(supplements::Column::Date.gt(fridays.last_friday))
            .filter(
                supplements::Column::Faction.eq(config
                    .factions
                    .get(&exts.unwrap().faction.clone().unwrap())
                    .unwrap()
                    .settings
                    .id),
            )
            .filter(supplements::Column::Date.lt(fridays.next_friday))
            .all(db)
            .await
            .expect("Leintések lekérése sikertelen az adatbázisból");
        let dbreturn_hails = hails::Entity::find()
            .filter(hails::Column::Owner.eq(exts.unwrap().driverid))
            .filter(hails::Column::Status.eq(statuses.accepted.id))
            .filter(hails::Column::Date.gt(fridays.last_friday))
            .filter(
                hails::Column::Faction.eq(config
                    .factions
                    .get(&exts.unwrap().faction.clone().unwrap())
                    .unwrap()
                    .settings
                    .id),
            )
            .filter(hails::Column::Date.lt(fridays.next_friday))
            .all(db)
            .await
            .expect("Leintések lekérése sikertelen az adatbázisból");
        let dbreturn_bills = bills::Entity::find()
            .filter(
                bills::Column::Owner
                    .eq(exts.unwrap().driverid)
                    .or(bills::Column::Driver.eq(exts.unwrap().driverid)),
            )
            .filter(bills::Column::Status.eq(statuses.accepted.id))
            .filter(bills::Column::Date.gt(fridays.last_friday))
            .filter(
                bills::Column::Faction.eq(config
                    .factions
                    .get(&exts.unwrap().faction.clone().unwrap())
                    .unwrap()
                    .settings
                    .id),
            )
            .filter(bills::Column::Date.lt(fridays.next_friday))
            .all(db)
            .await
            .expect("Leintések lekérése sikertelen az adatbázisból");
        let mut bills = 0;
        for bill in dbreturn_bills.iter() {
            if bill.price.is_some() {
                bills += bill.price.unwrap();
            }
        }
        let mut leintes = 0;
        let mut de_potlek = 0;
        let mut du_potlek = 0;
        for model in dbreturn_supp.iter() {
            if model.r#type.is_some() {
                if model.r#type.unwrap() == 1 {
                    de_potlek += 1
                }
                if model.r#type.unwrap() == 2 {
                    du_potlek += 1
                }
            }
        }
        for _ in dbreturn_hails.iter() {
            leintes += 1
        }
        if calls.is_ok() {
            let callsz = calls.unwrap().text().await.expect("Átalakítás sikertelen");
            let driver_records: Result<Vec<DriverRecord>, serde_json::Error> = from_str(&callsz);
            if driver_records.is_ok() {
                let drc = driver_records.unwrap();
                let rec: Option<&DriverRecord> = drc
                    .iter()
                    .find(|record| record.driver == exts.unwrap().name);
                return Ok(Json(Callz {
                    app: if rec.is_some() {
                        Some(rec.unwrap().count)
                    } else {
                        Some(0)
                    },
                    leintes,
                    price: bills as isize,
                    potlek: Potlek {
                        de: de_potlek,
                        du: du_potlek,
                    },
                }));
            }
        };
        Ok(Json(Callz {
            app: None,
            leintes,
            price: bills as isize,
            potlek: Potlek {
                de: de_potlek,
                du: du_potlek,
            },
        }))
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            "Frakciójelölés hiányzik!".to_string(),
        ));
    }
}

#[debug_handler]
pub async fn ucp_apms_calls(mut request: Request) -> Result<Json<ApmsCalls>, (StatusCode, String)> {
    let exts: Option<&Driver> = request.extensions_mut().get();
    if exts.unwrap().faction.is_some()
        && exts.unwrap().faction.clone().unwrap() == "APMS".to_string()
    {
        let db = DB_CLIENT.get().unwrap();
        let config = get_config().await;
        let statuses = get_statuses();
        let fridays: crate::utils::functions::Friday = get_fridays();
        let dbreturn_bills = bills::Entity::find()
            .filter(bills::Column::Status.ne(statuses.rejected.id))
            .filter(bills::Column::Date.gt(fridays.last_friday))
            .filter(
                bills::Column::Faction.eq(config
                    .factions
                    .get(&exts.unwrap().faction.clone().unwrap())
                    .unwrap()
                    .settings
                    .id),
            )
            .filter(bills::Column::Date.lt(fridays.next_friday))
            .all(db)
            .await
            .expect("Leintések lekérése sikertelen az adatbázisból");

        let mut accepted = vec![];
        let mut uploaded = vec![];

        for bill in dbreturn_bills.iter() {
            if bill.status == statuses.uploaded.id {
                uploaded.push(bill)
            }
            if bill.status == statuses.accepted.id {
                accepted.push(bill)
            }
        }

        Ok(Json(ApmsCalls {
            accepted: accepted.len(),
            uploaded: uploaded.len(),
        }))
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            "Frakciójelölés hiányzik!".to_string(),
        ));
    }
}
