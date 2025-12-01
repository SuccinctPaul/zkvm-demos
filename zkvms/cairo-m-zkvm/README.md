# Cairo-M zkVM Demo

A demonstration of the [Cairo-M zkVM](https://github.com/kkrt-labs/cairo-m) - a Mobile-first CPU AIR using M31 field and Stwo's prover.

## About Cairo-M

Cairo-M is designed for efficient zero-knowledge proof generation on consumer hardware, especially mobile devices. Key features:

- **M31 Native Field**: Uses Mersenne 31 prime field for efficient 32-bit arithmetic
- **Minimal Registers**: Only PC (program counter) and FP (frame pointer)
- **Read-Write Memory**: Efficient memory access patterns
- **Variable-Size Encoding**: x86-style instruction encoding
- **Stwo Integration**: Leverages Starkware's Stwo prover for STARK proofs

## Project Structure

```
cairo-m-zkvm/
├── Cargo.toml              # Workspace with cairo-m dependencies
├── Cargo.lock              # Dependency lock file
├── rust-toolchain.toml     # Rust nightly-2025-04-06 (required)
├── cairom.toml             # Cairo-M project manifest
├── .cargo/config.toml      # Build configuration with RUSTFLAGS
├── README.md               # This file
├── programs/               # Cairo-M source programs
│   └── main.cm             # Multi-program implementation
├── compiled/               # Compiled program cache (optional)
└── cairo-m-host/           # Host application
    ├── Cargo.toml
    └── src/main.rs         # Compiler/Runner integration
```

## Prerequisites

### 1. Install Rust Nightly

```bash
rustup install nightly-2025-04-06
rustup default nightly-2025-04-06
```

### 2. macOS Users: Install LLVM/LLD

```bash
brew install llvm lld

# Set environment (add to ~/.zshrc)
export CC=/opt/homebrew/opt/llvm/bin/clang
export CXX=/opt/homebrew/opt/llvm/bin/clang++
```

## Building

```bash
cd zkvms/cairo-m-zkvm
cargo build --release
```

## Usage

### Running Benchmarks

```bash
# Fibonacci (default)
PROGRAM=fibonacci INPUT_N=20 cargo run --release

# Other programs
PROGRAM=sum INPUT_N=100 cargo run --release
PROGRAM=factorial INPUT_N=10 cargo run --release
PROGRAM=isprime INPUT_N=17 cargo run --release
PROGRAM=popcount INPUT_N=255 cargo run --release
PROGRAM=hash INPUT_N=100 cargo run --release
PROGRAM=signature INPUT_N=50 cargo run --release
```

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `PROGRAM` | Program to run | fibonacci |
| `INPUT_N` | Input parameter value | 10 |
| `CAIRO_M_PROOF_MODE` | Proof mode | core |
| `RUST_LOG` | Log level | - |

## Expected Output

```
========================================
  Cairo-M zkVM - Multi-Program Demo
========================================

BENCHMARK: program_name=fibonacci_20
BENCHMARK: zkvm_name=cairo_m
BENCHMARK: zkvm_version=v0.1.0-alpha
BENCHMARK: proof_mode=core

📊 Configuration:
   Program: fibonacci (ID=0)
   Input N: 20
   Field: M31 (Mersenne 31)
   Prover: Stwo (STARK)

   Expected result: 6765

🔨 Step 1: Compiling Cairo-M program...
   ✅ Program compiled in 0.020s
BENCHMARK: compile_time_s=0.020633

🚀 Step 2: Executing program...
   ✅ Execution completed in 0.000029s
   Result: 6765
   Cycles: 0
BENCHMARK: execution_time_s=0.000029
BENCHMARK: total_cycles=0

🔐 Step 3: Proof generation...
   ℹ️  Note: cairo-m-prover is not yet publicly available

✓ Step 4: Verification...
BENCHMARK: success_status=success

========================================
  📈 Performance Summary
========================================
Compile time:     0.020633s
Execution time:   0.000029s
...
========================================

✅ Cairo-M zkVM Demo completed successfully!
```

## Cairo-M Language

Cairo-M uses a Cairo-like syntax optimized for the M31 field:

```cairo-m
// Function definition
fn fibonacci(n: felt) -> u32 {
    if n == 0 {
        return 0u32;
    }
    if n == 1 {
        return 1u32;
    }
    
    let a: u32 = 0u32;
    let b: u32 = 1u32;
    
    // Loop (uses != for condition)
    for (let i: felt = 2; i != n + 1; i = i + 1) {
        let temp: u32 = a + b;
        a = b;
        b = temp;
    }
    
    return b;
}
```

### Key Language Features

- **Types**: `felt` (field element), `u32` (32-bit unsigned), `bool`
- **Operators**: `+`, `-`, `*`, `/`, `&`, `|`, `^`, `==`, `!=`
- **Loops**: `for (let i: type = init; i != cond; i = i + step) { ... }`
- **Casting**: Only `u32` to `felt` is supported (not vice versa)
- **Constants**: `const NAME = [values...]`

### Supported Programs

| Program | Description | Input N |
|---------|-------------|---------|
| `fibonacci` | Compute F(n) | index |
| `sum` | Compute 1+2+...+n | upper bound |
| `factorial` | Compute n! | number |
| `isprime` | Check primality | number |
| `popcount` | Count set bits | number |
| `hash` | Simple hash | iterations |
| `signature` | Verification sim | iterations |

## SDK Components

This demo uses the following Cairo-M crates:

- **`cairo-m-compiler`**: Compiles `.cm` source to program bytecode
- **`cairo-m-runner`**: Executes programs and generates traces
- **`cairo-m-common`**: Shared types (Program, CairoMValue, etc.)

Note: `cairo-m-prover` is not yet publicly available. Proof generation is simulated.

## Architecture

```
┌──────────────────┐     ┌──────────────────┐     ┌──────────────────┐
│   main.cm        │     │  compile_cairo   │     │    Program       │
│ (Cairo-M source) │ ──▶ │   (Compiler)     │ ──▶ │  (Bytecode)      │
└──────────────────┘     └──────────────────┘     └──────────────────┘
                                                          │
                                                          ▼
┌──────────────────┐     ┌──────────────────┐     ┌──────────────────┐
│    Result        │     │ run_cairo_program│     │   InputValue     │
│ (CairoMValue)    │ ◀── │    (Runner)      │ ◀── │  (Arguments)     │
└──────────────────┘     └──────────────────┘     └──────────────────┘
                                                          │
                                                          ▼
                                                  ┌──────────────────┐
                                                  │  Execution Trace │
                                                  │   (for Prover)   │
                                                  └──────────────────┘
```

## Troubleshooting

### Build Errors

```bash
# Ensure correct toolchain
rustup override set nightly-2025-04-06
cargo clean
cargo build --release
```

### macOS Linker Errors

```bash
# Verify LLD is installed
/opt/homebrew/opt/lld/bin/ld64.lld --version
```

## Resources

- **Cairo-M Repository**: https://github.com/kkrt-labs/cairo-m
- **SHA-256 Example**: https://github.com/kkrt-labs/cairo-m/tree/main/examples/sha256-cairo-m
- **Stwo Prover**: https://github.com/starkware-libs/stwo

## License

MIT OR Apache-2.0

## Acknowledgments

- KKRT Labs for developing Cairo-M
- Starkware for the Stwo prover
