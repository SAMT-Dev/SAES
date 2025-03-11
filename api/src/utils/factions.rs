use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Factions {
    SCKK,
    TOW,
    APMS,
}

pub fn get_faction_id(faction: Factions) -> i8 {
    match faction {
        Factions::SCKK => 1,
        Factions::APMS => 2,
        Factions::TOW => 3,
    }
}

pub fn get_faction_string(faction: Factions) -> String {
    match faction {
        Factions::SCKK => "taxi".to_string(),
        Factions::APMS => "apms".to_string(),
        Factions::TOW => "tow".to_string(),
    }
}

// pub fn get_faction_by_id(id: i8) -> Factions {
//     match id {
//         1 => Factions::SCKK,
//         3 => Factions::TOW,
//         _ => Factions::SCKK,
//     }
// }
