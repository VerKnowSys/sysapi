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


#[allow(unused_imports)]
#[macro_use]
extern crate log;


#[allow(unused_imports)]
#[macro_use]
extern crate lazy_static;


#[allow(unused_imports)]
#[macro_use]
extern crate serde_derive;


#[allow(unused_imports)]
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
