use std::path::PathBuf;

use gcp_auth::{CustomServiceAccount, TokenProvider};

use crate::config::loader::get_module_config;

pub async fn get_google_auth() -> Option<String> {
    let config = get_module_config().await;
    let credentials_path = PathBuf::from(config.gbot.unwrap().service_json);
    let service_account = CustomServiceAccount::from_file(credentials_path)
        .expect("Google Service Account létrehozása sikertelen");
    let scopes = &[
        "https://www.googleapis.com/auth/cloud-platform",
        "https://www.googleapis.com/auth/spreadsheets",
    ];
    let token = service_account.token(scopes).await;
    if token.is_err() {
        return None;
    }
    Some(token.unwrap().as_str().to_owned())
}
