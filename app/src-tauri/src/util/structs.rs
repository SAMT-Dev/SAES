use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UCPReturn {
    discordid: String,
    driverid: i32,
    name: String,
    admin: bool,
    perms: Vec<String>,
}
