use std::{collections::HashMap, error::Error, thread, time::Duration};

use chrono::{Datelike, Local, Timelike};
use google_sheets4::{Sheets, api::ValueRange, hyper_rustls, hyper_util};
use serde::Deserialize;
use serde_json::Value;
use tokio::time::sleep;
use tracing::{info, warn};

use crate::config::loader::get_module_config;

mod auth;

#[derive(Debug, Deserialize, PartialEq, Clone)]
struct DriverData {
    driver: String,
    count: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Week {
    Current,
    Previous,
}

async fn get_week(prov: &String, week: Week) -> Option<Vec<DriverData>> {
    let config = get_module_config().await;
    let config = config.gbot.unwrap();
    let provider = config.providers.get(prov).unwrap();
    let calls_url = if week == Week::Current {
        provider.current.clone()
    } else {
        provider.previous.clone()
    };
    let res = reqwest::get(calls_url).await;
    if res.is_err() {
        return None;
    }
    let json_data = res.unwrap().text().await.unwrap();
    let drivers: Vec<DriverData> = serde_json::from_str(&json_data).expect("Átalakítás sikertelen");
    Some(drivers)
}

async fn retry_google_api<F, Fut, T>(mut f: F, desc: &str) -> Option<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = Result<T, google_sheets4::Error>>,
{
    let mut attempts = 0;
    loop {
        match f().await {
            Ok(val) => return Some(val),
            Err(e) => {
                attempts += 1;
                tracing::error!("{}: {} (attempt {})", desc, e, attempts);
                if attempts >= 5 {
                    tracing::error!("{}: giving up after {} attempts", desc, attempts);
                    return None;
                }
                sleep(Duration::from_secs(20 * attempts)).await;
            }
        }
    }
}

pub async fn run_gbot_checks() -> Result<(), Box<dyn Error>> {
    loop {
        let _ = run_gbot().await;
        warn!("GBOT crashed, restarting...")
    }
}

pub async fn run_gbot() -> Result<(), Box<dyn Error>> {
    let config = get_module_config().await;
    'outer: loop {
        info!("Calls sync BEGIN");
        let mut calls: HashMap<String, HashMap<Week, Vec<DriverData>>> = HashMap::new();
        for (k, _) in config.gbot.clone().unwrap().providers.iter() {
            let mut c = HashMap::new();
            let cw = get_week(k, Week::Current).await;
            let pw = get_week(k, Week::Previous).await;
            if cw.is_none() || pw.is_none() {
                sleep(Duration::from_secs(15)).await;
                continue 'outer;
            }
            c.insert(Week::Current, cw.unwrap());
            c.insert(Week::Previous, pw.unwrap());
            calls.insert(k.to_owned(), c);
        }
        let mut runners = Vec::new();
        let client =
            hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new());
        let token = auth::get_google_auth().await;
        if token.is_none() {
            warn!("[GBOT] Couldn't get the token!");
            return Err("Couldn't get the token!".into());
        }
        let token = token.unwrap();
        let sheets = Sheets::new(
            client.build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .unwrap()
                    .https_or_http()
                    .enable_http1()
                    .build(),
            ),
            token,
        );
        for range in config.gbot.clone().unwrap().ranges {
            let range2 = range.clone();
            let cc = calls.clone();
            let cs = sheets.clone();
            runners.push(thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let cc = cc.get(&range2.provider).unwrap().clone();
                rt.block_on(handle_tables(
                    range2.table.clone(),
                    range2.current.read,
                    range2.current.write,
                    cc.get(&Week::Current).unwrap().clone(),
                    range2.check_range,
                    cs.clone(),
                ));
                info!("{} CURRENT week DONE", range2.table);
                rt.block_on(handle_tables(
                    range.table.clone(),
                    range.previous.read,
                    range.previous.write,
                    cc.get(&Week::Previous).unwrap().clone(),
                    range.check_range,
                    cs,
                ));
                info!("{} PREVIOUS week DONE", range.table);
            }));
        }
        for runner in runners {
            runner.join().unwrap();
        }
        info!("Calls sync DONE");
        thread::sleep(Duration::from_secs(
            config.gbot.clone().unwrap().interval_secs,
        ));
    }
}

async fn handle_tables(
    table: String,
    read_range: String,
    write_range: String,
    calls: Vec<DriverData>,
    check_cell: String,
    sheets: Sheets<
        hyper_rustls::HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>,
    >,
) {
    let config = get_module_config().await;
    let spread_id = config.gbot.unwrap().spreadsheet_id;

    let res = retry_google_api(
        || {
            sheets
                .spreadsheets()
                .values_get(&spread_id, format!("{}!{}", table, read_range).as_str())
                .doit()
        },
        "Táblázat lekérés sikertelen.",
    )
    .await;
    if res.is_none() {
        tracing::error!(
            "Táblázat lekérés véglegesen sikertelen: {}!{}",
            table,
            read_range
        );
        return;
    }
    let values: Vec<Vec<serde_json::value::Value>> = res.unwrap().1.values.unwrap_or_default();
    let mut req = ValueRange::default();
    let mut vals: Vec<Vec<Value>> = vec![];
    for tag in values.iter() {
        if let Some(call) = calls.iter().find(|x| x.driver == tag[0]) {
            if tag.len() > 1 && tag[1] == call.count.to_string() {
                vals.push(vec![serde_json::Value::Null])
            } else {
                vals.push(vec![serde_json::Value::String(call.count.to_string())])
            }
        } else {
            if tag.len() > 1 && tag[1] == 0.to_string() {
                vals.push(vec![serde_json::Value::Null])
            } else {
                vals.push(vec![serde_json::Value::String(0.to_string())])
            }
        }
    }
    req.values = vals.into();
    let update_res = retry_google_api(
        || {
            sheets
                .spreadsheets()
                .values_update(
                    req.clone(),
                    &spread_id,
                    format!("{}!{}", table, write_range).as_str(),
                )
                .value_input_option("USER_ENTERED")
                .doit()
        },
        "Táblázat írás sikertelen",
    )
    .await;
    if update_res.is_none() {
        tracing::error!(
            "Táblázat írás véglegesen sikertelen: {}!{}",
            table,
            write_range
        );
        return;
    }
    let mut checkval = ValueRange::default();
    let now = Local::now();
    checkval.values = vec![vec![serde_json::Value::String(format!(
        "{}.{}.{}. {}:{}:{}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second()
    ))]]
    .into();
    let check_res = retry_google_api(
        || {
            sheets
                .spreadsheets()
                .values_update(
                    checkval.clone(),
                    &spread_id,
                    format!("{}!{}", table, check_cell).as_str(),
                )
                .value_input_option("USER_ENTERED")
                .doit()
        },
        "Check írás sikertelen",
    )
    .await;
    if check_res.is_none() {
        tracing::error!("Check írás véglegesen sikertelen: {}!{}", table, check_cell);
    }
}
