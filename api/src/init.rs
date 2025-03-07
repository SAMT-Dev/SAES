use core::panic;
use std::{
    fs::{self, File},
    path::Path,
};

use saes_shared::sql::test_db_conn;

use crate::config::loader::get_config;

async fn stores_dir_init() {
    let stores_dir = Path::new("stores");
    if !stores_dir.exists() {
        fs::create_dir(stores_dir).expect("Store dir létrehozása sikertelen");
    }
    let maintenance_path = Path::new("stores/maintenance.store");
    if maintenance_path.exists() == false {
        File::create(maintenance_path).expect("maintenance.store létrehozása sikertelen");
    }
    let announcement_path = Path::new("stores/announcement.store");
    if announcement_path.exists() == false {
        File::create(announcement_path).expect("announcement.store létrehozása sikertelen");
    }
}

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
    stores_dir_init().await;
    image_init();
    image_tmp_init();
}
