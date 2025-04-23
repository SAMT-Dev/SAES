use std::{
    env,
    fs::{self, File},
    path::Path,
};

use reqwest::StatusCode;
use saes_shared::structs::{
    factions::{get_factions_list, Factions},
    permissions::{get_perm, Permissions},
    user::Driver,
};
use tauri::{AppHandle, Emitter};
use tauri_plugin_dialog::DialogExt;
use tiny_http::{Response, Server};
use url::Url;

use crate::DISCORD_TOKEN;

use super::{
    config::{get_conf_path, load_config, save_config, Config},
    structs::UCPReturn,
};

#[tauri::command]
pub async fn begin_login(app: AppHandle) {
    let app2 = app.clone();
    tauri::async_runtime::spawn(async move {
        let server = Server::http("127.0.0.1:31313").unwrap();
        for req in server.incoming_requests() {
            let url: String = format!("http://127.0.0.1:31313{}", req.url());
            let parsed_url = Url::parse(&url).unwrap();
            let queries = parsed_url.query_pairs();
            for (key, value) in queries {
                if key == "code" {
                    if value == "noperm" {
                        app2.emit("loginFailed", "noperms").unwrap();
                        break;
                    }
                    let mut writer = DISCORD_TOKEN.write().await;
                    *writer = Some(value.to_string());
                    break;
                }
            }
            let response =
                Response::from_string("Bejelentkezés sikeres, kérlek lépj vissza az alkalmazásba!");
            return req.respond(response).unwrap();
        }
    })
    .await
    .unwrap();
    let client = reqwest::Client::new();
    let token = DISCORD_TOKEN.read().await;
    let api = get_api_url();
    let get = client
        .get(format!("{}/ucp", api))
        .header("cookie", token.clone().unwrap())
        .send()
        .await
        .unwrap();
    let data: Result<UCPReturn, reqwest::Error> = get.json().await;
    if data.is_ok() {
        let data = data.unwrap();
        if data.perms.contains(&"saes.login".to_string()) {
            app.emit("loginDone", format!("{}-{}", data.name, data.admin))
                .unwrap();
        } else {
            app.emit("loginFailed", "noperms").unwrap();
        }
    } else {
        app.emit("loginFailed", "unknown").unwrap();
    }
}

#[tauri::command]
pub async fn check_envs() -> String {
    let pat = get_conf_path();
    let realpat = format!("{}/.enverr", pat);
    let errcheck = Path::new(&realpat);
    let api = env::var("API_URL");
    if api.is_ok() {
        if errcheck.exists() {
            fs::remove_file(realpat).unwrap();
        }
        return String::from("ok");
    }
    if errcheck.exists() {
        return String::from("second");
    }
    File::create(&errcheck).unwrap();
    return String::from("first");
}

#[tauri::command]
pub fn get_api_url() -> String {
    env::var("API_URL").unwrap()
}

#[tauri::command]
pub async fn check_auth() -> bool {
    let auth = get_auth().await;
    return auth.is_some();
}

pub async fn get_auth() -> Option<Driver> {
    let conf = load_config().unwrap();
    let api = get_api_url();
    let client = reqwest::Client::new();
    let check = client
        .get(format!("{}/ucp", api))
        .header("cookie", conf.auth)
        .send()
        .await
        .unwrap();
    if StatusCode::is_success(&check.status()) {
        let json: Driver = check.json().await.unwrap();
        return Some(json);
    }
    return None;
}

#[tauri::command]
pub async fn set_game_dir(app: AppHandle) {
    app.dialog().file().pick_folder(move |folder| {
        if folder.is_some() {
            app.emit("selectedGameDir", folder.unwrap()).unwrap();
        }
    });
}

#[tauri::command]
pub async fn save_game_dir(dir: String) {
    let config = Config {
        auth: DISCORD_TOKEN.read().await.clone().unwrap(),
        game_dir: dir.clone(),
        faction: None,
    };
    save_config(config);
}

#[tauri::command]
pub async fn save_auth_token() {
    let conf = load_config().unwrap();
    let new_conf = Config {
        auth: DISCORD_TOKEN.read().await.clone().unwrap(),
        game_dir: conf.game_dir,
        faction: conf.faction,
    };
    save_config(new_conf);
}

#[tauri::command]
pub async fn done_setup(app: AppHandle) {
    app.restart();
}

#[tauri::command]
pub async fn check_faction() -> bool {
    let conf = load_config().unwrap();
    return conf.faction.is_some();
}

#[tauri::command]
pub async fn get_faction_options() -> Vec<Factions> {
    let auth = get_auth().await.unwrap();
    let facts = get_factions_list();
    facts
        .into_iter()
        .filter(|f| auth.perms.contains(&get_perm(Permissions::SaesUcp(*f))))
        .collect()
}

#[tauri::command]
pub async fn set_faction(faction: Factions) {
    let conf = load_config().unwrap();
    let new_conf = Config {
        auth: conf.auth,
        game_dir: conf.game_dir,
        faction: Some(faction),
    };
    save_config(new_conf);
}
