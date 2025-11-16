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
cd snarkvm-zkvm/snarkvm-host
```

Build the project:

```bash
cargo build --release
```

## Running the Demo

### Quick Start

```bash
cd snarkvm-zkvm/snarkvm-host
FIBONACCI_N=10 cargo run --release
```

### Using the Run Script

From the `snarkvm-zkvm` directory:

```bash
./run_demo.sh
```

## What This Demo Does

1. **Initializes** a constraint system using snarkVM's R1CS
2. **Builds** a circuit that computes the Fibonacci sequence
3. **Analyzes** the circuit to show:
   - Number of public inputs
   - Number of private inputs
   - Total number of constraints
4. **Verifies** that the circuit is satisfied (all constraints are met)
5. **Outputs** performance metrics and results

## Understanding the Output

The demo will display:
- The computed Fibonacci result
- Circuit statistics (public/private inputs, constraints)
- Performance timing for each phase
- Verification status

Example output:
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

## Architecture

- **snarkvm-host/**: Host program that builds and verifies the circuit
  - Uses snarkVM's circuit library to construct R1CS constraints
  - Computes Fibonacci sequence in zero-knowledge
  - Verifies circuit satisfaction
  
- **programs/**: Contains Aleo program examples
  - `fibonacci.aleo`: Example Aleo program (for reference)

## Advanced Usage

### Adjusting Fibonacci Input

Try different values:
```bash
FIBONACCI_N=5 cargo run --release   # Quick computation
FIBONACCI_N=15 cargo run --release  # More constraints
FIBONACCI_N=20 cargo run --release  # Larger circuit
```

### Understanding snarkVM Circuits

This demo uses snarkVM's circuit types:
- `U32<A>`: 32-bit unsigned integer in circuit
- `U64<A>`: 64-bit unsigned integer in circuit
- `Mode::Public`: Values visible to verifier
- `Mode::Private`: Values hidden from verifier

The circuit automatically generates R1CS constraints that prove:
1. The computation was performed correctly
2. Without revealing intermediate values (marked as private)

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

