use std::{env, thread};

use api::run_api;
use tracing::{info, warn};

use crate::{
    config::loader::get_module_config,
    modules::{cdn::run_cdn, gbot::run_gbot_checks},
};

pub mod api;
pub mod cdn;
pub mod gbot;

pub async fn enable_modules() {
    let module_config = get_module_config().await;
    let single_module = env::var("MODULE");
    let mut threads = vec![];
    // * API
    if module_config.api.is_some()
        && module_config.api.unwrap().enabled
        && (single_module.is_err() || single_module.as_ref().unwrap() == "API")
    {
        info!("Module API ENABLED");
        threads.push(thread::spawn(|| {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(4)
                .enable_all()
                .thread_name("api")
                .build()
                .unwrap();
            rt.block_on(run_api()).unwrap();
        }));
    } else {
        warn!("Module API DISABLED");
    }
    // * GBOT
    if module_config.gbot.is_some()
        && module_config.gbot.unwrap().enabled
        && (single_module.is_err() || single_module.as_ref().unwrap() == "GBOT")
    {
        info!("Module GBOT ENABLED");
        threads.push(thread::spawn(|| {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .thread_name("gbot")
                .build()
                .unwrap();
            rt.block_on(run_gbot_checks()).unwrap();
        }));
    } else {
        warn!("Module GBOT DISABLED");
    }
    // * CDN
    if module_config.cdn.is_some()
        && module_config.cdn.unwrap().enabled
        && (single_module.is_err() || single_module.as_ref().unwrap() == "CDN")
    {
        info!("Module CDN ENABLED");
        threads.push(thread::spawn(|| {
            let rt = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(4)
                .enable_all()
                .thread_name("cdn")
                .build()
                .unwrap();
            rt.block_on(run_cdn()).unwrap();
        }));
    } else {
        warn!("Module CDN DISABLED");
    }
    for thread in threads {
        let _ = thread.join().unwrap();
    }
}
