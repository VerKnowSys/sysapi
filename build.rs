// build.rs

extern crate cc;


fn main() {
    cc::Build::new()
        .include("/usr/include")
        .file("../kvmpro/src/kvmpro.h")
        .file("../kvmpro/src/kvm.cc")
        .file("../kvmpro/src/procstat.cc")
        .file("../kvmpro/src/utils.cc")
        .pic(true)
        .cpp(true)
        .flag("-O3")
        .flag("-fuse-ld=lld")
        .flag("-fPIC")
        .flag("-fPIE")
        .flag("-ftrapv")
        .flag("-mretpoline")
        .flag("-fstack-protector")
        .flag("-fstack-protector-strong")
        .flag("-fstack-protector-all")
        .flag("-fno-strict-overflow")
        .flag("-Wformat")
        .flag("-Wformat-security")
        .flag("-Wl,-z,relro,-z,now")
        .flag("-Wl,-z,retpolineplt")
        .flag("-D_FORTIFY_SOURCE=2")
        .static_flag(false)
        .shared_flag(true)
        .cpp_link_stdlib("c++")
        .compile("kvmpro");
}
