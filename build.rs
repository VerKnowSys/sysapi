// build.rs

extern crate cc;


// ??? NOTE: This process fails on svdOS builds older or than #131


fn main() {
    cc::Build::new()
        .cpp(true)
        .pic(true)
        .warnings(true)
        .flag("-Ofast")
        .flag("-std=c++11")
        .flag("-ftrapv")
        .flag("-fstack-protector")
        .flag("-fstack-protector-strong")
        .flag("-fstack-protector-all")
        .flag("-Wformat")
        .flag("-Wformat-security")
        .flag("-fno-strict-overflow")
        .flag("-mretpoline")
        .cpp_set_stdlib("c++")
        .cpp_link_stdlib("c++")
        .file("lib/kvmpro/src/kvmpro.h")
        .file("lib/kvmpro/src/kvm.cc")
        .file("lib/kvmpro/src/procstat.cc")
        .file("lib/kvmpro/src/utils.cc")
        .compile("kvmpro.so");
}
