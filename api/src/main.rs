use std::{collections::HashMap, env, error::Error};

use dotenvy::dotenv;
use lazy_static::lazy_static;
use modules::enable_modules;
use reqwest::Client;
use saes_shared::sql::{get_db_conn, test_db_conn};
use sea_orm::DatabaseConnection;
use tokio::sync::{OnceCell, RwLock};
use tracing_subscriber::FmtSubscriber;

mod config;
mod envs;
mod logging;
mod modules;

lazy_static! {
    pub static ref WEB_CLIENT: Client = Client::new();
    pub static ref BASE_HASHMAP: RwLock<HashMap<String, String>> = RwLock::new(HashMap::new());
}
pub static DB_CLIENT: OnceCell<DatabaseConnection> = OnceCell::const_new();

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().expect(".env nem létezik");
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    let env_mode = env::var("ENV_MODE");
    envs::load_envs().await;
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
    let env_mode = env_mode.unwrap();
    BASE_HASHMAP.write().await.insert(
        "env_mode".to_string(),
        env_mode.clone().to_uppercase().to_string(),
    );
    test_db_conn().await;
    DB_CLIENT.set(get_db_conn().await).unwrap();
    enable_modules().await;
    Ok(())
}
