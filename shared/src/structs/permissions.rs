use serde::{Deserialize, Serialize};

use super::factions::{get_faction_string, Factions};

#[derive(Debug, Deserialize, Serialize)]
pub enum Permissions {
    SaesLogin,
    SaesMaintenance,
    SaesTest,
    SaesUcp(Factions),
    SaesAdmin(Factions),
    SaesAdminShift(Factions),
    SaesAdminFleet(Factions),
    SaesAdminFaction(Factions),
}

pub fn get_subperm(perm: &str, fact: Factions) -> String {
    let faction = get_faction_string(fact);
    perm.replace("fc", &faction)
}

pub fn get_perm(perm: Permissions) -> String {
    match perm {
        Permissions::SaesLogin => "saes.login".to_string(),
        Permissions::SaesMaintenance => "saes.maintenance".to_string(),
        Permissions::SaesAdmin(fact) => get_subperm("saes.fc.admin", fact),
        Permissions::SaesUcp(fact) => get_subperm("saes.fc.ucp", fact),
        Permissions::SaesAdminShift(fact) => get_subperm("saes.fc.admin.shift", fact),
        Permissions::SaesAdminFaction(fact) => get_subperm("saes.fc.admin.faction", fact),
        Permissions::SaesAdminFleet(fact) => get_subperm("saes.fc.admin.fleet", fact),
        Permissions::SaesTest => "saes.test".to_string(),
    }
}
