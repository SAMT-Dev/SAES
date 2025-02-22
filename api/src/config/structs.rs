use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::utils::factions::Factions;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FactionConfig {
    pub shift_access: ShiftAccess,
    pub access: FactionAccessConfig,
    pub site_access: FactionSiteAccessConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FactionAccessConfig {
    pub supplements: bool,
    pub hails: bool,
    pub bills: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FactionSiteAccessConfig {
    pub ucp: bool,
    pub admin: bool,
    pub shift: bool,
    pub fleet: bool,
    pub faction: bool,
}

impl Default for FactionConfig {
    fn default() -> Self {
        Self {
            shift_access: ShiftAccess::default(),
            access: FactionAccessConfig {
                supplements: true,
                hails: true,
                bills: true,
            },
            site_access: FactionSiteAccessConfig {
                ucp: true,
                admin: true,
                shift: true,
                fleet: true,
                faction: true,
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MainConfig {
    pub global: GlobalConfig,
    pub factions: HashMap<Factions, FactionConfig>,
    pub access_keys: Vec<AccessKeysConfig>,
}

impl Default for MainConfig {
    fn default() -> Self {
        let mut factions = HashMap::new();
        factions.insert(Factions::SCKK, FactionConfig::default());
        factions.insert(Factions::TOW, FactionConfig::default());
        factions.insert(Factions::APMS, FactionConfig::default());
        Self {
            global: GlobalConfig::default(),
            factions,
            access_keys: vec![],
        }
    }
}
