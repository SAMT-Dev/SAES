use axum::{
    debug_handler,
    extract::{self, Query},
    response::IntoResponse,
    Extension, Json,
};
use http::StatusCode;
use saes_shared::{
    db::{bills, hails, supplements},
    structs::factions::{get_faction_id, Factions},
};
use serde::{Deserialize, Serialize};

use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder, Set};

use crate::{
    config::{loader::get_config, structs::ItemAccess},
    logging::db_log,
    utils::{
        middle::Driver,
        queries::SMItemsQuery,
        structs::SMGetItemsFull,
        types_statuses::{get_statuses_as_list, get_types, get_types_as_list},
    },
    DB_CLIENT,
};

#[derive(Debug, Deserialize)]
pub struct SMPostItemsBody {
    pub id: i32,
    pub status: i8,
    pub price: Option<i32>,
    pub target_faction: Option<Factions>,
    pub driver: Option<i32>,
    pub supp_type: Option<i8>,
    pub reason: Option<String>,
    pub tipus: i8,
}

#[derive(Debug, Serialize)]
pub struct StatDBAll {
    pub items: Vec<SMGetItemsFull>,
}

#[debug_handler]
pub async fn admin_items_get(
    ext: Extension<Driver>,
    quer: Query<SMItemsQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db = DB_CLIENT.get().unwrap();
    let config = get_config().await;
    let types = get_types();
    if quer.tipus == types.supplements.id
        && (config
            .factions
            .get(&ext.faction.unwrap())
            .unwrap()
            .access
            .supplements
            == ItemAccess::Read
            || config
                .factions
                .get(&ext.faction.unwrap())
                .unwrap()
                .access
                .supplements
                == ItemAccess::Write)
    {
        let statreturn = supplements::Entity::find()
            .filter(supplements::Column::Status.eq(quer.status.clone()))
            .order_by(supplements::Column::Date, Order::Desc)
            .filter(supplements::Column::Faction.eq(get_faction_id(ext.faction.unwrap())))
            .all(db)
            .await
            .expect("[ERROR] Statisztika lekérés sikertelen");
        let ret: Vec<SMGetItemsFull> = statreturn
            .iter()
            .map(|item| -> SMGetItemsFull {
                SMGetItemsFull {
                    id: item.id,
                    img_1: item.image,
                    img_2: None,
                    faction: item.faction,
                    status: item.status,
                    date: item.date,
                    price: None,
                    r#type: item.r#type,
                    handled_by: item.handled_by.clone(),
                    reason: item.reason.clone(),
                    driver: None,
                    target_faction: None,
                    owner: item.owner,
                    item_type: types.supplements.id,
                }
            })
            .collect();
        return Ok(Json(StatDBAll { items: ret }));
    } else if quer.tipus == types.hails.id
        && (config
            .factions
            .get(&ext.faction.unwrap())
            .unwrap()
            .access
            .hails
            == ItemAccess::Read
            || config
                .factions
                .get(&ext.faction.unwrap())
                .unwrap()
                .access
                .hails
                == ItemAccess::Write)
    {
        let statreturn = hails::Entity::find()
            .filter(hails::Column::Status.eq(quer.status.clone()))
            .order_by(hails::Column::Date, Order::Desc)
            .filter(hails::Column::Faction.eq(get_faction_id(ext.faction.unwrap())))
            .all(db)
            .await
            .expect("[ERROR] Statisztika lekérés sikertelen");
        let ret: Vec<SMGetItemsFull> = statreturn
            .iter()
            .map(|item| -> SMGetItemsFull {
                SMGetItemsFull {
                    id: item.id,
                    img_1: item.image_1,
                    img_2: Some(item.image_2),
                    faction: item.faction,
                    status: item.status,
                    date: item.date,
                    price: None,
                    r#type: None,
                    handled_by: item.handled_by.clone(),
                    reason: item.reason.clone(),
                    owner: item.owner,
                    driver: None,
                    target_faction: None,
                    item_type: types.hails.id,
                }
            })
            .collect();
        return Ok(Json(StatDBAll { items: ret }));
    } else if quer.tipus == types.bills.id
        && (config
            .factions
            .get(&ext.faction.unwrap())
            .unwrap()
            .access
            .bills
            == ItemAccess::Read
            || config
                .factions
                .get(&ext.faction.unwrap())
                .unwrap()
                .access
                .bills
                == ItemAccess::Write)
    {
        let statreturn = bills::Entity::find()
            .filter(bills::Column::Status.eq(quer.status.clone()))
            .order_by(bills::Column::Date, Order::Desc)
            .filter(bills::Column::Faction.eq(get_faction_id(ext.faction.unwrap())))
            .all(db)
            .await
            .expect("[ERROR] Statisztika lekérés sikertelen");
        let ret: Vec<SMGetItemsFull> = statreturn
            .iter()
            .map(|item| -> SMGetItemsFull {
                SMGetItemsFull {
                    id: item.id,
                    img_1: item.image,
                    img_2: None,
                    faction: item.faction,
                    price: item.price,
                    r#type: None,
                    status: item.status,
                    date: item.date,
                    handled_by: item.handled_by.clone(),
                    reason: item.reason.clone(),
                    owner: item.owner,
                    driver: item.driver.clone(),
                    target_faction: item.target_faction,
                    item_type: types.bills.id,
                }
            })
            .collect();
        return Ok(Json(StatDBAll { items: ret }));
    } else {
        return Err((
            StatusCode::NOT_FOUND,
            "Ilyen típus nem található!".to_string(),
        ));
    }
}

#[debug_handler]
pub async fn admin_items_post(
    ext: Extension<Driver>,
    extract::Json(body): extract::Json<SMPostItemsBody>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let status_list = get_statuses_as_list();
    let config = get_config().await;
    let types_list = get_types_as_list();
    if status_list.contains(&body.status) && types_list.contains(&body.tipus) {
        let types = get_types();
        let db = DB_CLIENT.get().unwrap();
        if body.tipus == types.supplements.id
            && config
                .factions
                .get(&ext.faction.unwrap())
                .unwrap()
                .access
                .supplements
                == ItemAccess::Write
        {
            let old_model = supplements::Entity::find()
                .filter(supplements::Column::Id.eq(body.id))
                .one(db)
                .await
                .expect("[ERROR] Régi lekérése sikertelen")
                .unwrap();
            let mut act = String::new();
            if old_model.status != body.status {
                act += format!(
                    "{}status FROM {} TO {}",
                    if act.len() > 0 { "; " } else { "" },
                    old_model.status,
                    body.status
                )
                .as_str();
            }
            if old_model.reason != body.reason {
                act += format!(
                    "{}reason FROM {} TO {}",
                    if act.len() > 0 { "; " } else { "" },
                    if old_model.reason.is_some() {
                        old_model.reason.unwrap().replace(";", "{saes_semicolon}")
                    } else {
                        String::from("null")
                    },
                    if body.reason.is_some() {
                        body.reason
                            .clone()
                            .unwrap()
                            .replace(";", "{saes_semicolon}")
                    } else {
                        String::from("null")
                    }
                )
                .as_str();
            }
            if old_model.r#type != body.supp_type {
                act += format!(
                    "{}supp_type FROM {} TO {}",
                    if act.len() > 0 { "; " } else { "" },
                    if old_model.r#type.is_some() {
                        old_model.r#type.unwrap()
                    } else {
                        0
                    },
                    if body.supp_type.is_some() {
                        body.supp_type.clone().unwrap()
                    } else {
                        0
                    }
                )
                .as_str();
            }
            db_log(
                ext.driverid,
                Some(get_faction_id(ext.faction.unwrap())),
                Some(body.id.clone()),
                Some(types.supplements.id),
                "UPDATE ITEM",
                Some(act),
            )
            .await;
            let activemodel = supplements::ActiveModel {
                id: Set(body.id),
                faction: Set(get_faction_id(ext.faction.unwrap())),
                status: Set(body.status),
                reason: Set(body.reason),
                r#type: Set(if !body.supp_type.is_some() {
                    None
                } else if vec![1, 2].contains(&body.supp_type.unwrap()) {
                    body.supp_type
                } else {
                    None
                }),
                handled_by: Set(Some(ext.driverid)),
                ..Default::default()
            };
            let statreturn = supplements::Entity::update(activemodel)
                .exec(db)
                .await
                .expect("[ERROR] Módosítás sikertelen");
            Ok(Json(SMGetItemsFull {
                faction: statreturn.faction,
                status: statreturn.status,
                date: statreturn.date,
                handled_by: statreturn.handled_by,
                reason: statreturn.reason,
                id: statreturn.id,
                img_1: statreturn.image,
                img_2: None,
                owner: statreturn.owner,
                price: None,
                r#type: statreturn.r#type,
                target_faction: None,
                driver: None,
                item_type: types.supplements.id,
            })
            .into_response())
        } else if body.tipus == types.hails.id
            && config
                .factions
                .get(&ext.faction.unwrap())
                .unwrap()
                .access
                .hails
                == ItemAccess::Write
        {
            let old_model = hails::Entity::find()
                .filter(hails::Column::Id.eq(body.id))
                .one(db)
                .await
                .expect("[ERROR] Régi lekérése sikertelen")
                .unwrap();
            let mut act = String::new();
            if old_model.status != body.status {
                act += format!(
                    "{}status FROM {} TO {}",
                    if act.len() > 0 { "; " } else { "" },
                    old_model.status,
                    body.status
                )
                .as_str();
            }
            if old_model.reason != body.reason {
                act += format!(
                    "{}reason FROM {} TO {}",
                    if act.len() > 0 { "; " } else { "" },
                    if old_model.reason.is_some() {
                        old_model.reason.unwrap().replace(";", "{saes_semicolon}")
                    } else {
                        String::from("null")
                    },
                    if body.reason.is_some() {
                        body.reason
                            .clone()
                            .unwrap()
                            .replace(";", "{saes_semicolon}")
                    } else {
                        String::from("null")
                    }
                )
                .as_str();
            }
            db_log(
                ext.driverid,
                Some(get_faction_id(ext.faction.unwrap())),
                Some(body.id.clone()),
                Some(types.hails.id),
                "UPDATE ITEM",
                Some(act),
            )
            .await;
            let activemodel = hails::ActiveModel {
                id: Set(body.id),
                faction: Set(get_faction_id(ext.faction.unwrap())),
                status: Set(body.status),
                reason: Set(body.reason),
                handled_by: Set(Some(ext.driverid)),
                ..Default::default()
            };
            let statreturn = hails::Entity::update(activemodel)
                .exec(db)
                .await
                .expect("[ERROR] Módosítás sikertelen");
            Ok(Json(SMGetItemsFull {
                faction: statreturn.faction,
                status: statreturn.status,
                date: statreturn.date,
                handled_by: statreturn.handled_by,
                reason: statreturn.reason,
                id: statreturn.id,
                img_1: statreturn.image_1,
                img_2: Some(statreturn.image_2),
                owner: statreturn.owner,
                r#type: None,
                price: None,
                target_faction: None,
                driver: None,
                item_type: types.hails.id,
            })
            .into_response())
        } else if body.tipus == types.bills.id
            && config
                .factions
                .get(&ext.faction.unwrap())
                .unwrap()
                .access
                .bills
                == ItemAccess::Write
        {
            let old_model = bills::Entity::find()
                .filter(bills::Column::Id.eq(body.id))
                .one(db)
                .await
                .expect("[ERROR] Régi lekérése sikertelen")
                .unwrap();
            let mut act = String::new();
            if old_model.status != body.status {
                act += format!(
                    "{}status FROM {} TO {}",
                    if act.len() > 0 { "; " } else { "" },
                    old_model.status,
                    body.status
                )
                .as_str();
            }
            if old_model.reason != body.reason {
                act += format!(
                    "{}reason FROM {} TO {}",
                    if act.len() > 0 { "; " } else { "" },
                    if old_model.reason.is_some() {
                        old_model.reason.unwrap().replace(";", "{saes_semicolon}")
                    } else {
                        String::from("null")
                    },
                    if body.reason.is_some() {
                        body.reason
                            .clone()
                            .unwrap()
                            .replace(";", "{saes_semicolon}")
                    } else {
                        String::from("null")
                    }
                )
                .as_str();
            }
            if old_model.price != body.price {
                act += format!(
                    "{}price FROM {} TO {}",
                    if act.len() > 0 { "; " } else { "" },
                    if old_model.price.is_some() {
                        old_model.price.unwrap()
                    } else {
                        0
                    },
                    if body.price.is_some() {
                        body.price.clone().unwrap()
                    } else {
                        0
                    }
                )
                .as_str();
            }
            if old_model.driver != body.driver {
                act += format!(
                    "{}driver FROM {} TO {}",
                    if act.len() > 0 { "; " } else { "" },
                    if old_model.driver.is_some() {
                        old_model.driver.unwrap().to_string()
                    } else {
                        String::from("null")
                    },
                    if body.driver.is_some() {
                        body.driver.clone().unwrap().to_string()
                    } else {
                        String::from("null")
                    }
                )
                .as_str();
            }
            if body.target_faction.is_some() {
                if old_model.target_faction != Some(get_faction_id(body.target_faction.unwrap())) {
                    act += format!(
                        "{}target_faction FROM {} TO {}",
                        if act.len() > 0 { "; " } else { "" },
                        if old_model.target_faction.is_some() {
                            old_model.target_faction.unwrap()
                        } else {
                            0
                        },
                        if body.target_faction.is_some() {
                            get_faction_id(body.target_faction.unwrap())
                        } else {
                            0
                        }
                    )
                    .as_str();
                }
            }
            db_log(
                ext.driverid,
                Some(get_faction_id(ext.faction.unwrap())),
                Some(body.id.clone()),
                Some(types.bills.id),
                "UPDATE ITEM",
                Some(act),
            )
            .await;
            let activemodel = bills::ActiveModel {
                id: Set(body.id),
                faction: Set(get_faction_id(ext.faction.unwrap())),
                status: Set(body.status),
                reason: Set(body.reason),
                target_faction: Set(if body.target_faction.is_some() {
                    Some(get_faction_id(body.target_faction.unwrap()))
                } else {
                    None
                }),
                driver: Set(body.driver),
                price: Set(body.price),
                handled_by: Set(Some(ext.driverid)),
                ..Default::default()
            };
            let statreturn = bills::Entity::update(activemodel)
                .exec(db)
                .await
                .expect("[ERROR] Módosítás sikertelen");
            Ok(Json(SMGetItemsFull {
                faction: statreturn.faction,
                status: statreturn.status,
                date: statreturn.date,
                handled_by: statreturn.handled_by,
                reason: statreturn.reason,
                id: statreturn.id,
                img_1: statreturn.image,
                img_2: None,
                owner: statreturn.owner,
                price: statreturn.price,
                driver: statreturn.driver,
                target_faction: statreturn.target_faction,
                r#type: None,
                item_type: types.bills.id,
            })
            .into_response())
        } else {
            Err((StatusCode::BAD_REQUEST, "Érvénytelen típus".to_string()))
        }
    } else {
        Err((StatusCode::NOT_FOUND, "Ejnye!".to_string()))
    }
}
