use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Read, Write},
    path::Path,
};

use base64::{prelude::BASE64_STANDARD, Engine};
use sha2::Digest;
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

use crate::util::config::{get_conf_path, load_config};

pub fn check_hashfile() -> bool {
    let pat = get_conf_path();
    let hash_pat = format!("{}/hash.json", pat);
    let hashfle = Path::new(&hash_pat);
    hashfle.exists()
}

pub fn get_file_hash(path: String) -> String {
    let mut s256 = sha2::Sha256::new();
    io::copy(&mut File::open(path).unwrap(), &mut s256).unwrap();
    let hash = s256.finalize();
    format!("{:x}", hash)
}

pub fn create_empty_hashfile() {
    let pat = get_conf_path();
    let mut file = File::create(format!("{}/hash.json", pat)).unwrap();
    file.write(b"{}").unwrap();
}

#[tauri::command]
pub async fn get_image(path: String) -> String {
    let conf = load_config().unwrap();
    let img = format!("{}/screenshots/{}", conf.game_dir, path);
    let img_path = Path::new(&img);
    if !img_path.exists() {
        return "Nem létezik a fájl".to_string();
    }
    let mut file = File::open(img_path).map_err(|e| e.to_string()).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| e.to_string())
        .unwrap();
    BASE64_STANDARD.encode(buffer)
}

#[tauri::command]
pub fn get_images() -> Vec<String> {
    let conf = load_config().unwrap();
    let dir = format!("{}/screenshots", conf.game_dir);
    let path = Path::new(&dir);
    match fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok()) // Filter out errors
            .filter(|entry| entry.path().is_file()) // Only include files
            .filter_map(|entry| entry.file_name().into_string().ok()) // Convert OsString to String
            .collect(),
        Err(_) => vec!["Failed to read directory".to_string()],
    }
}

pub fn get_hashes() -> HashMap<String, String> {
    let pat = get_conf_path();
    let mut hashfile = File::open(format!("{}/hash.json", pat)).unwrap();
    let mut buf = String::new();
    hashfile.read_to_string(&mut buf).unwrap();
    let hashes: HashMap<String, String> = serde_json::from_str(&buf).unwrap();
    hashes
}

#[tauri::command]
pub async fn get_image_hash(path: String) -> String {
    let conf = load_config().unwrap();
    get_file_hash(format!("{}/screenshots/{}", conf.game_dir, path))
}

#[tauri::command]
pub async fn check_hash() {
    let conf = load_config().unwrap();
    let pat = get_conf_path();
    if !check_hashfile() {
        create_empty_hashfile();
    }
    let images = get_images();
    let mut hashes = get_hashes();
    for image in images.iter() {
        if hashes.get(image).is_none() {
            let hash = get_file_hash(format!("{}/screenshots/{}", conf.game_dir, image));
            hashes.insert(image.to_owned(), hash);
        }
    }
    let noice = serde_json::to_string_pretty(&hashes).unwrap();
    let mut hashfile = OpenOptions::new()
        .write(true)
        .open(format!("{}/hash.json", pat))
        .await
        .unwrap();
    hashfile.write(noice.as_bytes()).await.unwrap();
}
