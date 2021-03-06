use gotham::state::State;
use std::io::{Error, ErrorKind};
use gotham::handler::IntoResponse;
use hyper::{StatusCode, Body, Response};
use serde_json;
use std::io::prelude::*;
use gotham::helpers::http::response::create_response;
use std::io::BufReader;
use std::fs::File;
use mime::*;
use libc::*;
use colored::Colorize;

use crate::{SENTRY_PATH, DEFAULT_CELL_NETID_FILE, soload::processes_of_uid};


/// List CellProcess type alias:
pub type ListCellProcesses = Vec<CellProcess>;


/// A single cell process status:
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CellProcess {

    /// Process-IDentifier:
    pub pid: Option<usize>,

    /// Parent Process-IDentifier:
    pub ppid: Option<usize>,

    /// Process-name:
    pub name: Option<String>,

    /// Process-command:
    pub cmd: Option<String>,

    /// Process-RSS (Ressident Set Size - ressident memory allocated by the process):
    pub rss: Option<usize>,

    /// Process-MRSS (Max Ressident Set Size - maximum amount of virtual memory process can allocate)
    pub mrss: Option<usize>,

    /// Process-run time:
    pub runtime: Option<usize>,

    /// Blocks written by Cell:
    pub blk_in: Option<usize>,

    /// Blocks read by Cell:
    pub blk_out: Option<usize>,

    /// Process threads count:
    pub nthr: Option<usize>,

    /// Process priority:
    pub pri_level: Option<usize>,

    /// Process stats including bound UDP or TCP ports/addresses and other info:
    pub stat_info: Option<String>,

}


/// Status of all processes running in a cell:
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CellProcesses {

    /// Cell processes list:
    pub list: Option<ListCellProcesses>,

}


/// Default CellProcesses implementation:
impl CellProcesses {


    /// Status of all ressident processes running as UID:
    pub fn of_uid(an_uid: uid_t) -> Result<Self, serde_json::Error> {
        let procs_json = processes_of_uid(an_uid); // Deserialize JSON to CellProcesses structure
        serde_json::from_str(&procs_json)
    }


    /// Status of all ressident processes of cell by a_name
    pub fn of_cell(a_name: &str) -> Result<Self, Error> {
        let sentry_dir = format!("{}/{}", SENTRY_PATH, a_name);
        let netid_file = format!("{}/{}", sentry_dir, DEFAULT_CELL_NETID_FILE);
        File::open(&netid_file)
            .and_then(|file| {
                let mut line = String::new();
                BufReader::new(file)
                    .read_line(&mut line)
                    .and_then(|_| Ok(str::trim(&line).to_string()))
            })
            .and_then(|uid_line| {
                uid_line
                    .parse::<u32>()
                    .map_err(|err| Error::new(ErrorKind::Other, err.to_string()))
            })
            .and_then(|cell_uid| {
                if cell_uid > 0 {
                    CellProcesses::of_uid(cell_uid)
                       .and_then(|ps_full| {
                           debug!("CellProcesses::of_cell(cell_uid: {}): {} JSON: '{}'",
                                  cell_uid.to_string().cyan(), a_name.cyan(), ps_full.to_string().cyan());
                           Ok(ps_full)
                       })
                       .map_err(|err| Error::new(ErrorKind::Other, err.to_string()))
                } else {
                    // NOTE: We can't return 0 for security reasons, so this is to explicitly return no data for "0":
                    warn!("CellProcesses::of_cell(cell_uid: 0). Using uid 0 (super-user) is disallowed for security reasons!");
                    Ok(CellProcesses::default())
                }
            })
    }
}


/// An empty CellProcesses as default:
impl Default for CellProcesses {
    fn default() -> CellProcesses {
        CellProcesses {
            list: None
        }
    }
}


/// Serialize to JSON on .to_string()
impl ToString for CellProcess {
    fn to_string(&self) -> String {
        serde_json::to_string(&self)
            .unwrap_or_else(|_| String::from("{\"status\": \"SerializationFailure: CellProcess\"}"))
    }
}


/// Serialize to JSON on .to_string()
impl ToString for CellProcesses {
    fn to_string(&self) -> String {
        serde_json::to_string(&self)
            .unwrap_or_else(|_| String::from("{\"status\": \"SerializationFailure: CellProcesses\"}"))
    }
}


/// Implement response for GETs:
impl IntoResponse for CellProcess {
    fn into_response(self, state: &State) -> Response<Body> {
        create_response(
            state,
            StatusCode::OK,
            APPLICATION_JSON,
            serde_json::to_string(&self)
                .unwrap_or_else(|_| String::from("{\"status\": \"SerializationFailure: CellProcess\"}")),
        )
    }
}


/// Implement response for GETs:
impl IntoResponse for CellProcesses {
    fn into_response(self, state: &State) -> Response<Body> {
        match self.list {
            Some(_) =>
                create_response(
                    state,
                    StatusCode::OK,
                    APPLICATION_JSON,
                    serde_json::to_string(&self)
                        .unwrap_or_else(|_| String::from("{\"status\": \"SerializationFailure: CellProcesses\"}")),
                ),
            None =>
                create_response(
                    state,
                    StatusCode::NOT_FOUND,
                    APPLICATION_JSON,
                    Body::from("{\"status\": \"CellProcesses: None\"}"),
                )
        }
    }
}
