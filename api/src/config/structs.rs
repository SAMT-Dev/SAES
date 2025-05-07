use std::collections::HashMap;

use saes_shared::structs::api_config::{FactionAccessConfig, FactionSiteAccessConfig};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ShiftAccess {
    SameShift,
    OtherManager,
    OtherShift,
}

impl Default for ShiftAccess {
    fn default() -> Self {
        Self::SameShift
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GlobalConfig {
    pub maintenance: Option<String>,
    pub announcement: Option<String>,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            maintenance: None,
            announcement: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessKeysConfig {
    pub key: String,
    pub access: Vec<AccessConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AccessConfig {
    UCP,
    Admin,
    Shift,
    Fleet,
    Faction,
    Supplements,
    Hails,
    Bills,
    Shorts,
    SeeAPI,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FactionConfig {
    pub shift_access: ShiftAccess,
    pub access: FactionAccessConfig,
    pub site_access: FactionSiteAccessConfig,
    pub settings: FactionSettings,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FactionSettings {
    pub id: i8,
    pub icon_id: i32,
    pub display: String,
    pub perm_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MainConfig {
    pub global: GlobalConfig,
    pub factions: HashMap<String, FactionConfig>,
    pub access_keys: Vec<AccessKeysConfig>,
}

impl Default for MainConfig {
    fn default() -> Self {
        let factions = HashMap::new();
        Self {
            global: GlobalConfig::default(),
            factions,
            access_keys: vec![],
        }
    }
}
