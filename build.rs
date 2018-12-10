// build.rs

extern crate cc;


fn main() {
    cc::Build::new()
        .include("/usr/include/c++/v1")
        .include("/usr/include")
        .include("../kvmpro/src/")
        .file("../kvmpro/src/kvm.cc")
        .file("../kvmpro/src/procstat.cc")
        .file("../kvmpro/src/utils.cc")
        .flag("-O3")
        .flag("-ftrapv")
        .flag("-fstack-protector")
        .flag("-fstack-protector-strong")
        .flag("-fstack-protector-all")
        .flag("-Wformat")
        .flag("-Wformat-security")
        .flag("-fno-strict-overflow")
        .flag("-mretpoline")
        .static_flag(false)
        .shared_flag(false)
        .pic(true)
        .cpp(true)
        .cpp_link_stdlib("c++")
        .compile("kvmpro");
}
