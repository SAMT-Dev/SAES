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
use saes_shared::{
    db::{bills, hails, images, images_bind, supplements},
    structs::{api_config::ItemAccess, user::Driver},
};
use sea_orm::{ColumnTrait, EntityTrait, Order, QueryFilter, QueryOrder, Set};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use tokio::fs::remove_file;

use crate::{
    config::loader::get_config,
    logging::db_log,
    modules::api::utils::{
        factions::get_faction_by_id,
        queries::{UCPTypeExtraQuery, UCPTypeQuery},
        types_statuses::{get_statuses, get_types, get_types_as_list},
    },
    DB_CLIENT,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemsStruct {
    pub id: i32,
    pub owner: i32,
    pub img_1: i32,
    pub img_2: Option<i32>,
    pub status: i8,
    pub driver: Option<i32>,
    pub reason: Option<String>,
    pub faction: String,
    pub handled_by: Option<i32>,
    pub price: Option<i32>,
    pub date: chrono::DateTime<Utc>,
}

pub fn routes() -> Router {
    Router::new()
        .route("/get", get(ucp_items_get))
        .route("/post", post(ucp_items_post))
        .route("/get_by_hash", get(get_items_by_hash))
        .route("/get_item_by_id", get(get_item_info_by_id))
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
    if ext.faction.is_none() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Frakciójelölés hiányzik!".to_string(),
        ));
    }
    if cucc.tipus == types.supplements.id
        && (config
            .factions
            .get(&ext.faction.clone().unwrap())
            .unwrap()
            .access
            .supplements
            == ItemAccess::Read
            || config
                .factions
                .get(&ext.faction.clone().unwrap())
                .unwrap()
                .access
                .supplements
                == ItemAccess::Write)
    {
        let items = supplements::Entity::find()
            .filter(supplements::Column::Owner.eq(ext.driverid))
            .filter(
                supplements::Column::Faction.eq(config
                    .factions
                    .get(&ext.faction.clone().unwrap())
                    .unwrap()
                    .settings
                    .id),
            )
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
                    faction: ext.faction.clone().unwrap(),
                    img_2: None,
                    driver: None,
                    reason: strucc.reason.clone(),
                    status: strucc.status,
                    date: strucc.date,
                    price: None,
                    id: strucc.id,
                    handled_by: strucc.handled_by.clone(),
                }
            })
            .collect();
        return Ok(Json(another));
    } else if cucc.tipus == types.hails.id
        && (config
            .factions
            .get(&ext.faction.clone().unwrap())
            .unwrap()
            .access
            .hails
            == ItemAccess::Read
            || config
                .factions
                .get(&ext.faction.clone().unwrap())
                .unwrap()
                .access
                .hails
                == ItemAccess::Write)
    {
        let items = hails::Entity::find()
            .filter(hails::Column::Owner.eq(ext.driverid))
            .filter(
                hails::Column::Faction.eq(config
                    .factions
                    .get(&ext.faction.clone().unwrap())
                    .unwrap()
                    .settings
                    .id),
            )
            .order_by(hails::Column::Date, Order::Desc)
            .all(db)
            .await
            .expect("Pótlékok lekérése sikertelen az adatbázisból");
        let another: Vec<ItemsStruct> = items
            .iter()
            .map(|strucc| -> ItemsStruct {
                ItemsStruct {
                    faction: ext.faction.clone().unwrap(),
                    owner: strucc.owner.clone(),
                    img_1: strucc.image_1,
                    img_2: Some(strucc.image_2),
                    reason: strucc.reason.clone(),
                    driver: None,
                    status: strucc.status,
                    date: strucc.date,
                    id: strucc.id,
                    price: None,
                    handled_by: strucc.handled_by.clone(),
                }
            })
            .collect();
        return Ok(Json(another));
    } else if cucc.tipus == types.bills.id
        && (config
            .factions
            .get(&ext.faction.clone().unwrap())
            .unwrap()
            .access
            .bills
            == ItemAccess::Read
            || config
                .factions
                .get(&ext.faction.clone().unwrap())
                .unwrap()
                .access
                .bills
                == ItemAccess::Write)
    {
        let items = bills::Entity::find()
            .filter(
                bills::Column::Owner
                    .eq(ext.driverid)
                    .or(bills::Column::Driver.eq(ext.driverid)),
            )
            .filter(
                bills::Column::Faction.eq(config
                    .factions
                    .get(&ext.faction.clone().unwrap())
                    .unwrap()
                    .settings
                    .id),
            )
            .order_by(bills::Column::Date, Order::Desc)
            .all(db)
            .await
            .expect("Pótlékok lekérése sikertelen az adatbázisból");
        let another: Vec<ItemsStruct> = items
            .iter()
            .map(|strucc| -> ItemsStruct {
                ItemsStruct {
                    faction: ext.faction.clone().unwrap(),
                    owner: strucc.owner,
                    img_1: strucc.image,
                    img_2: None,
                    reason: strucc.reason.clone(),
                    status: strucc.status,
                    date: strucc.date,
                    driver: strucc.driver,
                    id: strucc.id,
                    price: strucc.price,
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
                                .get(&ext.faction.clone().unwrap())
                                .unwrap()
                                .access
                                .hails
                                == ItemAccess::Write
                        {
                            if files_for_leintes.len().eq(&1) {
                                let img = images::ActiveModel {
                                    owner: Set(ext.driverid),
                                    tmp: Set(1),
                                    faction: Set(config
                                        .factions
                                        .get(&ext.faction.clone().unwrap())
                                        .unwrap()
                                        .settings
                                        .id),
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
                                    faction: Set(config
                                        .factions
                                        .get(&ext.faction.clone().unwrap())
                                        .unwrap()
                                        .settings
                                        .id),
                                    date: Set(DateTime::from_timestamp_millis(
                                        ditas[i].parse().unwrap(),
                                    )
                                    .unwrap()),
                                    owner: Set(ext.driverid),
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
                                    ext.driverid,
                                    Some(
                                        config
                                            .factions
                                            .get(&ext.faction.clone().unwrap())
                                            .unwrap()
                                            .settings
                                            .id,
                                    ),
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
                                    owner: Set(ext.driverid),
                                    filename: Set(real_file_name[1].clone()),
                                    checksum: Set(Some(hash_text)),
                                    faction: Set(config
                                        .factions
                                        .get(&ext.faction.clone().unwrap())
                                        .unwrap()
                                        .settings
                                        .id),
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
                                .get(&ext.faction.clone().unwrap())
                                .unwrap()
                                .access
                                .supplements
                                == ItemAccess::Write
                        {
                            let img = images::ActiveModel {
                                owner: Set(ext.driverid),
                                tmp: Set(1),
                                filename: Set(real_file_name[1].clone()),
                                checksum: Set(Some(hash_text)),
                                faction: Set(config
                                    .factions
                                    .get(&ext.faction.clone().unwrap())
                                    .unwrap()
                                    .settings
                                    .id),
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
                                faction: Set(config
                                    .factions
                                    .get(&ext.faction.clone().unwrap())
                                    .unwrap()
                                    .settings
                                    .id),
                                date: Set(DateTime::from_timestamp_millis(
                                    ditas[i].parse().unwrap(),
                                )
                                .unwrap()),
                                owner: Set(ext.driverid),
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
                                ext.driverid,
                                Some(
                                    config
                                        .factions
                                        .get(&ext.faction.clone().unwrap())
                                        .unwrap()
                                        .settings
                                        .id,
                                ),
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
                                .get(&ext.faction.clone().unwrap())
                                .unwrap()
                                .access
                                .bills
                                == ItemAccess::Write
                        {
                            let img = images::ActiveModel {
                                owner: Set(ext.driverid),
                                faction: Set(config
                                    .factions
                                    .get(&ext.faction.clone().unwrap())
                                    .unwrap()
                                    .settings
                                    .id),
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
                                faction: Set(config
                                    .factions
                                    .get(&ext.faction.clone().unwrap())
                                    .unwrap()
                                    .settings
                                    .id),
                                date: Set(DateTime::from_timestamp_millis(
                                    ditas[i].parse().unwrap(),
                                )
                                .unwrap()),
                                owner: Set(ext.driverid),
                                driver: Set(Some(ext.driverid)),
                                target_faction: Set(Some(
                                    config
                                        .factions
                                        .get(&ext.faction.clone().unwrap())
                                        .unwrap()
                                        .settings
                                        .id,
                                )),
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
                                ext.driverid,
                                Some(
                                    config
                                        .factions
                                        .get(&ext.faction.clone().unwrap())
                                        .unwrap()
                                        .settings
                                        .id,
                                ),
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

#[derive(Debug, Deserialize)]
pub struct GetItemsByHash {
    pub hash: String,
}

pub async fn get_items_by_hash(
    ext: Extension<Driver>,
    q: Query<GetItemsByHash>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db = DB_CLIENT.get().unwrap();
    let image = images::Entity::find()
        .filter(images::Column::Checksum.eq(q.hash.clone()))
        .filter(images::Column::Owner.eq(ext.driverid))
        .one(db)
        .await
        .unwrap();
    if image.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            "Ez a hash nem tartozik egy feltöltött képhez!".to_string(),
        ));
    }
    let binds = images_bind::Entity::find()
        .filter(images_bind::Column::ImageId.eq(image.unwrap().id))
        .all(db)
        .await
        .unwrap();
    return Ok(Json(binds));
}

#[derive(Debug, Deserialize)]
pub struct ItemInfo {
    pub item_type: i8,
    pub item_id: i32,
}

#[debug_handler]
pub async fn get_item_info_by_id(
    ext: Extension<Driver>,
    q: Query<ItemInfo>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let db = DB_CLIENT.get().unwrap();
    let types = get_types();
    if q.item_type == types.supplements.id {
        let item = supplements::Entity::find()
            .filter(supplements::Column::Owner.eq(ext.driverid))
            .filter(supplements::Column::Id.eq(q.item_id))
            .one(db)
            .await
            .unwrap();
        if item.is_none() {
            return Err((StatusCode::NOT_FOUND, "Elem nem található!".to_string()));
        }
        let item = item.unwrap();
        return Ok(Json(ItemsStruct {
            date: item.date,
            driver: None,
            faction: get_faction_by_id(item.faction).await.unwrap(),
            handled_by: item.handled_by,
            id: item.id,
            img_1: item.image,
            img_2: None,
            owner: item.owner,
            price: None,
            reason: item.reason,
            status: item.status,
        }));
    }
    if q.item_type == types.hails.id {
        let item = hails::Entity::find()
            .filter(hails::Column::Owner.eq(ext.driverid))
            .filter(hails::Column::Id.eq(q.item_id))
            .one(db)
            .await
            .unwrap();
        if item.is_none() {
            return Err((StatusCode::NOT_FOUND, "Elem nem található!".to_string()));
        }
        let item = item.unwrap();
        return Ok(Json(ItemsStruct {
            date: item.date,
            driver: None,
            faction: get_faction_by_id(item.faction).await.unwrap(),
            handled_by: item.handled_by,
            id: item.id,
            img_1: item.image_1,
            img_2: Some(item.image_2),
            owner: item.owner,
            price: None,
            reason: item.reason,
            status: item.status,
        }));
    }
    if q.item_type == types.bills.id {
        let item = bills::Entity::find()
            .filter(bills::Column::Owner.eq(ext.driverid))
            .filter(bills::Column::Id.eq(q.item_id))
            .one(db)
            .await
            .unwrap();
        if item.is_none() {
            return Err((StatusCode::NOT_FOUND, "Elem nem található!".to_string()));
        }
        let item = item.unwrap();
        return Ok(Json(ItemsStruct {
            date: item.date,
            driver: None,
            faction: get_faction_by_id(item.faction).await.unwrap(),
            handled_by: item.handled_by,
            id: item.id,
            img_1: item.image,
            img_2: None,
            owner: item.owner,
            price: item.price,
            reason: item.reason,
            status: item.status,
        }));
    }
    return Err((StatusCode::NOT_FOUND, "".to_string()));
}
