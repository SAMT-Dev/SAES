use crate::BASE_HASHMAP;

pub struct Apis {
    pub fms: String,
    pub fms_key: String,
    pub sckkapp_tow: String,
    pub sckkapp_taxi: String,
}

pub async fn get_api_envs() -> Apis {
    let hash = BASE_HASHMAP.read().await;
    let fms = hash.get("env_fms_api").unwrap();
    let fms_key = hash.get("env_fms_api_key").unwrap();
    let sckkapp_taxi = hash.get("env_sckkapp_api_taxi").unwrap();
    let sckkapp_tow = hash.get("env_sckkapp_api_tow").unwrap();
    Apis {
        fms: fms.to_owned(),
        fms_key: fms_key.to_owned(),
        sckkapp_taxi: sckkapp_taxi.to_owned(),
        sckkapp_tow: sckkapp_tow.to_owned(),
    }
}
