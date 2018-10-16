//! sysapi.centra.systems

#[macro_use]
extern crate lazy_static;
extern crate futures;
extern crate gotham;
extern crate hyper;
extern crate mime;
extern crate regex;

use hyper::{Body, Headers, HttpVersion, Method, Response, StatusCode, Uri};
use futures::{future, Future, Stream};

use gotham::http::response::create_response;
use gotham::state::{FromState, State};
use gotham::router::Router;
use gotham::router::builder::{build_simple_router, DefineSingleRoute, DrawRoutes};
use gotham::handler::{HandlerFuture, IntoHandlerError};

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use regex::Regex;


const LISTEN_ADDRESS: &'static str = "172.16.3.1";
const HOSTS_RESOURCE: &'static str = "/hosts/";
const CUSTODY_PATH: &'static str = "/Shared/Custody";
const CELLS_PATH: &'static str = "/Shared/Prison/Cells";


// Precompile NAME_PATTERN only once:
lazy_static! {
    static ref NAME_PATTERN: Regex = {
        Regex::new(r"^[a-zA-Z0-9]*$").unwrap()
    };
}


/// Extract the main elements of the request except for the `Body`
fn _print_request_elements(state: &State) {
    let method = Method::borrow_from(state);
    let uri = Uri::borrow_from(state);
    let http_version = HttpVersion::borrow_from(state);
    let headers = Headers::borrow_from(state);
    println!("Method: {:?}", method);
    println!("URI: {:?}", uri);
    println!("HTTP Version: {:?}", http_version);
    println!("Headers: {:?}", headers);
}


/// Define router
fn router() -> Router {
    build_simple_router(|route| {
        route
            .associate(&format!("{resource}:host", resource = HOSTS_RESOURCE), |handler| {
                handler.get().to(get_handler);
                handler.post().to(post_handler);
                handler.delete().to(delete_handler);
            })
    })
}


/// Handle DELETEs
fn delete_handler(mut state: State) -> Box<HandlerFuture> {
    // print_request_elements(&state);
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(_) => {
                let uri = Uri::borrow_from(&state).to_string();
                let name = uri.replace(HOSTS_RESOURCE, "");
                let cell_dir = format!("{}/{}", CELLS_PATH, name);

                if Path::new(&cell_dir).exists() {
                    println!("Destroy jail: {}", &name);
                    let res = create_response(&state, StatusCode::Ok, None);
                    return future::ok((state, res))
                } else {
                    let res = create_response(&state, StatusCode::NotModified, None);
                    return future::ok((state, res))
                }
            }
            Err(e) => return future::err((state, e.into_handler_error())),
        });

    Box::new(f)
}


/// Handle GETs
fn get_handler(state: State) -> (State, Response) {
    // print_request_elements(&state);
    let res = create_response(&state, StatusCode::NotImplemented, None);

    (state, res)
}


/// Handle POSTs
fn post_handler(mut state: State) -> Box<HandlerFuture> {
    // print_request_elements(&state);
    let f = Body::take_from(&mut state)
        .concat2()
        .then(|full_body| match full_body {
            Ok(valid_body) => {
                let uri = Uri::borrow_from(&state).to_string();
                let name = uri.replace(HOSTS_RESOURCE, "");
                let path = format!("{}/{}", CUSTODY_PATH, name);
                let ssh_pubkey = String::from_utf8(valid_body.to_vec()).unwrap(); // Read SSH pubkey from request body:
                println!("Hostname: {}, SSH-ED25519 pubkey: {} (key-length: {})", name, ssh_pubkey, ssh_pubkey.len());

                // Validate all input data:
                if !NAME_PATTERN.is_match(&name)
                    || ssh_pubkey.len() < 68 // Ed25519 should be at least 68, but not longer than 70 bytes long
                    || ssh_pubkey.len() > 70
                    || name.len() < 3        // Hostname can't be shorter than 3 chars and not longer than 27 chars
                    || name.len() > 27 {
                    let res = create_response(&state, StatusCode::NotAcceptable, None);
                    return future::ok((state, res))
                }
                let cell_dir = format!("{}/{}", CELLS_PATH, name);
                if Path::new(&path).exists() || Path::new(&cell_dir).exists() {
                    let res = create_response(&state, StatusCode::Conflict, None);
                    return future::ok((state, res))
                }
                match File::create(&path) {
                    Ok(mut valid_file) => {
                        valid_file.write_all((format!("{}\n", ssh_pubkey)).as_bytes()).unwrap();
                        let res = create_response(&state, StatusCode::Created, None);
                        return future::ok((state, res))
                    },
                    Err(failure) => {
                        println!("Error: Failed to create file: {}. Reason: {}", &path, &failure);
                        let res = create_response(&state, StatusCode::BadRequest, None);
                        return future::ok((state, res))
                    },
                }
            }
            Err(e) => return future::err((state, e.into_handler_error())),
        });

    Box::new(f)
}


/// Start a server and use a `Router` to dispatch requests
pub fn main() {
    let addr = LISTEN_ADDRESS;
    println!("Listening for requests at http://{}", addr);
    gotham::start(addr, router())
}


#[cfg(test)]
mod tests {
    use super::*;
    use gotham::test::TestServer;


    #[test]
    fn test_name_pattern() {
        assert!(NAME_PATTERN.is_match("2asd01F4013201d"));
        assert!(!NAME_PATTERN.is_match("2-asd01F4013201d"));
        assert!(!NAME_PATTERN.is_match("2.asd01F4013201d"));
        assert!(!NAME_PATTERN.is_match("2_asd01F4013201d"));
        assert!(!NAME_PATTERN.is_match("2 asd01F4013201d"));
        assert!(!NAME_PATTERN.is_match("2@asd01F4013201d"));
    }


    #[test]
    fn test_hostname_too_short() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .post("http://localhost/hosts/12", None, mime::TEXT_PLAIN)
            .perform()
            .unwrap();
        assert_eq!(response.status(), StatusCode::NotAcceptable);
    }


    #[test]
    fn test_hostname_too_long() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .post("http://localhost/hosts/123456789012345678901234567890", None, mime::TEXT_PLAIN)
            .perform()
            .unwrap();
        assert_eq!(response.status(), StatusCode::NotAcceptable);
    }


    #[test]
    fn test_no_ssh_pubkey_in_body() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .post("http://localhost/hosts/12345", None, mime::TEXT_PLAIN)
            .perform()
            .unwrap();
        assert_eq!(response.status(), StatusCode::NotAcceptable);
    }


    #[test]
    fn test_too_short_ssh_pubkey_in_body() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .post("http://localhost/hosts/12345", "my-nokey", mime::TEXT_PLAIN)
            .perform()
            .unwrap();
        assert_eq!(response.status(), StatusCode::NotAcceptable);
    }


    #[test]
    fn test_ssh_pubkey_in_body() {
        use std::fs;
        let test_server = TestServer::new(router()).unwrap();
        let hostname = "test12345";
        let valid_sshed25519_pubkey = "AAAAC3NzaC1lZDI1NTE5AAAAIEafihGp0at+QR94JaF+NkJ4XuZLjleEz/owVzRBqC9d";
        let filename = format!("{}/{}", CUSTODY_PATH, hostname);
        fs::remove_file(filename.clone()).unwrap_or(());
        let response = test_server
            .client()
            .post(&format!("http://localhost/hosts/{}", hostname), valid_sshed25519_pubkey, mime::TEXT_PLAIN)
            .perform()
            .unwrap();
        assert_eq!(response.status(), StatusCode::Created);

        let mut f = File::open(filename.clone()).unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap_or(0);
        assert_eq!(contents, format!("{}\n", valid_sshed25519_pubkey));
        fs::remove_file(filename).unwrap_or(());
    }


    #[test]
    fn test_delete_not_existing_is_not_modified() {
        let test_server = TestServer::new(router()).unwrap();
        let response = test_server
            .client()
            .delete("http://localhost/hosts/test12345")
            .perform()
            .unwrap();
        assert_eq!(response.status(), StatusCode::NotModified);
    }


}
