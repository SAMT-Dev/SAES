use axum::{
    debug_handler, extract::Query, middleware, response::IntoResponse, routing::get, Extension,
    Router,
};
use http::StatusCode;
use serde::Deserialize;

use crate::{
    shorts,
    utils::middle::{key_auth, AccessKeyExt},
    WEB_CLIENT,
};

pub fn routes() -> Router {
    Router::new()
        .route("/shorts", get(shorts::base_get_shorts))
        .route("/getusername", get(get_username))
        .layer(middleware::from_fn(key_auth))
}

#[derive(Debug, Deserialize)]
pub struct AccountIdParam {
    pub account_id: String,
}

#[debug_handler]
pub async fn get_username(
    ext: Extension<AccessKeyExt>,
    q: Query<AccountIdParam>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !ext
        .access
        .contains(&crate::config::structs::AccessConfig::SeeAPI)
    {
        return Err((StatusCode::FORBIDDEN, String::new()));
    }
    let req = WEB_CLIENT
        .get(format!(
            "https://premium.see-game.com/ajax/checkacc.php?server=v4&account={}",
            q.account_id
        ))
        .send()
        .await;
    if req.is_err() {
        return Err((StatusCode::BAD_REQUEST, req.unwrap_err().to_string()));
    }
    let text = req.unwrap().text().await.unwrap();
    if text == "0" {
        return Err((StatusCode::NOT_FOUND, "Account not found".to_string()));
    }
    return Ok(text.replace('_', " "));
}
