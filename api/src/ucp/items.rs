use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use axum::{
    debug_handler,
    extract::{DefaultBodyLimit, Multipart, Query},
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use saes_shared::db::{bills, hails, images, images_bind, supplements};
use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder, Set};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use tokio::fs::remove_file;

use crate::{
    config::{loader::get_config, structs::ItemAccess},
    logging::db_log,
    utils::{
        factions::{get_faction_id, Factions},
        middle::Driver,
        queries::{UCPTypeExtraQuery, UCPTypeQuery},
        types_statuses::{get_statuses, get_types, get_types_as_list},
    },
    DB_CLIENT,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemsStruct {
    pub id: i32,
    pub owner: String,
    pub img_1: i32,
    pub img_2: Option<i32>,
    pub status: i8,
    pub reason: Option<String>,
    pub faction: Factions,
    pub handled_by: Option<String>,
    pub date: chrono::DateTime<Utc>,
}

pub fn routes() -> Router {
    Router::new()
        .route("/get", get(ucp_items_get))
        .route("/post", post(ucp_items_post))
        .layer(DefaultBodyLimit::max(64000000))
}

#[debug_handler]
pub async fn ucp_items_get(
    ext: Extension<Driver>,
    cucc: Query<UCPTypeQuery>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db = DB_CLIENT.get().unwrap();
    let types = get_types();
    let config = get_config().await;
    if ext.faction.is_some() {
        if cucc.tipus == types.supplements.id
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
            let items = supplements::Entity::find()
                .filter(supplements::Column::Owner.eq(&ext.name))
                .filter(supplements::Column::Faction.eq(get_faction_id(ext.faction.unwrap())))
                .order_by(supplements::Column::Date, Order::Desc)
                .all(db)
                .await
                .expect("Pótlékok lekérése sikertelen az adatbázisból");
            let another: Vec<ItemsStruct> = items
                .iter()
                .map(|strucc| -> ItemsStruct {
                    ItemsStruct {
                        owner: strucc.owner.clone(),
                        img_1: strucc.image,
                        faction: ext.faction.unwrap(),
                        img_2: None,
                        reason: strucc.reason.clone(),
                        status: strucc.status,
                        date: strucc.date,
                        id: strucc.id,
                        handled_by: strucc.handled_by.clone(),
                    }
                })
                .collect();
            return Ok(Json(another));
        } else if cucc.tipus == types.hails.id
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
            let items = hails::Entity::find()
                .filter(hails::Column::Owner.eq(&ext.name))
                .filter(hails::Column::Faction.eq(get_faction_id(ext.faction.unwrap())))
                .order_by(hails::Column::Date, Order::Desc)
                .all(db)
                .await
                .expect("Pótlékok lekérése sikertelen az adatbázisból");
            let another: Vec<ItemsStruct> = items
                .iter()
                .map(|strucc| -> ItemsStruct {
                    ItemsStruct {
                        faction: ext.faction.unwrap(),
                        owner: strucc.owner.clone(),
                        img_1: strucc.image_1,
                        img_2: Some(strucc.image_2),
                        reason: strucc.reason.clone(),
                        status: strucc.status,
                        date: strucc.date,
                        id: strucc.id,
                        handled_by: strucc.handled_by.clone(),
                    }
                })
                .collect();
            return Ok(Json(another));
        } else if cucc.tipus == types.bills.id
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
            let items = bills::Entity::find()
                .filter(bills::Column::Owner.eq(&ext.name))
                .filter(bills::Column::Faction.eq(get_faction_id(ext.faction.unwrap())))
                .order_by(bills::Column::Date, Order::Desc)
                .all(db)
                .await
                .expect("Pótlékok lekérése sikertelen az adatbázisból");
            let another: Vec<ItemsStruct> = items
                .iter()
                .map(|strucc| -> ItemsStruct {
                    ItemsStruct {
                        faction: ext.faction.unwrap(),
                        owner: strucc.owner.clone(),
                        img_1: strucc.image,
                        img_2: None,
                        reason: strucc.reason.clone(),
                        status: strucc.status,
                        date: strucc.date,
                        id: strucc.id,
                        handled_by: strucc.handled_by.clone(),
                    }
                })
                .collect();
            return Ok(Json(another));
        } else {
            return Err((
                StatusCode::NOT_FOUND,
                "Ilyen típus nem található!".to_string(),
            ));
        }
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            "Frakciójelölés hiányzik!".to_string(),
        ));
    }
}

#[debug_handler]
pub async fn ucp_items_post(
    ext: Extension<Driver>,
    cucc: Query<UCPTypeExtraQuery>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let mut file_ids: Vec<[i32; 2]> = Vec::new();
    let mut files_for_leintes: Vec<i32> = vec![];
    let dates = cucc.dates.clone();
    let ditas: Vec<&str> = dates.split(",").collect();
    let types = get_types();
    let config = get_config().await;
    let statuses = get_statuses();
    let types_list = get_types_as_list();
    let mut i = 0;
    if ext.faction.is_some() {
        if types_list.contains(&cucc.tipus.clone()) {
            while let Some(field) = multipart.next_field().await.unwrap() {
                let field_name = field.name().unwrap().to_string();
                if field_name == "files" {
                    let file_name = field.file_name().unwrap().to_string();
                    let data = field.bytes().await;
                    if data.is_ok() {
                        let db = DB_CLIENT.get().unwrap();
                        let mut real_file_name = [
                            format!("./public/tmp/{}-{}", ext.name, file_name),
                            format!("{}-{}", ext.name, file_name),
                        ];
                        let mut j = 1;
                        loop {
                            if Path::new(&real_file_name[0]).exists() {
                                real_file_name[0] =
                                    format!("./public/tmp/{}-{}-{}", ext.name, j, file_name);
                                real_file_name[1] = format!("{}-{}-{}", ext.name, j, file_name);
                                j += 1;
                            } else {
                                break;
                            }
                        }
                        let mut file = File::create(real_file_name[0].clone()).unwrap();
                        file.write(&data.unwrap()).unwrap();
                        let mut s256 = sha2::Sha256::new();
                        io::copy(
                            &mut File::open(real_file_name[0].clone()).unwrap(),
                            &mut s256,
                        )
                        .unwrap();
                        let hash = s256.finalize();
                        let hash_text = format!("{:x}", hash);
                        let same_file = images::Entity::find()
                            .filter(images::Column::Checksum.eq(hash_text.clone()))
                            .one(db)
                            .await
                            .expect("Fájl ellenőrzése sikertelen");
                        if same_file.is_some() {
                            remove_file(real_file_name[0].clone()).await.unwrap();
                        }
                        if cucc.tipus == types.hails.id
                            && config
                                .factions
                                .get(&ext.faction.unwrap())
                                .unwrap()
                                .access
                                .hails
                                == ItemAccess::Write
                        {
                            if files_for_leintes.len().eq(&1) {
                                let img = images::ActiveModel {
                                    owner: Set(ext.name.clone()),
                                    tmp: Set(1),
                                    faction: Set(get_faction_id(ext.faction.unwrap())),
                                    filename: Set(real_file_name[1].clone()),
                                    checksum: Set(Some(hash_text)),
                                    date: Set(DateTime::from_timestamp_millis(
                                        ditas[i].parse().unwrap(),
                                    )
                                    .unwrap()),
                                    ..Default::default()
                                };
                                let new_img = if same_file.is_none() {
                                    images::Entity::insert(img)
                                        .exec(db)
                                        .await
                                        .expect("Fájl mentése sikertelen")
                                        .last_insert_id
                                } else {
                                    same_file.unwrap().id
                                };

                                let iten = hails::ActiveModel {
                                    faction: Set(get_faction_id(ext.faction.unwrap())),
                                    date: Set(DateTime::from_timestamp_millis(
                                        ditas[i].parse().unwrap(),
                                    )
                                    .unwrap()),
                                    owner: Set(ext.name.clone()),
                                    status: Set(statuses.uploaded.id),
                                    image_1: Set(files_for_leintes[0]),
                                    image_2: Set(new_img),
                                    ..Default::default()
                                };
                                let newitem = hails::Entity::insert(iten)
                                    .exec(db)
                                    .await
                                    .expect("Adatbázisba mentés sikertelen");
                                let newitem_bind = images_bind::ActiveModel {
                                    image_id: Set(new_img),
                                    r#type: Set(types.hails.id),
                                    type_id: Set(newitem.last_insert_id),
                                    ..Default::default()
                                };
                                images_bind::Entity::insert(newitem_bind)
                                    .exec(db)
                                    .await
                                    .expect("BIND Create failed");
                                let newitem2_bind = images_bind::ActiveModel {
                                    image_id: Set(files_for_leintes[0]),
                                    r#type: Set(types.hails.id),
                                    type_id: Set(newitem.last_insert_id),
                                    ..Default::default()
                                };
                                images_bind::Entity::insert(newitem2_bind)
                                    .exec(db)
                                    .await
                                    .expect("BIND Create failed");
                                db_log(
                                    ext.name.clone(),
                                    Some(get_faction_id(ext.faction.unwrap())),
                                    Some(newitem.last_insert_id),
                                    Some(types.hails.id),
                                    "UPLOAD ITEM",
                                    None,
                                )
                                .await;
                                file_ids.push([new_img, files_for_leintes[0]]);
                                files_for_leintes.clear();
                            } else {
                                let img = images::ActiveModel {
                                    owner: Set(ext.name.clone()),
                                    filename: Set(real_file_name[1].clone()),
                                    checksum: Set(Some(hash_text)),
                                    faction: Set(get_faction_id(ext.faction.unwrap())),
                                    tmp: Set(1),
                                    date: Set(DateTime::from_timestamp_millis(
                                        ditas[i].parse().unwrap(),
                                    )
                                    .unwrap()),
                                    ..Default::default()
                                };
                                let new_img = if same_file.is_none() {
                                    images::Entity::insert(img)
                                        .exec(db)
                                        .await
                                        .expect("Fájl mentése sikertelen")
                                        .last_insert_id
                                } else {
                                    same_file.unwrap().id
                                };
                                files_for_leintes.push(new_img)
                            }
                        } else if cucc.tipus == types.supplements.id
                            && config
                                .factions
                                .get(&ext.faction.unwrap())
                                .unwrap()
                                .access
                                .supplements
                                == ItemAccess::Write
                        {
                            let img = images::ActiveModel {
                                owner: Set(ext.name.clone()),
                                tmp: Set(1),
                                filename: Set(real_file_name[1].clone()),
                                checksum: Set(Some(hash_text)),
                                faction: Set(get_faction_id(ext.faction.unwrap())),
                                date: Set(DateTime::from_timestamp_millis(
                                    ditas[i].parse().unwrap(),
                                )
                                .unwrap()),
                                ..Default::default()
                            };
                            let new_img = if same_file.is_none() {
                                images::Entity::insert(img)
                                    .exec(db)
                                    .await
                                    .expect("Fájl mentése sikertelen")
                                    .last_insert_id
                            } else {
                                same_file.unwrap().id
                            };
                            let iten = supplements::ActiveModel {
                                faction: Set(get_faction_id(ext.faction.unwrap())),
                                date: Set(DateTime::from_timestamp_millis(
                                    ditas[i].parse().unwrap(),
                                )
                                .unwrap()),
                                owner: Set(ext.name.clone()),
                                status: Set(statuses.uploaded.id),
                                image: Set(new_img),
                                ..Default::default()
                            };
                            let newitem = supplements::Entity::insert(iten)
                                .exec(db)
                                .await
                                .expect("Adatbázisba mentés sikertelen");
                            let newitem_bind = images_bind::ActiveModel {
                                image_id: Set(new_img),
                                r#type: Set(types.supplements.id),
                                type_id: Set(newitem.last_insert_id),
                                ..Default::default()
                            };
                            images_bind::Entity::insert(newitem_bind)
                                .exec(db)
                                .await
                                .expect("BIND Create failed");
                            db_log(
                                ext.name.clone(),
                                Some(get_faction_id(ext.faction.unwrap())),
                                Some(newitem.last_insert_id),
                                Some(types.supplements.id),
                                "UPLOAD ITEM",
                                None,
                            )
                            .await;
                            file_ids.push([new_img, 0])
                        } else if cucc.tipus == types.bills.id
                            && config
                                .factions
                                .get(&ext.faction.unwrap())
                                .unwrap()
                                .access
                                .bills
                                == ItemAccess::Write
                        {
                            let img = images::ActiveModel {
                                owner: Set(ext.name.clone()),
                                faction: Set(get_faction_id(ext.faction.unwrap())),
                                checksum: Set(Some(hash_text)),
                                tmp: Set(1),
                                filename: Set(real_file_name[1].clone()),
                                date: Set(DateTime::from_timestamp_millis(
                                    ditas[i].parse().unwrap(),
                                )
                                .unwrap()),
                                ..Default::default()
                            };
                            let new_img = if same_file.is_none() {
                                images::Entity::insert(img)
                                    .exec(db)
                                    .await
                                    .expect("Fájl mentése sikertelen")
                                    .last_insert_id
                            } else {
                                same_file.unwrap().id
                            };
                            let iten = bills::ActiveModel {
                                faction: Set(get_faction_id(ext.faction.unwrap())),
                                date: Set(DateTime::from_timestamp_millis(
                                    ditas[i].parse().unwrap(),
                                )
                                .unwrap()),
                                owner: Set(ext.name.clone()),
                                status: Set(statuses.uploaded.id),
                                image: Set(new_img),
                                ..Default::default()
                            };
                            let newitem = bills::Entity::insert(iten)
                                .exec(db)
                                .await
                                .expect("Adatbázisba mentés sikertelen");
                            let newitem_bind = images_bind::ActiveModel {
                                image_id: Set(new_img),
                                r#type: Set(types.bills.id),
                                type_id: Set(newitem.last_insert_id),
                                ..Default::default()
                            };
                            images_bind::Entity::insert(newitem_bind)
                                .exec(db)
                                .await
                                .expect("BIND Create failed");
                            db_log(
                                ext.name.clone(),
                                Some(get_faction_id(ext.faction.unwrap())),
                                Some(newitem.last_insert_id),
                                Some(types.bills.id),
                                "UPLOAD ITEM",
                                None,
                            )
                            .await;
                            file_ids.push([new_img, 0])
                        }
                        i += 1
                    } else {
                        return Err((StatusCode::NOT_ACCEPTABLE, "toobig".to_string()));
                    }
                } else {
                    let data = field.text().await.unwrap();
                    println!("field: {}   value: {}", field_name, data)
                }
            }
            Ok(Json(file_ids))
        } else {
            return Err((StatusCode::NOT_ACCEPTABLE, "invalid_type".to_string()));
        }
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            "Frakciójelölés hiányzik!".to_string(),
        ));
    }
}
