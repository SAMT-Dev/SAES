use axum::{debug_handler, response::IntoResponse, Json};
use http::StatusCode;

use crate::config::{editor::write_config, loader::get_config, structs::GlobalConfig};

#[debug_handler]
pub async fn sys_get_configs() -> Result<impl IntoResponse, (StatusCode, String)> {
    let config = get_config().await;
    let conf = config.clone();
    return Ok(Json(conf));
}

#[debug_handler]
pub async fn sys_post_global_config(
    Json(c): Json<GlobalConfig>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let conf = get_config().await;
    let mut new_config = conf.clone();
    if c.announcement != conf.global.announcement {
        new_config.global.announcement = c.announcement;
    }

    if c.maintenance != conf.global.maintenance {
        new_config.global.maintenance = c.maintenance;
    }
    drop(conf);

    write_config(&new_config).await;
    Ok(Json(new_config.global))
}
