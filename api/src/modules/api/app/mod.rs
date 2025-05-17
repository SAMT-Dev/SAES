use axum::{routing::get, Router};

pub mod login;

pub fn routes() -> Router {
    Router::new().route("/login_with_auth", get(login::withauth))
}
