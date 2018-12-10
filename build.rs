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
        .flag("-D_FORTIFY_SOURCE=2")
        .flag("--param")
        .flag("ssp-buffer-size=4")
        .static_flag(false)
        .shared_flag(false)
        .pic(true)
        .cpp(true)
        .cpp_link_stdlib("c++")
        .compile("kvmpro");

        // .flag("-fuse-ld=lld")
        // .flag("-Wl,-z,relro,-z,now,-z,retpolineplt")
        // .flag("-w")
}
