# Cairo-M zkVM - Quick Start Guide

Get started with Cairo-M zkVM in just a few minutes!

## Prerequisites

- Rust 1.85+ installed
- Git
- 4GB+ RAM recommended
- MacOS: Homebrew (for LLVM/LLD)

## Installation

### Step 1: Install Cairo-M Toolchain

```bash
cd scripts/sdk_installers
./install_cairo_m_sdk.sh
```

This will install:
- `cairo-m-compiler` - Compiles .cm files
- `cairo-m-runner` - Executes programs
- `cairo-m-prover` - Generates proofs
- `cargo-cairo-m` - Project scaffolding (optional)

Installation takes ~5-10 minutes.

### Step 2: Verify Installation

```bash
cairo-m-compiler --version
cairo-m-runner --version
cairo-m-prover --version
```

## Running Your First Demo

### Quick Run

```bash
cd cairo-m-zkvm
./run_demo.sh
```

This runs the Fibonacci demo with default input (n=10).

### Custom Input

```bash
FIBONACCI_N=20 ./run_demo.sh
```

### With Detailed Logs

```bash
RUST_LOG=debug FIBONACCI_N=10 ./run_demo.sh
```

## Understanding the Output

```
========================================
  Cairo-M zkVM - Fibonacci Demo
========================================

📊 Configuration:
   Input: n = 10
   Cairo-M Program: programs/fibonacci.cm
   Expected result: fib(10) = 89

🔨 Step 1: Compiling Cairo-M program...
   ✅ Compilation completed in 0.15s

🚀 Step 2: Executing program...
   ✅ Execution completed in 0.02s
   Result: fibonacci(10) = 89

🔐 Step 3: Generating STARK proof...
   ✅ Proof generated in 3.45s
   Proof size: 45.2 KB

✓ Step 4: Verifying proof...
   ✅ Proof verified successfully in 0.08s
```

### What Just Happened?

1. **Compilation**: Your Cairo-M code was compiled to an executable format
2. **Execution**: The program ran and generated an execution trace
3. **Proving**: A STARK proof was generated using the Stwo prover
4. **Verification**: The proof was verified to ensure correctness

## Manual Workflow

You can also run each step manually:

### 1. Compile

```bash
cairo-m-compiler \
  --input programs/fibonacci.cm \
  --output compiled/fibonacci.json
```

### 2. Run

```bash
cairo-m-runner \
  compiled/fibonacci.json \
  --entrypoint fibonacci \
  --arguments 10
```

### 3. Prove

```bash
cairo-m-prover \
  compiled/fibonacci.json \
  --entrypoint fibonacci \
  --arguments 10
```

## Writing Your First Cairo-M Program

### 1. Create a new file: `programs/square.cm`

```cairo-m
// Compute the square of a number
func square(x: felt) -> felt {
    return x * x;
}

func main(input: felt) -> felt {
    let result = square(input);
    return result;
}
```

### 2. Compile it

```bash
cairo-m-compiler \
  --input programs/square.cm \
  --output compiled/square.json
```

### 3. Run it

```bash
cairo-m-runner \
  compiled/square.json \
  --entrypoint main \
  --arguments 5
```

### 4. Prove it

```bash
cairo-m-prover \
  compiled/square.json \
  --entrypoint main \
  --arguments 5
```

## Learning Cairo-M

### CairoMlings Tutorial

Interactive exercises to learn Cairo-M:

```bash
# Install CairoMlings
cargo install --path ~/.cairo-m/tutorials/cairomlings

# Initialize exercises
cairomlings init

# Start learning
cd cairomlings-exercises
cairomlings watch
```

### Example Programs

Check out examples in the Cairo-M repository:

```bash
cd ~/.cairo-m/examples
ls -la
```

## Project Structure

```
cairo-m-zkvm/
├── programs/           # Your Cairo-M source files (.cm)
│   └── fibonacci.cm
├── compiled/           # Compiled programs (JSON)
│   └── fibonacci.json
├── cairo-m-host/       # Host program (Rust)
│   └── src/main.rs
└── run_demo.sh         # Demo runner script
```

## Common Commands

### Run demo with different inputs

```bash
FIBONACCI_N=50 ./run_demo.sh
```

### Build host program

```bash
cd cairo-m-host
cargo build --release
```

### Run with custom environment

```bash
cd cairo-m-host
FIBONACCI_N=15 RUST_LOG=info cargo run --release
```

### Clean and rebuild

```bash
cd cairo-m-host
cargo clean
cargo build --release
```

## Tips & Tricks

### 1. Performance

For best performance, always use `--release`:

```bash
cargo run --release
```

### 2. Debugging

Enable detailed logs:

```bash
RUST_LOG=debug cargo run --release
```

### 3. Multiple Programs

Create multiple `.cm` files in `programs/` directory and compile them separately.

### 4. Benchmarking

Test with increasing input sizes:

```bash
for n in 10 20 50 100 200; do
  echo "Testing n=$n"
  FIBONACCI_N=$n ./run_demo.sh
done
```

## Common Issues

### Issue: "cairo-m-compiler not found"

**Solution**: Install the Cairo-M toolchain:

```bash
cd scripts/sdk_installers
./install_cairo_m_sdk.sh
```

### Issue: Build errors on MacOS

**Solution**: Install LLVM and set environment variables:

```bash
brew install llvm lld

# Add to ~/.zshrc or ~/.bash_profile:
export CC=/opt/homebrew/opt/llvm/bin/clang
export CXX=/opt/homebrew/opt/llvm/bin/clang++
export AR=/opt/homebrew/opt/llvm/bin/llvm-ar
export RANLIB=/opt/homebrew/opt/llvm/bin/llvm-ranlib

# Reload shell
source ~/.zshrc
```

### Issue: Slow proof generation

**Solutions**:
- Use smaller inputs for testing
- Ensure sufficient RAM available
- Close other applications
- Use release builds

### Issue: Compilation errors in Cairo-M code

**Solutions**:
- Check syntax against examples
- Verify type annotations
- Ensure all variables are declared
- Look at error messages carefully

## Next Steps

1. **Read the full README**: See `README.md` for detailed information
2. **Study the PROJECT_OVERVIEW**: Understand Cairo-M architecture
3. **Try CairoMlings**: Interactive tutorial for the language
4. **Explore examples**: Check Cairo-M repository examples
5. **Write your own programs**: Start simple, gradually increase complexity

## Resources

- **Cairo-M GitHub**: https://github.com/kkrt-labs/cairo-m
- **Design Document**: Architecture and design decisions
- **Getting Started Guide**: Official getting started guide
- **Examples**: Example programs and patterns
- **Test Programs**: Test suite for reference

## Getting Help

1. Check the documentation in `README.md`
2. Review error messages carefully
3. Look at example programs
4. Check Cairo-M GitHub issues
5. Enable debug logging for more info

## Performance Expectations

### Small Programs (n < 50)
- Compile: ~100ms
- Execute: ~10ms
- Prove: ~1-2s
- Verify: ~50ms

### Medium Programs (n < 200)
- Compile: ~200ms
- Execute: ~50ms
- Prove: ~3-5s
- Verify: ~80ms

### Large Programs (n < 1000)
- Compile: ~500ms
- Execute: ~200ms
- Prove: ~10-20s
- Verify: ~100ms

## What's Next?

Now that you've run your first Cairo-M program, you can:

1. ✅ Write more complex Cairo-M programs
2. ✅ Experiment with different algorithms
3. ✅ Learn the Cairo-M language through CairoMlings
4. ✅ Benchmark performance on your hardware
5. ✅ Contribute to the Cairo-M project

Happy proving! 🎉

