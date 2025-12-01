# snarkVM Demo

This demo showcases how to build and verify zero-knowledge proofs using [snarkVM](https://github.com/ProvableHQ/snarkVM), the virtual machine powering the Aleo blockchain.

## About snarkVM

snarkVM is a decentralized virtual machine that enables:
- Zero-knowledge proof generation and verification
- Private computation on a public blockchain
- R1CS constraint system for building circuits
- Integration with the Aleo ecosystem and Leo programming language

## Prerequisites

### Option 1: Using Installation Script (Recommended)

```bash
cd scripts/sdk_installers
./install_snarkvm_sdk.sh
```

This will:
- Install the snarkVM library and dependencies
- Set up the required Rust toolchain
- Verify the installation

### Option 2: Manual Installation

1. Install Rust (1.85 or later):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install 1.85
rustup default 1.85
```

2. The snarkVM library will be automatically pulled when building the project.

## Environment Setup

Create a `.env` file in the project root or set the environment variable:

```bash
# Set the Fibonacci number to compute
export FIBONACCI_N=10
```

Or create a `.env` file:
```
FIBONACCI_N=10
```

## Building

Navigate to the host directory:

```bash
cd zkvms/snarkvm-zkvm/snarkvm-host
```

Build the project:

```bash
cargo build --release
```

## Running the Demo

### Quick Start

```bash
cd zkvms/snarkvm-zkvm/snarkvm-host
FIBONACCI_N=10 cargo run --release
```

### Using the Run Script

From the `snarkvm-zkvm` directory:

```bash
./run_demo.sh
```

## What This Demo Does

This demo showcases the **cryptographic primitives** that power snarkVM:

1. **Computes Fibonacci** using native Rust (demonstrates the computation)
2. **Field Arithmetic** - demonstrates finite field operations:
   - Addition, multiplication, subtraction
   - Field inversion
   - Working with prime field elements
3. **Elliptic Curve Operations** - demonstrates curve cryptography:
   - Point addition and doubling
   - Scalar multiplication
   - Group operations on BLS12-377 curve
4. **Performance Metrics** - measures execution time for each operation

**Note**: Full zero-knowledge circuit programming in snarkVM is best done using the **Leo programming language**. This demo shows the underlying cryptographic building blocks.

## Understanding the Output

The demo will display:
- The computed Fibonacci result
- Field arithmetic operations and results
- Elliptic curve operations and verifications
- Performance timing for each phase

Example output:
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
```

## Architecture

- **snarkvm-host/**: Host program demonstrating snarkVM primitives
  - Uses snarkVM's console library for field and curve operations
  - Demonstrates cryptographic building blocks
  - Shows performance characteristics
  
- **programs/**: Contains Aleo program examples
  - `fibonacci.aleo`: Example Aleo program (for reference)
  - To actually compile and run Aleo programs, use Leo language

## Advanced Usage

### Adjusting Fibonacci Input

Try different values:
```bash
FIBONACCI_N=5 cargo run --release   # Quick computation
FIBONACCI_N=15 cargo run --release  # More constraints
FIBONACCI_N=20 cargo run --release  # Larger circuit
```

### Understanding snarkVM Cryptography

This demo uses snarkVM's cryptographic primitives:
- **Field Elements**: Elements in a prime field used for arithmetic
- **Elliptic Curves**: BLS12-377 curve for public key cryptography
- **Group Operations**: Point addition, doubling, and scalar multiplication
- **Testnet3**: The Aleo testnet configuration

These primitives form the foundation for:
1. Zero-knowledge proof systems
2. Private transactions on Aleo
3. zkSNARK circuits written in Leo

## Resources

- **snarkVM Repository**: https://github.com/ProvableHQ/snarkVM
- **Documentation**: https://developer.aleo.org/
- **Aleo Website**: https://aleo.org/
- **Leo Language**: https://leo-lang.org/
- **Discord Community**: https://discord.gg/aleo

## Troubleshooting

### Build Errors

If you encounter build errors:

1. Ensure Rust toolchain is up to date:
```bash
rustup update
```

2. Clean and rebuild:
```bash
cargo clean
cargo build --release
```

3. Check that you're using Rust 1.85 or later:
```bash
rustc --version
```

### Memory Issues

For large Fibonacci numbers (n > 30), you may need to increase stack size:

```bash
RUST_MIN_STACK=8388608 cargo run --release
```

## License

This demo is released under MIT OR Apache-2.0 license, consistent with the snarkVM project.

## Next Steps

1. Explore the `programs/fibonacci.aleo` file to see Aleo syntax
2. Try modifying the circuit to compute other functions
3. Learn more about Leo programming language for Aleo
4. Experiment with different proof modes and optimizations

For more advanced examples, visit the [snarkVM repository](https://github.com/ProvableHQ/snarkVM).

