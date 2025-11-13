use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Create build directory
    let out_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .parent()
        .unwrap()
        .join("build");
    fs::create_dir_all(&out_dir).expect("Failed to create build directory");

    // Get Fibonacci N from environment or use default
    let fib_n: u32 = env::var("FIBONACCI_N")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(10);

    // Write input.bin file
    let input_path = out_dir.join("input.bin");
    let mut file = fs::File::create(&input_path).expect("Failed to create input.bin");
    file.write_all(&fib_n.to_le_bytes())
        .expect("Failed to write to input.bin");

    println!("cargo:rerun-if-env-changed=FIBONACCI_N");
    println!("Generated input.bin with n = {}", fib_n);
}

