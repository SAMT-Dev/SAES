use std::thread::{self};

use api::run_api;
use tracing::{info, warn};

use crate::config::loader::get_module_config;

pub mod api;

pub async fn enable_modules() {
    let module_config = get_module_config().await;
    let mut threads = vec![];
    if module_config.api.is_some() && module_config.api.unwrap().enabled {
        info!("Module API ENABLED");
        threads.push(thread::spawn(|| {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(run_api())
                .unwrap();
        }));
    } else {
        warn!("Module API DISABLED");
    }
    for thread in threads {
        let _ = thread.join();
    }
}
