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
use futures::future;
use tokio::runtime::Runtime;


use crate::sysapi::*;
use crate::sysapi::helpers::*;
use crate::sysapi::webrouter::router;


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

    // Create the runtime:
    let mut runtime: Runtime = match Runtime::new() {
        Ok(runtime) => runtime,
        Err(err) => panic!("{}: SysAPI: Runtime: Assertion Failed! Details: {}", "FATAL ERROR".blue(), err.to_string().red())
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
        .chain(
            log_file(DEFAULT_LOG_FILE)
                .unwrap_or(
                    File::open(DEFAULT_STDOUT_DEV)
                        .expect(
                            &format!("{}: STDOUT device '{}' is not available! Something is terribly wrong here!",
                                     "FATAL ERROR".blue(), DEFAULT_STDOUT_DEV.cyan())
                        )
                )
        )
        .apply()
        .and_then(|_| { // Use initialized logger to introduce itself
            print_header();
            Ok(())
        })
        .map_err(|err| {
            error!("{}: Couldn't initialize SysAPI. Details: {}",
                   "FATAL ERROR".blue(), err.to_string().red());
        })
        .unwrap();

    // Perform sanity checks:
    sanity_checks();

    runtime.spawn(future::lazy(|| {
        info!("SysAPI: Status: {}, working on hostname: {}",
              "Online".cyan(), current_hostname().cyan());
        Ok(())
    }));

    // NOTE: Use runtime.spawn(_) to launch future services like this:
    let gotham = gotham::init_server(listen_address(), router());
    // Spawn the server task
    runtime
        .block_on_all(gotham) // Block forever on "serving duties"
        .unwrap_or_default();
}
