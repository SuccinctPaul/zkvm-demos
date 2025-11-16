# zkWasm Quick Start Guide

Get started with zkWasm in 5 minutes!

## Prerequisites Check

Run these commands to verify your setup:

```bash
# Check Rust
rustc --version
# Should show: rustc 1.81.0 or later

# Check WASM target
rustup target list | grep wasm32-unknown-unknown
# Should show: wasm32-unknown-unknown (installed)
# If not, run: rustup target add wasm32-unknown-unknown

# Check system dependencies
clang --version
# If missing on macOS: brew install llvm
# If missing on Linux: sudo apt-get install clang lld
```

## Step 1: Install zkWasm CLI (5-15 minutes)

```bash
cd scripts/sdk_installers
./install_zkwasm_sdk.sh
```

This will:
- Clone the zkWasm repository
- Build the `delphinus-cli` tool
- Install it to `~/.local/bin`

**Add to PATH** (if not already):
```bash
# For zsh (macOS default)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc

# For bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

Verify installation:
```bash
delphinus-cli --version
```

## Step 2: Run the Demo (5-10 minutes)

```bash
cd zkwasm-zkvm
./run_demo.sh
```

This will:
1. ✅ Build WASM guest program (~1 min)
2. ✅ Setup zkWasm circuit (~2-3 min)
3. ✅ Generate proof for Fibonacci(10) (~2-3 min)
4. ✅ Verify the proof (~10 sec)

## Step 3: Try Different Inputs

```bash
# Compute Fibonacci(20)
./run_demo.sh 20

# Compute Fibonacci(50) with larger circuit
./run_demo.sh 50 20
```

## Step 4: Understand the Code

### Guest Program (`zkwasm-guest/src/lib.rs`)

```rust
#[no_mangle]
pub extern "C" fn zkmain() -> i64 {
    unsafe {
        // Read public input
        let n = wasm_input(1) as u64;
        
        // Compute
        let result = fibonacci(n);
        
        // Output result
        wasm_output(result as i64);
        
        result as i64
    }
}
```

**Key Points**:
- Entry point must be named `zkmain`
- Use `wasm_input()` to read inputs
- Use `wasm_output()` to write outputs
- Compiled to WASM, not native code

### Host Program (`zkwasm-host/src/main.rs`)

The host program orchestrates the workflow:

```bash
# Individual steps
cd zkwasm-host

cargo run -- build          # Build WASM
cargo run -- setup --k 18   # Setup circuit
cargo run -- prove --n 10   # Generate proof
cargo run -- verify         # Verify proof

# All-in-one
cargo run -- run --n 10 --k 18
```

## Step 5: Modify the Guest Program

Let's change it to compute factorials instead:

```rust
// In zkwasm-guest/src/lib.rs
fn factorial(n: u64) -> u64 {
    if n <= 1 {
        return 1;
    }
    
    let mut result = 1u64;
    for i in 2..=n {
        result = result.wrapping_mul(i);
    }
    result
}

#[no_mangle]
pub extern "C" fn zkmain() -> i64 {
    unsafe {
        let n = wasm_input(1) as u64;
        let result = factorial(n);
        wasm_output(result as i64);
        result as i64
    }
}
```

Rebuild and run:
```bash
cd zkwasm-host
cargo run -- run --n 5
# Should compute 5! = 120
```

## Common Commands Cheatsheet

```bash
# Build only
cargo run -- build

# Setup with different circuit sizes
cargo run -- setup --k 18    # Small (default)
cargo run -- setup --k 20    # Medium
cargo run -- setup --k 22    # Large

# Prove with mock (faster, for testing)
cargo run -- prove --n 10 --mock

# Prove with actual proof generation
cargo run -- prove --n 10

# Verify
cargo run -- verify

# Complete workflow
cargo run -- run --n 20 --k 18
```

## Understanding the Output

After running, you'll see:

```
zkwasm-zkvm/
├── output/
│   ├── guest.wasm       # Compiled WASM binary
│   ├── proof.json       # Zero-knowledge proof (if generated)
│   └── output.json      # Computation outputs
├── params/
│   ├── K18.params       # Circuit parameters
│   └── fib-demo/        # Proving/verifying keys
└── target/              # Rust build artifacts
```

## Next Steps

### 1. Write Your Own Program

Create a new computation:
- Edit `zkwasm-guest/src/lib.rs`
- Keep the `zkmain` entry point
- Use `wasm_input()` and `wasm_output()`
- Rebuild and run

### 2. Handle Multiple Inputs

```rust
#[no_mangle]
pub extern "C" fn zkmain() -> i64 {
    unsafe {
        let a = wasm_input(1);  // Public input 1
        let b = wasm_input(1);  // Public input 2
        let result = a + b;
        wasm_output(result);
        result
    }
}
```

Run with:
```bash
# You'll need to modify the host to pass multiple inputs
# Or use the CLI directly:
delphinus-cli --params params demo prove \
  --wasm output/guest.wasm \
  --output output \
  --public 10:i64 \
  --public 20:i64
```

### 3. Private Inputs

```rust
// Same code, but pass input as private:
delphinus-cli --params params demo prove \
  --wasm output/guest.wasm \
  --output output \
  --private 42:i64    # Hidden in proof
```

### 4. Explore Examples

Check out official examples:
- [C Project Template](https://github.com/DelphinusLab/zkWasm-C)
- [Rust Demo](https://github.com/xgaozoyoe/zkWasm-Rust-Demo)
- [AssemblyScript](https://github.com/DelphinusLab/zkWasm-AssemblyScript-Demo)

## Troubleshooting

### "delphinus-cli not found"
```bash
# Check if installed
ls ~/.local/bin/delphinus-cli

# If not, run installer again
cd scripts/sdk_installers
./install_zkwasm_sdk.sh

# Add to PATH
export PATH="$HOME/.local/bin:$PATH"
```

### "wasm32-unknown-unknown target not found"
```bash
rustup target add wasm32-unknown-unknown
```

### "Setup failed"
```bash
# Install system dependencies
# macOS:
brew install llvm

# Linux:
sudo apt-get install clang lld
```

### "Circuit too small"
```bash
# Increase k parameter
cargo run -- setup --k 20
```

### Proof generation is slow
This is normal! Proof generation takes time:
- First time: 2-5 minutes
- Complex programs: 5-15 minutes
- Use `--mock` for testing without full proof

## Performance Tips

1. **Optimize WASM size** - Smaller WASM = faster proving
   ```toml
   [profile.release]
   opt-level = "z"
   lto = true
   strip = true
   ```

2. **Start with small k** - Use k=18 for development, increase as needed

3. **Use mock mode** - Test logic with `--mock` flag before full proof

4. **Minimize computation** - Simpler programs prove faster

## What's Next?

- 📖 Read [PROJECT_OVERVIEW.md](PROJECT_OVERVIEW.md) for deep dive
- 🔧 Customize the guest program for your use case
- 🌐 Explore browser-based zkWasm applications
- 📝 Read the [zkWasm paper](https://ieeexplore.ieee.org/document/10587123)
- 💬 Join the community on GitHub

## Summary

You've learned:
- ✅ How to install zkWasm CLI
- ✅ How to build WASM guest programs
- ✅ How to generate zero-knowledge proofs
- ✅ How to verify proofs
- ✅ How to modify and extend the demo

**The key workflow**:
```
Rust Code → WASM → zkWasm Setup → Prove → Verify
```

Happy proving! 🚀

