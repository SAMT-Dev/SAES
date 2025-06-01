use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

use axum::{
    Extension, Json, debug_handler,
    extract::{Multipart, Query},
    response::IntoResponse,
};

use chrono::DateTime;

use http::StatusCode;
use saes_shared::{db::images, db::images_bind, structs::user::Driver};
use serde::Deserialize;
use sha2::Digest;
use tokio::fs::remove_file;

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};

use crate::{
    DB_CLIENT,
    config::{editor::write_config, loader::get_config},
};

#[derive(Debug, Deserialize)]
pub struct SysChangeImage {
    pub faction: i8,
    pub dates: String,
}

#[debug_handler]
pub async fn sys_change_faction_image(
    ext: Extension<Driver>,
    cucc: Query<SysChangeImage>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let dates = cucc.dates.clone();
    let ditas: Vec<&str> = dates.split(",").collect();
    let mut ids = Vec::new();
    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();
        if field_name == "files" {
            let file_name = field.file_name().unwrap().to_string();
            let data = field.bytes().await;
            if data.is_ok() {
                let db = DB_CLIENT.get().unwrap();
                let mut real_file_name = [
                    format!("./public/{}-{}", ext.name, file_name),
                    format!("{}-{}", ext.name, file_name),
                ];
                let mut j = 1;
                loop {
                    if Path::new(&real_file_name[0]).exists() {
                        real_file_name[0] = format!("./public/{}-{}-{}", ext.name, j, file_name);
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
                let img = images::ActiveModel {
                    owner: Set(ext.driverid),
                    tmp: Set(0),
                    faction: Set(cucc.faction),
                    converted: Set(1),
                    filename: Set(real_file_name[1].clone()),
                    checksum: Set(Some(hash_text)),
                    date: Set(DateTime::from_timestamp_millis(ditas[0].parse().unwrap())
                        .unwrap()
                        .naive_utc()),
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
                let old_bind = images_bind::Entity::find()
                    .filter(images_bind::Column::TypeId.eq(cucc.faction))
                    .filter(images_bind::Column::Type.eq(4))
                    .one(db)
                    .await
                    .unwrap();
                if old_bind.is_some() {
                    let old_bind = old_bind.unwrap();
                    let oldmod = images_bind::ActiveModel {
                        id: Set(old_bind.id),
                        archived: Set(1),
                        ..Default::default()
                    };
                    images_bind::Entity::update(oldmod)
                        .exec(db)
                        .await
                        .expect("Old_Bind archive failed");
                }
                let newitem_bind = images_bind::ActiveModel {
                    image_id: Set(new_img),
                    r#type: Set(4),
                    type_id: Set(cucc.faction as i32),
                    ..Default::default()
                };
                images_bind::Entity::insert(newitem_bind)
                    .exec(db)
                    .await
                    .expect("BIND Create failed");
                let conf = get_config().await;
                let mut newconf = conf.clone();
                let fact = newconf
                    .factions
                    .get_mut(
                        conf.factions
                            .iter()
                            .find(|p| p.1.settings.id == cucc.faction)
                            .unwrap()
                            .0,
                    )
                    .unwrap();
                fact.settings.icon_id = new_img;
                write_config(&newconf).await;
                ids.push(new_img);
            }
        }
    }
    Ok(Json(ids))
}
