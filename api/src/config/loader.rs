use std::path::Path;

use serde_json::to_string_pretty;
use tokio::{
    fs::{self, File, OpenOptions},
    io::AsyncWriteExt,
};

use crate::SOCKET_IO;

use super::structs::MainConfig;

pub async fn get_config() -> MainConfig {
    let dir = Path::new("./config");
    if !dir.exists() {
        fs::create_dir(dir).await.unwrap();
    }
    let config_file = Path::new("./config/main.json");
    if !config_file.exists() {
        let mut file = File::create(config_file).await.unwrap();
        let config = MainConfig::default();
        let json = to_string_pretty(&config).unwrap();
        let buf = json.as_bytes();
        file.write_all(buf).await.unwrap();
    }
    let config = File::open(config_file).await.unwrap();
    let config: MainConfig = serde_json::from_reader(config.into_std().await).unwrap();
    config
}

pub async fn write_config(config: &MainConfig) {
    let config_file = Path::new("./config/main.json");
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(config_file)
        .await
        .unwrap();
    let json = to_string_pretty(config).unwrap();
    let buf = json.as_bytes();
    file.write(buf).await.unwrap();
    let io = SOCKET_IO.get().unwrap();
    io.emit("maintenance", &config.global.maintenance)
        .await
        .expect("Failed to resend maintenance");
    io.emit("announcement", &config.global.announcement)
        .await
        .expect("Failed to resend announcement");
}
