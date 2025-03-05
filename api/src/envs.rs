use std::env;

use tracing::info;

use crate::BASE_HASHMAP;

pub async fn load_envs() {
    info!("ENV Precheck");
    let envs: Vec<&str> = vec![
        "DATABASE_URL",
        "DISCORD_ID",
        "DISCORD_SECRET",
        "REDIRECT_URL",
        "DOMAIN",
        "FULL_DOMAIN",
        "FMS_API",
        "FMS_API_KEY",
        "SCKKAPP_API",
    ];
    let mut hash = BASE_HASHMAP.write().await;
    for env in envs {
        let env_val = env::var(env);
        if env_val.is_err() {
            panic!("{} nincs setelve!", env);
        }
        hash.insert(format!("env_{}", env.to_lowercase()), env_val.unwrap());
    }
}
