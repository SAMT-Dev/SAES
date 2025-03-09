use axum::response::IntoResponse;
use http::{HeaderMap, StatusCode};

use crate::APP_AUTHS;

pub async fn withauth(headers: HeaderMap) -> Result<impl IntoResponse, (StatusCode, String)> {
    let code = headers.get("auth_code");
    if code.is_none() {
        return Err((StatusCode::BAD_REQUEST, "Nincs auth code".to_string()));
    }
    let codes = APP_AUTHS.get().unwrap().read().await;
    let selected_code = codes
        .iter()
        .find(|val| val.authcode == code.unwrap().to_str().unwrap());
    if selected_code.is_none() {
        return Err((StatusCode::NOT_FOUND, "Wrong code".to_string()));
    }
    return Ok(selected_code.unwrap().usertoken.clone());
}
