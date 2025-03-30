use axum::{middleware, routing::get, Router};

use crate::utils::middle::ucp_auth;

mod admin;
mod base;
mod calls;
mod faction;
mod items;
mod shift;
mod sys;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(base::ucp_home))
        .route("/getusernames", get(base::ucp_getusernames))
        .route("/getlegacynames", get(base::ucp_getlegacyusernames))
        .route("/apms_calls", get(calls::ucp_apms_calls))
        .route("/calls", get(calls::ucp_calls))
        .nest("/admin", admin::routes())
        .nest("/items", items::routes())
        .nest("/sys", sys::routes())
        .layer(middleware::from_fn(ucp_auth))
}
