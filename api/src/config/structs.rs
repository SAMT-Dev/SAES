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
    pub color: FactionColorConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FactionColorConfig {
    pub primary: String,
    pub secondary: String,
    pub tertiary: String,
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

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct ModuleConfig {
    pub api: Option<ApiModuleConfig>,
    pub gbot: Option<GbotModuleConfig>,
}

impl Default for ModuleConfig {
    fn default() -> Self {
        Self {
            api: None,
            gbot: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiModuleConfig {
    pub enabled: bool,
    pub full_domain: String,
    pub domain: String,
    pub discord_secret: String,
    pub discord_id: String,
    pub api_base_url: String,
    pub fms_api: String,
    pub fms_api_key: String,
    pub jwt_key: String,
    pub sckkapp_api_taxi: String,
    pub sckkapp_api_tow: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Hash, PartialEq)]
pub enum GbotProviders {
    TAXI,
    TOW,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GbotRangeWeekConfig {
    pub read: String,
    pub write: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GbotRangeListConfig {
    pub current: GbotRangeWeekConfig,
    pub previous: GbotRangeWeekConfig,
    pub table: String,
    pub provider: GbotProviders,
    pub check_range: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct GbotModuleConfig {
    pub enabled: bool,
    pub service_json: String,
    pub providers: HashMap<GbotProviders, GbotProviderConfig>,
    pub spreadsheet_id: String,
    pub interval_secs: u64,
    pub ranges: Vec<GbotRangeListConfig>,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct GbotProviderConfig {
    pub current: String,
    pub previous: String,
}
