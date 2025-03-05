use axum::{extract::Request, http::HeaderMap, middleware::Next, response::IntoResponse};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{auth::get_discord_envs, config::structs::AccessConfig, MAIN_CONFIG, WEB_CLIENT};

use super::{
    api::get_api_envs,
    factions::Factions,
    functions::{get_env_mode, EnvModes},
    permissions::{get_perm, Permissions},
};

#[derive(Debug, Deserialize, Clone)]
pub struct DiscordUser {
    pub id: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct FactionRecord {
    pub factionid: i8,
    pub factionname: String,
    pub positionid: i8,
    pub positionname: String,
    pub shiftid: i8,
    pub shiftname: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GetUserRes {
    pub factionrecords: Vec<FactionRecord>,
    pub issysadmin: bool,
    pub permissions: Vec<String>,
    pub id: i8,
    pub username: String,
}

#[derive(Debug, Serialize)]
pub struct SAMTAuth {
    pub userdiscordid: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Driver {
    pub discordid: String,
    pub driverid: i8,
    pub name: String,
    pub admin: bool,
    pub perms: Vec<String>,
    pub taxi: Option<FactionRecord>,
    pub tow: Option<FactionRecord>,
    pub apms: Option<FactionRecord>,
    pub faction: Option<Factions>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AccessKeyExt {
    pub access: Vec<AccessConfig>,
    pub user: Option<String>,
}

pub async fn key_auth(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let config = MAIN_CONFIG.get().unwrap();
    let auth = headers.get("access-key");
    let user = headers.get("user-id");
    if auth.is_none() {
        return Err((StatusCode::UNAUTHORIZED, "Nincs access-key".to_string()));
    }
    let access = config
        .access_keys
        .iter()
        .find(|p| p.key == auth.unwrap().to_str().unwrap());
    if access.is_none() {
        return Err((StatusCode::FORBIDDEN, "Rossz access-key".to_string()));
    }
    request.extensions_mut().insert(AccessKeyExt {
        access: access.unwrap().access.clone(),
        user: if user.is_some() {
            Some(user.unwrap().to_str().unwrap().to_string())
        } else {
            None
        },
    });
    return Ok(next.run(request).await);
}

pub async fn ucp_auth(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let auth = headers.get("cookie");
    let faction = headers.get("faction");
    let ds = get_discord_envs().await;
    let envs = get_api_envs().await;
    if auth.is_some() {
        let dcuserget = WEB_CLIENT
            .get(format!("{}/users/@me", ds.api_endpoint))
            .header(
                "Authorization",
                format!("Bearer {}", auth.unwrap().to_str().unwrap()),
            )
            .send()
            .await
            .expect("Lekérés sikertelen");
        if dcuserget.status().as_u16() == 200 {
            let handled_user = dcuserget.text().await.expect("Átalakítás sikertelen");
            let parsed_user = serde_json::from_str(&handled_user);
            if parsed_user.is_ok() {
                let real_user: DiscordUser = parsed_user.unwrap();
                let getuser = WEB_CLIENT
                    .get(format!("{}/authenticate", envs.fms,))
                    .query(&[("dcid", real_user.id.clone())])
                    .header("authkey", envs.fms_key)
                    .send()
                    .await
                    .expect("Lekérés sikertelen");
                let status = getuser.status();
                let resp = getuser.text().await;
                if status == StatusCode::OK {
                    let parsed_tag = serde_json::from_str(&resp.unwrap());
                    if parsed_tag.is_ok() {
                        let real_tag: GetUserRes = parsed_tag.unwrap();
                        let env_mode = get_env_mode().await;
                        if (env_mode == EnvModes::Testing)
                            && !real_tag
                                .permissions
                                .contains(&get_perm(Permissions::SaesTest))
                            && !real_tag.issysadmin
                        {
                            return Err((
                                StatusCode::FORBIDDEN,
                                "Nincs jogod a teszt oldalhoz! (saes.test)".to_string(),
                            ));
                        }
                        if (env_mode == EnvModes::Devel) && !real_tag.issysadmin {
                            return Err((
                                StatusCode::FORBIDDEN,
                                "Nincs jogod a dev oldalhoz!".to_string(),
                            ));
                        }
                        if real_tag
                            .permissions
                            .contains(&get_perm(Permissions::SaesLogin))
                            || real_tag.issysadmin
                        {
                            let taxi = real_tag
                                .factionrecords
                                .iter()
                                .find(|fact| fact.factionid == 1)
                                .cloned();
                            let tow = real_tag
                                .factionrecords
                                .iter()
                                .find(|fact| fact.factionid == 3)
                                .cloned();
                            let apms = real_tag
                                .factionrecords
                                .iter()
                                .find(|fact| fact.factionid == 2)
                                .cloned();
                            let fact = match faction {
                                None => None,
                                Some(val) => {
                                    if val.to_str().is_ok() {
                                        if val.to_str().unwrap() == "SCKK" {
                                            if real_tag.permissions.contains(&get_perm(
                                                Permissions::SaesUcp(Factions::SCKK),
                                            )) || real_tag.issysadmin
                                            {
                                                Some(Factions::SCKK)
                                            } else {
                                                None
                                            }
                                        } else if val.to_str().unwrap() == "TOW" {
                                            if real_tag.permissions.contains(&get_perm(
                                                Permissions::SaesUcp(Factions::TOW),
                                            )) || real_tag.issysadmin
                                            {
                                                Some(Factions::TOW)
                                            } else {
                                                None
                                            }
                                        } else if val.to_str().unwrap() == "APMS" {
                                            if real_tag.permissions.contains(&get_perm(
                                                Permissions::SaesUcp(Factions::APMS),
                                            )) || real_tag.issysadmin
                                            {
                                                Some(Factions::APMS)
                                            } else {
                                                None
                                            }
                                        } else {
                                            None
                                        }
                                    } else {
                                        None
                                    }
                                }
                            };
                            if fact.is_some() {
                                match fact.unwrap() {
                                    Factions::SCKK => {
                                        let config = MAIN_CONFIG.get().unwrap();
                                        if !config
                                            .factions
                                            .get(&Factions::SCKK)
                                            .unwrap()
                                            .site_access
                                            .ucp
                                        {
                                            return Err((
                                                StatusCode::FORBIDDEN,
                                                "Frakciód nem rendelkezik ezzel a jogosultsággal!"
                                                    .to_string(),
                                            ));
                                        }
                                    }
                                    Factions::TOW => {
                                        let config = MAIN_CONFIG.get().unwrap();
                                        if !config
                                            .factions
                                            .get(&Factions::TOW)
                                            .unwrap()
                                            .site_access
                                            .ucp
                                        {
                                            return Err((
                                                StatusCode::FORBIDDEN,
                                                "Frakciód nem rendelkezik ezzel a jogosultsággal!"
                                                    .to_string(),
                                            ));
                                        }
                                    }
                                    Factions::APMS => {
                                        let config = MAIN_CONFIG.get().unwrap();
                                        if !config
                                            .factions
                                            .get(&Factions::APMS)
                                            .unwrap()
                                            .site_access
                                            .ucp
                                        {
                                            return Err((
                                                StatusCode::FORBIDDEN,
                                                "Frakciód nem rendelkezik ezzel a jogosultsággal!"
                                                    .to_string(),
                                            ));
                                        }
                                    }
                                }
                            }
                            let tag = Driver {
                                discordid: real_user.id,
                                name: real_tag.username,
                                driverid: real_tag.id,
                                admin: real_tag.issysadmin,
                                perms: real_tag.permissions,
                                faction: fact,
                                taxi,
                                tow,
                                apms,
                            };
                            request.extensions_mut().insert(tag);
                            return Ok(next.run(request).await);
                        } else {
                            return Err((
                                StatusCode::FORBIDDEN,
                                "Nincs jogod a belépéshez! (saes.login)".to_string(),
                            ));
                        }
                    } else {
                        return Err((
                            StatusCode::FORBIDDEN,
                            "Nem sikerült informatikai tudásunkat alkalmazni ehhez a lekéréshez!"
                                .to_string(),
                        ));
                    }
                } else {
                    if status == StatusCode::NOT_FOUND {
                        return Err((StatusCode::FORBIDDEN, "Nincs jogod ehhez!".to_string()));
                    } else if status == StatusCode::INTERNAL_SERVER_ERROR {
                        return Err((
                            StatusCode::PAYMENT_REQUIRED,
                            format!("SAMT API lekérés sikertelen! {:?}", resp.unwrap()),
                        ));
                    } else {
                        return Err((
                            StatusCode::PAYMENT_REQUIRED,
                            "SAMT API-t nem értük el. 🥺".to_string(),
                        ));
                    }
                }
            } else {
                return Err((StatusCode::BAD_REQUEST, "Érvénytelen lekérés!".to_string()));
            }
        } else {
            return Err((StatusCode::UNAUTHORIZED, "Discord Auth failed".to_string()));
        }
    } else {
        return Err((StatusCode::NOT_FOUND, "Nincs kuki".to_string()));
    };
}

pub async fn shift_auth(
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let exts: Option<&Driver> = req.extensions_mut().get();
    let uwrp = exts.expect("Tag lekérése sikertelen, ucp_auth megtörtént?");
    let config = MAIN_CONFIG.get().unwrap();
    if uwrp.faction.is_some() {
        let fact = if uwrp.perms.contains(&get_perm(Permissions::SaesAdminShift(
            uwrp.faction.unwrap(),
        ))) {
            true
        } else {
            false
        };

        if (uwrp.admin || fact)
            && config
                .factions
                .get(&uwrp.faction.unwrap())
                .unwrap()
                .site_access
                .shift
        {
            return Ok(next.run(req).await);
        } else {
            return Err((StatusCode::FORBIDDEN, "Nincs jogod! (saes.sm)".to_string()));
        }
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            "Frakciójelölés hiányzik!".to_string(),
        ));
    }
}

pub async fn admin_auth(
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let exts: Option<&Driver> = req.extensions_mut().get();
    let uwrp = exts.expect("Tag lekérése sikertelen, ucp_auth megtörtént?");
    let config = MAIN_CONFIG.get().unwrap();
    if uwrp.faction.is_some() {
        let fact = if uwrp
            .perms
            .contains(&get_perm(Permissions::SaesAdmin(uwrp.faction.unwrap())))
        {
            true
        } else {
            false
        };

        if (uwrp.admin || fact)
            && config
                .factions
                .get(&uwrp.faction.unwrap())
                .unwrap()
                .site_access
                .admin
        {
            return Ok(next.run(req).await);
        } else {
            return Err((
                StatusCode::FORBIDDEN,
                "Nincs jogod! (saes.admin)".to_string(),
            ));
        }
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            "Frakciójelölés hiányzik!".to_string(),
        ));
    }
}

pub async fn faction_auth(
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let exts: Option<&Driver> = req.extensions_mut().get();
    let config = MAIN_CONFIG.get().unwrap();
    let uwrp = exts.expect("Tag lekérése sikertelen, ucp_auth megtörtént?");
    if uwrp.faction.is_none() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Frakciójelölés hiányzik!".to_string(),
        ));
    }
    let fact = if uwrp.perms.contains(&get_perm(Permissions::SaesAdminFaction(
        uwrp.faction.unwrap(),
    ))) {
        true
    } else {
        false
    };

    if !uwrp.admin
        && !fact
        && !config
            .factions
            .get(&uwrp.faction.unwrap())
            .unwrap()
            .site_access
            .faction
    {
        return Err((StatusCode::FORBIDDEN, "Nincs jogod! (saes.fm)".to_string()));
    }
    return Ok(next.run(req).await);
}

pub async fn sysadmin_auth(
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let exts: Option<&Driver> = req.extensions_mut().get();
    let uwrp = exts.expect("Tag lekérése sikertelen, ucp_auth megtörtént?");
    if uwrp.admin {
        return Ok(next.run(req).await);
    }
    return Err((StatusCode::FORBIDDEN, "Nincs jogod!".to_string()));
}
