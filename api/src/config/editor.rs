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
