//! ServeD-SysAPI

//! Crate docs


#![deny(
        missing_docs,
        unstable_features,
        unsafe_code,
        missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unused_import_braces,
        unused_qualifications)]


#[macro_use]
extern crate log;
extern crate fern;
extern crate chrono;
#[macro_use]
extern crate lazy_static;
extern crate futures;
extern crate gotham;
extern crate hyper;
extern crate mime;
extern crate regex;
#[macro_use]
extern crate serde_derive;
extern crate tokio;
// extern crate tokio_core;
// extern crate tokio_reactor;
// extern crate tokio_current_thread;
extern crate abstract_ns;
// extern crate ns_dns_tokio;
extern crate ns_std_threaded;

extern crate serde;
extern crate serde_json;
extern crate glob;
extern crate hostname;
extern crate domain;
extern crate atomicwrites;
extern crate systemstat;


/// Public sysapi modules:
pub mod api;

/// Public webapi modules:
pub mod webapi;


// use std::sync::RwLock;
// use tokio_core::reactor::{Core, Remote};
pub use fern::colors::{Color, ColoredLevelConfig};

pub use api::*;
pub use webapi::*;
pub use webapi::cells::*;


#[cfg(test)]
mod tests {
    // Load all internal modules:
    use hyper::*;
    use gotham::test::TestServer;
    use regex::Regex;
    use super::*;


    // Precompile NAME_PATTERN only once:
    lazy_static! {
        pub static ref NAME_PATTERN: Regex = {
            Regex::new(r"^[a-zA-Z0-9]*$").unwrap()
        };
    }


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
        let test_server = TestServer::new(router::router()).unwrap();
        let response = test_server
            .client()
            .post("http://localhost/cell/12", Body::from("".to_string()), mime::TEXT_PLAIN)
            .perform()
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_ACCEPTABLE);
    }


    #[test]
    fn test_hostname_too_long() {
        let test_server = TestServer::new(router::router()).unwrap();
        let response = test_server
            .client()
            .post("http://localhost/cell/123456789012345678901234567890", Body::from("".to_string()), mime::TEXT_PLAIN)
            .perform()
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_ACCEPTABLE);
    }


    #[test]
    fn test_no_ssh_pubkey_in_body() {
        let test_server = TestServer::new(router::router()).unwrap();
        let response = test_server
            .client()
            .post("http://localhost/cell/12345", Body::from("".to_string()), mime::TEXT_PLAIN)
            .perform()
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_ACCEPTABLE);
    }


    #[test]
    fn test_too_short_ssh_pubkey_in_body() {
        let test_server = TestServer::new(router::router()).unwrap();
        let response = test_server
            .client()
            .post("http://localhost/cell/12345", "my-nokey", mime::TEXT_PLAIN)
            .perform()
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_ACCEPTABLE);
    }


    // #[test]
    // fn test_ssh_pubkey_in_body() {
    //     use std::fs;
    //     let test_server = TestServer::new(router::router()).unwrap();
    //     let hostname = "test12345";
    //     let valid_sshed25519_pubkey = "AAAAC3NzaC1lZDI1NTE5AAAAIEafihGp0at+QR94JaF+NkJ4XuZLjleEz/owVzRBqC9d";
    //     let filename = format!("{}/{}", CUSTODY_PATH, hostname);
    //     fs::remove_file(filename.clone()).unwrap_or(());
    //     let response = test_server
    //         .client()
    //         .post(&format!("http://localhost/cell/{}", hostname), valid_sshed25519_pubkey, mime::TEXT_PLAIN)
    //         .perform()
    //         .unwrap();
    //     assert_eq!(response.status(), StatusCode::Created);

    //     let mut f = File::open(filename.clone()).unwrap();
    //     let mut contents = String::new();
    //     f.read_to_string(&mut contents).unwrap_or(0);
    //     assert_eq!(contents, format!("{}\n", valid_sshed25519_pubkey));
    //     fs::remove_file(filename).unwrap_or(());
    // }


    #[test]
    fn test_delete_not_existing_is_not_modified() {
        let test_server = TestServer::new(router::router()).unwrap();
        let response = test_server
            .client()
            .delete("http://localhost/cell/test12345")
            .perform()
            .unwrap();
        assert_eq!(response.status(), StatusCode::NOT_MODIFIED);
    }


}
