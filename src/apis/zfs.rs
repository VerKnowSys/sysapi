use gotham::helpers::http::response::create_response;
use std::process::Command;
use std::io::{Error, ErrorKind};
use gotham::state::State;
use gotham::handler::IntoResponse;
use hyper::{StatusCode, Body, Response};
use serde_json;
use chrono::Local;
use mime::*;
use colored::Colorize;

use crate::*;
use crate::apis::cell::*;
use crate::processors::datasets::CUT_LAST_COMMA;


/// ZFS Rollback wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct Rollback {

    /// Rollback to snapshot of cell: "cell_name":
    pub cell_name: Option<String>,

    /// Rollback to snapshot with name: "@name":
    pub name: Option<String>,

    /// Full ZFS dataset path to perform rollback on:
    pub dataset_path: Option<String>,

    /// Rollback timestamp metadata:
    pub timestamp: Option<String>,
}


/// ZFS Snapshot wrapper
#[derive(Debug, Serialize, Deserialize)]
pub struct Snapshot {

    /// Snapshot of one of datasets of cell: "cell_name":
    pub cell_name: Option<String>,

    /// Snapshot name: "@name":
    pub name: Option<String>,

    /// Full ZFS dataset path to perform snapshot of:
    pub dataset_path: Option<String>,

    /// Snapshot timestamp metadata:
    pub timestamp: Option<String>,
}


impl Default for Snapshot {
    fn default() -> Snapshot {
        Snapshot {
           cell_name: None,
           name: None,
           dataset_path: None,
           timestamp: None,
        }
    }
}


/// List of Snapshots:
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Snapshots {
    /// List of all datasets of given cell
    pub list: Option<List>,
}


/// List of ZFS Datasets:
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Datasets {
    /// List of all datasets of given cell
    pub list: Option<List>,
}


/// Serialize to JSON on .to_string()
impl ToString for Snapshot {
    fn to_string(&self) -> String {
        serde_json::to_string(&self)
            .unwrap_or_else(|_| String::from("{\"status\": \"SerializationFailure\"}"))
    }
}


/// Serialize to JSON on .to_string()
impl ToString for Snapshots {
    fn to_string(&self) -> String {
        serde_json::to_string(&self)
            .unwrap_or_else(|_| String::from("{\"status\": \"SerializationFailure\"}"))
    }
}


/// Serialize to JSON on .to_string()
impl ToString for Rollback {
    fn to_string(&self) -> String {
        serde_json::to_string(&self)
            .unwrap_or_else(|_| String::from("{\"status\": \"SerializationFailure\"}"))
    }
}


/// Implement response for GETs:
impl IntoResponse for Snapshot {
    fn into_response(self, state: &State) -> Response<Body> {
        // serialize only if name is set - so Snapshot is initialized/ exists
        match self.name {
            Some(_) =>
                create_response(
                    state,
                    StatusCode::OK,
                    APPLICATION_JSON,
                    serde_json::to_string(&self)
                        .unwrap_or_else(|_| String::from("{\"status\": \"SerializationFailure\"}")),
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


impl IntoResponse for Rollback {
    fn into_response(self, state: &State) -> Response<Body> {
        // serialize only if name is set - so Rollback is initialized/ exists
        match self.name {
            Some(_) =>
                create_response(
                    state,
                    StatusCode::OK,
                    APPLICATION_JSON,
                    serde_json::to_string(&self)
                        .unwrap_or_else(|_| String::from("{\"status\": \"SerializationFailure\"}")),
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


/// Implementes listing of ZFS datasets of given cell:
impl Datasets {


    /// List all datasets of a cell:
    pub fn list(cell_name: &str) -> Result<String, Error> {
        Command::new(JEXEC_BIN)
            .arg("-U")
            .arg(CELL_USERNAME)
            .arg(cell_name)
            .arg(ZFS_BIN)
            .arg("list")
            .arg("-Hro")
            .arg("name")
            .arg("-t")
            .arg("filesystem")
            .output()
            .and_then(|after_snap| {
                if after_snap.status.success() {
                    let string_list: String = String::from_utf8_lossy(&after_snap.stdout)
                        .split('\n')
                        .filter(|elem| {
                            elem.contains(cell_name)
                        })
                        .map(|elem| {
                            format!("\"{}\", ", elem)
                        })
                        .collect();
                    let final_list = &CUT_LAST_COMMA.replace(&string_list, "");
                    debug!("List of ZFS snapshots of cell: {}: {}", cell_name.cyan(), final_list.cyan());
                    Ok(final_list.to_string())
                } else {
                    let error_msg = "ZFS snapshot listing failed!".to_string();
                    error!("{}", error_msg);
                    Err(
                        Error::new(ErrorKind::Other, error_msg)
                    )
                }
            })
    }


}


impl Snapshot {


    /// Create snapshot of dataset with given name:
    pub fn new(cell_name: &str, dataset_path: &str, snapshot_name: &str) -> Result<Snapshot, Error> {
        Ok(Snapshot {
            name: Some(snapshot_name.to_owned()),
            cell_name: Some(cell_name.to_owned()),
            dataset_path: Some(dataset_path.to_owned()),
            timestamp: Some(Local::now().format("%y-%m-%d_%H%M%S-%s").to_string()),
        })
   }


    /// Create snapshot of dataset with given name:
    pub fn create(cell_name: &str, dataset_path: &str, snapshot_name: &str) -> Result<Snapshot, Error> {
        Snapshot::new(&cell_name, &dataset_path, &snapshot_name)
            .and_then(|_new_cell| {
                Command::new(JEXEC_BIN)
                    .arg("-U")
                    .arg(CELL_USERNAME)
                    .arg(cell_name)
                    .arg(ZFS_BIN)
                    .arg("snapshot")
                    .arg(format!("{}@{}", dataset_path, snapshot_name))
                    .output()
                    .and_then(|after_snap| {
                        if after_snap.status.success() {
                            debug!("ZFS snapshot created:\n{}{}",
                                  String::from_utf8_lossy(&after_snap.stdout).blue(), String::from_utf8_lossy(&after_snap.stderr).white());
                            Ok(
                               Snapshot {
                                    name: Some(snapshot_name.to_owned()),
                                    cell_name: Some(cell_name.to_owned()),
                                    dataset_path: Some(dataset_path.to_owned()),
                                    timestamp: Some(Local::now().format("%y-%m-%d_%H%M%S-%s").to_string()),
                                }
                            )
                        } else {
                            let error_msg = format!("Unable to create snapshot: {}@{}", dataset_path.cyan(), snapshot_name.cyan());
                            error!("{}", error_msg);
                            Err(
                                Error::new(ErrorKind::Other, error_msg)
                            )
                        }
                    })
            })
    }


    /// Destroy existing snapshot with given name:
    pub fn destroy(cell_name: &str, dataset_path: &str, snapshot_name: &str) -> Result<(), Error> {
        Command::new(JEXEC_BIN)
            .arg("-U")
            .arg(CELL_USERNAME)
            .arg(cell_name)
            .arg(ZFS_BIN)
            .arg("destroy")
            .arg(format!("{}@{}", dataset_path, snapshot_name))
            .output()
            .and_then(|after_snap| {
                if after_snap.status.success() {
                    debug!("ZFS snapshot destroyed:\n{}{}",
                          String::from_utf8_lossy(&after_snap.stdout).blue(), String::from_utf8_lossy(&after_snap.stderr).white());
                    Ok(())
                } else {
                    let error_msg = format!("Unable to destroy snapshot: {}@{}", dataset_path.cyan(), snapshot_name.cyan());
                    error!("{}", error_msg);
                    Err(
                        Error::new(ErrorKind::Other, error_msg)
                    )
                }
            })
    }


    /// List all snapshots of a cell:
    pub fn list(cell_name: &str) -> Result<String, Error> {
        Command::new(JEXEC_BIN)
            .arg("-U")
            .arg(CELL_USERNAME)
            .arg(cell_name)
            .arg(ZFS_BIN)
            .arg("list")
            .arg("-Hro")
            .arg("name")
            .arg("-t")
            .arg("snapshot")
            .output()
            .and_then(|after_snap| {
                if after_snap.status.success() {
                    let string_list: String = String::from_utf8_lossy(&after_snap.stdout)
                        .split('\n')
                        .filter(|elem| {
                            elem.contains(cell_name)
                        })
                        .map(|elem| {
                            format!("\"{}\", ", elem)
                        })
                        .collect();
                    let final_list = &CUT_LAST_COMMA.replace(&string_list, "");
                    debug!("List of ZFS snapshots of cell: {}: [{}]", &cell_name.cyan(), &final_list.cyan());
                    Ok(final_list.to_string())
                } else {
                    let error_msg = "ZFS snapshot listing failed!".to_string();
                    error!("{}", error_msg);
                    Err(
                        Error::new(ErrorKind::Other, error_msg)
                    )
                }
            })
    }


    /// Check snapshot state under a cell:
    pub fn state(cell_name: &str, snapshot_name: &str) -> Result<String, Error> {
        Command::new(JEXEC_BIN)
            .arg("-U")
            .arg(CELL_USERNAME)
            .arg(cell_name)
            .arg(ZFS_BIN)
            .arg("list")
            .arg("-Hro")
            .arg("name")
            .arg("-t")
            .arg("snapshot")
            .output()
            .and_then(|after_snap| {
                if after_snap.status.success() {
                    let stdout = String::from_utf8_lossy(&after_snap.stdout);
                    let pre_line: String = stdout
                        .split('\n')
                        .filter(|elem| {
                            elem.contains(&format!("@{}", snapshot_name))
                        })
                        .map(|elem| {
                            format!("\"{}\", ", elem)
                        })
                        .collect();
                    let matching_line = &CUT_LAST_COMMA.replace(&pre_line, "");
                    match matching_line.as_ref() {
                        "" => {
                            let error_msg = format!("No such snapshot: {}!", snapshot_name.cyan());
                            error!("{}", error_msg);
                            Err(
                                Error::new(ErrorKind::Other, error_msg)
                            )
                        },
                        entry => {
                            debug!("ZFS snapshot matching pattern: '{}' is present. Output matched to: '{}'", snapshot_name.cyan(), entry.cyan());
                            Ok(entry.to_string())
                        }
                    }
                } else {
                    let error_msg = "Failed to list any snapshot!".to_string();
                    error!("{}", error_msg);
                    Err(
                        Error::new(ErrorKind::Other, error_msg)
                    )
                }
            })
    }


}


impl Rollback {


    /// Rollback dataset to given snapshot name:
    pub fn new(cell_name: &str, dataset_path: &str, snapshot_name: &str) -> Result<Rollback, Error> {
        Command::new(JEXEC_BIN)
            .arg("-U")
            .arg(CELL_USERNAME)
            .arg(cell_name)
            .arg(ZFS_BIN)
            .arg("rollback")
            .arg("-Rf")
            .arg(format!("{}@{}", dataset_path, snapshot_name))
            .output()
            .and_then(|after_rollback| {
                if after_rollback.status.success() {
                    debug!("ZFS rollbacked! Output:\n{}{}",
                          String::from_utf8_lossy(&after_rollback.stdout).blue(), String::from_utf8_lossy(&after_rollback.stderr).white());
                    Ok(
                       Rollback {
                           name: Some(snapshot_name.to_owned()),
                           dataset_path: Some(dataset_path.to_owned()),
                           cell_name: Some(cell_name.to_owned()),
                           timestamp: Some(Local::now().format("%y-%m-%d_%H%M%S").to_string()),
                       }
                    )
                } else {
                    let error_msg = format!("Unable to rollback to: {}@{}", dataset_path.cyan(), snapshot_name.cyan());
                    error!("{}", error_msg);
                    Err(
                        Error::new(ErrorKind::Other, error_msg)
                    )
                }
            })
    }


}
