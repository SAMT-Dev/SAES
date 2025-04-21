use saes_shared::structs::factions::Factions;

use super::{middle::FactionRecord, structs::AuthJWT};

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
