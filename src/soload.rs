use libc::*;
use libloading::os::unix::*;
use std::{mem::forget, thread::spawn, thread::sleep, sync::{Arc, Mutex}, sync::atomic::{AtomicUsize, Ordering}};
use colored::Colorize;
use std::time::Duration;
use crate::{*, helpers::empty_list_string};


#[allow(missing_debug_implementations)]
#[derive(Copy, Clone)]
#[repr(C)]
/// Data structure, returned by functions dynamically loaded from: libkvmpro.so:
pub struct kvmpro_t {
    length: size_t,
    bytes: [u8; 262144],
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
    Library::open(Some(DEFAULT_LIBKVMPRO_SHARED), RTLD_NOW) // dynamic shared object loading, using libDL
        .and_then(|lib| {
            let function_from_symbol: Symbol<extern "C" fn(uid_t) -> kvmpro_t> = unsafe { lib.get(fun_symbol_name) }?;
            let object: kvmpro_t = function_from_symbol(uid);
            forget(lib); // NOTE: Skipping this call causes significant memory leak per-each function call!
            Ok(
               String::from_utf8(object.bytes[0..object.length].to_vec()).unwrap_or_else(|_| empty_list_string())
            )
        })
        .map_err(|err| {
            let function_name = String::from_utf8(fun_symbol_name.to_vec()).unwrap_or_else(|_| "fn_with_no_name".to_string());
            error!("FAILURE of: {}(): No such function-symbol found in library: {}. Details: {}.",
                   function_name.cyan(), DEFAULT_LIBKVMPRO_SHARED.cyan(), err.to_string().red());
        })
        .unwrap_or_else(|_| empty_list_string())
}


/// Call kernel directly through C++ function from kvmpro library:
pub fn processes_of_uid(uid: uid_t) -> String {
    spawn(
        move || {
            match SOLOAD_MT_CALLS_SYNCHRONIZER.try_lock() {
                Ok(locked_resource) => {
                    locked_resource.fetch_add(1, Ordering::SeqCst); // Increment atomic counter:
                    let value = locked_resource.load(Ordering::SeqCst);
                    if value % SOLOAD_MT_INFO_TRIGGER_MODULO_NUM == 0 {
                        info!("API calls-counter processed: {} calls so far.", value.to_string().cyan());
                    }
                    string_from_native_fn(b"get_process_usage_t\0", uid)
                },
                Err(err) => {
                    debug!("Failed to acquire thread lock. Details: {}", err.to_string().red());
                    sleep(Duration::from_millis(SOLOAD_MT_CALLS_INTERVAL));
                    return processes_of_uid(uid)
                }
            }
        }
    )
    .join()
    .unwrap_or_else(|_| empty_list_string())
}


/// Call kernel directly through C++ function from kvmpro library:
pub fn processes_of_uid_short(uid: uid_t) -> String {
    spawn(
        move || {
            match SOLOAD_MT_CALLS_SYNCHRONIZER.try_lock() {
                Ok(locked_resource) => {
                    locked_resource.fetch_add(1, Ordering::SeqCst); // Increment atomic counter:
                    let value = locked_resource.load(Ordering::SeqCst);
                    if value % SOLOAD_MT_INFO_TRIGGER_MODULO_NUM == 0 {
                        info!("API calls-counter processed: {} calls so far.", value.to_string().cyan());
                    }
                    string_from_native_fn(b"get_process_usage_short_t\0", uid)
                },
                Err(err) => {
                    debug!("Failed to acquire thread lock. Details: {}", err.to_string().red());
                    sleep(Duration::from_millis(SOLOAD_MT_CALLS_INTERVAL));
                    return processes_of_uid_short(uid)
                }
            }
        }
    )
    .join()
    .unwrap_or_else(|_| empty_list_string())
}


//
// NOTE: Direct load C/C++ functions approach kinda works, but this approach implies
// that linker has to explicitly require custom library which I wish to avoid:
//
//
// let object: kvmpro_t = unsafe { get_process_usage_t(uid) };
// String::from_utf8(object.bytes[0..object.length].to_vec()).unwrap_or_else(|_| empty_list_string())
//
// let object: kvmpro_t = unsafe { get_process_usage_short_t(uid) };
// String::from_utf8(object.bytes[0..object.length].to_vec()).unwrap_or_else(|_| empty_list_string())
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

