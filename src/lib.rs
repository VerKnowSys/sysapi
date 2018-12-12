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














