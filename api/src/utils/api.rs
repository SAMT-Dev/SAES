use crate::BASE_HASHMAP;

pub struct Apis {
    pub fms: String,
    pub fms_key: String,
    pub sckkapp: String,
}

pub async fn get_api_envs() -> Apis {
    let hash = BASE_HASHMAP.read().await;
    let fms = hash.get("env_fms_api").unwrap();
    let fms_key = hash.get("env_fms_api_key").unwrap();
    let sckkapp = hash.get("env_sckkapp_api").unwrap();
    Apis {
        fms: fms.to_owned(),
        fms_key: fms_key.to_owned(),
        sckkapp: sckkapp.to_owned(),
    }
}
