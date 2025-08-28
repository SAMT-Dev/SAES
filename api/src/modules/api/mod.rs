use std::{env, error::Error};

use axum::{Router, routing::get};
use socket::InitialData;
use socketioxide::{
    SocketIo,
    extract::{Data, SocketRef},
};
use tokio::sync::{OnceCell, RwLock};
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{info, warn};
use utils::structs::AppUser;

use crate::BASE_HASHMAP;

pub mod api;
pub mod app;
pub mod auth;
mod init;
pub mod list;
pub mod shorts;
pub mod socket;
pub mod sys;
pub mod ucp;
pub mod utils;

pub static SOCKET_IO: OnceCell<SocketIo> = OnceCell::const_new();
pub static APP_AUTHS: OnceCell<RwLock<Vec<AppUser>>> = OnceCell::const_new();

pub async fn run_api_checks() -> Result<(), Box<dyn Error>> {
    loop {
        let _ = run_api().await;
        warn!("API crashed, restarting...")
    }
}

pub async fn run_api() -> Result<(), Box<dyn Error>> {
    let hash = BASE_HASHMAP.read().await;
    let env_mode = hash.get("env_mode").unwrap().clone();
    init::main().await;
    let hash = env::var("COMMIT_HASH");
    let (layer, io) = SocketIo::new_layer();
    SOCKET_IO.set(io).unwrap();
    SOCKET_IO.get().unwrap().ns(
        "/",
        move |socket: SocketRef, Data(data): Data<InitialData>| socket::on_connect(socket, data),
    );

    info!("[API] Running in {} mode", &env_mode.clone().to_uppercase());
    APP_AUTHS.set(RwLock::new(Vec::new())).unwrap();
    let app = Router::new()
        .route(
            "/",
            get(|| async move {
                format!(
                    "SAES API V2 ({} / {}) Axum & Sea-ORM használatával",
                    if hash.is_ok() {
                        hash.unwrap()
                    } else {
                        String::from("")
                    },
                    env_mode
                )
            }),
        )
        .route("/auth", get(auth::auth_home))
        .route("/auth/cb", get(auth::base_callback))
        .route("/auth/jwt", get(auth::get_jwt))
        // .route("/auth/transfer", get(auth::webtransfer)) --> OBSOLETE
        .route("/list", get(list::base_list_get))
        .nest("/api", api::routes())
        .nest("/ucp", ucp::routes())
        .nest("/app", app::routes())
        .nest("/sys", sys::routes())
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        )
        .layer(TraceLayer::new_for_http())
        .layer(CookieManagerLayer::new());
    info!("[API] Server runs on :3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
