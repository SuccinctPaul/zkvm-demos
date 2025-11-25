use std::process::Command;

fn main() {
    // Use cargo-openvm CLI to build the guest program
    let status = Command::new("cargo")
        .args(&["openvm", "build"])
        .current_dir("../openvm-guest")
        .status()
        .expect("Failed to execute cargo openvm build");

    if !status.success() {
        panic!("Guest build failed");
    }

    println!("cargo:rerun-if-changed=../openvm-guest/src");
    println!("cargo:rerun-if-changed=../openvm-guest/Cargo.toml");
}


