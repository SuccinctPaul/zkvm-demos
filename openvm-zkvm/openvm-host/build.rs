use std::process::Command;
use std::path::Path;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let manifest_path = Path::new(&manifest_dir);
    let guest_path = manifest_path.parent().expect("Failed to get parent").join("openvm-guest");

    // Use cargo-openvm CLI to build the guest program
    let status = Command::new("cargo")
        .args(&["openvm", "build"])
        .current_dir(&guest_path)
        .status()
        .expect("Failed to execute cargo openvm build");

    if !status.success() {
        panic!("Guest build failed");
    }

    println!("cargo:rerun-if-changed={}", guest_path.join("src").display());
    println!("cargo:rerun-if-changed={}", guest_path.join("Cargo.toml").display());
}


