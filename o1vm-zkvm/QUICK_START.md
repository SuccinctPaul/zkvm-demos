# o1vm Quick Start Guide

Get up and running with o1vm in minutes!

## Prerequisites

### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install 1.75
```

### 2. Install MIPS Cross-Compiler (Optional for demo)

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install gcc-mips-linux-gnu binutils-mips-linux-gnu
```

**macOS:**
Use Docker as MIPS toolchain is not readily available.

## Quick Build and Run

### Option 1: Run Demo (Simplest)

```bash
cd o1vm-zkvm
./run_demo.sh
```

This script will:
1. Attempt to build MIPS guest program (if toolchain available)
2. Build Rust host program
3. Run the demo

### Option 2: Manual Build

```bash
# Build host program
cd o1vm-host
cargo build --release

# Run demo
cargo run --release
```

### Option 3: With MIPS Guest Program

```bash
# Compile MIPS guest program
cd o1vm-guest
make

# Build and run host
cd ../o1vm-host
cargo build --release
cargo run --release
```

## What You'll See

The demo will output:
1. System information about o1vm
2. MIPS binary status
3. Conceptual workflow demonstration
4. Resources and documentation links

## Expected Output

```
========================================
o1vm zkVM Demo - MIPS Program Proving
========================================

📋 System Information:
   • Architecture: MIPS32
   • Proof System: Kimchi (based on PLONK)
   • Backend: Mina curves (Pallas/Vesta)

✅ Found MIPS guest binary: ../o1vm-guest/fibonacci.elf

🔄 Demo Workflow:

1️⃣  Loading MIPS binary...
   ✓ Binary loaded: XXXX bytes
   ✓ Load time: X.XXms

2️⃣  Executing MIPS program and generating trace...
   • Simulating MIPS32 instruction execution
   • Collecting execution trace for proof generation
   • Computing fibonacci(10)...
   ✓ Execution completed
   ✓ Result: fibonacci(10) = 55

...
```

## Troubleshooting

### Issue: MIPS binary not found

**Solution**: The demo works without the compiled MIPS binary. It will show instructions on how to compile if needed.

### Issue: Cargo build errors

**Solution**: 
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

### Issue: Missing system dependencies

**Solution** (Ubuntu/Debian):
```bash
sudo apt-get install build-essential pkg-config libssl-dev
```

## Next Steps

1. **Explore the code**: Check out `o1vm-host/src/main.rs` and `o1vm-guest/fibonacci.c`
2. **Read the README**: See `README.md` for detailed information
3. **Modify guest program**: Edit `fibonacci.c` and recompile
4. **Learn Kimchi**: Visit https://o1-labs.github.io/proof-systems/

## Using Docker for MIPS Compilation

If you don't have MIPS toolchain:

```bash
docker run --rm -v $(pwd):/work -w /work ubuntu:22.04 bash -c "
    apt-get update && 
    apt-get install -y gcc-mips-linux-gnu make && 
    cd o1vm-guest && 
    make
"
```

## Configuration

The demo uses default values. To customize:

```bash
# Set log level
RUST_LOG=debug cargo run --release

# Use different Fibonacci number (would need code modification)
# Edit o1vm-guest/fibonacci.c and change the value
```

## Common Commands

```bash
# Clean build
cargo clean

# Build with verbose output
cargo build --release --verbose

# Run with logging
RUST_LOG=info cargo run --release

# Check code
cargo check

# Format code
cargo fmt

# Run clippy
cargo clippy
```

## Performance Tips

1. **Always use `--release`**: Debug builds are significantly slower
2. **Optimize MIPS code**: Use `-O2` or `-O3` flags
3. **System resources**: More RAM and CPU cores help with proof generation

## Learn More

- **Full README**: `README.md` - Comprehensive documentation
- **Project Overview**: `PROJECT_OVERVIEW.md` - Technical details
- **Official Docs**: https://o1-labs.github.io/proof-systems/
- **Source Code**: https://github.com/o1-labs/proof-systems/tree/master/o1vm

## Getting Help

- **GitHub Issues**: Report bugs or ask questions
- **Mina Discord**: Join the community at https://discord.gg/minaprotocol
- **Forum**: https://forums.minaprotocol.com/

---

**Note**: This demo provides a structural overview of o1vm. Full proof generation requires integration with the complete o1vm and Kimchi systems from the proof-systems repository.

