use std::collections::HashMap;

use axum::extract::Query;
use axum::response::IntoResponse;
use axum::Json;
use axum::{debug_handler, response::Redirect};
use base64::engine::general_purpose;
use base64::Engine;
use cookie::time::Duration;
use http::{HeaderMap, StatusCode};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Header, Validation};
use random_string::{charsets, generate};
use saes_shared::structs::factions::FactionRecord;
use tokio::sync::RwLock;
use tower_cookies::{Cookie, Cookies};
use url_builder::URLBuilder;

use serde::{Deserialize, Serialize};

use crate::utils::api::get_api_envs;
use crate::utils::structs::{AuthJWT, StateList};
use crate::{BASE_HASHMAP, WEB_CLIENT};

static STATE_MANAGEMENT: RwLock<Vec<StateList>> = RwLock::const_new(Vec::new());

pub struct AuthEnvs {
    pub api_endpoint: String,
    pub client_id: String,
    pub client_secret: String,
    pub base_url: String,
    pub domain: String,
    pub fdomain: String,
    pub jwt_key: String,
}

pub async fn get_auth_envs() -> AuthEnvs {
    let hash = BASE_HASHMAP.read().await;
    let id = hash.get("env_discord_id").unwrap();
    let secret = hash.get("env_discord_secret").unwrap();
    let url = "https://discord.com";
    let domain = hash.get("env_domain").unwrap();
    let base_url = hash.get("env_api_base_url").unwrap();
    let fdomain = hash.get("env_full_domain").unwrap();
    let jwt = hash.get("env_jwt_key").unwrap();
    AuthEnvs {
        api_endpoint: url.to_owned(),
        domain: domain.to_owned(),
        fdomain: fdomain.to_owned(),
        client_id: id.to_owned(),
        client_secret: secret.to_owned(),
        base_url: base_url.to_owned(),
        jwt_key: jwt.to_owned(),
    }
}

#[derive(Debug, Deserialize)]
pub struct Code {
    code: String,
    state: String,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: i64,
    refresh_token: String,
}

#[debug_handler]
pub async fn base_callback(Query(query): Query<Code>, cookies: Cookies) -> Redirect {
    let ds = get_auth_envs().await;
    let path = String::from_utf8(general_purpose::STANDARD.decode(query.state).unwrap()).unwrap();
    let path_full: AuthState = serde_json::from_str(&path).expect("Nem megy");
    let code_verifier = cookies.get("oauth-session");
    if code_verifier.is_none() {
        if path_full.mode == "app".to_string() {
            return Redirect::to("http://localhost:31313/app-auth/cb?code=oauth2");
        }
        return Redirect::to(&format!("{}?error=oauth2", &ds.fdomain));
    }
    let code_verifier = code_verifier.unwrap();
    let item = {
        let states = STATE_MANAGEMENT.read().await;
        let state = states
            .iter()
            .find(|p| p.token == code_verifier.value() && p.id == path_full.id);
        if state.is_none() {
            None
        } else {
            Some(StateList {
                id: state.unwrap().id,
                token: state.unwrap().token.clone(),
            })
        }
    };
    if item.is_none() {
        if path_full.mode == "app".to_string() {
            return Redirect::to("http://localhost:31313/app-auth/cb?code=oauth22");
        }
        return Redirect::to(&format!("{}?error=oauth22", &ds.fdomain));
    }
    {
        let pos = STATE_MANAGEMENT
            .read()
            .await
            .iter()
            .position(|p| p.id == p.id)
            .unwrap();
        STATE_MANAGEMENT.write().await.remove(pos);
    }
    let data = [
        ("grant_type", "authorization_code"),
        ("code", &query.code),
        ("redirect_uri", &format!("{}/auth/cb", ds.base_url)),
        ("client_id", &ds.client_id),
        ("client_secret", &ds.client_secret),
    ];
    let url = format!("{}/api/oauth2/token", ds.api_endpoint);
    let response = WEB_CLIENT.post(&url).form(&data).send().await;
    let token_response: String = response
        .expect("Lekérés sikertelen")
        .text()
        .await
        .expect("Átalakítás sikertelen");
    let object = serde_json::from_str(&token_response);
    if object.is_err() {
        if path_full.mode == "app".to_string() {
            return Redirect::to("http://localhost:31313/app-auth/cb?code=noperm");
        }
        return Redirect::to(&format!(
            "{}?error=noperm&details={}",
            &ds.fdomain, token_response
        ));
    }
    let object: TokenResponse = object.unwrap();
    if path_full.mode == "app".to_string() {
        return Redirect::to(&format!(
            "http://localhost:31313/app-auth/cb?code={}",
            object.access_token
        ));
    }
    cookies.add(
        Cookie::build(("dc-auth", object.access_token.clone()))
            .max_age(Duration::seconds(object.expires_in))
            .domain(ds.domain.clone())
            .same_site(cookie::SameSite::Lax)
            .http_only(true)
            .secure(true)
            .path("/")
            .build(),
    );
    cookies.add(
        Cookie::build(("dc-refresh", object.refresh_token))
            .max_age(Duration::seconds(object.expires_in * 30))
            .domain(ds.domain.clone())
            .same_site(cookie::SameSite::Lax)
            .secure(true)
            .http_only(true)
            .path("/")
            .build(),
    );
    return Redirect::to(&format!("{}{}", &ds.fdomain, path_full.path));
}

#[derive(Debug, Deserialize)]
struct AtMe {
    id: String,
}

#[derive(Debug, Deserialize)]
struct FMSRes {
    factionrecords: Vec<FactionRecord>,
    id: i32,
    issysadmin: bool,
    permissions: Vec<String>,
    username: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct JWTRet {
    jwt: String,
    exp: i64,
}

#[debug_handler]
pub async fn get_jwt(h: HeaderMap) -> Result<impl IntoResponse, (StatusCode, String)> {
    let auth = h.get("cookie");
    if auth.is_none() {
        return Err((StatusCode::UNAUTHORIZED, "No auth".to_string()));
    };
    let auth = auth.unwrap().to_str().unwrap().to_owned();
    let me = WEB_CLIENT
        .get("https://discord.com/api/v10/users/@me")
        .bearer_auth(auth)
        .send()
        .await;
    if me.is_err() {
        return Err((StatusCode::BAD_REQUEST, "".to_string()));
    }
    let me = me.unwrap();
    if !me.status().is_success() {
        return Err((StatusCode::BAD_REQUEST, "".to_string()));
    }
    let me: AtMe = me.json().await.unwrap();
    let apis = get_api_envs().await;
    let fmsget = WEB_CLIENT
        .get(format!("{}/authenticate?dcid={}", apis.fms, me.id))
        .header("authkey", apis.fms_key)
        .send()
        .await;
    if fmsget.is_err() {
        if fmsget.unwrap_err().status() == Some(StatusCode::NOT_FOUND) {
            return Err((StatusCode::UNAUTHORIZED, "Nem létezel".to_string()));
        }
        return Err((StatusCode::BAD_REQUEST, "".to_string()));
    }
    let fmsget: FMSRes = fmsget.unwrap().json().await.unwrap();
    let mut jwt = AuthJWT {
        admin: fmsget.issysadmin,
        exp: (chrono::Utc::now() + chrono::Duration::hours(6)).timestamp(),
        id: fmsget.id,
        permissions: fmsget.permissions,
        username: fmsget.username,
        factions: HashMap::new(),
    };
    for faction in fmsget.factionrecords {
        jwt.factions.insert(
            faction.factionid,
            FactionRecord {
                factionid: faction.factionid,
                factionname: faction.factionname,
                factionshortname: faction.factionshortname,
                positionname: faction.positionname,
                shiftname: faction.shiftname,
                positionid: faction.positionid,
                shiftid: faction.shiftid,
            },
        );
    }
    let envs = get_auth_envs().await;
    let jwt_encoded = jsonwebtoken::encode(
        &Header::default(),
        &jwt,
        &EncodingKey::from_secret(&envs.jwt_key.as_bytes()),
    )
    .unwrap();
    return Ok(Json(JWTRet {
        jwt: jwt_encoded,
        exp: jwt.exp,
    }));
}

fn base_path() -> String {
    "/ucp".to_string()
}

// * OBSOLETE BY NEW AUTH

// #[derive(Deserialize, Debug)]
// pub struct WebtransferQuery {
//     pub jwt: String,
// }

// pub async fn webtransfer(q: Query<WebtransferQuery>, c: Cookies) -> Redirect {
//     let ds = get_auth_envs().await;
//     let jwt = validate_jwt(q.jwt.clone()).await;
//     let mut setcookie = true;
//     if jwt.is_some() {
//         let jwt = jwt.unwrap();
//         let old_jwt = c.get("auth_token");
//         if old_jwt.is_some() {
//             let old_jwt = validate_jwt(old_jwt.unwrap().value().to_string()).await;
//             if old_jwt.is_some() {
//                 setcookie = false;
//             }
//         }
//         if setcookie {
//             c.add(
//                 Cookie::build(("auth_token", q.jwt.clone()))
//                     .expires(OffsetDateTime::from_unix_timestamp(jwt.exp).unwrap())
//                     .http_only(true)
//                     .secure(true)
//                     .domain(ds.domain.clone())
//                     .path("/")
//                     .build(),
//             );
//         }
//     }
//     Redirect::to(&format!("{}/ucp", ds.fdomain.clone()))
// }

pub async fn validate_jwt(token: String) -> Option<AuthJWT> {
    let hash = BASE_HASHMAP.read().await;
    let key = hash.get("env_jwt_key").unwrap();
    let jwt = decode::<AuthJWT>(
        &token,
        &DecodingKey::from_secret(key.as_bytes()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    );
    if jwt.is_err() {
        return None;
    }
    return Some(jwt.unwrap().claims);
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthHomeCode {
    #[serde(default = "base_path")]
    path: String,
    mode: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthState {
    path: String,
    id: i32,
    mode: String,
}

#[debug_handler]
pub async fn auth_home(Query(q): Query<AuthHomeCode>, cookies: Cookies) -> Redirect {
    let auth_envs = get_auth_envs().await;
    let mut ub = URLBuilder::new();
    let code_verifier = generate(32, charsets::ALPHANUMERIC);
    let id = loop {
        let r = rand::random_range(0..1000);
        if !STATE_MANAGEMENT.read().await.iter().any(|s| s.id == r) {
            break r;
        }
    };
    {
        STATE_MANAGEMENT.write().await.push(StateList {
            id,
            token: code_verifier.clone(),
        });
    };
    cookies.add(
        Cookie::build(("oauth-session", code_verifier.clone()))
            .domain(auth_envs.domain)
            .http_only(true)
            .secure(true)
            .max_age(Duration::minutes(60))
            .same_site(cookie::SameSite::Lax)
            .path("/")
            .build(),
    );
    let state = AuthState {
        path: q.path,
        id,
        mode: if q.mode.is_some() {
            q.mode.unwrap()
        } else {
            "web".to_string()
        },
    };
    let state_str = serde_json::to_string(&state).expect("Sikertelen átalakítás");
    ub.set_host(&format!("{}/oauth2/authorize", auth_envs.api_endpoint))
        .add_param("state", &general_purpose::STANDARD.encode(state_str))
        .add_param("client_id", &auth_envs.client_id)
        .add_param("scope", "identify")
        .add_param("response_type", "code")
        .add_param("redirect_uri", &format!("{}/auth/cb", auth_envs.base_url));
    let mut built_url = ub.build();
    built_url.remove(0);
    built_url.remove(0);
    built_url.remove(0);
    Redirect::to(&built_url)
}
