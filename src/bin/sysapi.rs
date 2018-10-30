//! sysapi.centra.systems

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate simple_logger;
extern crate futures;
extern crate gotham;
extern crate hyper;
extern crate mime;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;


use std::env;
use log::Level;


extern crate sysapi;
use sysapi::api::*;
use sysapi::webapi::*;


/// Start a server and use a `Router` to dispatch requests
pub fn main() {
    simple_logger::init_with_level(Level::Info).unwrap_or(());

    let key = "LISTEN_ADDRESS";
    let listen_address = match env::var(key) {
        Ok(addr) => addr,
        Err(_) => DEFAULT_ADDRESS.to_string(),
    };
    let version = env!("CARGO_PKG_VERSION");
    info!("ServeD SysAPI v{} started on: http://{}", version, listen_address);
    gotham::start(listen_address, router::router())
}