use std::path::Path;

// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    build_bindings();
}

fn build_bindings() {
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    cbindgen::Builder::new()
        .with_config(cbindgen::Config::from_file("cbindgen.toml").expect("cbindgen config file."))
        .with_crate(crate_dir)
        .generate()
        .expect("Failed to generate C bindings!")
        .write_to_file("include/quibble-rs.h");
}
