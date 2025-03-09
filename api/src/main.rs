use std::{collections::HashMap, env, error::Error};

use axum::{routing::get, Router};
use dotenvy::dotenv;
use lazy_static::lazy_static;
use reqwest::Client;
use saes_shared::sql::get_db_conn;
use sea_orm::DatabaseConnection;
use socket::InitialData;
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use tokio::sync::{OnceCell, RwLock};
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use utils::structs::AppUser;

mod api;
mod app;
mod auth;
mod config;
mod envs;
mod init;
mod list;
mod logging;
mod shorts;
mod socket;
mod ucp;
mod utils;

lazy_static! {
    pub static ref WEB_CLIENT: Client = Client::new();
    pub static ref BASE_HASHMAP: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
}
pub static DB_CLIENT: OnceCell<DatabaseConnection> = OnceCell::const_new();
pub static SOCKET_IO: OnceCell<SocketIo> = OnceCell::const_new();
pub static APP_AUTHS: OnceCell<RwLock<Vec<AppUser>>> = OnceCell::const_new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().expect(".env nem létezik");
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    let (layer, io) = SocketIo::new_layer();
    SOCKET_IO.set(io).unwrap();
    envs::load_envs().await;
    let env_mode = env::var("ENV_MODE");
    if env_mode.is_err() {
        panic!("ENV_MODE nincs setelve! production / testing / devel")
    }
    if ![
        "production".to_string(),
        "testing".to_string(),
        "devel".to_string(),
    ]
    .contains(&env_mode.clone()?)
    {
        panic!("ENV_MODE rosszul setelve! production / testing / devel")
    }
    info!("Running in {} mode", &env_mode.clone()?.to_uppercase());
    let env_mode = env_mode.unwrap();
    BASE_HASHMAP.write().await.insert(
        "env_mode".to_string(),
        env_mode.clone().to_uppercase().to_string(),
    );
    init::main().await;
    DB_CLIENT.set(get_db_conn().await).unwrap();
    APP_AUTHS.set(RwLock::new(Vec::new())).unwrap();
    SOCKET_IO.get().unwrap().ns(
        "/",
        move |socket: SocketRef, Data(data): Data<InitialData>| socket::on_connect(socket, data),
    );
    let hash = env::var("COMMIT_HASH");
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
        .route("/list", get(list::base_list_get))
        .nest("/api", api::routes())
        .nest("/ucp", ucp::routes())
        .nest("/app", app::routes())
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        )
        .layer(TraceLayer::new_for_http())
        .layer(CookieManagerLayer::new());
    info!("Server runs on :3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
