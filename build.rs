use std::os::unix::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Profile {
    Release,
    Debug,
}

impl Profile {
    fn from_str(profile: &str) -> Option<Self> {
        if profile == "release" {
            Some(Self::Release)
        } else if profile == "debug" {
            Some(Self::Debug)
        } else {
            None
        }
    }

    fn for_cmake(&self) -> &str {
        match self {
            Profile::Release => "Release",
            Profile::Debug => "Debug",
        }
    }
}

fn main() {
    let profile = Profile::from_str(&std::env::var("PROFILE").expect("profile")).expect("profile");
    let quibble = build_quibble(profile);

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
    println!("cargo:rustc-link-lib=static=harfbuzz");
    println!("cargo:rustc-link-lib=static=freetype");
}

fn build_quibble(profile: Profile) -> std::path::PathBuf {
    let dst = cmake::Config::new(".")
        .profile(profile.for_cmake())
        .define("CMAKE_EXPORT_COMPILE_COMMANDS", "1")
        .define("CMAKE_SYSTEM_PROCESSOR", "AMD64")
        .define("CMAKE_EXE_LINKER_FLAGS", "-static")
        .build();

    let build_dir = dst.join("build");

    _ = std::fs::copy(
        build_dir.join("ntfs").join("ntfs.efi"),
        "qemu/esp/EFI/BOOT/drivers/ntfs.efi",
    );
    _ = std::fs::copy(
        build_dir.join("btrfs").join("btrfs.efi"),
        "qemu/esp/EFI/BOOT/drivers/btrfs.efi",
    );

    _ = fs::symlink(
        build_dir.join("compile_commands.json"),
        "compile_commands.json",
    );
    dst
}
