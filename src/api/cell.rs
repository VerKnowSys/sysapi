use std::path::Path;
use std::process::Command;
use std::io::{Error, ErrorKind};
use gotham::state::State;
use gotham::handler::IntoResponse;
use hyper::{StatusCode, Body, Response};
use serde_json;
use std::io::prelude::*;
use gotham::helpers::http::response::create_response;
use std::io::BufReader;
use std::fs::File;
use mime::*;


// Load all internal modules:
use api::*;
use utils::*;


use regex::Regex;
lazy_static! {
    /// Regex extractor match for Unbound 1.7+ local-zone definition:
    pub static ref CELL_DOMAIN_PATTERN: Regex = {
        Regex::new(r"local-zone: (?:([a-zA-Z0-9.]+)). ").unwrap()
    };
}


pub type List = Vec<String>;


#[derive(Debug, Serialize, Deserialize)]
pub struct Cell {

    /// Cell name:
    pub name: Option<String>,

    /// Cell IPv4:
    pub ipv4: Option<String>,

    /// Cell worker uid and network card id:
    pub netid: Option<String>,

    /// Cell default zone:
    pub domain: Option<String>,

    /// Cell creator ED25519 SSH public key:
    pub key: Option<String>,

    /// Cell attributes (mostly RCTL and ZFS settings override)
    pub attributes: Option<List>,

    /// Cell status:
    pub status: CellState,

}


#[derive(Debug, Serialize, Deserialize)]
pub struct Cells {
    /// List of all cells
    pub list: Vec<Cell>
}



/// State of the cell
#[derive(Debug, Serialize, Deserialize)]
pub enum CellState {
    Offline,
    Online,
    NotFound,
}


impl Default for Cell {
    fn default() -> Cell {
        Cell {
           name: None,
           ipv4: None,
           domain: None,
           key: None,
           attributes: None,
           netid: None,
           status: CellState::NotFound,
        }
    }
}


impl Default for Cells {
    fn default() -> Cells {
        Cells {
            list:
                list_cells()
                    .iter()
                    .flat_map(|cell| {
                        let state = Cell::state(&cell);
                        debug!("Cells STATE: {:?}", state);
                        state
                    })
                    .collect()
        }
    }
}


/// Serialize to JSON on .to_string()
impl ToString for Cell {
    fn to_string(&self) -> String {
        serde_json::to_string(&self)
            .unwrap_or(String::from("{\"status\": \"SerializationFailure\"}"))
    }
}


impl ToString for Cells {
    fn to_string(&self) -> String {
        serde_json::to_string(&self)
            .unwrap_or(String::from("{\"status\": \"SerializationFailure\"}"))
    }
}


/// Implement response for GETs:
impl IntoResponse for Cell {
    fn into_response(self, state: &State) -> Response<Body> {
        // serialize only if name is set - so Cell is initialized/ exists
        match self.name {
            Some(_) =>
                create_response(
                    state,
                    StatusCode::OK,
                    APPLICATION_JSON,
                    serde_json::to_string(&self)
                        .unwrap_or(String::from("{\"status\": \"SerializationFailure\"}")),
                ),
            None =>
                create_response(
                    state,
                    StatusCode::NOT_FOUND,
                    APPLICATION_JSON,
                    Body::from("{\"status\": \"NotFound\"}"),
                )
        }
    }
}


/// Implement response for GETs:
impl IntoResponse for Cells {
    fn into_response(self, state: &State) -> Response<Body> {
        create_response(
            state,
            StatusCode::OK,
            APPLICATION_JSON,
            serde_json::to_string(&self)
                .unwrap_or(String::from("{\"status\": \"SerializationFailure\"}")),
        )
    }
}


impl Cell {


    /// New cell - no values are set:
    pub fn new() -> Cell {
        Cell::default()
    }


    /// Load cell state from system files:
    pub fn state(name: &String) -> Option<Cell> {
        // TODO: attributes => /Shared/Prison/Sentry/CELLNAME/cell-attributes/*

        let sentry_dir = format!("{}/{}", SENTRY_PATH, name);
        let key_file = format!("{}/{}", sentry_dir, "cell-attributes/key");
        let status_file = format!("{}/{}", sentry_dir, "cell.running");
        let netid_file = format!("{}/{}", sentry_dir, "cell.vlan.number");
        let ipv4_file = format!("{}/{}", sentry_dir, "cell.ip.addresses");
        let domain_file = format!("{}/{}", sentry_dir, "cell-domains/local.conf");
        debug!("SENTRY DIR: {}", sentry_dir);
        if Path::new(&sentry_dir).exists() {
            // key => /Shared/Prison/Sentry/CELLNAME/cell-attributes/key
            let key = File::open(&key_file)
                .and_then(|file| {
                    let mut line = String::new();
                    BufReader::new(file)
                        .read_line(&mut line)
                        .and_then(|_| {
                            // trim newlines and other whitespaces:
                            Ok(str::trim(&line).to_string())
                        })
                })
                .map_err(|err| {
                    error!("Couldn't read default key from file: {}. Fallback to no key.", key_file);
                    err
                })
                .unwrap_or("".to_string());

            // ip => /Shared/Prison/Sentry/CELLNAME/cell.ip.addresses
            let ipv4 = File::open(&ipv4_file)
                .and_then(|file| {
                    let mut line = String::new();
                    BufReader::new(file)
                        .read_line(&mut line)
                        .and_then(|_| {
                            // trim newlines and other whitespaces:
                            Ok(str::trim(&line).to_string())
                        })
                })
                .map_err(|err| {
                    error!("Couldn't read cell file: {}. Fallback to 127.1", ipv4_file);
                    err
                })
                .unwrap_or("127.0.0.1".to_string());

            // netid => /Shared/Prison/Sentry/CELLNAME/cell.vlan.number
            let netid = File::open(&netid_file)
                .and_then(|file| {
                    let mut line = String::new();
                    BufReader::new(file)
                        .read_line(&mut line)
                        .and_then(|_| {
                            // trim newlines and other whitespaces:
                            Ok(str::trim(&line).to_string())
                        })
                })
                .map_err(|err| {
                    error!("Couldn't read cell netid file: {}. Fallback to 0", netid_file);
                    err
                })
                .unwrap_or("0".to_string());

            // domain => /Shared/Prison/Sentry/CELLNAME/cell-domains/local.conf
            let domain = File::open(&domain_file)
                .and_then(|file| {
                    let mut line = String::new();
                    BufReader::new(file)
                        .read_to_string(&mut line)
                        .and_then(|_| {
                            let trim_line = line.replace("\n", " ").replace("\"", "");
                            let cap = &CELL_DOMAIN_PATTERN
                                .captures(&trim_line)
                                .and_then(|cap| {
                                    match cap.get(1).map_or("", |m| m.as_str()) {
                                        "" => None,
                                        domain => Some(domain),
                                    }
                                });
                            debug!("Got domain: {:?}. Full domain definition file contents: {:?}", cap, trim_line);
                            match cap {
                                Some(domain) => Ok(domain.to_string()),
                                None => Err(Error::new(ErrorKind::Other, format!("Empty domain entry in file: {}", domain_file)))
                            }
                        })
                })
                .map_err(|err| {
                    error!("Couldn't read domain file: {}. Reason: {}. Fallback to localhost!", domain_file, err);
                    err
                })
                .unwrap_or("localhost".to_string());

            // status => /Shared/Prison/Sentry/CELLNAME/cell.status
            let status = File::open(&status_file)
                .and_then(|file| {
                    let mut line = String::new();
                    BufReader::new(file)
                        .read_line(&mut line)
                        .and_then(|_| {
                            // trim newlines and other whitespaces:
                            Ok(CellState::Online)
                        })
                })
                .unwrap_or(CellState::Offline);

            let cell_result = Cell {
                name: Some(name.to_string()),
                key: Some(key),
                ipv4: Some(ipv4),
                domain: Some(domain),
                netid: Some(netid),
                status: status,

                .. Cell::default()
            };
            debug!("Get cell: {:?}", cell_result);
            Some(cell_result)
        } else {
            debug!("Cells list is empty!");
            None
        }
    }


}


pub fn add_ssh_pubkey_to_cell(name: &String, ssh_pubkey: &String) -> Result<(), Error> {
    Command::new(GVR_BIN)
        .arg("set")
        .arg(name)
        .arg(format!("key='{}'", ssh_pubkey))
        .output()
        .and_then(|add_ssh_pubkey| {
            if add_ssh_pubkey.status.success() {
                info!("add_ssh_pubkey_to_cell():\n{}", String::from_utf8_lossy(&add_ssh_pubkey.stdout));
                Ok(())
            } else {
                let error_msg = format!("Something went wrong and key: '{}' couldn't be set for cell: {}. Please contact administator or file a bug!", ssh_pubkey, name);
                error!("{}", error_msg);
                Err(Error::new(ErrorKind::Other, error_msg))
            }
        })
}


pub fn create_cell(name: &String) -> Result<(), Error> {
    Command::new(GVR_BIN)
        .arg("create")
        .arg(name)
        .output()
        .and_then(|gvr_handle| {
            info!("create_cell():\n{}{}",
                 String::from_utf8_lossy(&gvr_handle.stdout),
                 String::from_utf8_lossy(&gvr_handle.stderr));
            if gvr_handle.status.success() {
                Ok(())
            } else {
                Err(Error::new(ErrorKind::Other, format!("Failed to create_cell(): {}", name)))
            }
        })
}


pub fn destroy_cell(name: &String) -> Result<(), Error> {
    Command::new(GVR_BIN)
        .arg("destroy")
        .arg(name)
        .arg("I_KNOW_EXACTLY_WHAT_I_AM_DOING") // NOTE: special GVR_BIN argument - "non interactive destroy"
        .output()
        .and_then(|gvr_handle| {
            if gvr_handle.status.success() {
                info!("destroy_cell():\n{}{}",
                       String::from_utf8_lossy(&gvr_handle.stdout),
                       String::from_utf8_lossy(&gvr_handle.stderr));
                Command::new(JAIL_BIN)
                    .arg("-r") // NOTE: Sometimes jail services are locking "some" resources for a very long time,
                    .arg(name) //       and will remain "started" until the-process-lock is released..
                    .output()  //       Let's make sure there's no running jail with our name after destroy command:
                    .and_then(|jail_handle| {
                        if jail_handle.status.success() {
                            warn!("Dangling cell stopped: {}!", name);
                        }
                        Ok(())
                    })
                    // .map_err(|err| {
                    //     debug!("No dangling cell found. Looks clean.");
                    //     err
                    // })
            } else {
                Err(Error::new(ErrorKind::Other, format!("Couldn't destroy_cell(): {}", name)))
            }
        })
        .map_err(|err| {
            let error_msg = format!("Failure: {}", err);
            error!("ERROR: {}", error_msg);
            debug!("DEBUG(err): {:?}", err);
            err
        })
}

