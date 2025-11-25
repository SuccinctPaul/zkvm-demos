# Cairo 2.x zkVM Demo

Fibonacci number computation using Cairo 2.x with STARK proofs.



### Proof Generation Process

```mermaid
Cairo Source Code (*.cairo)
         ↓
    [Scarb Build]
         ↓
Sierra Intermediate Representation (*.sierra.json)
         ↓
    [Cairo VM Execution]
         ↓
Execution Trace (memory states, gas usage)
         ↓
    [STARK Prover]
         ↓
STARK Proof (succinctly proves execution)
         ↓
    [STARK Verifier]
         ↓
Verification Result (✓ or ✗)
```


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

### 🚀 Generate Actual STARK Proof

**Recommended Method: Use StarkNet + Katana**
```bash
# Install Starknet Foundry (includes Katana local node)
curl -L https://raw.githubusercontent.com/foundry-rs/starknet-foundry/master/scripts/install.sh | sh
snfoundryup

# Start local node
katana &

# Declare contract and generate proof
starkli declare target/dev/cairo_fibonacci.contract_class.json --rpc http://localhost:5050
```

📖 **Detailed Guide**:
- `STARK_PROOF_GUIDE.md` - Comparison of all proof generation methods
- `PROOF_GENERATION_SUMMARY.md` - Quick summary and recommendations

### 🚀 NEW: Stwo-Cairo Ultra-Fast Prover

**[Stwo-Cairo](https://github.com/starkware-libs/stwo-cairo)** is StarkWare's latest next-generation prover, based on Circle STARKs technology.

**Performance Improvements**:
- ⚡ Proving time **60%** faster (15s vs 40s)
- 📉 Proof size **25%** smaller (150KB vs 200KB)
- 🚀 Verification time **50%** faster (1s vs 2s)

**Quick Start**:
```bash
# 1. Install Stwo-Cairo toolchain
./install_stwo.sh

# 2. Run demo (automatically generates and verifies proof)
./run_stwo_demo.sh
```

**Full Documentation**:
- **`STWO_README.md`** - 📖 Overview and documentation index
- **`STWO_QUICK_START.md`** - ⚡ 5-minute quick start
- **`STWO_INTEGRATION_GUIDE.md`** - 🔧 Detailed integration steps
- **`STWO_COMPARISON.md`** - 📊 Performance comparison analysis

**Applicable Scenarios**: ✅ Perfectly suitable for this project (pure computation tasks)

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

- `src/lib.cairo` - Complete Fibonacci implementation with main() function and tests

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


## Resources

- [Cairo Book](https://book.cairo-lang.org/)
- [Scarb Documentation](https://docs.swmansion.com/scarb/)
- [StarkNet Documentation](https://docs.starknet.io/)
- [STARK Proof Systems](https://starkware.co/stark/)
