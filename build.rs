// build.rs

fn main() {
    println!("{}\n{}\n",
            "cargo:rustc-link-lib=dylib=kvmpro",
            "cargo:rustc-link-search=native=/usr/lib");
}
