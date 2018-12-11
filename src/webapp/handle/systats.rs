use gotham::state::State;


use self::systat::*;


/// handle GET for /version
pub fn api_version_get_handler(state: State) -> (State, String) {
    let api_version = env!("CARGO_PKG_VERSION");
    let formatted_version = format!("{{\"status\": \"OK\", \"version\": \"{}\"}}", api_version);
    (state, formatted_version)
}


/// handle GET for /systat
pub fn api_systat_get_handler(state: State) -> (State, Systat) {
    (state, Systat::default())
}
