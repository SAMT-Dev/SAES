use std::{collections::HashMap, env, thread, time::Duration};

use reqwest::StatusCode;
use saes_shared::structs::{
    api_config::PubFactionReturn,
    permissions::{get_perm, Permissions},
    user::Driver,
};
use serde::Deserialize;
use tauri::{AppHandle, Emitter};
use tauri_plugin_dialog::DialogExt;
use tiny_http::{Response, Server};
use url::Url;

use crate::{util::structs::BundledDriver, AUTH, DISCORD_TOKEN, JWT_TOKEN, WEB_CLIENT};

use super::{
    config::{load_config, save_config, Config},
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
    let api = get_api_url();
    let token = DISCORD_TOKEN.read().await;
    let getjwt = client
        .get(format!("{}/auth/jwt", api))
        .header("cookie", token.clone().unwrap())
        .send()
        .await;
    if getjwt.is_err() {
        app.emit("loginFailed", "unknown").unwrap();
        return;
    }
    let jwt: JWTRet = getjwt.unwrap().json().await.unwrap();
    let mut jwtwriter = JWT_TOKEN.write().await;
    *jwtwriter = Some(jwt.jwt.clone());
    let get = client
        .get(format!("{}/ucp", api))
        .header("cookie", jwt.jwt)
        .send()
        .await
        .unwrap();
    let data: Result<UCPReturn, reqwest::Error> = get.json().await;
    if data.is_ok() {
        let data = data.unwrap();
        if data.driver.perms.contains(&"saes.login".to_string()) {
            app.emit(
                "loginDone",
                format!("{}-{}", data.driver.name, data.driver.admin),
            )
            .unwrap();
        } else {
            app.emit("loginFailed", "noperms").unwrap();
        }
    } else {
        app.emit("loginFailed", format!("unknown/{:?}", data.unwrap_err()))
            .unwrap();
    }
}

#[tauri::command]
pub fn get_api_url() -> String {
    env!("API_URL").to_owned()
}

#[tauri::command]
pub async fn check_auth(app: AppHandle) -> bool {
    let auth = get_auth(app).await;
    if auth.is_none() {
        return false;
    }
    let mut rauth = AUTH.write().await;
    *rauth = Some(auth.unwrap());
    let mut rjwt = JWT_TOKEN.write().await;
    let jwt_tok = get_jwt_token().await.unwrap();
    *rjwt = Some(jwt_tok.jwt);
    return true;
}

#[derive(Debug, Deserialize)]
pub struct JWTRet {
    pub jwt: String,
}

pub async fn get_jwt_token() -> Option<JWTRet> {
    let conf = load_config().unwrap();
    let api = get_api_url();
    let client = reqwest::Client::new();
    let jwt = client
        .get(format!("{}/auth/jwt", api))
        .header("cookie", conf.dc_auth)
        .send()
        .await;
    if jwt.is_err() {
        return None;
    }
    let jwt = jwt.unwrap();
    if !jwt.status().is_success() {
        return None;
    }
    let authkey: JWTRet = jwt.json().await.unwrap();
    Some(authkey)
}

pub async fn get_auth(app: AppHandle) -> Option<Driver> {
    let conf = load_config();
    if conf.is_none() {
        app.restart();
    }
    let api = get_api_url();
    let client = reqwest::Client::new();
    let jwt = get_jwt_token().await;
    if jwt.is_none() {
        return None;
    }
    let jwt = jwt.unwrap();
    let check = client
        .get(format!("{}/ucp", api))
        .header("cookie", jwt.jwt.clone())
        .send()
        .await;
    if check.is_err() {
        app.emit("apiFailed", ()).unwrap();
        thread::sleep(Duration::from_secs(1));
        return None;
    }
    let check = check.unwrap();
    if StatusCode::is_success(&check.status()) {
        let json: BundledDriver = check.json().await.unwrap();
        return Some(json.driver);
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
        dc_auth: DISCORD_TOKEN.read().await.clone().unwrap(),
        jwt_token: JWT_TOKEN.read().await.clone().unwrap(),
        game_dir: dir.clone(),
        faction: None,
    };
    save_config(config);
}

#[tauri::command]
pub async fn save_auth_token() {
    let conf = load_config().unwrap();
    let new_conf = Config {
        dc_auth: DISCORD_TOKEN.read().await.clone().unwrap(),
        jwt_token: DISCORD_TOKEN.read().await.clone().unwrap(),
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
    if conf.faction.is_none() {
        return false;
    }
    let auth = AUTH.read().await.clone().unwrap();
    if auth
        .perms
        .contains(&get_perm(Permissions::SaesUcp(conf.faction.unwrap())))
        || auth.admin
    {
        return true;
    }
    return false;
}

#[tauri::command]
pub async fn get_faction_options() -> HashMap<String, PubFactionReturn> {
    let jwt = JWT_TOKEN.read().await;
    let api = get_api_url();
    let check = WEB_CLIENT
        .get()
        .unwrap()
        .get(format!("{}/ucp/getfactions", api))
        .header("cookie", jwt.clone().unwrap())
        .send()
        .await;
    if check.is_err() {
        return HashMap::new();
    };
    let factions: HashMap<String, PubFactionReturn> = check.unwrap().json().await.unwrap();
    factions
}

#[tauri::command]
pub async fn set_faction(faction: String) {
    let conf = load_config().unwrap();
    let new_conf = Config {
        dc_auth: conf.dc_auth,
        jwt_token: conf.jwt_token,
        game_dir: conf.game_dir,
        faction: Some(faction),
    };
    save_config(new_conf);
}
