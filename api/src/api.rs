use axum::{middleware, routing::get, Router};

use crate::{shorts, utils::middle::key_auth};

pub fn routes() -> Router {
    Router::new()
        .route("/shorts", get(shorts::base_get_shorts))
        .layer(middleware::from_fn(key_auth))
}

#[derive(Debug,Deserialize)]
pub struct AccountIdParam {
    pub account_id: String
}

#[debug_handler]
pub async fn get_accountid(ext: Extension<AccessKeyExt>,q: Query<AccountIdParam>) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !ext.access.contains(){}
}