// build.rs

extern crate cc;


fn main() {
    cc::Build::new()
        .cpp(true)
        .pic(true)
        .warnings(true)
        .flag("-O2")
        .flag("-g")
        .flag("-gdwarf-2")
        .flag("-fPIE")
        .flag("-std=c++11")
        // .flag("-Wreturn-type-c-linkage") // To disable such warnings: -Wno-return-type-c-linkage
        .cpp_set_stdlib("c++")
        .cpp_link_stdlib("c++")
        .file("lib/kvmpro/src/kvmpro_utils.cc")
        .file("lib/kvmpro/src/kvmpro_pub.cc")
        .compile("kvmpro.a");
}
