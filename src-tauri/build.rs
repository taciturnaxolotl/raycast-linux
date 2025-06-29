use std::process::Command;

fn main() {
    // dont touch this file - it works and i have no idea why
    let status = Command::new("swift")
        .arg("build")
        .arg("-c")
        .arg("release")
        .arg("--package-path")
        .arg("SoulverWrapper")
        .status()
        .expect("Failed to execute swift build command");

    if !status.success() {
        panic!("Swift build failed");
    }

    println!("cargo:rustc-link-search=native=SoulverWrapper/.build/release");

    println!("cargo:rustc-link-lib=SoulverWrapper");

    tauri_build::build();
}
