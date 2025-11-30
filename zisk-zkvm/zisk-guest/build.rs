use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Create build directory in zisk-guest (avoid path traversal issues)
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = manifest_dir.join("build");
    fs::create_dir_all(&out_dir).expect("Failed to create build directory");
    
    // Also create in parent for backward compatibility
    let parent_build = manifest_dir.parent().unwrap().join("build");
    fs::create_dir_all(&parent_build).ok();

    // Get program ID from environment (default: 0 = Fibonacci)
    let program_id: u32 = env::var("PROGRAM_ID")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);

    // Get program input N from environment
    // Support both PROGRAM_N (new) and FIBONACCI_N (legacy) for backward compatibility
    let n: u32 = env::var("PROGRAM_N")
        .ok()
        .and_then(|v| v.parse().ok())
        .or_else(|| {
            env::var("FIBONACCI_N")
                .ok()
                .and_then(|v| v.parse().ok())
        })
        .unwrap_or(10);

    // Prepare input data: two u32 values [program_id, n]
    let mut input_data = Vec::new();
    input_data.extend_from_slice(&program_id.to_le_bytes());
    input_data.extend_from_slice(&n.to_le_bytes());

    // Write input.bin to zisk-guest/build/ (primary location)
    let input_path = out_dir.join("input.bin");
    fs::write(&input_path, &input_data).expect("Failed to create input.bin");
    
    // Also write to parent/build/ for backward compatibility
    let parent_input = parent_build.join("input.bin");
    fs::write(&parent_input, &input_data).ok();

    println!("cargo:rerun-if-env-changed=PROGRAM_ID");
    println!("cargo:rerun-if-env-changed=PROGRAM_N");
    println!("cargo:rerun-if-env-changed=FIBONACCI_N");
    println!("Generated input.bin with program_id = {}, n = {}", program_id, n);
}

