# Valida zkVM Fibonacci Demo

This directory contains a demonstration of using Valida zkVM to compute Fibonacci numbers with zero-knowledge proofs.

## About Valida zkVM

Valida is a STARK-based zkVM that supports C and other LLVM-based languages. It features:
- LLVM-based compiler toolchain
- Support for C/C++ programs
- Browser-based proving (as of v0.10.0)
- Continuations for long-running programs
- Docker support for easy deployment

## Project Structure

```
valida-zkvm/
├── valida-guest/          # Guest program (runs in zkVM)
│   └── fib.c              # Fibonacci calculation in C
├── valida-host/           # Host program (manages proving)
│   └── src/
│       └── main.rs        # Host logic for compilation, proving, verification
└── Cargo.toml             # Workspace configuration
```

## Prerequisites

### Option 1: Docker (Recommended)

```bash
docker pull lita-xyz/valida
```

### Option 2: Local Installation

Install the Valida toolchain following the official guide:
- For Ubuntu 24.04 or Arch Linux users, download the installer from Lita Foundation
- Requires LLVM 18.1.7+ and Rust 1.86+

## Resources

- **Official Website**: https://www.lita.foundation/
- **Documentation**: https://www.lita.foundation/blog/introducing-valida-zkvm-1-0
- **GitHub**: https://github.com/litaio/valida
- **Version 0.10.0 Release**: https://www.lita.foundation/blog/announcing-valida-0-10-0

## Running the Demo

### Using Docker

```bash
# Navigate to the Valida workspace
cd valida-zkvm

# Compile the guest program
docker run --rm -v $(pwd):/workspace lita-xyz/valida \
  valida-cc -o /workspace/fib.elf /workspace/valida-guest/fib.c

# Run in the zkVM
docker run --rm -v $(pwd):/workspace lita-xyz/valida \
  valida run /workspace/fib.elf

# Generate proof
docker run --rm -v $(pwd):/workspace lita-xyz/valida \
  valida prove /workspace/fib.elf -o /workspace/proof.bin

# Verify proof
docker run --rm -v $(pwd):/workspace lita-xyz/valida \
  valida verify /workspace/proof.bin
```

### Using Local Toolchain

```bash
# Navigate to the Valida workspace
cd valida-zkvm

# Compile the guest program
valida-cc -o fib.elf valida-guest/fib.c

# Run in the zkVM (execution only)
valida run fib.elf

# Generate proof
valida prove fib.elf -o proof.bin

# Verify proof
valida verify proof.bin
```

### Using the Rust Host Program (Demo Structure)

```bash
# Navigate to the host directory
cd valida-zkvm/valida-host

# Run the demonstration
RUST_LOG=info cargo run --release
```

**Note**: The Rust host program is a demonstration structure showing the workflow. For actual Valida zkVM usage, use the Docker or local toolchain methods above.

## Configuration

You can modify the Fibonacci number to compute by:

1. Setting the `FIB_N` environment variable:
   ```bash
   FIB_N=20 cargo run --release
   ```

2. Or by editing the `.env` file in the project root:
   ```
   FIB_N=20
   ```

## Features Demonstrated

1. **C Guest Program**: The Fibonacci calculation is implemented in C (`valida-guest/fib.c`)
2. **Compilation**: Compiling C code to Valida bytecode using LLVM-based toolchain
3. **Execution**: Running the program in the Valida zkVM
4. **Proof Generation**: Creating STARK proofs of correct execution
5. **Verification**: Verifying the proofs

## Performance Notes

- **Compilation**: Fast, leverages LLVM optimizations
- **Execution**: Efficient for programs of various sizes
- **Proving**: Supports continuations for long-running programs
- **Verification**: Fast verification, suitable for on-chain use

## Differences from Other zkVMs

| Feature | Valida | SP1/Risc0 | Nexus |
|---------|--------|-----------|-------|
| Primary Language | C/C++ | Rust | Rust |
| Toolchain | LLVM-based | Custom | Custom |
| Proof System | STARK | STARK/SNARK | STARK |
| Browser Support | Yes (v0.10.0+) | Limited | No |
| Continuations | Yes | Yes | Yes |

## Troubleshooting

### Docker Issues

If you encounter permission errors with Docker:
```bash
# On Linux, you may need to run with sudo or add your user to the docker group
sudo docker run ...
```

### Compilation Errors

Ensure you're using a compatible version of the Valida toolchain. The demo is tested with:
- Valida zkVM 1.0.0+
- LLVM 18.1.7+
- Rust 1.86+ (for host program)

## Next Steps

- Try modifying `fib.c` to compute different values
- Implement other algorithms in C for the Valida zkVM
- Explore browser-based proving with Valida 0.10.0+
- Experiment with continuations for longer computations

## License

MIT OR Apache-2.0


