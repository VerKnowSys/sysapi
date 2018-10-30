// use hyper::{Body, headers::Headers, headers::HttpVersion, Method, Response, StatusCode, Uri};
use hyper::*;
use std::path::Path;
use futures::{future, Future, Stream};
use gotham::helpers::http::response::create_response;
use gotham::state::{FromState, State};
use gotham::handler::{HandlerFuture, IntoHandlerError};


// Load all internal modules:
use api::*;
use api::cell::*;


// Precompile CELL_NAME_PATTERN only once:
use regex::Regex;
lazy_static! {
    pub static ref CELL_NAME_PATTERN: Regex = {
        Regex::new(r"^[a-zA-Z0-9]*$").unwrap()
    };
}



/// Extract the main elements of the request except for the `Body`
// fn print_request_elements(state: &State) {
//     let method = Method::borrow_from(state);
//     let uri = Uri::borrow_from(state);
//     let http_version = HttpVersion::borrow_from(state);
//     let headers = Headers::borrow_from(state);
//     info!("Method: {:?}", method);
//     info!("URI: {:?}", uri);
//     info!("HTTP Version: {:?}", http_version);
//     info!("Headers: {:?}", headers);
// }


/// Handle DELETEs
pub fn delete_handler(state: State) -> (State, Response<Body>) {
    let uri = Uri::borrow_from(&state).to_string();
    let name = uri.replace(CELL_RESOURCE, "");
    let cell_dir = format!("{}/{}", CELLS_PATH, name);

    if Path::new(&cell_dir).exists() {
        match cell::destroy_cell(&name) {
            Ok(_) => {
                let res = create_response(&state, StatusCode::OK, mime::APPLICATION_JSON, Body::from("{\"status\": \"Ok\"}"));
                (state, res)
            },
            Err(_) => {
                let res = create_response(&state, StatusCode::BAD_REQUEST, mime::APPLICATION_JSON, Body::from("{\"status\": \"Bad Request\"}"));
                (state, res)
            }
        }
    } else {
        let res = create_response(&state, StatusCode::NOT_MODIFIED, mime::APPLICATION_JSON, Body::from("{\"status\": \"Not Modified\"}"));
        (state, res)
    }
}


/// Handle GETs
pub fn get_handler(state: State) -> (State, Cell) {
    let uri = Uri::borrow_from(&state).to_string();
    let name = uri.replace(CELL_RESOURCE, "");
    (state, Cell::state(&name).unwrap_or(Cell::new())) // XXX: TODO: it should load current service state and return json
}


/// Handle POSTs
pub fn post_handler(mut state: State) -> Box<HandlerFuture> {
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(valid_body) => {
                let uri = Uri::borrow_from(&state).to_string();
                let name = uri.replace(CELL_RESOURCE, "");
                let ssh_pubkey = String::from_utf8(valid_body.to_vec()).unwrap_or(String::new()); // Read SSH pubkey from request body:
                info!("Got request to create new cell: {}, with ed25519-pubkey: {} (key-length: {})",
                      name, ssh_pubkey, ssh_pubkey.len());

                // Validate all input data:
                let cell_dir = format!("{}/{}", CELLS_PATH, name);
                if Path::new(&cell_dir).exists() {
                    let res = create_response(&state, StatusCode::CONFLICT, mime::APPLICATION_JSON, Body::from("{\"status\": \"Conflict\"}"));
                    return future::ok((state, res))
                }

                if !CELL_NAME_PATTERN.is_match(&name)
                    || ssh_pubkey.len() < 68 // Ed25519 should be at least 68, but not longer than 70 bytes long
                    || ssh_pubkey.len() > 70
                    || name.len() < 3        // Hostname can't be shorter than 3 chars and not longer than 27 chars
                    || name.len() > 27 {
                    let res = create_response(&state, StatusCode::NOT_ACCEPTABLE, mime::APPLICATION_JSON, Body::from("{\"status\": \"Not Acceptable\"}"));
                    return future::ok((state, res))
                }

                // Execute gvr create + gvr set
                match create_cell(&name)
                    .and_then(|_| {
                        info!("Cell created: {}.", name);
                        add_ssh_pubkey_to_cell(&name, &ssh_pubkey)
                    })
                    .map_err(|err| {
                        error!("Failure: Cell: {} couldn't be created! Please contact administator or file a bug!\nError details: {}", name, err);
                        err
                    }) {

                    // create a new response based on the result:
                    Ok(_) => {
                        info!("Cell started: {}", name);
                        let res = create_response(&state, StatusCode::CREATED, mime::APPLICATION_JSON, Body::from("{\"status\": \"Created\"}"));
                        future::ok((state, res))
                    },
                    Err(err) => {
                        error!("Cell failed: {}. Last error: {}", name, err);
                        let res = create_response(&state, StatusCode::EXPECTATION_FAILED, mime::APPLICATION_JSON, Body::from("{\"status\": \"Expectation Failed\"}"));
                        future::ok((state, res))
                    }
                }
            }
            Err(e) => future::err((state, e.into_handler_error()))
        });

    Box::new(f)
}
