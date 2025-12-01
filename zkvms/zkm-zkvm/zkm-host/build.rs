use zkm_build::build_program_with_args;
use std::path::Path;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let manifest_path = Path::new(&manifest_dir);
    let guest_path = manifest_path.parent().expect("Failed to get parent").join("zkm-guest");
    build_program_with_args(guest_path.to_str().expect("Path not utf8"), Default::default())
}
