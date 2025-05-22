use std::path::Path;

use serde_json::to_string_pretty;
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

use crate::modules::api::SOCKET_IO;

use super::structs::{MainConfig, ModuleConfig};

pub async fn write_config(config: &MainConfig) {
    let config_file = Path::new("./config/default.json");
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

pub async fn write_module_config(config: &ModuleConfig) {
    let config_file = Path::new("./config/modules.json");
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(config_file)
        .await
        .unwrap();
    let json = to_string_pretty(config).unwrap();
    let buf = json.as_bytes();
    file.write(buf).await.unwrap();
}
