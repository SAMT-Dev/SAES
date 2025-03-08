use std::{env, time::Duration};

use chrono::Local;
use dotenvy::dotenv;
use google_sheets4::{api::ValueRange, hyper_rustls, hyper_util, Sheets};
use serde::Deserialize;
use serde_json::Value;
use tokio::time::interval;

mod auth;
mod envs;

#[derive(Debug, Deserialize, PartialEq)]
struct DriverData {
    driver: String,
    count: u32,
}

#[derive(Debug, Clone)]
enum Apis {
    SCKK,
    TOW,
}

async fn get_current_week(mode: Apis) -> Vec<DriverData> {
    let calls_url = env::var(match mode {
        Apis::SCKK => "CALLS_URL",
        Apis::TOW => "CALLS_URL_TOW",
    })
    .expect("CALLS_URL lekérése sikertelen");
    let res = reqwest::get(calls_url)
        .await
        .expect("Hívás lekérés sikertelen");
    let json_data = res.text().await.unwrap();
    let drivers: Vec<DriverData> = serde_json::from_str(&json_data).expect("Átalakítás sikertelen");
    drivers
}

async fn get_previous_week(mode: Apis) -> Vec<DriverData> {
    let calls_url = env::var(match mode {
        Apis::SCKK => "CALLS_URL_PREV",
        Apis::TOW => "CALLS_URL_PREV_TOW",
    })
    .expect("CALLS_URL_PREV lekérése sikertelen");
    let res = reqwest::get(calls_url)
        .await
        .expect("Hívás lekérés sikertelen");
    let json_data = res.text().await.unwrap();
    let drivers: Vec<DriverData> = serde_json::from_str(&json_data).expect("Átalakítás sikertelen");
    drivers
}

#[tokio::main]
async fn main() {
    dotenv().expect(".env fájl olvasása sikertelen");
    envs::load_envs().await;
    let interval_var = env::var("INTERVAL_SECS").expect("INTERVAL_SECS váltózó nem található!");
    let invt: u64 = interval_var.parse().expect("INTERVAL_SECS nem u64");
    let mut interval = interval(Duration::from_secs(invt));
    loop {
        interval.tick().await;
        println!(" ");
        println!("==== Taxi A műszak aktuális hét ====");
        println!(" ");
        handle_tables("Taxi A műszak", "B3:C22", "C3", "current", Apis::SCKK).await;
        println!(" ");
        println!("==== Taxi B műszak aktuális hét ====");
        println!(" ");
        handle_tables("Taxi B műszak", "B3:C22", "C3", "current", Apis::SCKK).await;
        println!(" ");
        println!("==== Taxi C műszak aktuális hét ====");
        println!(" ");
        handle_tables("Taxi C műszak", "B3:C22", "C3", "current", Apis::SCKK).await;
        println!(" ");
        println!("==== Taxi műszakfüggetlen aktuális hét ====");
        println!(" ");
        handle_tables(
            "Taxi műszakfüggetlen",
            "B3:C29",
            "C3",
            "current",
            Apis::SCKK,
        )
        .await;
        println!(" ");
        println!("==== TOW A műszak aktuális hét ====");
        println!(" ");
        handle_tables("TOW A műszak", "B3:C22", "C3", "current", Apis::TOW).await;
        println!(" ");
        println!("==== TOW műszakfüggetlen aktuális hét ====");
        println!(" ");
        handle_tables("TOW műszakfüggetlen", "B3:C22", "C3", "current", Apis::TOW).await;
        println!(" ");
        println!("=======================");
        println!(" ");
        println!("==== Taxi A műszak előző hét ====");
        println!(" ");
        handle_tables("Taxi A műszak", "H3:I22", "I3", "previous", Apis::SCKK).await;
        println!(" ");
        println!("==== Taxi B műszak előző hét ====");
        println!(" ");
        handle_tables("Taxi B műszak", "H3:I22", "I3", "previous", Apis::SCKK).await;
        println!(" ");
        println!("==== Taxi C műszak előző hét ====");
        println!(" ");
        handle_tables("Taxi C műszak", "H3:I22", "I3", "previous", Apis::SCKK).await;
        println!(" ");
        println!("==== Taxi műszakfüggetlen előző hét ====");
        println!(" ");
        handle_tables(
            "Taxi műszakfüggetlen",
            "H3:I29",
            "I3",
            "previous",
            Apis::SCKK,
        )
        .await;
        println!(" ");
        println!("==== TOW a műszak előző hét ====");
        println!(" ");
        handle_tables("TOW A műszak", "H3:I22", "I3", "previous", Apis::TOW).await;
        println!(" ");
        println!("==== TOW műszakfüggetlen előző hét ====");
        println!(" ");
        handle_tables("TOW műszakfüggetlen", "H3:I22", "I3", "previous", Apis::TOW).await;
        println!(" ");
        println!("=======================");
        println!(" ");
        handle_now("Taxi A műszak", "O23").await;
    }
}

async fn handle_now(table: &str, cell: &str) {
    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new());
    let spread_id = env::var("SPREADSHEET_ID").expect("SPREADSHEET_ID lekérése sikertelen");
    let token = auth::get_google_auth().await;
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
    let mut valrange = ValueRange::default();
    let most = Local::now().format("%Y.%m.%d %T").to_string();
    let recell: Vec<Vec<Value>> = vec![vec![most.into()]];
    valrange.values = recell.into();
    sheets
        .spreadsheets()
        .values_update(
            valrange,
            &spread_id,
            &format!("{}!{}", table, cell).to_string(),
        )
        .value_input_option("USER_ENTERED")
        .doit()
        .await
        .expect("Dátum beírása sikertelen");
}

async fn handle_tables(table: &str, read_range: &str, write_range: &str, week: &str, mode: Apis) {
    let client = hyper_util::client::legacy::Client::builder(hyper_util::rt::TokioExecutor::new());
    if week == "current" {
        let spread_id = env::var("SPREADSHEET_ID").expect("SPREADSHEET_ID lekérése sikertelen");
        let token = auth::get_google_auth().await;
        let calls = get_current_week(mode.clone()).await;
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
                    println!("{}: {}!", tag[0], call.count);
                    vals.push(vec![serde_json::Value::Null])
                } else {
                    println!("{}: {}", tag[0], call.count);
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
    }
    if week == "previous" {
        let spread_id = env::var("SPREADSHEET_ID").expect("SPREADSHEET_ID lekérése sikertelen");
        let token = auth::get_google_auth().await;
        let calls = get_previous_week(mode).await;
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
                    println!("{}: {}!", tag[0], call.count);
                    vals.push(vec![serde_json::Value::Null])
                } else {
                    println!("{}: {}", tag[0], call.count);
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
    }
}
