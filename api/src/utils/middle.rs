use axum::{extract::Request, http::HeaderMap, middleware::Next, response::IntoResponse};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::{
    auth::validate_jwt,
    config::{loader::get_config, structs::AccessConfig},
};

use super::{
    factions::{get_faction_from_jwt, Factions},
    functions::{get_env_mode, EnvModes},
    permissions::{get_perm, Permissions},
};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct FactionRecord {
    pub factionid: i8,
    pub factionname: String,
    pub factionshortname: String,
    pub positionid: i8,
    pub positionname: String,
    pub shiftid: i8,
    pub shiftname: String,
}

#[derive(Debug, Serialize)]
pub struct SAMTAuth {
    pub userdiscordid: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Driver {
    pub driverid: i32,
    pub name: String,
    pub admin: bool,
    pub perms: Vec<String>,
    pub faction: Option<Factions>,
    pub factions: Option<FactionRecord>,
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
    let config = get_config().await;
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
    let config = get_config().await;
    if auth.is_none() {
        return Err((StatusCode::NOT_FOUND, "Nincs kuki".to_string()));
    }

    let jwt = validate_jwt(auth.unwrap().to_str().unwrap().to_owned()).await;
    if jwt.is_none() {
        return Err((StatusCode::NOT_ACCEPTABLE, "Invalid JWT Token".to_string()));
    }
    let jwt = jwt.unwrap();
    let env_mode = get_env_mode().await;
    if (env_mode == EnvModes::Testing)
        && !jwt.permissions.contains(&get_perm(Permissions::SaesTest))
        && !jwt.is_sys_admin
    {
        return Err((
            StatusCode::FORBIDDEN,
            "Nincs jogod a teszt oldalhoz! (saes.test)".to_string(),
        ));
    }
    if (env_mode == EnvModes::Devel) && !jwt.is_sys_admin {
        return Err((
            StatusCode::FORBIDDEN,
            "Nincs jogod a dev oldalhoz!".to_string(),
        ));
    }
    if jwt.permissions.contains(&get_perm(Permissions::SaesLogin)) || jwt.is_sys_admin {
        let fact = match faction {
            None => None,
            Some(val) => {
                if val.to_str().is_ok() {
                    if val.to_str().unwrap() == Factions::SCKK.to_string() {
                        if jwt
                            .permissions
                            .contains(&get_perm(Permissions::SaesUcp(Factions::SCKK)))
                            || jwt.is_sys_admin
                        {
                            Some(Factions::SCKK)
                        } else {
                            None
                        }
                    } else if val.to_str().unwrap() == Factions::TOW.to_string() {
                        if jwt
                            .permissions
                            .contains(&get_perm(Permissions::SaesUcp(Factions::TOW)))
                            || jwt.is_sys_admin
                        {
                            Some(Factions::TOW)
                        } else {
                            None
                        }
                    } else if val.to_str().unwrap() == Factions::APMS.to_string() {
                        if jwt
                            .permissions
                            .contains(&get_perm(Permissions::SaesUcp(Factions::APMS)))
                            || jwt.is_sys_admin
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
            let records = get_faction_from_jwt(jwt.clone(), fact.unwrap());
            match fact.unwrap() {
                facto => {
                    if !config.factions.get(&facto).unwrap().site_access.ucp {
                        return Err((
                            StatusCode::FORBIDDEN,
                            "Frakciód nem rendelkezik ezzel a jogosultsággal!".to_string(),
                        ));
                    }
                }
            }
            let tag = Driver {
                name: jwt.username,
                driverid: jwt.id,
                admin: jwt.is_sys_admin,
                perms: jwt.permissions,
                faction: fact,
                factions: records,
            };
            request.extensions_mut().insert(tag);
        } else {
            let tag = Driver {
                name: jwt.username,
                driverid: jwt.id,
                admin: jwt.is_sys_admin,
                perms: jwt.permissions,
                faction: None,
                factions: None,
            };
            request.extensions_mut().insert(tag);
        }
        return Ok(next.run(request).await);
    } else {
        return Err((
            StatusCode::FORBIDDEN,
            "Nincs jogod a belépéshez! (saes.login)".to_string(),
        ));
    }
}

pub async fn shift_auth(
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let exts: Option<&Driver> = req.extensions_mut().get();
    let uwrp = exts.expect("Tag lekérése sikertelen, ucp_auth megtörtént?");
    let config = get_config().await;
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
    let config = get_config().await;
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
    let config = get_config().await;
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
