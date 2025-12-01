# Novanet zkVM Demo

This directory contains a demonstration implementation of Novanet zkVM, a zero-knowledge virtual machine based on the Nova proof system.

## Overview

Novanet zkVM is designed to leverage Nova's recursive SNARK properties for efficient zero-knowledge proofs. Nova is particularly well-suited for incrementally verifiable computation (IVC), making it ideal for iterative and recursive computations.

### Key Features

- **Recursive SNARKs**: Based on Nova proof system
- **Incrementally Verifiable Computation (IVC)**: Efficient for iterative computations  
- **Fibonacci Computation**: Demo computes the nth Fibonacci number with zero-knowledge proof
- **Performance Tracking**: Measures compilation, setup, proving, and verification times

## Architecture

The project follows a standard host-guest architecture:

```
novanet-zkvm/
├── novanet-guest/     # Guest program (runs in zkVM)
│   └── src/
│       └── lib.rs     # Fibonacci computation
└── novanet-host/      # Host program (orchestrates proving)
    └── src/
        └── main.rs    # Prover and verifier
```

## Prerequisites

- Rust 1.85 or later
- Cargo

## Installation

No additional dependencies are required beyond the Rust toolchain. The demo uses a simulated implementation structure.

## Usage

### Run the Demo

```bash
# From the novanet-zkvm directory
cargo run --release

# Or with custom Fibonacci number
FIBONACCI_N=15 cargo run --release
```

### Run Tests

```bash
cargo test
```

## Implementation Details

### Nova Proof System

Nova is a recursive SNARK construction that enables:
- **Efficient Recursion**: Constant-size proofs regardless of computation depth
- **IVC Support**: Incremental computation with proof composition
- **No Trusted Setup**: Unlike some SNARKs, Nova doesn't require a trusted setup ceremony

### Guest Program

The guest program (`novanet-guest`) contains the computation to be proven:
- Input: `n` (the Fibonacci index)
- Output: The nth Fibonacci number
- Proves: Correct computation of `fib(n)`

### Host Program

The host program (`novanet-host`) orchestrates the proof generation:
1. **Compile**: Converts guest code to circuit representation
2. **Setup**: Initializes proving system parameters
3. **Prove**: Generates zero-knowledge proof of computation
4. **Verify**: Verifies the proof's correctness

## Performance

Typical performance for fibonacci(10) on modern hardware:
- Compile time: < 0.1s
- Setup time: < 0.1s  
- Prove time: < 0.1s (simulated)
- Verify time: < 0.1s
- Proof size: ~50 bytes (simulated)

**Note**: These are simulated timings. A real Nova implementation would have different performance characteristics.

## Comparison with Other zkVMs

| Feature | Novanet (Nova) | SP1 | RISC Zero | Jolt |
|---------|---------------|-----|-----------|------|
| Proof System | Nova (recursive SNARKs) | STARK/SNARK | STARK | Lasso+Jolt |
| Trusted Setup | No | No | No | No |
| Recursion | Native | Yes | Yes | Limited |
| IVC Support | Native | Limited | Limited | No |
| Best For | Iterative computations | General purpose | General purpose | Simple programs |

## Development Status

⚠️ **Important Note**: This is a demonstration implementation showing the expected structure and API of a Novanet zkVM. A production implementation would require:

1. **Nova Library Integration**: Integration with actual Nova proof system (e.g., `nova-snark` crate)
2. **Circuit Compilation**: Real circuit generation from guest code
3. **Proof Generation**: Actual Nova proof construction and recursion
4. **Verification**: Real cryptographic verification using Nova's verifier

## References

- [Nova: Recursive Zero-Knowledge Arguments from Folding Schemes](https://eprint.iacr.org/2021/370)
- [Nova GitHub Repository](https://github.com/microsoft/nova)
- [zkVM Benchmarks](https://github.com/kkrt-labs/zkvm-benchmarks)

## Future Improvements

- [ ] Integrate real Nova proof system
- [ ] Add support for more complex guest programs
- [ ] Implement proof recursion and composition
- [ ] Add benchmarking suite
- [ ] Support custom circuits beyond Fibonacci

## Contributing

This is a demonstration project. For production use, consider:
- Integrating with actual Nova implementation
- Adding comprehensive test suite
- Implementing proper error handling
- Adding circuit optimization passes

## License

MIT OR Apache-2.0

## See Also

- **SP1**: General-purpose zkVM with STARK/SNARK backend
- **RISC Zero**: STARK-based zkVM for RISC-V
- **Jolt**: Lookup-based zkVM with Lasso
- **Nexus**: zkVM with multiple backend options

