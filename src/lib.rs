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

/// Absolute path to libkvmpro.so shared library installed on the production system:
pub const DEFAULT_LIBKVMPRO_SHARED: &str = "/usr/lib/libkvmpro.so";

/// Project directory (for static files access for router):
pub const PROJECT_DIRECTORY: &str = "/Projects/sysapi";

/// Default log output file:
pub const DEFAULT_LOG_FILE: &str = "/var/log/sysapi.log";

/// Default stdout POSIX system device:
pub const DEFAULT_STDOUT_DEV: &str = "/dev/stdout";

/// Default stderr POSIX system device:
pub const DEFAULT_STDERR_DEV: &str = "/dev/stderr";

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


#[cfg(test)]
mod tests;


/// Map C functions from a Shared-Object system library:
pub mod soload {
    use libc::*;
    use libloading::os::unix::*;
    use std::{mem::forget, thread::spawn, sync::{Arc, Mutex}};
    use crate::{helpers::empty_string, DEFAULT_LIBKVMPRO_SHARED};


    #[allow(missing_debug_implementations)]
    #[derive(Copy, Clone)]
    #[repr(C)]
    /// libkvmpro.so:
    pub struct kvmpro_t {
        length: size_t,
        bytes: [u8; 262144],
    }


    lazy_static! {
        /// Global SOLOAD_MT_CALLS_SYNCHRONIZER - required when handling requests,
        /// that call kernel APIs (which are often NOT Thread-safe):
        pub static ref SOLOAD_MT_CALLS_SYNCHRONIZER: Arc<Mutex<u32>> = {
            Arc::new(Mutex::new(0_u32))
        };
    }


    /// Helper to dynamically call function from shared object:
    #[allow(unsafe_code)]
    pub fn string_from_native_fn(fun_symbol_name: &[u8], uid: uid_t) -> String {
        Library::open(Some(DEFAULT_LIBKVMPRO_SHARED), RTLD_NOW) // dynamic shared object loading, using libDL
            .and_then(|lib| {
                let function_from_symbol: Symbol<extern "C" fn(uid_t) -> kvmpro_t> = unsafe { lib.get(fun_symbol_name) }?;
                let object: kvmpro_t = function_from_symbol(uid);
                forget(lib); // NOTE: Skipping this call causes significant memory leak per-each function call!
                Ok(
                   String::from_utf8(object.bytes[0..object.length].to_vec()).unwrap_or(empty_string())
                )
            })
            .map_err(|err| {
                let function_name = String::from_utf8(fun_symbol_name.to_vec()).unwrap_or("fn_with_no_name".to_string());
                error!("FAILURE of: {}(): No such function-symbol found in library: {}. Details: {}.",
                       function_name, DEFAULT_LIBKVMPRO_SHARED, err.to_string());
            })
            .unwrap_or(empty_string())
    }


    /// Call kernel directly through C++ function from kvmpro library:
    pub fn processes_of_uid(uid: uid_t) -> String {
        spawn(
            move || {
                match SOLOAD_MT_CALLS_SYNCHRONIZER.try_lock() {
                    Ok(_) => string_from_native_fn(b"get_process_usage_t\0", uid),
                    Err(err) => {
                        debug!("Failed to acquire thread lock. Details: {}", err.to_string());
                        empty_string()
                    }
                }
            }
        )
        .join()
        .unwrap_or(empty_string())
    }


    /// Call kernel directly through C++ function from kvmpro library:
    pub fn processes_of_uid_short(uid: uid_t) -> String {
        spawn(
            move || {
                match SOLOAD_MT_CALLS_SYNCHRONIZER.try_lock() {
                    Ok(_) => string_from_native_fn(b"get_process_usage_short_t\0", uid),
                    Err(err) => {
                        debug!("Failed to acquire thread lock. Details: {}", err.to_string());
                        empty_string()
                    }
                }
            }
        )
        .join()
        .unwrap_or(empty_string())
    }


    //
    // NOTE: Direct load C/C++ functions approach kinda works, but this approach implies
    // that linker has to explicitly require custom library which I wish to avoid:
    //
    //
    // let object: kvmpro_t = unsafe { get_process_usage_t(uid) };
    // String::from_utf8(object.bytes[0..object.length].to_vec()).unwrap_or(empty_string())
    //
    // let object: kvmpro_t = unsafe { get_process_usage_short_t(uid) };
    // String::from_utf8(object.bytes[0..object.length].to_vec()).unwrap_or(empty_string())
    //
    // extern "C" {
    //     /// Get processes + network connections - directly from kernel
    //     #[no_mangle]
    //     pub fn get_process_usage_t(user_uid: uid_t) -> kvmpro_t;
    //     /// Get processes - directly from kernel
    //     #[no_mangle]
    //     pub fn get_process_usage_short_t(user_uid: uid_t) -> kvmpro_t;
    // }
    //


}
