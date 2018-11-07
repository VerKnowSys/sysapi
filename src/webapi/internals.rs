use gotham::state::State;


/// handle GET for /version
pub fn api_version_get_handler(state: State) -> (State, String) {
    let api_version = env!("CARGO_PKG_VERSION");
    let formatted_version = format!("{{\"status\": \"OK\", \"version\": \"{}\"}}", api_version);
    (state, formatted_version)
}

