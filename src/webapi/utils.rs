use glob::glob;
use libc::*;
use std::ffi::CStr;
use std::str;


use api::SENTRY_PATH;


// Link with core FreeBSD system libraries:
// #[link(name = "kvmpro")]
#[link(name = "kvm")]
#[link(name = "procstat")]
#[link(name = "kvmpro")]

/// Extern functions from kvmpro C++ library
extern "C" {

    /// Get processes + network connections - directly from kernel
    fn get_process_usage(user_uid: uid_t) -> *const c_char;

    /// Get processes - directly from kernel
    fn get_process_usage_short(user_uid: uid_t) -> *const c_char;

}


/// Call kernel directly through C++ function from libkvmpro library:
#[allow(unsafe_code)]
pub fn processes_of_pid(uid: uid_t) -> String {
    let c_buf: *const c_char = unsafe { get_process_usage(uid) };
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let a_slice: &str = c_str.to_str().unwrap_or("");
    a_slice.to_owned()
}


/// Call kernel directly through C++ function from libkvmpro library:
#[allow(unsafe_code)]
pub fn processes_of_pid_short(uid: uid_t) -> String {
    let c_buf: *const c_char = unsafe { get_process_usage_short(uid) };
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let a_slice: &str = c_str.to_str().unwrap_or("");
    a_slice.to_owned()
}


/// Produce list of dirs/files matching given glob pattern:
pub fn produce_list(glob_pattern: &String) -> Vec<String> {
    let mut list = vec!();
    for entry in glob(&glob_pattern).unwrap() {
        match entry {
            Ok(path) => {
                match path.file_name() {
                    Some(element) => {
                        element
                            .to_str()
                            .and_then(|elem| {
                                list.push(elem.to_string());
                                Some(elem.to_string())
                            });
                    },
                    None => (),
                }
            },
            Err(err) => {
                error!("Error: produce_list(): {}", err);
            },
        }
    }
    debug!("produce_list(): Elements: {:?}", list);
    list
}


/// Lists all cell attributes => /Shared/Prison/Sentry/CELLNAME/cell-attributes/*
pub fn list_attributes(cell_name: &String) -> Vec<String> {
    let glob_pattern = format!("{}/{}/cell-attributes/*", SENTRY_PATH, cell_name);
    debug!("list_attributes(): {}", glob_pattern);
    produce_list(&glob_pattern)
}


/// Lists all available cells based on files found under Sentry dir:
pub fn list_cells() -> Vec<String> {
    let glob_pattern = format!("{}/*", SENTRY_PATH);
    debug!("list_cells(): {}", glob_pattern);
    produce_list(&glob_pattern)
}


/// Lists all available proxies based on files found under Sentry dirs:
pub fn list_proxies() -> Vec<String> {
    let glob_pattern = format!("{}/**/cell-webconfs/*", SENTRY_PATH);
    debug!("list_proxies(): {}", glob_pattern);
    produce_list(&glob_pattern)
}
