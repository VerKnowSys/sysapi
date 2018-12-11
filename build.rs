// build.rs

extern crate cc;


fn main() {
    cc::Build::new()
        .cpp(true)
        .pic(true)
        .warnings(true)
        .flag("-O2")
        .flag("-fPIE")
        .flag("-std=c++11")
        .cpp_set_stdlib("c++")
        .cpp_link_stdlib("c++")
        .file("lib/kvmpro/src/kvm.cc")
        .file("lib/kvmpro/src/procstat.cc")
        .file("lib/kvmpro/src/utils.cc")
        .compile("kvmpro.so");
}
