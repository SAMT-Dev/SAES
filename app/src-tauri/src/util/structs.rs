use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UCPReturn {
    pub name: String,
    pub admin: bool,
    pub perms: Vec<String>,
}
