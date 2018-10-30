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



// Load all internal modules:
use api::*;


use regex::Regex;
lazy_static! {
    pub static ref CELL_DOMAIN_PATTERN: Regex = {
        Regex::new(r"local-zone: (?:([a-zA-Z0-9.]+)). ").unwrap()
    };
}


pub type List = Vec<String>;


#[derive(Debug, Serialize, Deserialize)]
pub struct Cell {
    pub name: Option<String>,
    pub ipv4: Option<String>,
    pub netid: Option<String>,
    pub domain: Option<String>,
    pub keys: Option<List>,
    pub attributes: Option<List>,
    pub zones: Option<List>,
    pub status: CellState,
}


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
           keys: None,
           attributes: None,
           zones: None,
           netid: None,
           status: CellState::NotFound,
        }
    }
}


impl ToString for Cell {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}


impl IntoResponse for Cell {
    fn into_response(self, state: &State) -> Response<Body> {
        // serialize only if name is set - so Cell is initialized/ exists
        match self.name {
            Some(_) => {
                create_response(
                    state,
                    StatusCode::OK,
                    mime::APPLICATION_JSON,
                    serde_json::to_string(&self).expect("Cell object should be serializable!"),
                )
            },
            None => {
                create_response(
                    state,
                    StatusCode::NOT_FOUND,
                    mime::APPLICATION_JSON,
                    Body::from("{\"status\": \"NotFound\"}"),
                )
            }
        }
    }
}


impl Cell {


    pub fn new(name: &String) -> Cell {
        Cell {
            name: name.to_string(),
            ipv4: "127.0.0.1".to_string(),
            domain: "some.local".to_string(),
            action: Actions::Create,
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

