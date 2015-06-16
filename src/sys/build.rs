

#[cfg(target_os="macos")]
fn main() {
    println!("cargo:rustc-link-search={}/../../openvr/bin/osx32", env!("CARGO_MANIFEST_DIR"));
}

//fn main() {}