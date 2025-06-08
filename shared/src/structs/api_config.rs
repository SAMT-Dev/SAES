use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FactionAccessConfig {
    pub supplements: ItemAccess,
    pub hails: ItemAccess,
    pub bills: ItemAccess,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FactionSiteAccessConfig {
    pub ucp: bool,
    pub admin: bool,
    pub shift: bool,
    pub fleet: bool,
    pub faction: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ItemAccess {
    None,
    Read,
    Write,
}

impl Default for ItemAccess {
    fn default() -> Self {
        Self::Write
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PubFactionReturn {
    pub icon_id: i32,
    pub name: String,
    pub perm_name: String,
    pub primary: String,
    pub secondary: String,
    pub tertiary: String,
}
