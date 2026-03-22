use regex::Regex;
use rocket::{delete, get, http::Status, post, response::status::Custom, serde::json::Json};
use std::path::Path;

// Load all internal modules:
use crate::apis::cell::*;
use crate::apis::status::*;
use crate::*;


lazy_static! {

    /// Cell name restriction - has to match following pattern:
    pub static ref CELL_NAME_PATTERN: Regex = {
        Regex::new(r"^[a-zA-Z0-9]*$").unwrap()
    };

}


/// Handle DELETEs for /cell/:cell
#[delete("/cell/<cell>")]
pub fn cell_delete_handler(cell: String) -> Json<String> {
    let cell_dir = format!("{}/{}", CELLS_PATH, cell);

    if Path::new(&cell_dir).exists() {
        match destroy_cell(&cell) {
            Ok(_) => Json("{\"status\": \"Ok\"}".to_string()),
            Err(_) => Json("{\"status\": \"Bad Request\"}".to_string()),
        }
    } else {
        Json("{\"status\": \"Not Modified\"}".to_string())
    }
}


/// Handle GET for /cells/list
#[get("/cells/list")]
pub fn cells_get_handler() -> Json<Cells> {
    Json(Cells::default())
}


/// handle GET for /cell/:cell
#[get("/cell/<cell>")]
pub fn cell_get_handler(cell: String) -> Json<Cell> {
    Json(Cell::state(&cell).unwrap_or_default())
}


/// handle GET for /status/:cell
#[get("/status/<cell>")]
pub fn cell_status_get_handler(cell: String) -> Json<CellProcesses> {
    Json(CellProcesses::of_cell(&cell).unwrap_or_default())
}


/// Handle POSTs for /cell/:cell
#[post("/cell/<cell>", data = "<body>")]
pub async fn cell_post_handler(cell: String, body: String) -> Custom<Json<String>> {
    let ssh_pubkey = body.trim().to_string();
    info!(
        "Got request to create new cell: {}, with ed25519-pubkey: {} (key-length: {})",
        cell,
        ssh_pubkey,
        ssh_pubkey.len()
    );

    // Validate all input data:
    let cell_dir = format!("{}/{}", CELLS_PATH, cell);
    if Path::new(&cell_dir).exists() {
        return Custom(
            Status::Conflict,
            Json("{\"status\": \"Conflict\"}".to_string()),
        );
    }

    if !CELL_NAME_PATTERN.is_match(&cell)
        || ssh_pubkey.len() < 68 // Ed25519 should be at least 68, but not longer than 70 bytes long
        || ssh_pubkey.len() > 70
        || cell.len() < 3        // Hostname can't be shorter than 3 chars and not longer than 27 chars
        || cell.len() > 27
    {
        return Custom(
            Status::NotAcceptable,
            Json("{\"status\": \"Not Acceptable\"}".to_string()),
        );
    }

    // Execute gvr create + gvr set
    match create_cell(&cell).and_then(|_| {
        info!("Cell created: {}.", cell);
        add_ssh_pubkey_to_cell(&cell, &ssh_pubkey)
    }) {
        // create a new response based on the result:
        Ok(_) => {
            info!("Cell started: {}", cell);
            Custom(
                Status::Created,
                Json("{\"status\": \"Created\"}".to_string()),
            )
        }
        Err(err) => {
            error!("Failed to create cell: {}. Last error: {}", cell, err);
            Custom(
                Status::ExpectationFailed,
                Json("{\"status\": \"Expectation Failed\"}".to_string()),
            )
        }
    }
}
