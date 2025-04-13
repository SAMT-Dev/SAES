use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UCPReturn {
    pub driverid: i32,
    pub name: String,
    pub admin: bool,
    pub perms: Vec<String>,
}
