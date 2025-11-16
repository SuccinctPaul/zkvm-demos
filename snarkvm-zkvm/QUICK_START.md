# snarkVM Demo - Quick Start Guide

Get up and running with snarkVM in minutes!

## Prerequisites

- **macOS** or **Linux** (Windows via WSL)
- **Rust 1.85+** (will be installed by script if needed)

## Installation & Run (3 Steps)

### Step 1: Install SDK

```bash
cd scripts/sdk_installers
./install_snarkvm_sdk.sh
```

This installs:
- Rust toolchain (1.85)
- Build dependencies
- snarkVM library

**Time**: ~5-10 minutes (first time)

### Step 2: Navigate to Demo

```bash
cd ../../snarkvm-zkvm
```

### Step 3: Run Demo

```bash
./run_demo.sh
```

Or manually:
```bash
cd snarkvm-host
cargo run --release
```

**Time**: ~2-5 minutes (first build)

## Expected Output

```
========================================
snarkVM Demo - Fibonacci Computation
========================================

📊 Computing fibonacci(10)...

1️⃣  Initializing constraint system...
   ✓ Initialization completed in 0.05s

2️⃣  Building circuit...
   ✓ Circuit built in 0.12s
   ✓ Result: fibonacci(10) = 55

3️⃣  Analyzing circuit...
   ✓ Analysis completed in 0.01s
   📈 Circuit Statistics:
      - Public inputs:    1
      - Private inputs:   20
      - Constraints:      100

4️⃣  Verifying circuit satisfaction...
   ✓ Circuit is satisfied in 0.03s

========================================
📈 Performance Summary
========================================
Initialize time:  0.05s
Setup time:       0.12s
Analyze time:     0.01s
Verify time:      0.03s
Total time:       0.21s
========================================
✅ snarkVM Demo completed successfully!
```

## Try Different Values

```bash
# Small computation
FIBONACCI_N=5 cargo run --release

# Medium computation
FIBONACCI_N=15 cargo run --release

# Larger circuit
FIBONACCI_N=25 cargo run --release
```

## What's Happening?

1. **Circuit Creation**: Builds R1CS constraints for Fibonacci
2. **Constraint Generation**: Creates mathematical constraints
3. **Satisfaction Check**: Verifies all constraints are met
4. **Metrics**: Shows circuit size and performance

## Understanding the Circuit

The demo computes Fibonacci in zero-knowledge:

```rust
// Initialize (private values)
let mut prev = U64::<A>::new(Mode::Private, 0);
let mut curr = U64::<A>::new(Mode::Private, 1);

// Compute with constraints
for i in 0..n {
    let temp = curr.clone();
    curr = prev.add(&curr);  // Generates R1CS constraints
    prev = temp;
}
```

Each operation creates constraints that prove correctness without revealing intermediate values.

## Next Steps

### 1. Explore the Code
```bash
# View the main implementation
cat snarkvm-host/src/main.rs

# Check Aleo program example
cat programs/fibonacci.aleo
```

### 2. Modify the Circuit

Try computing other functions:
- Sum of squares
- Prime checking
- Hash functions

### 3. Learn More

- Read `PROJECT_OVERVIEW.md` for technical details
- Read `README.md` for comprehensive documentation
- Visit [snarkVM GitHub](https://github.com/ProvableHQ/snarkVM)
- Explore [Aleo Documentation](https://developer.aleo.org/)

## Common Issues

### Build Fails

```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

### Out of Memory

For large inputs:
```bash
RUST_MIN_STACK=8388608 cargo run --release
```

### Slow First Build

First build takes 5-10 minutes as it compiles snarkVM and dependencies. Subsequent builds are much faster.

## Performance Tips

- Use `--release` flag (10-100x faster)
- Start with small values (n < 20)
- Increase gradually to understand scaling

## Architecture Overview

```
┌─────────────────────────────────────┐
│         snarkVM Circuit             │
│                                     │
│  ┌────────────────────────────┐    │
│  │  Fibonacci Computation     │    │
│  │  (Zero-Knowledge)          │    │
│  └────────────────────────────┘    │
│               ↓                     │
│  ┌────────────────────────────┐    │
│  │  R1CS Constraints          │    │
│  │  (Mathematical Proof)      │    │
│  └────────────────────────────┘    │
│               ↓                     │
│  ┌────────────────────────────┐    │
│  │  Satisfaction Check        │    │
│  │  (Verification)            │    │
│  └────────────────────────────┘    │
└─────────────────────────────────────┘
```

## Resources

- **This Demo**: Introduction to snarkVM circuits
- **snarkVM**: https://github.com/ProvableHQ/snarkVM
- **Aleo**: https://aleo.org/
- **Leo Language**: https://leo-lang.org/
- **Discord**: https://discord.gg/aleo

## Help

If you encounter issues:

1. Check `README.md` troubleshooting section
2. Ensure Rust 1.85+ is installed: `rustc --version`
3. Try clean build: `cargo clean && cargo build --release`
4. Visit [snarkVM Issues](https://github.com/ProvableHQ/snarkVM/issues)

## Success! What Now?

✅ You've successfully run a zero-knowledge computation on snarkVM!

Continue learning:
- Understand R1CS constraint systems
- Learn Leo programming language
- Build private applications on Aleo
- Explore other zkVM demos in this repository

Happy coding! 🚀

