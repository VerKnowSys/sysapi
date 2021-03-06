use hyper::*;
use futures::{future, Future, Stream};
use gotham::helpers::http::response::create_response;
use gotham::state::{FromState, State};
use gotham::handler::{HandlerFuture, IntoHandlerError};
use mime::*;
use regex::Regex;


// Load all internal modules:
use crate::*;
use crate::apis::zfs::*;


lazy_static! {
    /// Regex extractor match for Unbound 1.7+ local-zone definition:
    pub static ref CUT_LAST_COMMA: Regex = {
        Regex::new(r", $").unwrap()
    };
}


/// Handle DELETE on snapshots:
pub fn zfs_snapshot_delete_handler(mut state: State) -> Box<HandlerFuture> {
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(valid_body) => {
                let uri = Uri::borrow_from(&state).to_string();
                let cell_and_snapshot_name = uri.replace(SNAPSHOT_RESOURCE, "");
                let snapshot_name: String = cell_and_snapshot_name.split('/').skip(1).take(1).collect();
                let cell_name: String = cell_and_snapshot_name.split('/').take(1).collect();
                let dataset_path = String::from_utf8(valid_body.to_vec()).unwrap_or_default(); // Read full ZFS dataset path from the body
                debug!("zfs_snapshot_get_handler(): About to destroy snapshot: {}@{} of cell: {}",
                       dataset_path, snapshot_name, cell_name);
                if dataset_path.len() < 10
                    || snapshot_name.len() < 2 {
                    let res = create_response(&state, StatusCode::NOT_ACCEPTABLE, APPLICATION_JSON,
                                              Body::from("{\"status\": \"Not Acceptable\"}"));
                    future::ok((state, res))
                } else {
                    let destroy = Snapshot::destroy(&cell_name, &dataset_path, &snapshot_name)
                        .and_then(|_| {
                            info!("Snapshot destroyed: {}@{}",
                                  &dataset_path, &snapshot_name);
                            Ok((&dataset_path, &snapshot_name))
                        })
                        .map_err(|err| {
                            error!("Unable to destroy snapshot: {}@{}. Error: {}",
                                              &dataset_path, &snapshot_name, err);
                            err
                        });
                    match destroy {
                        Ok(_) => {
                            let res = create_response(&state, StatusCode::OK, APPLICATION_JSON,
                                                      Body::from("{\"status\": \"Destroyed\"}"));
                            future::ok((state, res))
                        },
                        Err(_) => {
                            let res = create_response(&state, StatusCode::BAD_REQUEST, APPLICATION_JSON,
                                                      Body::from("{\"status\": \"Bad Request\"}"));
                            future::ok((state, res))
                        }
                    }
                }

            },
            Err(e) =>
                future::err(
                            (state, e.into_handler_error()))
        });

    Box::new(f)
}


/// handle GET for /datasets/list/:cell - list all datasets of cell
pub fn zfs_dataset_list_handler(state: State) -> (State, Response<Body>) {
    let uri = Uri::borrow_from(&state).to_string();
    let cell_and_snapshot_name = uri.replace(DATASETS_RESOURCE, "");
    let cell_name: String = cell_and_snapshot_name.split('/').skip(1).take(1).collect(); // first is "list", second "cell_name"

    let pre_list = Datasets::list(&cell_name)
        .and_then(|string_list| {
            debug!("zfs_dataset_list_handler(): Cell name: {}, string_list: [{}]", cell_name, string_list);
            Ok(string_list)
        })
        .map_err(|err| {
            error!("Datasets list. Error details: {}", err);
            err
        })
        .unwrap_or_else(|_| String::from(""));
    match pre_list.as_ref() {
        "" => {
            let res = create_response(&state, StatusCode::NOT_FOUND, APPLICATION_JSON,
                                      Body::from("{\"status\": \"No ZFS Datasets.\", \"list\": []}"));
            (state, res)
        },
        raw_list => {
            let res = create_response(&state, StatusCode::OK, APPLICATION_JSON,
                                      Body::from(format!("{{\"status\": \"OK\", \"list\": [{}]}}",
                                                         raw_list)));
            (state, res)
        }
    }
}


/// handle GET for /snapshot/list/:cell - list all snapshots of cell
pub fn zfs_snapshot_list_handler(state: State) -> (State, Response<Body>) {
    let uri = Uri::borrow_from(&state).to_string();
    let cell_and_snapshot_name = uri.replace(SNAPSHOT_RESOURCE, "");
    let cell_name: String = cell_and_snapshot_name.split('/').skip(1).take(1).collect(); // first is "list", second "cell_name"

    let pre_list = Snapshot::list(&cell_name)
        .and_then(|string_list| {
            debug!("zfs_snapshot_list_handler(): Cell name: {}, string_list: [{}]", cell_name, string_list);
            Ok(string_list)
        })
        .map_err(|err| {
            error!("Snapshot list error: {}", err);
            err
        })
        .unwrap_or_else(|_| String::from(""));
    match pre_list.as_ref() {
        "" => {
            let res = create_response(&state, StatusCode::NOT_FOUND, APPLICATION_JSON,
                                      Body::from("{\"status\": \"No Snapshots.\", \"list\": []}"));
            (state, res)
        },
        raw_list => {
            let res = create_response(&state, StatusCode::OK, APPLICATION_JSON,
                                      Body::from(format!("{{\"status\": \"OK\", \"list\": [{}]}}",
                                                         raw_list)));
            (state, res)
        }
    }
}


/// handle GET for /snapshot/:cell - get name of snapshot of a cell
pub fn zfs_snapshot_get_handler(state: State) -> (State, Snapshot) {
    let uri = Uri::borrow_from(&state).to_string();
    let cell_and_snapshot_name = uri.replace(SNAPSHOT_RESOURCE, "");
    let cell_name: String = cell_and_snapshot_name.split('/').take(1).collect();
    let snapshot_name: String = cell_and_snapshot_name.split('/').skip(1).take(1).collect();

    let list = Snapshot::state(&cell_name, &snapshot_name)
        .map_err(|err| {
            error!("Snapshot state check error: {}", err);
            err
        })
        .unwrap_or_else(|_| String::from(""));
    match list.as_ref() {
        "" => {
            debug!("Empty snapshot - Not found: @{}", snapshot_name);
            (state, Snapshot::default())
        },
        snapshot => {
            let dataset_path: &String = &snapshot.split('@').take(1).collect();
            let snapshot_obj = Snapshot::new(&cell_name,
                                             &dataset_path.replace("\\\"", "").replace("\"", ""),
                                             &snapshot_name).unwrap();
            debug!("zfs_snapshot_get_handler(): Dataset path: '{}'. Full snapshot: '{}'. Snapshot object: '{}'",
                   dataset_path, snapshot, snapshot_obj.to_string());
            (state, snapshot_obj)
        }
    }
}


/// Handle POSTs /snapshot/:cell/:snapshot + body with dataset_path
pub fn zfs_snapshot_post_handler(mut state: State) -> Box<HandlerFuture> {
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(valid_body) => {
                let uri = Uri::borrow_from(&state).to_string();
                let cell_and_snapshot_name = uri.replace(SNAPSHOT_RESOURCE, "");
                let cell_name: String = cell_and_snapshot_name.split('/').take(1).collect();
                let snapshot_name: String = cell_and_snapshot_name.split('/').skip(1).take(1).collect();

                let dataset_path = String::from_utf8(valid_body.to_vec()).unwrap_or_default(); // Read ZFS dataset_path
                info!("Got request to create new snapshot: {}@{} for cell: {}",
                      dataset_path, snapshot_name, cell_name);

                if cell_name.len() < 3 // cell name has to be 3-27 chars long
                    || cell_name.len() > 27
                    || snapshot_name.len() < 3 // @nme - minimal snapname
                    || snapshot_name.len() > 27
                    || dataset_path.len() < 9 // zroot/nme - minimal dataset path
                    || dataset_path.len() > 512
                    || dataset_path.contains('@') {
                    let res = create_response(&state, StatusCode::NOT_ACCEPTABLE, APPLICATION_JSON, Body::from("{\"status\": \"Not Acceptable\"}"));
                    future::ok((state, res))
                } else {
                    match Snapshot::create(&cell_name, &dataset_path, &snapshot_name) {
                        Ok(snapshot) => {
                            debug!("Snapshot created: {}", snapshot.to_string());
                            let res = create_response(&state, StatusCode::CREATED, APPLICATION_JSON, Body::from("{\"status\": \"Created\"}"));
                            future::ok((state, res))
                        },
                        Err(err) => {
                            error!("{}", err);
                            let res = create_response(&state, StatusCode::EXPECTATION_FAILED, APPLICATION_JSON, Body::from("{\"status\": \"Failed to create snapshot\"}"));
                            future::ok((state, res))
                        }
                    }
                }
            }
            Err(e) => future::err((state, e.into_handler_error()))
        });

    Box::new(f)
}


/// Handle POSTs /rollback/:cell/:snapshot + dataset_path in body
pub fn zfs_rollback_post_handler(mut state: State) -> Box<HandlerFuture> {
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(valid_body) => {
                let uri = Uri::borrow_from(&state).to_string();
                let cell_and_snapshot_name = uri.replace(ROLLBACK_RESOURCE, "");
                let cell_name: String = cell_and_snapshot_name.split('/').take(1).collect();
                let snapshot_name: String = cell_and_snapshot_name.split('/').skip(1).take(1).collect();
                let dataset_path = String::from_utf8(valid_body.to_vec()).unwrap_or_default(); // Read ZFS dataset_path
                info!("Got request for rollback to: {}@{} for cell: {}",
                      dataset_path, snapshot_name, cell_name);

                if cell_name.len() < 3 // cell name has to be 3-27 chars long
                    || cell_name.len() > 27
                    || snapshot_name.len() < 3 // @nme - minimal snapname
                    || snapshot_name.len() > 27
                    || dataset_path.len() < 9 // zroot/nme - minimal dataset path
                    || dataset_path.len() > 512
                    || dataset_path.contains('@') {
                    let res = create_response(&state, StatusCode::NOT_ACCEPTABLE, APPLICATION_JSON, Body::from("{\"status\": \"Not Acceptable\"}"));
                    future::ok((state, res))
                } else {
                    match Rollback::new(&cell_name, &dataset_path, &snapshot_name) {
                        Ok(snapshot) => {
                            debug!("Rollback committed to snapshot: {}", snapshot.to_string());
                            let res = create_response(&state, StatusCode::OK, APPLICATION_JSON, Body::from("{\"status\": \"Rollback completed.\"}"));
                            future::ok((state, res))
                        },
                        Err(err) => {
                            error!("{}", err);
                            let res = create_response(&state, StatusCode::EXPECTATION_FAILED, APPLICATION_JSON, Body::from("{\"status\": \"Failed rollback!\"}"));
                            future::ok((state, res))
                        }
                    }
                }
            }
            Err(e) => future::err((state, e.into_handler_error()))
        });

    Box::new(f)
}

