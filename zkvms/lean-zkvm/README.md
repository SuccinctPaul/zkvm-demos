# Lean zkVM Fibonacci Demo (Reference Implementation)

This is a **reference demonstration** of [leanMultisig zkVM](https://github.com/leanEthereum/leanMultisig) workflow, showing how to compute Fibonacci numbers with zero-knowledge proofs.

⚠️ **Important**: This is a reference implementation showing the expected structure and workflow. Full integration requires the `lean_prover` SDK to be publicly available.

## About Lean zkVM

Lean zkVM (leanMultisig) is a **minimal, high-performance zkVM** designed for:
- **XMSS + minimal zkVM** = lightweight post-quantum signatures with unbounded aggregation
- **Ultra-fast proving**: 1.0-1.7 MHz on consumer hardware
- **Compact proofs**: Target 128-256 KiB (optimized)
- **Advanced proof systems**: WHIR, SuperSpartan (AIR-optimized), Univariate Skip, Logup*
- **Security**: ~128 bits of security

### Key Features

- **🚀 Blazing Fast**: 1.0 MHz on i9-12900H, 1.7 MHz on M4 Max
- **🔐 Post-Quantum Ready**: XMSS signature aggregation
- **📦 Compact Proofs**: 400-500 KiB currently, targeting 128-256 KiB
- **🎯 Minimal Design**: Inspired by Cairo, optimized for efficiency
- **⚡ Modern Cryptography**: KoalaBear field, Poseidon2 hash

### Proof System Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Lean zkVM Stack                      │
├─────────────────────────────────────────────────────────┤
│  WHIR Polynomial Commitment (~300 KiB)                 │
├─────────────────────────────────────────────────────────┤
│  SuperSpartan (AIR-optimized) (~100-200 KiB)          │
├─────────────────────────────────────────────────────────┤
│  Univariate Skip + Logup* (optimizations)              │
├─────────────────────────────────────────────────────────┤
│  Cairo-inspired VM Design                               │
└─────────────────────────────────────────────────────────┘
```

## Project Structure

```
lean-zkvm/
├── lean-guest/          # Guest program (runs inside zkVM)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs      # Fibonacci computation (no_std)
├── lean-host/           # Host program (manages proving/verification)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs      # Reference prover and verifier
├── Cargo.toml           # Workspace configuration
├── rust-toolchain.toml  # Rust toolchain (1.85)
├── run_demo.sh          # Convenient demo runner
└── README.md            # This file
```

## Prerequisites

**Note**: The actual lean_prover SDK is not yet publicly available. This demo shows the expected workflow structure.

1. **Rust 1.85+** (install from https://rustup.rs)

2. **For actual integration** (when available):
   ```bash
   cd scripts/sdk_installers
   ./install_lean_sdk.sh  # Not yet available
   ```

## Running the Demo

### Quick Start

```bash
cd zkvms/lean-zkvm
./run_demo.sh
```

### Custom Input

```bash
# Compute Fibonacci(20)
./run_demo.sh 20

# Or set environment variable
FIBONACCI_N=30 ./run_demo.sh
```

### Direct Cargo Execution

```bash
cd lean-host
FIBONACCI_N=15 RUST_LOG=info cargo run --release
```

### With Native CPU Optimizations

```bash
RUSTFLAGS='-C target-cpu=native' cargo run --release
```

## Expected Output

```
╔══════════════════════════════════════════════════════════╗
║          Lean zkVM Fibonacci Demo (Reference)           ║
╚══════════════════════════════════════════════════════════╝

📊 Configuration
   Fibonacci input: n = 10
   Target: ~128 bits of security
   Proof system: WHIR + SuperSpartan (AIR-optimized)

🔢 Step 1: Computing Fibonacci(10)...
   Result: fib(10) = 55
   Computation time: 0.001ms

🔧 Step 2: Setting up prover...
   [Reference] In actual lean zkVM, this would:
   • Compile guest program to bytecode
   • Initialize WHIR prover parameters
   • Setup KoalaBear field (p = 2^31 - 2^24 + 1)
   • Configure AIR constraints

🔐 Step 3: Generating zero-knowledge proof...
   [Reference] Proof generation workflow:
   • Execute guest program in zkVM
   • Generate execution trace
   • Build AIR constraints
   • Run WHIR commitment phase
   • Generate SuperSpartan proof
   • Optimize with univariate skip

   ✅ Proof generated successfully!
   Proving time: 0.100s
   Estimated cycles: ~500
   Proof size: ~450 KiB (with rate=1/2)
      └─ WHIR: ~300 KiB
      └─ AIR proof: ~150 KiB

🔍 Step 4: Verifying proof...
   [Reference] Verification workflow:
   • Check WHIR commitment opening
   • Verify AIR constraints
   • Validate public outputs

   ✅ Proof verified successfully!
   Verification time: 10.000ms

╔══════════════════════════════════════════════════════════╗
║                    Execution Summary                     ║
╚══════════════════════════════════════════════════════════╝
  Input:            n = 10
  Output:           fib(10) = 55
  Proving time:     0.100s
  Verification:     10.000ms
  Proof size:       ~450 KiB
  Security level:   ~128 bits
```

## Real-World Benchmarks (from leanMultisig)

### Fibonacci (n = 2,000,000 steps)

| Hardware    | Proving Speed | Proving Time | Throughput |
|-------------|---------------|--------------|------------|
| i9-12900H   | 1.0 MHz       | 2.0 s        | Fast       |
| M4 Max      | 1.7 MHz       | 1.2 s        | Very Fast  |

### Poseidon2 Hash (2^20 permutations)

- Efficient field arithmetic over KoalaBear
- Optimized for batch hashing operations

### XMSS Signature Aggregation

- 990 signatures aggregated
- Constant verification time
- Unbounded aggregation support

## Technical Details

### Field Arithmetic

- **Field**: KoalaBear (p = 2^31 - 2^24 + 1)
- **Fast modular arithmetic**: Single instruction on modern CPUs
- **Optimized for mobile**: 32-bit operations

### Proof System Components

1. **WHIR** (Polynomial Commitment)
   - ~300 KiB of current proof size
   - Will be reduced with Merkle pruning

2. **SuperSpartan** (AIR-specific optimizations)
   - Multivariate AIR argument
   - Inspired by W. Borgeaud's work
   - ~100-200 KiB proof component

3. **Univariate Skip** + **Logup***
   - Additional optimizations
   - Reduce proof size and prover time

### Security Parameters

- **Target security**: ~128 bits
- **Soundness**: Based on WHIR conjecture 4.12 "up to capacity"
- **Note**: Provable security analysis is ongoing (TODO in project)

## Project Status

This demo represents the **expected workflow** for leanMultisig zkVM:

| Component | Status | Notes |
|-----------|--------|-------|
| VM Design | ✅ Complete | Cairo-inspired, minimal design |
| WHIR Integration | ✅ Complete | ~300 KiB proofs |
| SuperSpartan | ✅ Complete | AIR-optimized |
| Recursion | 🚧 In Progress | Full recursion not finished |
| Proof Optimization | 🚧 In Progress | Targeting 128-256 KiB |
| Public SDK | ⏳ Pending | Not yet released |
| Provable Security | 📝 TODO | Security analysis ongoing |

## Use Cases

Lean zkVM is particularly well-suited for:

- **Post-Quantum Signatures**: XMSS aggregation with ZK proofs
- **Lightweight Proofs**: Mobile and edge device proving
- **Signature Aggregation**: Unbounded signature batching
- **High-Throughput Apps**: Fast proving for production systems
- **Recursive Proofs**: Proof composition (in development)

## Comparison with Other zkVMs

| Feature | Lean zkVM | SP1 | Risc0 | Nexus |
|---------|-----------|-----|-------|-------|
| **Proving Speed** | 1.0-1.7 MHz | 0.5-1.0 MHz | 0.3-0.5 MHz | 0.2-0.4 MHz |
| **Proof Size** | 128-450 KiB* | 256 B - 500 KiB | 150-200 KiB | 300-400 KiB |
| **Field** | KoalaBear | BabyBear | BabyBear | BN254 |
| **Recursion** | 🚧 In Progress | ✅ Full | ✅ Full | ✅ Full |
| **Post-Quantum** | ✅ XMSS | ❌ No | ❌ No | ❌ No |

*Target: 128-256 KiB with optimizations

## Resources

- **GitHub**: https://github.com/leanEthereum/leanMultisig
- **Paper**: `minimal_zkVM.pdf` (in repository)
- **WHIR**: Polynomial commitment scheme
- **SuperSpartan**: Multivariate AIR argument
- **Plonky3**: Underlying field and crypto libraries

## Related Projects

- **Plonky3**: High-performance ZK crypto primitives
- **whir-p3**: Plonky3-compatible WHIR implementation
- **Whirlaway**: Multilinear SNARK for AIR + minimal zkVM

## Current Limitations

1. **SDK Not Public**: Reference implementation only
2. **Recursion Incomplete**: Full recursion program in progress
3. **Proof Size**: Currently 400-500 KiB, targeting 128-256 KiB
4. **Security**: Based on conjecture, provable security TODO
5. **Documentation**: Limited public documentation

## Future Roadmap

Based on the leanMultisig TODO list:

- ✅ **Phase 1**: Basic VM and proof system (Complete)
- 🚧 **Phase 2**: Recursion and optimization (In Progress)
- ⏳ **Phase 3**: Proof size reduction (128 KiB target)
- ⏳ **Phase 4**: Public SDK release
- ⏳ **Phase 5**: Provable security analysis
- ⏳ **Phase 6**: Production hardening

## Testing

```bash
# Run unit tests
cargo test

# Run with logging
RUST_LOG=debug cargo test -- --nocapture

# Test specific Fibonacci values
cargo test test_fibonacci_values -- --nocapture
```

## Contributing

This is a reference implementation. For the actual leanMultisig project:
- GitHub: https://github.com/leanEthereum/leanMultisig
- Issues: https://github.com/leanEthereum/leanMultisig/issues

## License

MIT OR Apache-2.0

## Acknowledgments

- **leanMultisig Team**: For the innovative minimal zkVM design
- **Plonky3**: For high-performance crypto primitives
- **Cairo**: For VM design inspiration
- **WHIR Authors**: For the polynomial commitment scheme

---

**⚠️ Disclaimer**: This is a reference implementation showing the expected workflow and structure. It does not include the actual lean_prover SDK, which is not yet publicly available. The benchmarks and metrics shown are based on the leanMultisig project's published results.

