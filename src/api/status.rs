use libc::*;

use utils::*;


/// Default CellProcesses implementation:
impl CellProcesses {


    /// Status of all ressident processes running as UID:
    pub fn of_uid(an_uid: uid_t) -> Result<Self, serde_json::Error> {
        // Deserialize JSON to CellProcesses structure:
        let procs_json = processes_of_uid(an_uid);
        match serde_json::from_str(&procs_json) {
            Ok(all_processes) => {
                Ok(
                    CellProcesses {
                        list: all_processes
                    }
                )
            },

            Err(err) => {
                error!("CellProcesses::of_uid({}) has failed! Error: {:?}", an_uid, err);
                Err(err)
            }
        }
    }


    /// Status of all ressident processes of given cell:
    pub fn of_cell(a_name: &String) -> Result<Self, serde_json::Error> {
        let sentry_dir = format!("{}/{}", SENTRY_PATH, a_name);
        let netid_file = format!("{}/{}", sentry_dir, "cell.vlan.number");
        let cell_net_id_and_uid = File::open(&netid_file)
            .and_then(|file| {
                let mut line = String::new();
                BufReader::new(file)
                    .read_line(&mut line)
                    .and_then(|_| {
                        // trim newlines and other whitespaces:
                        Ok(str::trim(&line).to_string())
                    })
            })
            .unwrap_or("0".to_string());
        let cell_uid = cell_net_id_and_uid
            .parse::<usize>()
            .unwrap_or(0);
        debug!("CellProcesses::of_cell(uid: {})", cell_uid);
        CellProcesses::of_uid(cell_uid)
           .and_then(|ps_full| {
               warn!("PS USAGE of cell: {} JSON: '{}'", &a_name, ps_full.to_string());
               Ok(ps_full)
           })
    }


}


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

    /// Job status:
    pub status: Option<String>,

}


/// An empty CellProcesses as default:
impl Default for CellProcesses {
    fn default() -> CellProcesses {
        CellProcesses {
            list: vec!()
        }
    }
}


/// Serialize to JSON on .to_string()
impl ToString for CellProcess {
    fn to_string(&self) -> String {
        serde_json::to_string(&self)
            .unwrap_or(String::from("{\"status\": \"SerializationFailure: CellProcess\"}"))
    }
}


/// Serialize to JSON on .to_string()
impl ToString for CellProcesses {
    fn to_string(&self) -> String {
        serde_json::to_string(&self)
            .unwrap_or(String::from("{\"status\": \"SerializationFailure: CellProcesses\"}"))
    }
}


/// Implement response for GETs:
impl IntoResponse for CellProcesses {
    fn into_response(self, state: &State) -> Response<Body> {
        // serialize only if name is set - so CellProcesses is initialized/ exists
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
