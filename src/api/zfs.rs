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
    name: Option<String>,
    dataset_path: Option<String>,
    timestamp: Option<String>,
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
    pub fn of(dataset_path: &String, snapshot_name: &String) -> Result<Snapshot, Error> {
        Command::new(ZFS_BIN)
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
    pub fn destroy(dataset_path: &String, snapshot_name: &String) -> Result<(), Error> {
        Command::new(ZFS_BIN)
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


    /// Destroy existing snapshot with given name:
    pub fn list(&self) -> Result<Vec<String>, Error> {
        Command::new(ZFS_BIN)
            .arg("list")
            .arg("-Hro")
            .arg("name")
            .arg("-t")
            .arg("snapshot")
            .output()
            .and_then(|after_snap| {
                if after_snap.status.success() {
                    let list = String::from_utf8_lossy(&after_snap.stdout)
                        .split("\n")
                        .collect();
                    debug!("List of ZFS snapshots: {}", list);
                    Ok(vec!(list))
                } else {
                    let error_msg = format!("ZFS snapshot listing failed!");
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
    pub fn to(dataset_path: &String, snapshot_name: &String) -> Result<Rollback, Error> {
        Command::new(ZFS_BIN)
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


