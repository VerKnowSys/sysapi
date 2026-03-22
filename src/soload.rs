use crate::{helpers::empty_list_string, *};
use libc::{RTLD_NOW, size_t, uid_t};
use libloading::os::unix::{Library, Symbol};
use std::{
    mem::forget,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    thread::{sleep, spawn},
    time::Duration,
};


#[allow(missing_debug_implementations)]
#[derive(Copy, Clone)]
#[repr(C)]
/// Data structure, returned by functions dynamically loaded from: libkvmpro.so:
pub struct kvmpro_t {
    length: size_t,
    bytes: [u8; 262_144],
}


lazy_static! {
    /// Global SOLOAD_MT_CALLS_SYNCHRONIZER - required when handling requests,
    /// that call kernel APIs (which are often NOT Thread-safe):
    pub static ref SOLOAD_MT_CALLS_SYNCHRONIZER: Arc<Mutex<AtomicUsize>> = {
        Arc::new(
            Mutex::new(
                AtomicUsize::new(0_usize) // Atomic counter of function calls since last restart
            )
        )
    };
}


/// Helper to dynamically call function from shared object:
#[allow(unsafe_code)]
pub fn string_from_native_fn(fun_symbol_name: &[u8], uid: uid_t) -> String {
    unsafe { Library::open(Some(DEFAULT_LIBKVMPRO_SHARED), RTLD_NOW) } // dynamic shared object loading, using libDL
        .and_then(|lib| {
            let function_from_symbol: Symbol<extern "C" fn(uid_t) -> kvmpro_t> =
                unsafe { lib.get(fun_symbol_name) }?;
            let object: kvmpro_t = function_from_symbol(uid);
            forget(lib); // NOTE: Skipping this call causes significant memory leak per-each function call!
            Ok(String::from_utf8(object.bytes[0..object.length].to_vec())
                .unwrap_or_else(|_| empty_list_string()))
        })
        .map_err(|err| {
            let function_name = String::from_utf8(fun_symbol_name.to_vec())
                .unwrap_or_else(|_| "fn_with_no_name".to_string());
            error!(
                "FAILURE of: {}(): No such function-symbol found in library: {}. Details: {}.",
                function_name.cyan(),
                DEFAULT_LIBKVMPRO_SHARED.cyan(),
                err.to_string().red()
            );
        })
        .unwrap_or_else(|_| empty_list_string())
}


/// Call kernel directly through C++ function from kvmpro library:
pub fn processes_of_uid(uid: uid_t) -> String {
    spawn(move || {
        match SOLOAD_MT_CALLS_SYNCHRONIZER.try_lock() {
            Ok(locked_resource) => {
                locked_resource.fetch_add(1, Ordering::SeqCst); // Increment atomic counter:
                let value = locked_resource.load(Ordering::SeqCst);
                if value % SOLOAD_MT_INFO_TRIGGER_MODULO_NUM == 0 {
                    info!(
                        "API calls-counter processed: {} calls so far.",
                        value.to_string().cyan()
                    );
                }
                string_from_native_fn(b"get_process_usage_t\0", uid)
            }
            Err(err) => {
                debug!(
                    "Failed to acquire thread lock. Details: {}",
                    err.to_string().red()
                );
                sleep(Duration::from_millis(SOLOAD_MT_CALLS_INTERVAL));
                processes_of_uid(uid)
            }
        }
    })
    .join()
    .unwrap_or_else(|_| empty_list_string())
}


/// Call kernel directly through C++ function from kvmpro library:
pub fn processes_of_uid_short(uid: uid_t) -> String {
    spawn(move || {
        match SOLOAD_MT_CALLS_SYNCHRONIZER.try_lock() {
            Ok(locked_resource) => {
                locked_resource.fetch_add(1, Ordering::SeqCst); // Increment atomic counter:
                let value = locked_resource.load(Ordering::SeqCst);
                if value % SOLOAD_MT_INFO_TRIGGER_MODULO_NUM == 0 {
                    info!(
                        "API calls-counter processed: {} calls so far.",
                        value.to_string().cyan()
                    );
                }
                string_from_native_fn(b"get_process_usage_short_t\0", uid)
            }
            Err(err) => {
                debug!(
                    "Failed to acquire thread lock. Details: {}",
                    err.to_string().red()
                );
                sleep(Duration::from_millis(SOLOAD_MT_CALLS_INTERVAL));
                processes_of_uid_short(uid)
            }
        }
    })
    .join()
    .unwrap_or_else(|_| empty_list_string())
}
