use std::collections::HashMap;

use chrono::Utc;
use saes_shared::structs::factions::FactionRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct SMGetItemsFull {
    pub id: i32,
    pub owner: i32,
    pub img_1: i32,
    pub img_2: Option<i32>,
    pub target_faction: Option<i8>,
    pub driver: Option<i32>,
    pub status: i8,
    pub reason: Option<String>,
    pub r#type: Option<i8>,
    pub price: Option<i32>,
    pub faction: i8,
    pub handled_by: Option<i32>,
    pub date: chrono::DateTime<Utc>,
    pub item_type: i8,
}

#[derive(Debug, Serialize)]
pub struct AppUser {
    pub usertoken: String,
    pub authcode: String,
}

#[derive(Debug)]
pub struct StateList {
    pub id: i32,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthJWT {
    pub id: i32,
    pub admin: bool,
    pub username: String,
    pub factions: HashMap<i8, FactionRecord>,
    pub permissions: Vec<String>,
    pub exp: i64,
}
