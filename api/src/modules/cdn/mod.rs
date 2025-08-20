use std::error::Error;

use axum::{
    Router,
    body::Body,
    debug_handler,
    extract::Path,
    response::{IntoResponse, Response},
    routing::get,
};
use http::{StatusCode, header};
use saes_shared::db::images;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;

use crate::{DB_CLIENT, config::loader::get_module_config};

pub async fn run_cdn() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route(
            "/",
            get(|| async move { "SAES CDN V2 Axum & Sea-ORM használatával".to_string() }),
        )
        .route("/get/{end}", get(serve_get))
        .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
        .layer(TraceLayer::new_for_http());
    info!("[CDN] Server runs on :3100");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3100").await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[debug_handler]
pub async fn serve_get(Path(end): Path<String>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let conf = get_module_config().await;
    let p: Vec<&str> = end.split(".").collect();
    let img = images::Entity::find()
        .filter(images::Column::Id.eq(p[0]))
        .one(DB_CLIENT.get().unwrap())
        .await
        .unwrap();
    if img.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            "The img cannot be found.".to_string(),
        ));
    }
    let img = img.unwrap();
    let file = File::open(format!(
        "{}/{}{}",
        conf.cdn.unwrap().files_path,
        if img.tmp == 1 { "tmp/" } else { "" },
        img.filename
    ))
    .await;
    if file.is_err() {
        return Err((
            StatusCode::BAD_REQUEST,
            "The img cannot be opened.".to_string(),
        ));
    }
    let file = file.unwrap();
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let ext = std::path::Path::new(&img.filename)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();

    // Optionally, set content-type based on file extension
    let content_type = match ext.as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "avif" => "image/avif",
        "gif" => "image/gif",
        "webp" => "image/webp",
        _ => "application/octet-stream",
    };

    Ok(Response::builder()
        .header(header::CONTENT_TYPE, content_type)
        .body(body)
        .unwrap())
}
