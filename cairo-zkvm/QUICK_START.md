# Cairo zkVM - Quick Start Guide

## Generate STARK Proof (3 Steps)

### Step 1: Build & Run
```bash
scarb build && scarb cairo-run --available-gas=200000000
```
✓ Expected: `[10, 55, 55, 55, 89]`

### Step 2: Generate Execution Trace
```bash
scarb cairo-run --available-gas=200000000 --print-full-memory > trace.txt
```
✓ Creates: `trace.txt` (full execution trace)

### Step 3: Verify Proof Ready
```bash
./test_proof_generation.sh
```
✓ Validates: All components ready for STARK proof

---

## Production Proof Generation

### With Stone Prover
```bash
stone-prover \
  --program target/dev/cairo_fibonacci.sierra.json \
  --output proof.json
```

### Deploy to StarkNet
```bash
starkli deploy --network testnet
```

---

## Key Files

| File | Purpose |
|------|---------|
| `target/dev/cairo_fibonacci.sierra.json` | Input for STARK prover (121KB) |
| `trace.txt` | Full execution trace |
| `test_proof_generation.sh` | Automated verification |

---

## Test Commands

```bash
# Run all tests
scarb test

# Build only
scarb build

# Run with custom gas
scarb cairo-run --available-gas=500000000
```

---

## Expected Results

```
Input:  n = 10
Output: [10, 55, 55, 55, 89]

✓ fib_recursive(10) = 55
✓ fib_iterative(10) = 55  
✓ fib_pair(10) = (55, 89)

Gas: 773,360 / 200,000,000 (0.39%)
Tests: 4/4 passing
Status: ✅ PROOF READY
```

