use libloading::*;

use self::common::*;


/// Explicitly link with required system shared libs:
#[link(name = "kvm")]
#[link(name = "procstat")]
#[link(name = "kvmpro", kind = "dylib")]


/// Call kernel directly through C++ function from libkvmpro library:
#[allow(unsafe_code)]
pub fn processes_of_uid(uid: uid_t) -> String {
    Library::new(DEFAULT_LIBKVMPRO_SHARED)
        .and_then(|lib| {
            unsafe {
                let ps_of_uid: Symbol<unsafe extern fn(uid_t) -> String> = lib.get(b"_ZN6Kvmpro18protocol_to_stringEiii\0").unwrap();
                Ok(ps_of_uid(uid))
            }
        })
        .unwrap()
}


/// Call kernel directly through C++ function from libkvmpro library:
#[allow(unsafe_code)]
pub fn processes_of_uid_short(uid: uid_t) -> String {
    Library::new(DEFAULT_LIBKVMPRO_SHARED)
        .and_then(|lib| {
            unsafe {
                let ps_short: Symbol<unsafe extern fn(uid_t) -> String> = lib.get(b"_ZN6Kvmpro23get_process_usage_shortEj\0").unwrap();
                Ok(ps_short(uid))
            }
        })
        .unwrap()
}

