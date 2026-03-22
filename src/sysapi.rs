//! SysAPI Dashboard Web Server

#![deny(
    missing_docs,
    unstable_features,
    unsafe_code,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]


use rocket::fs::FileServer;
use rocket::{self, launch};
use std::path::Path;

use sysapi::helpers::listen_address;
use sysapi::webrouter::routes;
use sysapi::*;


/// Rocket server entry point
#[launch]
fn rocket() -> _ {
    // Initial logger setup
    let _handle = initialize_logger();

    info!("SysAPI: Server started on address: {}", listen_address());

    rocket::build().mount("/", routes()).mount(
        "/",
        FileServer::from(Path::new(PROJECT_DIRECTORY).join("web")),
    )
}
