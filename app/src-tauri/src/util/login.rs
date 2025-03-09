use serde::Deserialize;
use tauri::AppHandle;
use tiny_http::{Response, Server};
use tokio::net::TcpListener;
use url::Url;

#[derive(Debug, Deserialize)]
struct AuthQuery {
    code: String,
}

#[tauri::command]
pub async fn begin_login() {
    tauri::async_runtime::spawn(async move {
        let server = Server::http("127.0.0.1:31313").unwrap();
        for req in server.incoming_requests() {
            let url = format!("http://127.0.0.1:31313{}", req.url());
            let parsed_url = Url::parse(&url).unwrap();
            let queries = parsed_url.query_pairs();
            let mut code = None;
            for (key, value) in queries {
                if key == "code" {
                    code = Some(value.to_string());
                    break;
                }
            }
            let response = Response::from_string(code.unwrap());
            req.respond(response).unwrap();
        }
    });
}
