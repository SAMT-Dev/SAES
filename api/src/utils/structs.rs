use chrono::Utc;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SMGetItemsFull {
    pub id: i32,
    pub owner: String,
    pub img_1: i32,
    pub img_2: Option<i32>,
    pub target_faction: Option<i8>,
    pub driver: Option<String>,
    pub status: i8,
    pub reason: Option<String>,
    pub r#type: Option<i8>,
    pub price: Option<i32>,
    pub faction: i8,
    pub handled_by: Option<String>,
    pub date: chrono::DateTime<Utc>,
    pub item_type: i8,
}

#[derive(Debug, Serialize)]
pub struct AppUser {
    pub usertoken: String,
    pub authcode: String,
}
