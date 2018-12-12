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
extern crate lazy_static;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

    /// Project directory (for static files access for router):
    pub const PROJECT_DIRECTORY: &str = "/Projects/sysapi";

    /// Default log output file:
    pub const DEFAULT_LOG_FILE: &str = "/var/log/sysapi.log";

    /// svdOS cell governor:
    pub const GVR_BIN: &str = "/usr/bin/gvr";

    /// ZFS utility:
    pub const ZFS_BIN: &str = "/sbin/zfs";

    /// BSD jail utility:
    pub const JAIL_BIN: &str = "/usr/sbin/jail";

    /// BSD jail-exec utility:
    pub const JEXEC_BIN: &str = "/usr/sbin/jexec";

    /// Default username (jail user):
    pub const CELL_USERNAME: &str = "worker";

    /// Default local DNS server address:
    pub const DEFAULT_DNS: &str = "172.16.3.1";

    /// Default listen address to listen on:
    pub const DEFAULT_ADDRESS: &str = "172.16.3.1:80";

    /// Default path to Prison root dir:
    pub const PRISON_PATH: &str = "/Shared/Prison";

    /// Default path to cells data dirs:
    pub const CELLS_PATH: &str = "/Shared/Prison/Cells";

    /// Default path to sentry metadata dirs:
    pub const SENTRY_PATH: &str = "/Shared/Prison/Sentry";


    /// WebAPI

    /// Cell management:
    pub const CELL_RESOURCE: &str = "/cell/";

    /// Cell lists management:
    pub const CELLS_RESOURCE: &str = "/cells/";

    /// Igniter management:
    pub const IGNITER_RESOURCE: &str = "/igniter/";

    /// DNS zone management:
    pub const ZONE_RESOURCE: &str = "/zone/";

    /// Web proxy management:
    pub const PROXY_RESOURCE: &str = "/proxy/";

    /// Web proxies management:
    pub const PROXIES_RESOURCE: &str = "/proxies/";

    /// Cell status management:
    pub const STATUS_RESOURCE: &str = "/status/";

    /// Cell ZFS Snapshot management:
    pub const SNAPSHOT_RESOURCE: &str = "/snapshot/";

    /// Cell ZFS Rollback management:
    pub const ROLLBACK_RESOURCE: &str = "/rollback/";

    /// Cell ZFS datasets management:
    pub const DATASETS_RESOURCE: &str = "/datasets/";

}



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
