// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenvy::dotenv;

fn main() {
    if dotenv().is_err() {
        println!("No .env")
    }
    samt_app_lib::run()
}
