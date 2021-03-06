use glob::glob;
use hostname::get_hostname;
use std::{path::Path, env};
use colored::Colorize;

use crate::*;


/// Get current hostname as String
pub fn current_hostname() -> String {
    get_hostname()
        .unwrap_or_else(|| DEFAULT_HOSTNAME_FALLBACK.to_string())
}


/// Get listen address to bind Web-Service
pub fn listen_address() -> String {
    match env::var("LISTEN_ADDRESS") {
        Ok(addr) => addr,
        Err(_) => DEFAULT_ADDRESS.to_string(),
    }
}


/// Prints message pack on SysAPI service start:
pub fn print_header() {
    let version = format!("v{}", env!("CARGO_PKG_VERSION"));
    info!("_________________________________________________________________________________________________");
    info!("SysAPI {} - design, implementation: {} - licensed under: {}.", version.cyan(), CREATED_BY.cyan(), "MIT/BSD".cyan());
    info!("");
    info!("  This project is only a component of the '{}':", "ServeD-OS project".cyan());
    info!("  {}-driven, modern, production-quality operating-system, developed since 2011.", "HardenedBSD".cyan());
    info!("");
    info!("  Other projects used by the system:");
    info!("    - {} -> {}", "sysapi".cyan(), "https://github.com/VerKnowSys/sysapi".green());
    info!("    - {} -> {}", "svdOS".cyan(), "https://github.com/VerKnowSys/svdOS".green());
    info!("    - {} -> {}", "Sofin".cyan(), "https://github.com/VerKnowSys/sofin".green());
    info!("    - {} -> {}", "Sofin-definitions".cyan(), "https://github.com/VerKnowSys/sofin-definitions".green());
    info!("    - {} -> {}", "kvmpro".cyan(), "https://github.com/VerKnowSys/kvmpro".green());
    info!("    - {} -> {}", "Shable".cyan(), "https://github.com/VerKnowSys/Shable".green());
    info!("");
    info!("SysAPI: ControlPane URL: {}", format!("{}://{}", DEFAULT_CONTROLPANE_PROTOCOL, listen_address()).cyan());
}


/// Perform checks before starting web service:
pub fn sanity_checks() {
    info!("SysAPI: Validating availability of all mission-critical features of the underlying operating system…");
    if !Path::new(DEFAULT_LIBKVMPRO_SHARED).exists() {
        warn!("SysAPI: Shared library: '{}' is not installed yet!", DEFAULT_LIBKVMPRO_SHARED.red());
        warn!("SysAPI: This lib provides APIs of: '{}' and '{}' kernel features used to fetch processes data directly from the kernel.", "Kvm".red(), "procstat".red(),);
        warn!("SysAPI: These APIs are: {} without that library!\n", "completely unusable and unavailable".red());
        warn!("HINT: To install library from provided git-module: {}, simply do:", "lib/kvmpro".cyan());
        warn!("      {}\n", "cd lib/kvmpro && bin/test && bin/install && cd ../..".cyan());
        warn!("NOTE: You don't have to restart SysAPI server after installation of the shared-library.");
        warn!("NOTE: Once shared-library will be installed - it will be loaded automatically.\n\n");
    }
    if !Path::new(ZFS_BIN).exists() {
        error!("SysAPI requires ZFS functionality to be available in base system!");
        panic!("{}: ZFS is {}!", "FATAL ERROR".blue(), "mission-critical".red());
    }
    if !Path::new(GVR_BIN).exists() {
        error!("SysAPI requires 'gvr' script to be available in base system!");
        panic!("{}: 'Cell-GoVeRnor' is {}!", "FATAL ERROR".blue(), "mission-critical".red());
    }
    if !Path::new(JAIL_BIN).exists() || !Path::new(JEXEC_BIN).exists() {
        error!("SysAPI requires both 'jail' and 'jexec' utilities to be available in base system!");
        panic!("{}: Both 'jail' and 'jexec' system utilities are {}!", "FATAL ERROR".blue(), "mission-critical".red());
    }
}



/// Produce list of dirs/files matching given glob pattern:
pub fn produce_list(glob_pattern: &str) -> Vec<String> {
    let mut list = vec!();
    for entry in glob(&glob_pattern).unwrap() {
        match entry {
            Ok(path) => {
                match path.file_name() {
                    Some(element) => {
                        element
                            .to_str()
                            .and_then(|elem| {
                                list.push(elem.to_string());
                                Some(elem.to_string())
                            });
                    },
                    None => (),
                }
            },
            Err(err) => {
                error!("Error: produce_list(): {}", err);
            },
        }
    }
    debug!("produce_list(): Elements: {:?}", list);
    list
}


/// Lists all cell attributes => /Shared/Prison/Sentry/CELLNAME/cell-attributes/*
pub fn list_attributes(cell_name: &str) -> Vec<String> {
    let glob_pattern = format!("{}/{}/cell-attributes/*", SENTRY_PATH, cell_name);
    debug!("list_attributes(): {}", glob_pattern);
    produce_list(&glob_pattern)
}


/// Lists all available cells based on files found under Sentry dir:
pub fn list_cells() -> Vec<String> {
    let glob_pattern = format!("{}/*", SENTRY_PATH);
    debug!("list_cells(): {}", glob_pattern);
    produce_list(&glob_pattern)
}


/// Lists all available proxies based on files found under Sentry dirs:
pub fn list_proxies() -> Vec<String> {
    let glob_pattern = format!("{}/**/cell-webconfs/*", SENTRY_PATH);
    debug!("list_proxies(): {}", glob_pattern);
    produce_list(&glob_pattern)
}


/// Returns empty JSON list:
pub fn empty_list_string() -> String {
    "[]".to_string()
}
