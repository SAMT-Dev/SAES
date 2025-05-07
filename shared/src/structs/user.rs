use serde::{Deserialize, Serialize};

use super::{
    api_config::{FactionAccessConfig, FactionSiteAccessConfig},
    factions::FactionRecord,
};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Driver {
    pub driverid: i32,
    pub name: String,
    pub admin: bool,
    pub perms: Vec<String>,
    pub access: Option<FactionAccessConfig>,
    pub site_access: Option<FactionSiteAccessConfig>,
    pub faction: Option<String>,
    pub factions: Option<FactionRecord>,
}
