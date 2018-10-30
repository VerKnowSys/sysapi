//! sysapi.centra.systems

// #[macro_use]
extern crate log;
extern crate fern;
extern crate colored;
// #[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate futures;
extern crate gotham;
extern crate hyper;
extern crate mime;
extern crate regex;
extern crate serde;
// #[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate sysapi;


use std::env;
use log::*;
use fern::*;
use chrono::Local;
use std::fs::File;
use fern::colors::{Color, ColoredLevelConfig};
use colored::*;


use sysapi::router;
use sysapi::DEFAULT_ADDRESS;


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

    // Dispatch logger and start the server:
    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {}] {}: {}",
                Local::now().format("%d-%H%M%S"),
                default_colors.color(record.level()),
                record.target().cyan(),
                message
            ))
        })
        .level(loglevel)
        // .chain(std::io::stdout())
        .chain(log_file("/var/log/sysapi.log")
                    .unwrap_or(File::open("/dev/stdout")
                    .expect("FATAL: No /dev/stdout!?")))
        .apply()
        .and_then(|_| {
            // Read environment values and start server:
            let listen_address = match env::var("LISTEN_ADDRESS") {
                Ok(addr) => addr,
                Err(_) => DEFAULT_ADDRESS.to_string(),
            };
            let version = env!("CARGO_PKG_VERSION");
            info!("ServeD-SysAPI v{}, started on: http://{}", version, listen_address);
            Ok(gotham::start(listen_address, router::router()))
        })
        .map_err(|err| {
            panic!("FATAL: Couldn't initialize default logger. Error: {:?}", err);
        })
        .unwrap_or(());

}
