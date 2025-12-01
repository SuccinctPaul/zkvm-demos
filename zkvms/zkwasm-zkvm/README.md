# zkWasm Demo - Fibonacci Computation

This demo showcases how to use [zkWasm](https://github.com/DelphinusLab/zkWasm) to generate zero-knowledge proofs for WebAssembly programs.

## What is zkWasm?

zkWasm is a ZKSNARK-backed virtual machine that executes WebAssembly bytecode. It allows existing WASM applications to leverage zero-knowledge proofs without any modification to the source code. The key benefits are:

- **Platform Independence**: Compile from C, C++, Rust, AssemblyScript, and more
- **No Code Changes**: Existing WASM programs work without modification
- **Privacy**: Prove computation correctness without revealing inputs
- **Verifiability**: Anyone can verify the computation was performed correctly

## Demo Overview

This demo implements a simple Fibonacci number calculator:
- **Guest Program** (`zkwasm-guest`): Compiled to WASM, computes Fibonacci numbers
- **Host Program** (`zkwasm-host`): Orchestrates proof generation and verification

## Architecture

```
┌─────────────────────┐
│   Rust Guest Code   │  (src/lib.rs)
└──────────┬──────────┘
           │ compile to wasm32-unknown-unknown
           ▼
┌─────────────────────┐
│   WASM Binary       │  (guest.wasm)
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│   zkWasm Prover     │  generates zkSNARK proof
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│   Proof + Output    │  verifiable by anyone
└─────────────────────┘
```

## Prerequisites

1. **Rust toolchain** (version 1.81.0)
   ```bash
   rustup install 1.81.0
   rustup target add wasm32-unknown-unknown
   ```

2. **zkWasm CLI** (delphinus-cli)
   
   Install using the provided script:
   ```bash
   cd scripts/sdk_installers
   ./install_zkwasm_sdk.sh
   ```
   
   Or install manually:
   ```bash
   git clone --recurse-submodules https://github.com/DelphinusLab/zkwasm
   cd zkwasm
   cargo build --release
   # Add target/release/delphinus-cli to your PATH
   ```

3. **Required system packages**
   ```bash
   # macOS
   brew install llvm

   # Ubuntu/Debian
   sudo apt-get install clang lld
   ```

## Quick Start

### Option 1: Run Everything (Recommended for First Time)

```bash
./run_demo.sh
```

This script will:
1. Build the WASM guest program
2. Setup the zkWasm circuit
3. Generate a proof for Fibonacci(10)
4. Verify the proof

### Option 2: Step-by-Step Execution

```bash
# 1. Build the WASM guest program
cd zkwasm-host
cargo run -- build

# 2. Setup the circuit (one-time setup)
cargo run -- setup --k 18

# 3. Generate a proof for Fibonacci(20)
cargo run -- prove --n 20

# 4. Verify the proof
cargo run -- verify
```

### Option 3: All-in-One Command

```bash
cd zkwasm-host
cargo run -- run --n 15 --k 18
```

## CLI Options

The host program supports the following commands:

```bash
# Build WASM
cargo run -- build

# Setup circuit with custom k value
cargo run -- setup --k 20

# Prove with custom input
cargo run -- prove --n 30

# Prove with mock test (faster, no actual proof)
cargo run -- prove --n 30 --mock

# Verify proof
cargo run -- verify

# Run complete workflow
cargo run -- run --n 25 --k 18
```

## Guest Program Details

The guest program (`zkwasm-guest/src/lib.rs`) must follow these requirements:

1. **Entry Point**: Must have a function named `zkmain`
   ```rust
   #[no_mangle]
   pub extern "C" fn zkmain() -> i64 { ... }
   ```

2. **Host Functions**: Can call zkWasm-provided functions:
   ```rust
   extern "C" {
       fn wasm_input(is_public: i32) -> i64;  // Read input
       fn wasm_output(value: i64);             // Write output
   }
   ```

3. **Input Types**: Supported formats:
   - `value:i64` - 64-bit integer
   - `value:bytes` - byte array
   - `value:bytes-packed` - packed byte array

## Project Structure

```
zkwasm-zkvm/
├── Cargo.toml                 # Workspace configuration
├── rust-toolchain.toml        # Rust version pinning
├── README.md                  # This file
├── run_demo.sh               # Quick start script
├── zkwasm-guest/             # Guest program (compiles to WASM)
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs            # Fibonacci implementation
└── zkwasm-host/              # Host program (orchestration)
    ├── Cargo.toml
    └── src/
        └── main.rs           # CLI for setup/prove/verify
```

## Understanding the Output

After running the demo, you'll see several directories:

- `output/` - Contains the WASM binary and generated proofs
- `params/` - Contains the circuit parameters and setup files
- `target/` - Rust build artifacts

## Customizing the Guest Program

To compute something other than Fibonacci:

1. Edit `zkwasm-guest/src/lib.rs`
2. Keep the `zkmain` entry point
3. Use `wasm_input()` to read inputs
4. Use `wasm_output()` to write outputs
5. Rebuild and re-run

Example template:
```rust
#[no_mangle]
pub extern "C" fn zkmain() -> i64 {
    unsafe {
        let input = wasm_input(1);  // 1 = public
        
        // Your computation here
        let result = my_computation(input);
        
        wasm_output(result);
        result
    }
}
```

## Performance Considerations

- **Circuit Size (k)**: Larger k values support more complex computations but take longer
  - k=18: Good for simple programs (default)
  - k=20: Medium complexity
  - k=22: Complex programs

- **Optimization**: The guest is built with aggressive optimizations:
  - `opt-level = "z"` - Optimize for size
  - `lto = true` - Link-time optimization
  - `strip = true` - Remove debug symbols

## Troubleshooting

### "zkWasm CLI not found"
Install the CLI using the provided script or manually (see Prerequisites)

### "Failed to build WASM"
Ensure you have the wasm32-unknown-unknown target:
```bash
rustup target add wasm32-unknown-unknown
```

### "Setup failed"
Make sure you have clang and lld installed:
```bash
# macOS
brew install llvm

# Linux
sudo apt-get install clang lld
```

### Circuit too small
If the computation is too complex, increase k:
```bash
cargo run -- setup --k 20
```

## Further Resources

- [zkWasm GitHub](https://github.com/DelphinusLab/zkWasm)
- [zkWasm Paper](https://ieeexplore.ieee.org/document/10587123)
- [C Project Template](https://github.com/DelphinusLab/zkWasm-C)
- [Rust Demo](https://github.com/xgaozoyoe/zkWasm-Rust-Demo)
- [AssemblyScript Demo](https://github.com/DelphinusLab/zkWasm-AssemblyScript-Demo)

## Comparison with Other zkVMs

| Feature | zkWasm | RISC0 | SP1 |
|---------|--------|-------|-----|
| **Target** | WebAssembly | RISC-V | RISC-V |
| **Languages** | Any → WASM | Rust | Rust, C, C++ |
| **Web Compatible** | ✅ Excellent | ⚠️ Limited | ⚠️ Limited |
| **Maturity** | 🟡 Moderate | 🟢 High | 🟢 High |

## License

This demo is provided as-is for educational purposes.

