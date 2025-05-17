use crate::config::loader::get_config;

pub async fn get_faction_by_id(id: i8) -> Option<String> {
    let config = get_config().await;
    let faction = config.factions.iter().find(|f| f.1.settings.id == id);
    match faction {
        Some(f) => Some(f.0.clone()),
        None => None,
    }
}
