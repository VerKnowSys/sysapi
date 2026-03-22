use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    fs::File,
    io::{BufReader, Error, prelude::*},
    path::Path,
    process::Command,
};

use crate::helpers::*;
use crate::*;


lazy_static! {
    /// Regex extractor match for Unbound 1.7+ local-zone definition:
    pub static ref CELL_DOMAIN_PATTERN: Regex = {
        Regex::new(r"local-zone: (?:([a-zA-Z0-9.]+)). ").unwrap()
    };
}


/// Shortcut List type:
pub type List = Vec<String>;


/// Cell structure:
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Cell {
    /// Cell name:
    pub name: Option<String>,

    /// Cell IPv4:
    pub ipv4: Option<String>,

    /// Cell worker uid and network card id:
    pub netid: Option<String>,

    /// Cell default zone:
    pub domain: Option<String>,

    /// Cell attributes (mostly RCTL and ZFS settings override)
    pub attributes: Option<List>,

    /// Cell status:
    pub status: CellState,
}


/// Cells (Cell List) structure for easy list management
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Cells {
    /// List of all cells
    pub list: Vec<Cell>,
}


/// State of the Cell
#[derive(Copy, Clone, Debug, Serialize, Deserialize, Default)]
pub enum CellState {
    /// Cell is Offline:
    Offline,

    /// Cell is Online:
    Online,

    /// Cell doesn't exist:
    #[default]
    NotFound,
}


impl Default for Cells {
    fn default() -> Cells {
        Cells {
            list: list_cells()
                .iter()
                .flat_map(|cell| {
                    let state = Cell::state(cell);
                    debug!("Cells STATE: {:?}", state);
                    state
                })
                .collect(),
        }
    }
}


/// Serialize to JSON on .to_string()
impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self)
                .unwrap_or_else(|_| String::from("{\"status\": \"SerializationFailure\"}"))
        )
    }
}


/// Serialize to JSON on .to_string()
impl std::fmt::Display for Cells {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self)
                .unwrap_or_else(|_| String::from("{\"status\": \"SerializationFailure\"}"))
        )
    }
}


impl Cell {
    /// New cell - no values are set:
    pub fn new() -> Cell {
        Cell::default()
    }


    /// Load cell state from system files:
    pub fn state(name: &str) -> Option<Cell> {
        let sentry_dir = format!("{}/{}", SENTRY_PATH, name);
        let attributes_dir = format!("{}/{}", sentry_dir, "cell-attributes");
        let status_file = format!("{}/{}", sentry_dir, DEFAULT_CELL_RUNSTATE_FILE);
        let netid_file = format!("{}/{}", sentry_dir, DEFAULT_CELL_NETID_FILE);
        let ipv4_file = format!("{}/{}", sentry_dir, DEFAULT_CELL_IP_FILE);
        let domain_file = format!("{}/{}", sentry_dir, "cell-domains/local.conf");
        debug!(
            "state() dirs: Sentry dir: {}, Attributes dir: {}",
            sentry_dir.cyan(),
            attributes_dir.cyan()
        );

        if Path::new(&sentry_dir).exists() {
            // ip => /Shared/Prison/Sentry/CELLNAME/cell.ip.addresses
            let ipv4 = File::open(&ipv4_file)
                .and_then(|file| {
                    let mut line = String::new();
                    BufReader::new(file)
                        .read_line(&mut line)
                        .map(|_| str::trim(&line).to_string())
                })
                .inspect_err(|_err| {
                    error!(
                        "Couldn't read cell file: {}. Fallback to 127.1",
                        ipv4_file.cyan()
                    );
                })
                .unwrap_or_else(|_| DEFAULT_IP_FALLBACK.to_string());

            // netid => /Shared/Prison/Sentry/CELLNAME/cell.vlan.number
            let netid = File::open(&netid_file)
                .and_then(|file| {
                    let mut line = String::new();
                    BufReader::new(file)
                        .read_line(&mut line)
                        .map(|_| str::trim(&line).to_string())
                })
                .inspect_err(|_err| {
                    error!(
                        "Couldn't read cell netid file: {}. Fallback to 0",
                        netid_file.cyan()
                    );
                })
                .unwrap_or_else(|_| "0".to_string());

            // domain => /Shared/Prison/Sentry/CELLNAME/cell-domains/local.conf
            let domain = File::open(&domain_file)
                .and_then(|file| {
                    let mut line = String::new();
                    BufReader::new(file)
                        .read_to_string(&mut line)
                        .and_then(|_| {
                            let trim_line = line.replace("\n", " ").replace("\"", "");
                            let cap =
                                &CELL_DOMAIN_PATTERN.captures(&trim_line).and_then(|cap| {
                                    match cap.get(1).map_or("", |m| m.as_str()) {
                                        "" => None,
                                        domain => Some(domain),
                                    }
                                });
                            debug!(
                                "Got domain: {}. Full domain definition file contents: {}",
                                cap.unwrap_or("").to_string().cyan(),
                                trim_line.to_string().cyan()
                            );
                            match cap {
                                Some(domain) => Ok(domain.to_string()),
                                None => {
                                    Err(Error::other(format!(
                                        "Empty domain entry in file: {}",
                                        domain_file.cyan()
                                    )))
                                }
                            }
                        })
                })
                .inspect_err(|err| {
                    error!(
                        "Couldn't read domain file: {}. Reason: {}. Fallback to localhost!",
                        domain_file.cyan(),
                        err.to_string().cyan()
                    );
                })
                .unwrap_or_else(|_| DEFAULT_HOSTNAME_FALLBACK.to_string());

            // status => /Shared/Prison/Sentry/CELLNAME/cell.status
            let status = File::open(&status_file)
                .and_then(|file| {
                    let mut line = String::new();
                    BufReader::new(file)
                        .read_line(&mut line)
                        .map(|_| CellState::Online)
                })
                .unwrap_or(CellState::Offline);

            // attributes => /Shared/Prison/Sentry/CELLNAME/cell-attributes/*
            let attributes = list_attributes(name)
                .iter()
                .map(|attribute| {
                    let attribute_value =
                        File::open(format!("{}/{}", attributes_dir, attribute))
                            .and_then(|file| {
                                let mut line = String::new();
                                BufReader::new(file)
                                    .read_line(&mut line)
                                    .map(|_| str::trim(&line).to_string())
                            })
                            .map_err(|err| {
                                error!("{}", err);
                                err
                            })
                            .unwrap_or_else(|_| String::from(""));
                    debug!(
                        "Cell: {}, attribute: {}, value: {}",
                        name.cyan(),
                        attribute.cyan(),
                        attribute_value.cyan()
                    );
                    format!("{}={}", attribute, attribute_value)
                })
                .collect();

            let cell_result = Cell {
                name: Some(name.to_string()),
                ipv4: Some(ipv4),
                domain: Some(domain),
                netid: Some(netid),
                attributes: Some(attributes),
                status,
            };
            debug!("Get cell: {}", cell_result.to_string().cyan());
            Some(cell_result)
        } else {
            debug!("Cells list is empty!");
            None
        }
    }
}


/// Add SSH pubkey to a cell
pub fn add_ssh_pubkey_to_cell(name: &str, ssh_pubkey: &str) -> Result<(), Error> {
    Command::new(GVR_BIN)
        .arg("set")
        .arg(name)
        .arg(format!("key='{}'", ssh_pubkey))
        .output()
        .and_then(|add_ssh_pubkey| {
            if add_ssh_pubkey.status.success() {
                debug!("add_ssh_pubkey_to_cell():\n{}", String::from_utf8_lossy(&add_ssh_pubkey.stdout).cyan());
                Ok(())
            } else {
                let errmsg = format!("Something went wrong and key: '{}' couldn't be set for cell: '{}'. Please contact administator or file a bug!",
                                     ssh_pubkey.cyan(), name.cyan());
                error!("{}", errmsg);
                Err(Error::other(errmsg))
            }
        })
}


/// Create cell
pub fn create_cell(name: &str) -> Result<(), Error> {
    Command::new(GVR_BIN)
        .arg("create")
        .arg(name)
        .output()
        .and_then(|gvr_handle| {
            debug!(
                "create_cell():\n{}{}",
                String::from_utf8_lossy(&gvr_handle.stdout).blue(),
                String::from_utf8_lossy(&gvr_handle.stderr).white()
            );
            if gvr_handle.status.success() {
                Ok(())
            } else {
                Err(Error::other(format!(
                    "Failed to create_cell(): {}",
                    name.cyan()
                )))
            }
        })
}


/// Destroy cell
pub fn destroy_cell(name: &str) -> Result<(), Error> {
    Command::new(GVR_BIN)
        .arg("destroy")
        .arg(name)
        .arg("I_KNOW_EXACTLY_WHAT_I_AM_DOING") // NOTE: special GVR_BIN argument - "non interactive destroy"
        .output()
        .and_then(|gvr_handle| {
            if gvr_handle.status.success() {
                debug!("destroy_cell():\n{}{}",
                       String::from_utf8_lossy(&gvr_handle.stdout).blue(),
                       String::from_utf8_lossy(&gvr_handle.stderr).white());
                Command::new(JAIL_BIN)
                    .arg("-r") // NOTE: Sometimes jail services are locking "some" resources for a very long time,
                    .arg(name) //       and will remain "started" until the-process-lock is released..
                    .output()  //       Let's make sure there's no running jail with our name after destroy command:
                    .map(|jail_handle| {
                        if jail_handle.status.success() {
                            warn!("Dangling cell stopped: {}!", name.cyan());
                        }

                    })
                    // .map_err(|err| {
                    //     debug!("No dangling cell found. Looks clean.");
                    //     err
                    // })
            } else {
                Err(Error::other(format!("Couldn't destroy_cell(): {}", name.cyan())))
            }
        })
}
