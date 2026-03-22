use regex::Regex;
use rocket::{delete, get, http::Status, post, response::status::Custom, serde::json::Json};

// Load all internal modules:
use crate::apis::zfs::*;
use crate::*;


lazy_static! {
    /// Regex extractor match for Unbound 1.7+ local-zone definition:
    pub static ref CUT_LAST_COMMA: Regex = {
        Regex::new(r", $").unwrap()
    };
}


/// Handle DELETE on snapshots: /snapshot/<cell>/<snapshot>
#[delete("/snapshot/<cell>/<snapshot>", data = "<dataset_path>")]
pub fn zfs_snapshot_delete_handler(
    cell: String,
    snapshot: String,
    dataset_path: String,
) -> Custom<Json<String>> {
    debug!(
        "zfs_snapshot_delete_handler(): About to destroy snapshot: {}@{} of cell: {}",
        dataset_path, snapshot, cell
    );
    if dataset_path.len() < 10 || snapshot.len() < 2 {
        return Custom(
            Status::NotAcceptable,
            Json("{\"status\": \"Not Acceptable\"}".to_string()),
        );
    }

    match Snapshot::destroy(&cell, &dataset_path, &snapshot) {
        Ok(_) => {
            info!("Snapshot destroyed: {}@{}", &dataset_path, &snapshot);
            Custom(Status::Ok, Json("{\"status\": \"Destroyed\"}".to_string()))
        }
        Err(err) => {
            error!(
                "Unable to destroy snapshot: {}@{}. Error: {}",
                &dataset_path, &snapshot, err
            );
            Custom(
                Status::BadRequest,
                Json("{\"status\": \"Bad Request\"}".to_string()),
            )
        }
    }
}


/// handle GET for /datasets/list/<cell> - list all datasets of cell
#[get("/datasets/list/<cell>")]
pub fn zfs_dataset_list_handler(cell: String) -> Custom<Json<String>> {
    match Datasets::list(&cell) {
        Ok(raw_list) if !raw_list.is_empty() => {
            Custom(
                Status::Ok,
                Json(format!("{{\"status\": \"OK\", \"list\": [{}]}}", raw_list)),
            )
        }
        _ => {
            Custom(
                Status::NotFound,
                Json("{\"status\": \"No ZFS Datasets.\", \"list\": []}".to_string()),
            )
        }
    }
}


/// handle GET for /snapshot/list/<cell> - list all snapshots of cell
#[get("/snapshot/list/<cell>", rank = 1)]
pub fn zfs_snapshot_list_handler(cell: String) -> Custom<Json<String>> {
    match Snapshot::list(&cell) {
        Ok(raw_list) if !raw_list.is_empty() => {
            Custom(
                Status::Ok,
                Json(format!("{{\"status\": \"OK\", \"list\": [{}]}}", raw_list)),
            )
        }
        _ => {
            Custom(
                Status::NotFound,
                Json("{\"status\": \"No Snapshots.\", \"list\": []}".to_string()),
            )
        }
    }
}


/// handle GET for /snapshot/<cell>/<snapshot>
#[get("/snapshot/<cell>/<snapshot>", rank = 2)]
pub fn zfs_snapshot_get_handler(cell: String, snapshot: String) -> Json<Snapshot> {
    match Snapshot::state(&cell, &snapshot) {
        Ok(snapshot_entry) if !snapshot_entry.is_empty() => {
            let dataset_path = snapshot_entry
                .split('@')
                .next()
                .unwrap_or("")
                .replace("\\\"", "")
                .replace("\"", "");
            Json(Snapshot::new(&cell, &dataset_path, &snapshot).unwrap_or_default())
        }
        _ => Json(Snapshot::default()),
    }
}


/// Handle POSTs /snapshot/<cell>/<snapshot> + body with dataset_path
#[post("/snapshot/<cell>/<snapshot>", data = "<dataset_path>")]
pub fn zfs_snapshot_post_handler(
    cell: String,
    snapshot: String,
    dataset_path: String,
) -> Custom<Json<String>> {
    info!(
        "Got request to create new snapshot: {}@{} for cell: {}",
        dataset_path, snapshot, cell
    );

    if cell.len() < 3
        || cell.len() > 27
        || snapshot.len() < 3
        || snapshot.len() > 27
        || dataset_path.len() < 9
        || dataset_path.len() > 512
        || dataset_path.contains('@')
    {
        return Custom(
            Status::NotAcceptable,
            Json("{\"status\": \"Not Acceptable\"}".to_string()),
        );
    }

    match Snapshot::create(&cell, &dataset_path, &snapshot) {
        Ok(_) => {
            Custom(
                Status::Created,
                Json("{\"status\": \"Created\"}".to_string()),
            )
        }
        Err(err) => {
            error!("{}", err);
            Custom(
                Status::ExpectationFailed,
                Json("{\"status\": \"Failed to create snapshot\"}".to_string()),
            )
        }
    }
}


/// Handle POSTs /rollback/<cell>/<snapshot> + dataset_path in body
#[post("/rollback/<cell>/<snapshot>", data = "<dataset_path>")]
pub fn zfs_rollback_post_handler(
    cell: String,
    snapshot: String,
    dataset_path: String,
) -> Custom<Json<String>> {
    info!(
        "Got request for rollback to: {}@{} for cell: {}",
        dataset_path, snapshot, cell
    );

    if cell.len() < 3
        || cell.len() > 27
        || snapshot.len() < 3
        || snapshot.len() > 27
        || dataset_path.len() < 9
        || dataset_path.len() > 512
        || dataset_path.contains('@')
    {
        return Custom(
            Status::NotAcceptable,
            Json("{\"status\": \"Not Acceptable\"}".to_string()),
        );
    }

    match Rollback::new(&cell, &dataset_path, &snapshot) {
        Ok(_) => {
            Custom(
                Status::Ok,
                Json("{\"status\": \"Rollback completed.\"}".to_string()),
            )
        }
        Err(err) => {
            error!("{}", err);
            Custom(
                Status::ExpectationFailed,
                Json("{\"status\": \"Failed rollback!\"}".to_string()),
            )
        }
    }
}
