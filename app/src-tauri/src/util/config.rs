use std::{
    env::consts::OS,
    fs::{create_dir, File},
    io::Write,
    path::Path,
};

use homedir::my_home;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub auth: String,
    pub game_dir: String,
    pub faction: Option<String>,
}

pub fn get_conf_path() -> String {
    let os = OS;
    let dir = my_home().unwrap().unwrap();
    let dir_str = dir.to_str().unwrap().to_string();
    let main_folder_path = if os == "windows" {
        format!("{}/AppData/Roaming/SAMT_App", dir_str)
    } else {
        format!("{}/.config/SAMT_App", dir_str)
    };
    return main_folder_path;
}

pub fn setup_folders() {
    let pat = get_conf_path();
    let main_folder = Path::new(&pat);
    if !main_folder.exists() {
        create_dir(main_folder).unwrap();
    }
}

pub fn load_config() -> Option<Config> {
    let pat = get_conf_path();
    let config = File::open(format!("{}/config.json", pat));
    if config.is_err() {
        return None;
    }
    let file = config.unwrap();
    let real_config: Option<Config> = serde_json::from_reader(file).ok();
    if real_config.is_none() {
        return None;
    }
    return Some(real_config.unwrap());
}

pub fn save_config(config: Config) {
    let pat = get_conf_path();
    let config_str = serde_json::to_string(&config).unwrap();
    let mut file = File::create(format!("{}/config.json", pat)).unwrap();
    file.write(config_str.as_bytes()).unwrap();
}
