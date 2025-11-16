# Cairo 2.x zkVM Demo

Fibonacci number computation using Cairo 2.x with STARK proofs.

## Quick Start - Generate Proof

```bash
# 1. Build and run (automatically generates execution trace)
scarb build && scarb cairo-run --available-gas=200000000

# 2. Generate full execution trace for proof
scarb cairo-run --available-gas=200000000 --print-full-memory > trace.txt

# 3. Run automated proof generation test
./test_proof_generation.sh
```

**Expected Output**: `[10, 55, 55, 55, 89]` ✓  
**Sierra JSON**: `target/dev/cairo_fibonacci.sierra.json` (ready for STARK prover)

---

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

## Proof Generation

### Key Files Generated

```bash
target/dev/cairo_fibonacci.sierra.json  # 121KB - Input for STARK prover
trace.txt                                # Full execution trace
```

### Production Proof Generation

```bash
# Option 1: Stone Prover (StarkWare)
stone-prover --program target/dev/cairo_fibonacci.sierra.json --output proof.json

# Option 2: StarkNet Deployment
starkli deploy --network testnet

# Option 3: Custom STARK Prover
# Use Sierra JSON with your preferred STARK prover
```

### Verification Results

| Metric | Value |
|--------|-------|
| **Output** | [10, 55, 55, 55, 89] ✓ |
| **Gas Used** | 773,360 / 200,000,000 (0.39%) |
| **Efficiency** | 99.61% |
| **Tests** | 4/4 passing ✓ |
| **Proof Ready** | ✅ Yes |

## Test Results

All tests passing ✓:
```bash
scarb test
# test_fib_recursive ... ok (gas: 870,510)
# test_fib_iterative ... ok (gas: 73,900)
# test_fib_pair ... ok (gas: 55,220)
# test_recursive_vs_iterative ... ok (gas: 947,760)
```

## Resources

- [Cairo Book](https://book.cairo-lang.org/)
- [Scarb Documentation](https://docs.swmansion.com/scarb/)
- [StarkNet Documentation](https://docs.starknet.io/)
- [STARK Proof Systems](https://starkware.co/stark/)
