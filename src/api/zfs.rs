use gotham::helpers::http::response::create_response;
use std::process::Command;
use std::io::{Error, ErrorKind};
use gotham::state::State;
use gotham::handler::IntoResponse;
use hyper::{StatusCode, Body, Response};
use serde_json;
use chrono::Local;
use mime::*;


use api::*;


#[derive(Debug, Serialize, Deserialize)]
pub struct Rollback {
    name: Option<String>,
    dataset_path: Option<String>,
    timestamp: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Snapshot {
    pub name: Option<String>,
    pub cell_name: Option<String>,
    pub dataset_path: Option<String>,

    pub timestamp: Option<String>,
}


/// Serialize to JSON on .to_string()
impl ToString for Snapshot {
    fn to_string(&self) -> String {
        serde_json::to_string(&self)
            .unwrap_or(String::from("{\"status\": \"SerializationFailure\"}"))
    }
}


/// Serialize to JSON on .to_string()
impl ToString for Rollback {
    fn to_string(&self) -> String {
        serde_json::to_string(&self)
            .unwrap_or(String::from("{\"status\": \"SerializationFailure\"}"))
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


// pub enum Datasets {
//     Shared(String),
//     User(String),
//     Software(String),
//     Services(String),
// }


impl Snapshot {


    /// Create snapshot of dataset with given name:
    pub fn new(cell_name: &String, dataset_path: &String, snapshot_name: &String) -> Result<Snapshot, Error> {
        // ex: jexec -U worker centra24 zfs snapshot zroot/User/centra24@dupa
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
                          String::from_utf8_lossy(&after_snap.stdout), String::from_utf8_lossy(&after_snap.stderr));
                    Ok(
                       Snapshot {
                            name: Some(snapshot_name.to_owned()),
                            cell_name: Some(cell_name.to_owned()),
                            dataset_path: Some(dataset_path.to_owned()),
                            timestamp: Some(Local::now().format("%y-%m-%d_%H%M%S-%s").to_string()),
                        }
                    )
                } else {
                    let error_msg = format!("Unable to create snapshot: {}@{}", dataset_path, snapshot_name);
                    error!("{}", error_msg);
                    Err(
                        Error::new(ErrorKind::Other, error_msg)
                    )
                }
            })
    }


    /// Destroy existing snapshot with given name:
    pub fn destroy(cell_name: &String, dataset_path: &String, snapshot_name: &String) -> Result<(), Error> {
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
                          String::from_utf8_lossy(&after_snap.stdout), String::from_utf8_lossy(&after_snap.stderr));
                    Ok(())
                } else {
                    let error_msg = format!("Unable to destroy snapshot: {}@{}", dataset_path, snapshot_name);
                    error!("{}", error_msg);
                    Err(
                        Error::new(ErrorKind::Other, error_msg)
                    )
                }
            })
    }


    /// List all snapshots of a cell:
    pub fn list(cell_name: &String) -> Result<String, Error> {
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
                    let strng = String::from_utf8_lossy(&after_snap.stdout);
                    debug!("List of ZFS snapshots: {}", strng);
                    Ok(strng.to_string())
                } else {
                    let error_msg = format!("ZFS snapshot listing failed!");
                    error!("{}", error_msg);
                    Err(
                        Error::new(ErrorKind::Other, error_msg)
                    )
                }
            })
    }


    /// Checj snapshot state under a cell:
    pub fn state(cell_name: &String, snapshot_name: &String) -> Result<String, Error> {
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
            .arg(snapshot_name)
            .output()
            .and_then(|after_snap| {
                if after_snap.status.success() {
                    let strng = String::from_utf8_lossy(&after_snap.stdout);
                    debug!("ZFS snapshot matching pattern: {} is present. Output: {}", snapshot_name, strng);
                    Ok(strng.to_string())
                } else {
                    let error_msg = format!("No such snapshot: {}!", snapshot_name);
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
    pub fn new(cell_name: &String, dataset_path: &String, snapshot_name: &String) -> Result<Rollback, Error> {
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
                          String::from_utf8_lossy(&after_rollback.stdout), String::from_utf8_lossy(&after_rollback.stderr));
                    Ok(
                       Rollback {
                           name: Some(snapshot_name.to_owned()),
                           dataset_path: Some(dataset_path.to_owned()),
                           timestamp: Some(Local::now().format("%y-%m-%d_%H%M%S").to_string()),
                       }
                    )
                } else {
                    let error_msg = format!("Unable to create snapshot: {}@{}", dataset_path, snapshot_name);
                    error!("{}", error_msg);
                    Err(
                        Error::new(ErrorKind::Other, error_msg)
                    )
                }
            })

    }



}


