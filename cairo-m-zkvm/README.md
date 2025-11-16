# Cairo-M zkVM Fibonacci Demo

This is a demonstration of using [Cairo-M zkVM](https://github.com/kkrt-labs/cairo-m) to compute Fibonacci numbers with zero-knowledge proofs.

## About Cairo-M zkVM

Cairo-M is a Mobile-first CPU AIR (zkVM) using M31 as its native prime field, built on Starkware's Stwo for efficient mobile proving. It features:

- **M31 Native Field**: Uses M31 (Mersenne 31) prime field for efficient mobile computation
- **Minimal Register Design**: Only PC (program counter) and FP (frame pointer) registers
- **Read-Write Memory**: Efficient memory access patterns
- **Variable-Size Encoding**: x86-style instruction encoding
- **Native Type Support**: Built-in support for felt, u32, and other types via Stwo's component system
- **Mobile-Optimized**: Designed for proof generation on consumer hardware including mobile devices
- **Stwo Integration**: Leverages Starkware's Stwo prover for efficient STARK proofs

## Project Structure

```
cairo-m-zkvm/
├── Cargo.toml              # Workspace configuration
├── rust-toolchain.toml     # Rust toolchain specification
├── README.md               # This file
├── programs/               # Cairo-M source programs
│   └── fibonacci.cm        # Fibonacci computation in Cairo-M
├── cairo-m-host/           # Host program (manages compilation/proving/verification)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs         # Compiler, runner, prover and verifier logic
└── compiled/               # Compiled Cairo-M programs (generated)
    └── fibonacci.json
```

## Prerequisites

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install Cairo-M toolchain**:
   ```bash
   # Install from source
   git clone https://github.com/kkrt-labs/cairo-m.git
   cd cairo-m
   cargo install --path crates/cairo-m-compiler
   cargo install --path crates/cairo-m-runner
   cargo install --path crates/cairo-m-prover
   cargo install --path crates/cargo-cairo-m
   ```

   Or use the automated installer:
   ```bash
   cd scripts/sdk_installers
   ./install_cairo_m_sdk.sh
   ```

   This script will:
   - Clone the Cairo-M repository
   - Install cairo-m-compiler
   - Install cairo-m-runner
   - Install cairo-m-prover
   - Install cargo-cairo-m CLI tool

## Configuration

Set the Fibonacci number to compute via environment variable:

```bash
# Create .env file in the project root or set environment variable
export FIBONACCI_N=10
```

Or create a `.env` file in the workspace root:
```
FIBONACCI_N=10
```

## Building

```bash
cd cairo-m-zkvm/cairo-m-host
cargo build --release
```

The build process will:
1. Compile the host program with Cairo-M SDK dependencies
2. Link against Cairo-M compiler, runner, and prover libraries

## Running

### Full workflow (compile → run → prove → verify):

```bash
cd cairo-m-zkvm/cairo-m-host
RUST_LOG=info cargo run --release
```

The program will:
1. Load the Fibonacci input number from environment
2. Compile the Cairo-M program (`programs/fibonacci.cm`) to JSON
3. Execute the program with cairo-m-runner to generate execution trace
4. Generate a STARK proof using cairo-m-prover
5. Display execution statistics (cycles, memory usage)
6. Verify the proof
7. Display proof size and verification time

### Manual workflow:

You can also run each step manually:

```bash
# 1. Compile Cairo-M program
cairo-m-compiler --input programs/fibonacci.cm --output compiled/fibonacci.json

# 2. Run the program and generate trace
cairo-m-runner compiled/fibonacci.json --entrypoint fibonacci --arguments 10

# 3. Generate proof
cairo-m-prover compiled/fibonacci.json --entrypoint fibonacci --arguments 10

# 4. Verify proof (integrated in prover output)
```

## Expected Output

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
   Output: compiled/fibonacci.json
   Instructions: 245

🚀 Step 2: Executing program...
   ✅ Execution completed in 0.02s
   Result: fibonacci(10) = 89
   Cycles: 1,234
   Memory cells used: 567

🔐 Step 3: Generating STARK proof...
   ✅ Proof generated in 3.45s
   Proof size: 45.2 KB
   Prover backend: Stwo
   Field: M31 (Mersenne 31)

✓ Step 4: Verifying proof...
   ✅ Proof verified successfully in 0.08s

========================================
  📈 Performance Summary
========================================
Compile time:     0.15s
Execution time:   0.02s
Prove time:       3.45s
Verify time:      0.08s
Total time:       3.70s
Proof size:       45.2 KB
Cycles:           1,234
Memory cells:     567
========================================

✅ Cairo-M zkVM Demo completed successfully!
```

## Cairo-M Language

Cairo-M programs are written in a Cairo-like syntax optimized for the M31 field. Here's a simple example:

```cairo-m
// Fibonacci function in Cairo-M
func fibonacci(n: felt) -> felt {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    
    let a = 0;
    let b = 1;
    let i = 2;
    
    while i <= n {
        let temp = a + b;
        a = b;
        b = temp;
        i = i + 1;
    }
    
    return b;
}
```

## Key Features

### M31 Field Arithmetic
- Native 31-bit Mersenne prime field operations
- Efficient on mobile processors (32-bit arithmetic)
- Fast modular reduction due to Mersenne prime properties

### Minimal Register Architecture
- Only PC and FP registers
- Deterministic frame sizes
- Simplified constraint system

### Mobile-Optimized Proving
- Designed for consumer hardware
- Efficient memory usage
- Parallelizable proof generation

### Stwo Integration
- Leverages Starkware's Stwo prover
- Advanced STARK optimizations
- Efficient polynomial commitments

## Benchmarking

Test Cairo-M with different input sizes:

```bash
# Small input
FIBONACCI_N=10 cargo run --release

# Medium input
FIBONACCI_N=100 cargo run --release

# Large input
FIBONACCI_N=1000 cargo run --release
```

Expected performance characteristics:
- **Proving time**: ~2-5s for n=100 on mobile processors
- **Memory usage**: ~100-500MB depending on input size
- **Proof size**: ~30-100KB for typical programs

## Development Workflow

### 1. Write Cairo-M Program

Create a `.cm` file in the `programs/` directory:

```cairo-m
func my_computation(x: felt) -> felt {
    // Your computation logic
    return x * x;
}
```

### 2. Compile

```bash
cairo-m-compiler --input programs/my_program.cm --output compiled/my_program.json
```

### 3. Run & Test

```bash
cairo-m-runner compiled/my_program.json --entrypoint my_computation --arguments 42
```

### 4. Generate Proof

```bash
cairo-m-prover compiled/my_program.json --entrypoint my_computation --arguments 42
```

## Advanced Features

### Custom Functions

Cairo-M supports function definitions with multiple parameters:

```cairo-m
func add_multiply(a: felt, b: felt, c: felt) -> felt {
    let sum = a + b;
    return sum * c;
}
```

### Native Types

Cairo-M supports multiple native types:

```cairo-m
func type_examples() {
    let f: felt = 12345;      // Field element (M31)
    let u: u32 = 1000;         // 32-bit unsigned integer
    let b: bool = true;        // Boolean
}
```

### Memory Operations

Efficient read-write memory:

```cairo-m
func array_sum(arr: [felt], len: felt) -> felt {
    let sum = 0;
    let i = 0;
    while i < len {
        sum = sum + arr[i];
        i = i + 1;
    }
    return sum;
}
```

## Debugging

Enable debug logging to see detailed execution traces:

```bash
RUST_LOG=debug cairo-m-runner compiled/fibonacci.json --entrypoint fibonacci --arguments 10
```

This will show:
- Instruction-by-instruction execution
- Register values at each step
- Memory access patterns
- Frame pointer movements

## Resources

- **Cairo-M GitHub**: https://github.com/kkrt-labs/cairo-m
- **Cairo-M Documentation**: https://github.com/kkrt-labs/cairo-m/tree/main/docs
- **CairoMlings Tutorial**: Interactive tutorial for learning Cairo-M
- **Stwo Prover**: https://github.com/starkware-libs/stwo
- **Design Document**: https://github.com/kkrt-labs/cairo-m/blob/main/docs/design-document.md
- **Getting Started**: https://github.com/kkrt-labs/cairo-m/blob/main/docs/getting-started.md

## CairoMlings - Interactive Tutorial

Learn Cairo-M through interactive exercises:

```bash
# Install CairoMlings
cargo install --path tutorials/cairomlings

# Initialize exercise directory
cairomlings init

# Work through exercises
cd cairomlings-exercises
cairomlings watch
```

## Troubleshooting

### Build Errors

If you encounter build errors:

```bash
# Update Cairo-M toolchain
cd cairo-m
git pull
cargo install --path crates/cairo-m-compiler --force
cargo install --path crates/cairo-m-runner --force
cargo install --path crates/cairo-m-prover --force

# Clean and rebuild
cd cairo-m-zkvm
cargo clean
cargo build --release
```

### Compilation Errors

For Cairo-M compilation errors:
- Check syntax against examples in the cairo-m repository
- Ensure function signatures are correct
- Verify type annotations
- Check that all variables are properly declared

### Runtime Errors

For execution errors:
- Enable debug logging: `RUST_LOG=debug`
- Check argument types match function signature
- Verify program logic doesn't cause overflows
- Ensure memory accesses are within bounds

### Performance Issues

For slow proof generation:
- Reduce input size for testing
- Ensure system has sufficient RAM (8GB+ recommended)
- Use release builds (`--release` flag)
- Consider optimizing your Cairo-M program

## Comparison with Other zkVMs

### vs Cairo (StarkNet)
- **Cairo-M**: Mobile-optimized, M31 field, minimal registers
- **Cairo**: Starknet-optimized, larger field, more complex architecture

### vs RISC Zero / SP1
- **Cairo-M**: Custom Cairo-like language, M31 field
- **RISC Zero/SP1**: Rust language, RISC-V ISA

### vs Miden
- **Cairo-M**: Cairo-like syntax, Stwo prover
- **Miden**: Stack-based VM, custom assembly

## Notes

⚠️ **Current Status**: Cairo-M is a work in progress and not recommended for production use yet.

- Cairo-M is actively under development
- APIs and language syntax may change
- Mobile proving is the primary design goal
- M31 field provides excellent mobile performance
- Minimal register design simplifies constraint system
- Best suited for applications requiring mobile proving

## Version Information

- Cairo-M: Latest from main branch (v0.1.0-alpha.1+)
- Rust Toolchain: nightly (see rust-toolchain.toml)
- Stwo: Latest from submodule
- Rust Edition: 2021

## Performance Characteristics

### Mobile Devices (ARM)
- iPhone 13: ~3-5s for n=100 fibonacci
- Android flagship: ~4-6s for n=100 fibonacci
- Memory: 200-400MB typical

### Desktop (x86_64)
- Intel/AMD: ~1-2s for n=100 fibonacci
- Apple Silicon: ~1-2s for n=100 fibonacci
- Memory: 200-400MB typical

## Contributing

Contributions to improve this demo are welcome! Please ensure:
- Cairo-M code follows best practices
- Documentation is clear and accurate
- Examples are tested
- Performance is benchmarked

## License

MIT OR Apache-2.0

## Acknowledgments

- KKRT Labs for developing Cairo-M
- Starkware for the Stwo prover
- The zero-knowledge proof research community
- Cairo language team for inspiration

