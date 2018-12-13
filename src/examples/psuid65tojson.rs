//! Example #01

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
extern crate serde_derive;

extern crate sysapi;

use crate::sysapi::soload::processes_of_uid;
use crate::sysapi::soload::processes_of_uid_short;


#[link(name = "kvmpro", kind = "dylib")]

/// ProcessList for uid 65 into JSON
pub fn main() {
    println!("Process of UID 65 (LONG)  OUT: {}", processes_of_uid(65));
    println!("Process of UID 65 (SHORT) OUT: {}", processes_of_uid_short(65));
}
