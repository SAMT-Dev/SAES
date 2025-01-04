use serde::Deserialize;

use super::factions::Factions;

#[derive(Debug, Deserialize)]
pub struct BaseListQuery {
    pub tipus: String,
    pub faction: Factions,
    pub driver: String,
}

#[derive(Debug, Deserialize)]
pub struct UCPTypeQuery {
    pub tipus: i8,
}

#[derive(Debug, Deserialize)]
pub struct UCPTypeExtraQuery {
    pub tipus: i8,
    pub dates: String,
}

#[derive(Debug, Deserialize)]
pub struct SMStatQuery {
    pub week: String,
}

#[derive(Debug, Deserialize)]
pub struct SMItemsQuery {
    pub tipus: i8,
    pub status: i8,
}
