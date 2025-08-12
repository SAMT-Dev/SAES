use std::{error::Error, thread, time::Duration};

use chrono::{Datelike, Local, Timelike};
use google_sheets4::{Sheets, api::ValueRange, hyper_rustls, hyper_util};
use serde::Deserialize;
use serde_json::Value;
use tracing::info;

use crate::config::{loader::get_module_config, structs::GbotProviders};

mod auth;

#[derive(Debug, Deserialize, PartialEq)]
struct DriverData {
    driver: String,
    count: u32,
}

#[derive(Debug, PartialEq)]
enum Week {
    Current,
    Previous,
}

async fn get_week(mode: GbotProviders, week: Week) -> Vec<DriverData> {
    let config = get_module_config().await;
    let config = config.gbot.unwrap();
    let provider = config.providers.get(&mode).unwrap();
    let calls_url = if week == Week::Current {
        provider.current.clone()
    } else {
        provider.previous.clone()
    };
    let res = reqwest::get(calls_url)
        .await
        .expect("Hívás lekérés sikertelen");
    let json_data = res.text().await.unwrap();
    let drivers: Vec<DriverData> = serde_json::from_str(&json_data).expect("Átalakítás sikertelen");
    drivers
}

pub async fn run_gbot() -> Result<(), Box<dyn Error>> {
    let config = get_module_config().await;
    loop {
        info!("Calls sync BEGIN");
        let mut runners = Vec::new();
        for range in config.gbot.clone().unwrap().ranges {
            let range2 = range.clone();
            runners.push(thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(handle_tables(
                    range2.table.clone(),
                    range2.current.read,
                    range2.current.write,
                    Week::Current,
                    range2.provider.clone(),
                    range2.check_range,
                ));
                info!("{} CURRENT week DONE", range2.table);
                rt.block_on(handle_tables(
                    range.table.clone(),
                    range.previous.read,
                    range.previous.write,
                    Week::Previous,
                    range.provider,
                    range.check_range,
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
    week: Week,
    mode: GbotProviders,
    check_cell: String,
) {
    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new());
    let config = get_module_config().await;
    let spread_id = config.gbot.unwrap().spreadsheet_id;
    let token = auth::get_google_auth().await;
    let calls = get_week(mode.clone(), week).await;
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

    let res = sheets
        .spreadsheets()
        .values_get(&spread_id, format!("{}!{}", table, read_range).as_str())
        .doit()
        .await
        .expect("Táblázat lekérés sikertelen.");
    let values: Vec<Vec<serde_json::value::Value>> = res.1.values.unwrap_or_default();
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
    sheets
        .spreadsheets()
        .values_update(
            req,
            &spread_id,
            format!("{}!{}", table, write_range).as_str(),
        )
        .value_input_option("USER_ENTERED")
        .doit()
        .await
        .expect("Táblázat írás sikertelen");
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
    sheets
        .spreadsheets()
        .values_update(
            checkval,
            &spread_id,
            format!("{}!{}", table, check_cell).as_str(),
        )
        .value_input_option("USER_ENTERED")
        .doit()
        .await
        .expect("Check írás sikertelen");
}
