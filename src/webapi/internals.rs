use gotham::state::State;


use api::systat::*;
use api::status::*;


/// handle GET for /version
pub fn api_version_get_handler(state: State) -> (State, String) {
    let api_version = env!("CARGO_PKG_VERSION");
    let formatted_version = format!("{{\"status\": \"OK\", \"version\": \"{}\"}}", api_version);
    (state, formatted_version)
}


/// handle GET for /systat
pub fn api_systat_get_handler(state: State) -> (State, Systat) {
    CellProcesses::of_uid(0)
        .and_then(|ps_full| {
            warn!("PS USAGE JSON: '{}'", ps_full.to_string());
            Ok(ps_full)
        })
        .unwrap_or_default();

    (state, Systat::default())
}
