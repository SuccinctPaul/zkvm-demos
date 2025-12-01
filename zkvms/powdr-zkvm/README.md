# Powdr zkVM Fibonacci Demo

This is a demonstration of using [Powdr zkVM](https://github.com/powdr-labs/powdr) to compute Fibonacci numbers with zero-knowledge proofs.

## About Powdr zkVM

Powdr is a modular zkVM toolkit that allows developers to build custom zero-knowledge virtual machines by combining different components:

- **Multiple Frontends**: Support for RISC-V, WASM, and custom instruction sets
- **Flexible Backends**: Choose from Halo2, Plonky2, STARK, or other proving systems
- **PIL (Polynomial Identity Language)**: Custom constraint language for defining circuits
- **Modular Architecture**: Mix and match components to suit your needs
- **High Performance**: Optimized for modern proving systems

Unlike monolithic zkVMs, Powdr provides a toolkit approach that enables:
- Custom instruction set extensions
- Application-specific optimizations
- Easy backend switching for different deployment scenarios
- Research and experimentation with new proving techniques

## Project Structure

```
powdr-zkvm/
├── Cargo.toml              # Workspace configuration
├── rust-toolchain.toml     # Rust toolchain specification
├── README.md               # This file
├── powdr-guest/            # Guest program (runs inside zkVM)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs         # Fibonacci computation logic
└── powdr-host/             # Host program (manages proving/verification)
    ├── Cargo.toml
    ├── build.rs            # Build script for guest compilation
    └── src/
        └── main.rs         # Prover and verifier logic
```

## Prerequisites

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Install Powdr toolchain**:
   ```bash
   cd scripts/sdk_installers
   ./install_powdr_sdk.sh
   ```

   This script will:
   - Install the Powdr CLI tools
   - Install necessary Rust components
   - Setup the Powdr toolchain for guest program compilation

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
cd zkvms/powdr-zkvm/powdr-host
cargo build --release
```

The build process will:
1. Compile the guest program to Powdr's intermediate representation (PIL)
2. Generate the execution circuit
3. Setup the proving system with selected backend
4. Compile the host program with Powdr SDK dependencies

## Running

### Generate and verify proof:

```bash
cd zkvms/powdr-zkvm/powdr-host
RUST_LOG=info cargo run --release
```

The program will:
1. Load the Fibonacci input number from environment
2. Compile the guest program to a circuit
3. Execute the program and capture the execution trace
4. Generate a zero-knowledge proof
5. Display proof statistics (size, generation time)
6. Verify the proof

## Expected Output

```
========================================
  Powdr zkVM - Fibonacci Demo
========================================

📊 Configuration:
   Input: n = 10
   Expected result: fib(10) = 89

🔨 Step 1: Compiling guest program...
   ✅ Compilation completed in 0.12s
   Circuit generated: Fibonacci computation

🔧 Step 2: Setting up proving system...
   ✅ Setup completed in 0.05s
   Proving keys generated

🚀 Step 3: Executing program...
   ✅ Execution completed in 0.00s
   Result: fibonacci(10) = 89

🔐 Step 4: Generating zero-knowledge proof...
   ✅ Proof generated in 2.34s
   Proof size: ~2KB (estimated)

✓ Step 5: Verifying proof...
   ✅ Proof verified successfully in 0.02s

========================================
  📈 Performance Summary
========================================
Compile time:     0.12s
Setup time:       0.05s
Execution time:   0.00s
Prove time:       2.34s
Verify time:      0.02s
Total time:       2.53s
========================================

✅ Powdr zkVM Demo completed successfully!
```

## Backend Options

Powdr supports multiple proving backends. You can configure the backend in `powdr-host/src/main.rs`:

```rust
// Available options:
Backend::Halo2      // Halo2 SNARK backend (default)
Backend::Plonky2    // Plonky2 SNARK backend
Backend::STARK      // STARK backend
Backend::Custom     // Custom backend implementation
```

### Backend Comparison

| Backend | Proof Size | Generation Time | Verification Time | Use Case |
|---------|-----------|-----------------|-------------------|----------|
| STARK | Large (~100KB) | Fast | Fast | Development/Testing |
| Plonky2 | Medium (~10KB) | Medium | Fast | General Purpose |
| Halo2 | Small (~2KB) | Slow | Very Fast | On-chain Verification |

## Features

- **Modular Design**: Choose your frontend and backend independently
- **PIL Circuits**: Define custom constraints in Polynomial Identity Language
- **Flexible I/O**: Support for complex input/output types
- **Performance Optimization**: Application-specific circuit optimizations
- **Multiple Targets**: Generate proofs for different verification contexts

## Development Workflow

### 1. Write Guest Program

The guest program runs inside the zkVM:

```rust
#![no_main]
#![no_std]

#[no_mangle]
pub extern "C" fn main() {
    let n = powdr_read_u32();
    let result = fibonacci(n);
    powdr_write_u32(result);
}
```

### 2. Compile to Circuit

```bash
powdr compile --backend halo2 guest/src/main.rs
```

### 3. Generate Proof

```rust
let proof = prover.prove(&input)?;
```

### 4. Verify Proof

```rust
let is_valid = verifier.verify(&proof)?;
```

## Advanced Features

### Custom Backends

Powdr allows implementing custom proving backends:

```rust
use powdr_backend::Backend;

struct MyCustomBackend {
    // Custom implementation
}

impl Backend for MyCustomBackend {
    // Implement required methods
}
```

### PIL Constraints

Define custom circuit constraints using PIL:

```pil
// Fibonacci constraint
pol commit a, b;
pol constant STEP;

// Constraint: b' = a + b
b' = a + b;
```

### Optimizations

- **Instruction Set Extensions**: Add custom instructions for common operations
- **Circuit Specialization**: Optimize circuits for specific applications
- **Lookup Tables**: Use precomputed tables for expensive operations
- **Batch Proving**: Generate proofs for multiple executions efficiently

## Benchmarking

Compare Powdr with other zkVMs:

```bash
# Run with different input sizes
FIBONACCI_N=10 cargo run --release
FIBONACCI_N=100 cargo run --release
FIBONACCI_N=1000 cargo run --release

# Try different backends
# Edit powdr-host/src/main.rs to change backend
cargo run --release
```

## Resources

- **Powdr GitHub**: https://github.com/powdr-labs/powdr
- **Powdr Documentation**: https://docs.powdr.org/
- **PIL Language Spec**: https://docs.powdr.org/pil/
- **zkVM Benchmarks**: https://github.com/kkrt-labs/zkvm-benchmarks
- **Powdr Examples**: https://github.com/powdr-labs/powdr/tree/main/examples

## Troubleshooting

### Build Errors

If you encounter build errors:

```bash
# Update Powdr toolchain
powdr update

# Clean build artifacts
cargo clean
cargo build --release
```

### Performance Issues

For slow proof generation:
- Choose STARK backend for faster proving
- Reduce input size for testing
- Enable parallel proving (if available)
- Ensure system has sufficient RAM (16GB+ recommended)

### Circuit Compilation Issues

If circuit compilation fails:
- Check guest program syntax
- Verify PIL constraints are well-formed
- Ensure all dependencies are up to date
- Check Powdr CLI version compatibility

## Notes

⚠️ **Current Status**: This is a reference implementation showing the expected workflow for Powdr zkVM. The actual SDK integration will be updated once officially released.

- Powdr is a toolkit, not a ready-to-use zkVM
- Requires understanding of circuit design concepts
- More flexible but requires more setup than monolithic zkVMs
- Best suited for applications needing custom optimizations
- Active development - APIs may change

## Comparison with Other zkVMs

### vs RISC Zero / SP1
- **Powdr**: Modular toolkit, choose your components
- **RISC Zero/SP1**: Complete solution, less flexibility

### vs Miden
- **Powdr**: Multiple frontend options (RISC-V, WASM)
- **Miden**: Custom instruction set optimized for Miden VM

### vs Cairo
- **Powdr**: General purpose VM construction
- **Cairo**: Optimized for Starknet ecosystem

## Contributing

Contributions to improve this demo are welcome! Please ensure:
- Code follows Rust best practices
- Documentation is updated
- Examples are tested
- Performance optimizations are benchmarked

## Version Information

- Powdr: Latest from main branch
- Rust Toolchain: 1.85.0
- Rust Edition: 2021

## License

MIT OR Apache-2.0

## Acknowledgments

- Powdr Labs for the zkVM toolkit
- zkVM Benchmarks repository for reference implementations
- The zero-knowledge proof research community

