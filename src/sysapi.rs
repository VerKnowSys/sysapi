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
    unused_qualifications)]


#[allow(unused_imports)]
#[macro_use]
extern crate log;

#[allow(unused_imports)]
#[macro_use]
extern crate serde_derive;

extern crate sysapi;


use fern::log_file;
use colored::Colorize;
use std::env;
use log::LevelFilter;
use fern::Dispatch;
use chrono::Local;
use std::fs::File;
use fern::colors::{Color, ColoredLevelConfig};
use hostname::get_hostname;
use std::path::Path;
use futures::future;
use tokio::runtime::Runtime;


use crate::sysapi::{DEFAULT_ADDRESS, DEFAULT_LOG_FILE, ZFS_BIN};
use crate::sysapi::webrouter::router;


#[link(name = "kvmpro", kind = "dylib")]

/// Start a server and use a `Router` to dispatch requests
pub fn main() {
    // Set up ANSI colors for output:
    let default_colors = ColoredLevelConfig::new()
        .info(Color::White)
        .debug(Color::Magenta)
        .error(Color::Red)
        .warn(Color::Yellow);

    // Read value of DEBUG from env, if defined switch log level to Debug:
    let loglevel = match env::var("DEBUG") {
        Ok(_) => LevelFilter::Debug,
        Err(_) => LevelFilter::Info,
    };

    // Read environment values:
    let listen_address = match env::var("LISTEN_ADDRESS") {
        Ok(addr) => addr,
        Err(_) => DEFAULT_ADDRESS.to_string(),
    };
    let version = env!("CARGO_PKG_VERSION");

    // Create the runtime
    let mut runtime: Runtime = match Runtime::new() {
        Ok(runtime) => runtime,
        Err(err) => {
            panic!("SysAPI: Runtime: Assertion Failed! Details: {}", err);
        }
    };

    // Dispatch logger:
    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} {}: {}: {}",
                Local::now().format("%d-%H%M%S").to_string().black(),
                default_colors.color(record.level()),
                record.target().cyan(),
                message
            ))
        })
        .level(loglevel)
        .chain(log_file(DEFAULT_LOG_FILE)
                    .unwrap_or(File::open("/dev/stdout")
                    .expect("FATAL: No /dev/stdout!?")))
        .apply()
        .and_then(|_| {
            // Start main event loop:
            info!("ServeD-SysAPI (v{}) - started on hostname: {}: http://{}",
                  version, get_hostname().unwrap_or(String::from("localhost")), listen_address);
            Ok(())
        })
        .map_err(|err| {
            error!("FATAL: Couldn't initialize SysAPI. Error details: {:?}", err);
        })
        .unwrap();

    // Last check - sysapi relies on zfs feature:
    if !Path::new(ZFS_BIN).exists() {
        error!("SysAPI requires ZFS functionality available in system!");
        panic!("FATAL: ZFS feature is NOT available!");
    }

    // Define gotham server Future:
    runtime.spawn(future::lazy(|| {
        info!("Example async-lazy-worker-thread… Yay!");

        Ok(())
    }));

    // NOTE: Use runtime.spawn(_) to launch future services like this:
    let gotham = gotham::init_server(listen_address, router());
    // Spawn the server task
    runtime
        .block_on_all(gotham) // Block forever on "serving duties"
        .unwrap_or_default();
}
