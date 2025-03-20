use chrono::Utc;
use serde::{Deserialize, Serialize};

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

#[derive(Debug)]
pub struct StateList {
    pub id: i32,
    pub token: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    /// Single value
    One(T),
    /// Array of values
    Vec(Vec<T>),
}
impl<T> From<OneOrMany<T>> for Vec<T> {
    fn from(from: OneOrMany<T>) -> Self {
        match from {
            OneOrMany::One(val) => vec![val],
            OneOrMany::Vec(vec) => vec,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthJWT {
    #[serde(rename = "Id")]
    pub id: isize,
    #[serde(rename = "IsSysAdmin")]
    pub is_sys_admin: bool,
    #[serde(rename = "UserName")]
    pub username: String,
    #[serde(rename = "FactionId")]
    pub faction_id: OneOrMany<i32>,
    #[serde(rename = "FactionName")]
    pub faction_name: OneOrMany<String>,
    #[serde(rename = "FactionShortName")]
    pub faction_short_name: OneOrMany<String>,
    #[serde(rename = "ShiftId")]
    pub shift_id: OneOrMany<i32>,
    #[serde(rename = "ShiftName")]
    pub shift_name: OneOrMany<String>,
    #[serde(rename = "PositionId")]
    pub position_id: OneOrMany<i32>,
    #[serde(rename = "PositionName")]
    pub position_name: OneOrMany<String>,
    #[serde(rename = "Permission")]
    pub permissions: Vec<String>,
    pub nbf: i64,
    pub exp: i64,
    pub iat: i64,
    pub iss: String,
}
