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


// Library constants, used by the project:

/// Software author :)
pub const CREATED_BY: &str = "Daniel (@dmilith) Dettlaff";

/// Default network device name - used by virtual network stack for cells.
/// Link schema: Host: epairaNET_ID_NUMBER <---> Cell: epairbNET_ID_NUMBER
pub const CELL_NET_INTERFACE: &str = "epair";

/// Absolute path to libkvmpro.so shared library installed on the production system:
pub const DEFAULT_LIBKVMPRO_SHARED: &str = "/usr/lib/libkvmpro.so";

/// Project directory (for static files access for router):
pub const PROJECT_DIRECTORY: &str = "/Projects/sysapi";

/// Default log output file:
pub const DEFAULT_LOG_FILE: &str = "/var/log/sysapi.log";

/// Default stdout POSIX system device:
pub const DEFAULT_STDOUT_DEV: &str = "/dev/stdout";

// /// Default stderr POSIX system device:
// pub const DEFAULT_STDERR_DEV: &str = "/dev/stderr";

/// Default fallback for hostname:
pub const DEFAULT_HOSTNAME_FALLBACK: &str = "localhost";

/// Default fallback for IPv4:
pub const DEFAULT_IP_FALLBACK: &str = "127.0.0.1";

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

/// Default protocol for ControlPane:
pub const DEFAULT_CONTROLPANE_PROTOCOL: &str = "http";

/// Default Cell NetID/ UID state filename:
pub const DEFAULT_CELL_NETID_FILE: &str = "cell.vlan.number";

/// Default Cell IP address state filename:
pub const DEFAULT_CELL_IP_FILE: &str = "cell.ip.addresses";

/// Default Cell running state filename:
pub const DEFAULT_CELL_RUNSTATE_FILE: &str = "cell.running";

/// CpuStat measure interval:
pub const SYSTAT_CPUSTAT_INTERVAL: u64 = 397; /* ms of interval before CPU usage measure takes place */

/// Time in miliseconds to pause before calling same function again (retry):
/// NOTE: Don't set this value too low, to avoid flood of the "new threads"
pub const SOLOAD_MT_CALLS_INTERVAL: u64 = 43; /* ms of interval before trying to call function again (waiting for lock) */

/// Modulo this number == 0, then print info with counter state:
pub const SOLOAD_MT_INFO_TRIGGER_MODULO_NUM: usize = 10000; /* print info with counter state each 10000 calls */



// EOF project constants.



/// HTTP Request params static strings:

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



//
// Public modules:
//


/// Public helpers, functions used by other modules:
pub mod helpers;

/// Public api modules used to "talk" with underlying system:
pub mod apis;

/// Web processors to handle WebAPI calls over HTTP:
pub mod processors;

/// Main router for Web processors:
pub mod webrouter;

/// Map C functions from a Shared-Object system library:
pub mod soload;


//
// Private modules:
//

#[cfg(test)]
mod tests;
