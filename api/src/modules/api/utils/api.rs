use crate::config::loader::get_module_config;

pub struct Apis {
    pub fms: String,
    pub fms_key: String,
    pub sckkapp_tow: String,
    pub sckkapp_taxi: String,
}

pub async fn get_api_envs() -> Apis {
    let conf = get_module_config().await.api.unwrap();
    let fms = conf.fms_api;
    let fms_key = conf.fms_api_key;
    let sckkapp_taxi = conf.sckkapp_api_taxi;
    let sckkapp_tow = conf.sckkapp_api_tow;
    Apis {
        fms: fms.to_owned(),
        fms_key: fms_key.to_owned(),
        sckkapp_taxi: sckkapp_taxi.to_owned(),
        sckkapp_tow: sckkapp_tow.to_owned(),
    }
}
