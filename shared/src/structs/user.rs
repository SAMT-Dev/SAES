use serde::{Deserialize, Serialize};

use super::{
    api_config::{FactionAccessConfig, FactionSiteAccessConfig},
    factions::{FactionRecord, Factions},
};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Driver {
    pub driverid: i32,
    pub name: String,
    pub admin: bool,
    pub perms: Vec<String>,
    pub access: Option<FactionAccessConfig>,
    pub site_access: Option<FactionSiteAccessConfig>,
    pub faction: Option<Factions>,
    pub factions: Option<FactionRecord>,
}
