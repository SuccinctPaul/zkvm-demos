# Cairo 2.x zkVM Demo

Fibonacci number computation using Cairo 2.x with STARK proofs.

## Prerequisites

Install Scarb (Cairo package manager):

```bash
# Option 1: Using installation script
curl --proto '=https' --tlsv1.2 -sSf https://docs.swmansion.com/scarb/install.sh | sh

# Option 2: Using project installer
cd /Users/paul/zkp/zkvms/zkvm-demos
./scripts/sdk_installers/install_cairo_sdk.sh

# Verify installation
scarb --version  # Should show 2.8.0+
```

## How to Run

```bash
cd cairo-zkvm

# Build the project
scarb build

# Run tests
scarb test

# Run the program
scarb cairo-run --available-gas=200000000
```

## Generate Proof (Optional)

```bash
# Compile to Sierra
scarb build

# Run with proof generation
cairo-run \
  --program=target/dev/cairo_fibonacci.sierra.json \
  --layout=recursive \
  --print_output \
  --proof_mode
```

## Resources

- [Cairo Book](https://book.cairo-lang.org/)
- [Scarb Documentation](https://docs.swmansion.com/scarb/)
- [StarkNet Documentation](https://docs.starknet.io/)
