// build.rs

fn main() {
    println!("cargo:rustc-link-lib=dylib=kvmpro\ncargo:rustc-link-search=native=/usr/lib\n");
}
