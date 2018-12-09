

/// A single cell process status:
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CellProcess {

    /// Process-IDentifier:
    pid: Option<usize>,

    /// Parent Process-IDentifier:
    ppid: Option<usize>,

    /// Process-name:
    name: Option<String>,

    /// Process-command:
    cmd: Option<String>,

    /// Process-RSS (Ressident Set Size - ressident memory allocated by the process):
    rss: Option<usize>,

    /// Process-MRSS (Max Ressident Set Size - maximum amount of virtual memory process can allocate)
    mrss: Option<usize>,

    /// Process-run time:
    runtime: Option<usize>,

    /// Blocks written by Cell:
    blk_in: Option<usize>,

    /// Blocks read by Cell:
    blk_out: Option<usize>,

    /// Process threads count:
    nthr: Option<usize>,

    /// Process priority:
    pri_level: Option<usize>,

    /// Process stats including bound UDP or TCP ports/addresses and other info:
    stat_info: Option<String>,

}


/// Status of all processes running in a cell:
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CellProcesses {

    /// List of all processes
    pub list: Vec<CellProcess>

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
        serde_json::to_string(&self.list)
            .unwrap_or(String::from("{\"status\": \"SerializationFailure: CellProcesses\"}"))
    }
}
