use axum::{debug_handler, response::IntoResponse, Extension, Json};
use http::StatusCode;
use saes_shared::db::shorts;

use sea_orm::EntityTrait;

use crate::{utils::middle::AccessKeyExt, DB_CLIENT};

#[debug_handler]
pub async fn base_get_shorts(
    ext: Extension<AccessKeyExt>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !ext
        .access
        .contains(&crate::config::structs::AccessConfig::Shorts)
    {
        return Err((StatusCode::FORBIDDEN, String::new()));
    }
    let db = DB_CLIENT.get().unwrap();
    let shorts = shorts::Entity::find()
        .all(db)
        .await
        .expect("[SHORTS:ERROR] Lekérés sikertelen");
    Ok(Json(shorts))
}
