# snarkVM Demo - Testing Guide

## Overview

This guide explains how to test and verify the snarkVM demo works correctly.

## Prerequisites Check

Before testing, ensure you have:

```bash
# Check Rust version (should be 1.85+)
rustc --version

# If not installed or wrong version
rustup install 1.85
rustup default 1.85
```

## Testing Methods

### Method 1: Quick Test (Recommended)

```bash
cd /Users/paul/zkp/zkvms/zkvm-demos/snarkvm-zkvm
./run_demo.sh
```

This will:
1. Build the project (first time takes 5-10 minutes)
2. Run the demo
3. Show output with field arithmetic and curve operations

### Method 2: Manual Build and Run

```bash
cd /Users/paul/zkp/zkvms/zkvm-demos/snarkvm-zkvm/snarkvm-host

# Build (release mode for better performance)
cargo build --release

# Run
cargo run --release
```

### Method 3: Compilation Check Only

```bash
cd /Users/paul/zkp/zkvms/zkvm-demos/snarkvm-zkvm/snarkvm-host

# Just check if it compiles (faster)
cargo check
```

## Expected Build Time

### First Build
- **Time**: 5-10 minutes
- **Reason**: Downloads and compiles ~360 dependencies
- **Normal**: This is expected for cryptography libraries

### Subsequent Builds
- **Time**: < 30 seconds
- **Reason**: Dependencies already compiled

## Expected Output

### Successful Compilation

```
   Compiling snarkvm-host v0.1.0
    Finished release [optimized] target(s) in X.XXs
```

### Successful Execution

```
========================================
snarkVM Demo - Fibonacci Computation
========================================

📊 Computing fibonacci(10)...

1️⃣  Computing Fibonacci natively...
   ✓ Computation completed in 0.00s
   ✓ Result: fibonacci(10) = 55

2️⃣  Demonstrating snarkVM field arithmetic...
   • Field element a = 1
   • Field element b = 2
   • Field element c = 3
   • a + b = 3
   • b * c = 6
   • c - a = 2
   • b^(-1) exists (field inversion)
   • b * b^(-1) = 1 (should be 1)
   ✓ Field operations completed in 0.01s

3️⃣  Demonstrating snarkVM curve operations...
   • Generator point G (base point on curve)
   • Computed 2G (point doubling)
   • Computed 3G (point addition)
   • Verified: G + G + G = 3G ✓
   • Computed 5G (scalar multiplication)
   • Verified: 5 * G = G + G + G + G + G ✓
   ✓ Curve operations completed in 0.02s

========================================
📈 Performance Summary
========================================
Fibonacci computation: 0.00s
Field arithmetic:      0.01s
Curve operations:      0.02s
Total time:            0.03s
========================================
✅ snarkVM Demo completed successfully!

💡 About snarkVM:
   snarkVM is the virtual machine powering Aleo blockchain
   This demo showcases basic cryptographic primitives
   For full zkVM features, use Leo programming language

💡 Next Steps:
   - Install Leo: https://leo-lang.org/
   - Write Aleo programs in Leo language
   - Deploy to Aleo testnet

💡 Tip: Try running with different FIBONACCI_N values!
   Example: FIBONACCI_N=20 cargo run --release
```

## Verification Checklist

### ✅ Compilation
- [ ] Code compiles without errors
- [ ] No warnings (or only expected warnings)
- [ ] Dependencies download successfully

### ✅ Execution
- [ ] Program runs without crashing
- [ ] Fibonacci result is correct (fib(10) = 55)
- [ ] Field arithmetic operations complete
- [ ] Curve operations complete
- [ ] All verifications pass (✓ symbols shown)

### ✅ Output
- [ ] Colored output displays correctly
- [ ] Performance metrics shown
- [ ] All sections complete (1️⃣ 2️⃣ 3️⃣)
- [ ] Success message at end

## Testing Different Inputs

Try various Fibonacci numbers:

```bash
# Small value (fast)
FIBONACCI_N=5 cargo run --release

# Medium value
FIBONACCI_N=15 cargo run --release

# Larger value
FIBONACCI_N=25 cargo run --release

# Very large (will overflow but demonstrates wrapping)
FIBONACCI_N=100 cargo run --release
```

## Common Issues and Solutions

### Issue 1: Rust Not Installed

**Error**: `command not found: cargo`

**Solution**:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Issue 2: Wrong Rust Version

**Error**: Compilation errors about features

**Solution**:
```bash
rustup install 1.85
rustup default 1.85
cd snarkvm-zkvm/snarkvm-host
cargo clean
cargo build --release
```

### Issue 3: Network Errors During Download

**Error**: `spurious network error` or timeout

**Solution**:
```bash
# Retry - it usually works on second try
cargo build --release

# Or increase timeout
export CARGO_HTTP_TIMEOUT=300
cargo build --release
```

### Issue 4: Out of Disk Space

**Error**: `No space left on device`

**Solution**:
```bash
# Clean cargo cache
cargo clean

# Remove old build artifacts
cd target && rm -rf debug/ release/

# Try again
cargo build --release
```

### Issue 5: Build Takes Very Long

**Not an error** - First build can take 5-10 minutes.

**Optimization**:
```bash
# Use all CPU cores
cargo build --release -j $(nproc)
```

## Performance Benchmarks

Expected performance on modern hardware:

| Operation | Time | Notes |
|-----------|------|-------|
| Fibonacci (n=10) | < 0.001s | Native Rust |
| Field arithmetic | 0.01-0.02s | Batch of operations |
| Curve operations | 0.02-0.05s | Multiple point ops |
| **Total** | **0.03-0.08s** | Complete demo |

If significantly slower, check:
- Using `--release` flag (debug is 10-100x slower)
- CPU governor settings (performance vs powersave)
- Other background processes

## Automated Testing Script

Save as `test_all.sh`:

```bash
#!/bin/bash
set -e

echo "Testing snarkVM Demo..."
echo ""

cd snarkvm-zkvm/snarkvm-host

echo "1. Checking compilation..."
if cargo check --release 2>&1 | grep -q "Finished"; then
    echo "   ✅ Compilation check passed"
else
    echo "   ❌ Compilation check failed"
    exit 1
fi

echo ""
echo "2. Testing with FIBONACCI_N=5..."
if FIBONACCI_N=5 cargo run --release 2>&1 | grep -q "completed successfully"; then
    echo "   ✅ Test passed"
else
    echo "   ❌ Test failed"
    exit 1
fi

echo ""
echo "3. Testing with FIBONACCI_N=15..."
if FIBONACCI_N=15 cargo run --release 2>&1 | grep -q "completed successfully"; then
    echo "   ✅ Test passed"
else
    echo "   ❌ Test failed"
    exit 1
fi

echo ""
echo "✅ All tests passed!"
```

## CI/CD Testing

For automated testing:

```yaml
# .github/workflows/test-snarkvm.yml
name: Test snarkVM Demo

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: 1.85
      - name: Build
        run: cd snarkvm-zkvm/snarkvm-host && cargo build --release
      - name: Test
        run: cd snarkvm-zkvm/snarkvm-host && cargo run --release
```

## Success Criteria

The demo is working correctly if:

1. ✅ Compiles without errors
2. ✅ Runs to completion
3. ✅ Outputs correct Fibonacci result
4. ✅ All cryptographic operations complete
5. ✅ Verifications pass (✓ symbols shown)
6. ✅ Performance metrics reasonable
7. ✅ No panics or crashes

## Reporting Issues

If you encounter problems:

1. **Check this guide** for common solutions
2. **Verify prerequisites** (Rust version, etc.)
3. **Try clean build**: `cargo clean && cargo build --release`
4. **Check logs**: Look for specific error messages
5. **Report**: Create issue with:
   - Rust version (`rustc --version`)
   - OS and version
   - Full error message
   - Steps to reproduce

## Conclusion

This testing guide ensures the snarkVM demo works correctly. The demo has been designed to:

- ✅ Compile cleanly with Rust 1.85+
- ✅ Run without errors
- ✅ Demonstrate snarkVM cryptographic primitives
- ✅ Provide educational value
- ✅ Serve as foundation for learning

If all tests pass, you're ready to explore snarkVM and Aleo! 🚀

