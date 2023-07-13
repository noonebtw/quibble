use std::path::Path;

// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    let quibble = build_quibble();

    println!(
        "cargo:rustc-link-search=native={}",
        quibble.join("build").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        quibble.join("build").join("freetype").display()
    );
    println!(
        "cargo:rustc-link-search=native={}",
        quibble.join("build").join("harfbuzz").display()
    );
    println!("cargo:rustc-link-lib=static=libquibble");
    println!("cargo:rustc-link-lib=static=freetype");
    println!("cargo:rustc-link-lib=static=harfbuzz");
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

fn build_quibble() -> std::path::PathBuf {
    let dst = cmake::Config::new(".").profile("Release").build();
    dst
}

fn build_freetype() -> std::path::PathBuf {
    let dst = cmake::Config::new("freetype")
        .define("FT_DISABLE_BZIP2", "TRUE")
        .define("FT_DISABLE_HARFBUZZ", "TRUE")
        .define("FT_DISABLE_PNG", "TRUE")
        .profile("Release")
        .define("FT_DISABLE_ZLIB", "TRUE")
        .define("FT_DISABLE_BROTLI", "TRUE")
        .cflag("-static -fno-stack-check -fno-stack-protector -mno-stack-arg-probe -DFT_CONFIG_OPTION_DISABLE_STREAM_SUPPORT=1")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=freetype");
    dst
}

fn build_harfbuzz() -> std::path::PathBuf {
    let dst = cmake::Config::new("harfbuzz")
        .profile("Release")
        .cflag("-static -fno-stack-check -fno-stack-protector -mno-stack-arg-probe -Wa,-mbig-obj -DHB_TINY -Dhb_malloc_impl=hb_malloc_impl2 -Dhb_calloc_impl=hb_calloc_impl2 -Dhb_realloc_impl=hb_realloc_impl2 -Dhb_free_impl=hb_free_impl2")
        .build();

    println!(
        "cargo:rustc-link-search=native={}",
        dst.join("lib").display()
    );
    println!("cargo:rustc-link-lib=static=harfbuzz");

    dst
}
