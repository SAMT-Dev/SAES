use core::panic;
use std::{
    fs::{self},
    path::Path,
};

use saes_shared::sql::test_db_conn;

use crate::config::loader::get_config;

fn image_init() {
    let image_dir = Path::new("public");
    if !image_dir.exists() {
        let mkdir = fs::create_dir(image_dir);
        if mkdir.is_err() {
            panic!("public dir létrehozása sikertelen");
        }
    }
}

fn image_tmp_init() {
    let image_tmp_dir = Path::new("public/tmp");
    if !image_tmp_dir.exists() {
        let mkdir = fs::create_dir(image_tmp_dir);
        if mkdir.is_err() {
            panic!("public/tmp dir létrehozása sikertelen");
        }
    }
}

pub async fn main() {
    get_config().await;
    test_db_conn().await;
    image_init();
    image_tmp_init();
}
