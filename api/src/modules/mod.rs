use api::run_api;
use gbot::run_gbot;
use tracing::{info, warn};

use crate::config::loader::get_module_config;

pub mod api;
pub mod gbot;

pub async fn enable_modules() {
    let module_config = get_module_config().await;
    let mut threads = vec![];
    // * API
    if module_config.api.is_some() && module_config.api.unwrap().enabled {
        info!("Module API ENABLED");
        threads.push(tokio::spawn(async {
            run_api().await.unwrap();
        }));
    } else {
        warn!("Module API DISABLED");
    }

    // * GBOT
    if module_config.gbot.is_some() && module_config.gbot.unwrap().enabled {
        info!("Module GBOT ENABLED");
        threads.push(tokio::spawn(async {
            run_gbot().await.unwrap();
        }));
    } else {
        warn!("Module GBOT DISABLED");
    }
    for thread in threads {
        let _ = thread.await.unwrap();
    }
}
