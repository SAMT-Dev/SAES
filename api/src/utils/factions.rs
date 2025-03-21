use std::fmt::Display;

use serde::{Deserialize, Serialize};

use super::{middle::FactionRecord, structs::AuthJWT};

#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Factions {
    SCKK,
    TOW,
    APMS,
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
    }
}

pub fn get_faction_string(faction: Factions) -> String {
    match faction {
        Factions::SCKK => "taxi".to_string(),
        Factions::APMS => "apms".to_string(),
        Factions::TOW => "tow".to_string(),
    }
}

pub fn get_faction_from_jwt(jwt: AuthJWT, faction: Factions) -> Option<FactionRecord> {
    let shorts: Vec<String> = jwt.faction_short_name.into();
    let i = shorts.iter().position(|p| p == &faction.to_string());
    if i.is_none() {
        return None;
    }
    let i = i.unwrap();

    Some(FactionRecord {
        factionid: Vec::<i8>::from(jwt.faction_id)[i],
        factionname: Vec::<String>::from(jwt.faction_name)[i].clone(),
        factionshortname: shorts[i].clone(),
        positionid: Vec::<i8>::from(jwt.position_id)[i],
        positionname: Vec::<String>::from(jwt.position_name)[i].clone(),
        shiftid: Vec::<i8>::from(jwt.shift_id)[i],
        shiftname: Vec::<String>::from(jwt.shift_name)[i].clone(),
    })
}

// pub fn get_faction_by_id(id: i8) -> Factions {
//     match id {
//         1 => Factions::SCKK,
//         3 => Factions::TOW,
//         _ => Factions::SCKK,
//     }
// }
