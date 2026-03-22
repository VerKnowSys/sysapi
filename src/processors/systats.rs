use crate::apis::systat::Systat;
use rocket::{get, serde::json::Json};


/// handle GET for /version
#[get("/version")]
pub fn api_version_get_handler() -> String {
    let api_version = env!("CARGO_PKG_VERSION");
    format!("{{\"status\": \"OK\", \"version\": \"{}\"}}", api_version)
}


/// handle GET for /systat
#[get("/systat")]
pub fn api_systat_get_handler() -> Json<Systat> {
    Json(Systat::default())
}
