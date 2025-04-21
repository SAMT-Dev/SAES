use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Factions {
    SCKK,
    TOW,
    APMS,
    UNI,
}

impl Display for Factions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn get_faction_id(faction: Factions) -> i8 {
    match faction {
        Factions::SCKK => 1,
        Factions::APMS => 3,
        Factions::TOW => 2,
        Factions::UNI => 5,
    }
}

pub fn get_faction_string(faction: Factions) -> String {
    match faction {
        Factions::SCKK => "taxi".to_string(),
        Factions::APMS => "apms".to_string(),
        Factions::TOW => "tow".to_string(),
        Factions::UNI => "uni".to_string(),
    }
}

pub fn get_faction_by_id(id: i8) -> Factions {
    match id {
        1 => Factions::SCKK,
        2 => Factions::TOW,
        3 => Factions::APMS,
        5 => Factions::UNI,
        _ => Factions::SCKK,
    }
}
