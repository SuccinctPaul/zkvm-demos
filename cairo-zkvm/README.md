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

# Run the main program (returns Fibonacci computations for n=10)
scarb cairo-run --available-gas=200000000
# Expected output: [10, 55, 55, 55, 89]
# - n = 10
# - fib_recursive(10) = 55
# - fib_iterative(10) = 55  
# - fib_pair(10) = (55, 89)
```

## Project Structure

- `src/lib.cairo` - Main library with Fibonacci implementations and main() function
- `src/contract.cairo` - StarkNet smart contract example
- `src/main.cairo` - Alternative main program structure
- `examples/` - Additional example programs

## Generate Proof (Advanced)

Cairo 2.x uses STARK proofs by default. To generate explicit proofs, you can use the Cairo proving tools:

```bash
# Build the project to generate Sierra JSON
scarb build

# The Sierra JSON is at: target/dev/cairo_fibonacci.sierra.json
# Use with Cairo proving tools for explicit proof generation
```

## Resources

- [Cairo Book](https://book.cairo-lang.org/)
- [Scarb Documentation](https://docs.swmansion.com/scarb/)
- [StarkNet Documentation](https://docs.starknet.io/)
