use saes_shared::structs::user::Driver;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UCPReturnDriver {
    pub name: String,
    pub admin: bool,
    pub perms: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UCPReturn {
    pub driver: UCPReturnDriver,
}

#[derive(Debug, Deserialize)]
pub struct BundledDriver {
    pub driver: Driver,
}
