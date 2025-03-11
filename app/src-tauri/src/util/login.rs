use std::env;

use tiny_http::{Response, Server};
use url::Url;

use crate::DISCORD_TOKEN;

use super::structs::UCPReturn;

#[tauri::command]
pub async fn begin_login() {
    tauri::async_runtime::spawn(async move {
        let server = Server::http("127.0.0.1:31313").unwrap();
        for req in server.incoming_requests() {
            let url: String = format!("http://127.0.0.1:31313{}", req.url());
            let parsed_url = Url::parse(&url).unwrap();
            let queries = parsed_url.query_pairs();
            for (key, value) in queries {
                if key == "code" {
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
    let api = get_api_url().await;
    let get = client
        .get(format!("{}/ucp", api))
        .header("cookie", token.clone().unwrap())
        .send()
        .await
        .unwrap();
    let data: Result<UCPReturn, reqwest::Error> = get.json().await;
    if data.is_ok() {
        println!("{:?}", data.unwrap());
    }
}

#[tauri::command]
pub async fn check_envs() -> bool {
    let api = env::var("API_URL");
    api.is_ok()
}

#[tauri::command]
pub async fn get_api_url() -> String {
    env::var("API_URL").unwrap()
}
