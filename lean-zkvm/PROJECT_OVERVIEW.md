# Lean zkVM - Project Overview

## Executive Summary

Lean zkVM (leanMultisig) is a **minimal, high-performance zero-knowledge virtual machine** designed for post-quantum signature aggregation and general-purpose zero-knowledge computation. It combines:

- **XMSS** post-quantum signatures
- **Minimal zkVM** inspired by Cairo
- **WHIR** polynomial commitment
- **SuperSpartan** AIR-optimized proving

**Result**: Lightweight PQ signatures with unbounded aggregation at 1.0-1.7 MHz proving speed.

## Architecture Deep Dive

### 1. Virtual Machine Design

Lean zkVM follows the Cairo model:

```
┌─────────────────────────────────────────────────────┐
│              Lean zkVM Architecture                 │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ┌─────────────┐      ┌──────────────┐            │
│  │   Program   │──────▶│ Execution    │            │
│  │  Bytecode   │      │   Trace      │            │
│  └─────────────┘      └──────┬───────┘            │
│                               │                     │
│                               ▼                     │
│  ┌─────────────────────────────────────┐           │
│  │     Algebraic Intermediate          │           │
│  │     Representation (AIR)            │           │
│  │  • Minimal constraints              │           │
│  │  • Cairo-inspired design            │           │
│  └─────────────┬───────────────────────┘           │
│                │                                    │
│                ▼                                    │
│  ┌─────────────────────────────────────┐           │
│  │   SuperSpartan (AIR-optimized)      │           │
│  │  • Multivariate polynomial          │           │
│  │  • W. Borgeaud optimizations        │           │
│  └─────────────┬───────────────────────┘           │
│                │                                    │
│                ▼                                    │
│  ┌─────────────────────────────────────┐           │
│  │   WHIR Polynomial Commitment        │           │
│  │  • Conjecture-based security        │           │
│  │  • ~300 KiB commitment proof        │           │
│  └─────────────────────────────────────┘           │
│                                                     │
└─────────────────────────────────────────────────────┘
```

### 2. Field Arithmetic

**KoalaBear Field**: p = 2^31 - 2^24 + 1

```rust
// Fast modular reduction (single CPU instruction)
p = 2_147_483_648 - 16_777_216 + 1
p = 2_130_706_433

// Why KoalaBear?
// • 32-bit operations (mobile-friendly)
// • Fast modular arithmetic
// • Good for FFT operations
// • Compatible with modern CPUs
```

**Comparison with other fields**:

| Field | Prime | Bits | Speed | Use Case |
|-------|-------|------|-------|----------|
| KoalaBear | 2^31 - 2^24 + 1 | 31 | Fast | Mobile proving |
| BabyBear | 2^31 - 1 | 31 | Fast | SP1, Risc0 |
| Goldilocks | 2^64 - 2^32 + 1 | 64 | Medium | Plonky2 |
| BN254 | ~254 bits | 254 | Slow | EVM-compatible |

### 3. Proof System Stack

#### Layer 1: WHIR (Polynomial Commitment)

- **Purpose**: Commit to execution trace polynomials
- **Size**: ~300 KiB of current proof
- **Security**: Based on conjecture 4.12 (rate = 1/2)
- **Optimization**: Merkle pruning (TODO) will reduce size

```
WHIR Properties:
• Type: Polynomial commitment scheme
• Basis: List recovery + proximity testing
• Rate: 1/2, 1/4, or 1/8 (trade-off: size vs. speed)
• Security: ~128 bits (conjectured)
```

#### Layer 2: SuperSpartan (AIR Argument)

- **Purpose**: Prove AIR constraints satisfaction
- **Size**: ~100-200 KiB
- **Optimization**: AIR-specific from "A simple multivariate AIR argument"
- **Innovation**: Tailored for minimal VM constraints

```
SuperSpartan AIR Optimizations:
• Multivariate polynomial commitments
• Sum-check protocol optimizations  
• Reduced round complexity
• Efficient for sparse constraints
```

#### Layer 3: Additional Optimizations

**Univariate Skip**:
- Skip unnecessary polynomial evaluations
- Reduce prover computation

**Logup***:
- Lookup argument optimization
- Efficient table lookups

### 4. Proving Workflow

```
┌─────────────┐
│   Source    │
│   Code      │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Compile   │ ──▶ Bytecode (VM instructions)
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Execute   │ ──▶ Execution Trace
└──────┬──────┘     (memory, registers, PC)
       │
       ▼
┌─────────────┐
│ AIR Check   │ ──▶ Constraint polynomials
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ SuperSpartan│ ──▶ AIR argument proof
└──────┬──────┘     (~100-200 KiB)
       │
       ▼
┌─────────────┐
│    WHIR     │ ──▶ Polynomial commitment
└──────┬──────┘     (~300 KiB)
       │
       ▼
┌─────────────┐
│ Final Proof │ ──▶ ~400-500 KiB total
└─────────────┘     (target: 128-256 KiB)
```

## Performance Characteristics

### Proving Speed Benchmarks

**Fibonacci (n = 2,000,000)**:

| Hardware | CPU | RAM | Speed | Time |
|----------|-----|-----|-------|------|
| Desktop | i9-12900H | 32 GB | 1.0 MHz | 2.0 s |
| Laptop | M4 Max | 64 GB | 1.7 MHz | 1.2 s |

**Poseidon2 (2^20 permutations)**:

```bash
RUSTFLAGS='-C target-cpu=native' cargo run --release -- poseidon --log-n-perms 20
```

- Optimized for KoalaBear field
- Efficient batch operations
- Used for hash-based commitments

**XMSS Aggregation (990 signatures)**:

```bash
RUSTFLAGS='-C target-cpu=native' cargo run --release -- xmss --n-signatures 990
```

- Post-quantum signature aggregation
- Constant verification time
- Unbounded aggregation capability

### Proof Size Breakdown

**Current (rate = 1/2)**:
```
Total: ~400-500 KiB
├─ WHIR: ~300 KiB (60-75%)
├─ SuperSpartan: ~100-150 KiB (20-30%)
└─ Metadata: ~50 KiB (10%)
```

**Target (with optimizations)**:
```
Fast Proofs (rate = 1/2):
Total: ~256 KiB
├─ WHIR: ~180 KiB
└─ Rest: ~76 KiB

Compact Proofs (rate = 1/4 or 1/8):
Total: ~128 KiB
├─ WHIR: ~90 KiB
└─ Rest: ~38 KiB
```

## Key Innovations

### 1. Minimal VM Design

Inspired by Cairo but optimized further:

- **Fewer constraints**: Minimal AIR complexity
- **Efficient encoding**: Compact bytecode
- **Fast execution**: Optimized trace generation
- **Simple verification**: Reduced verifier complexity

### 2. AIR-Specific SuperSpartan

Based on [W. Borgeaud's work](https://eprint.iacr.org/2024/xxx):

- Multivariate polynomial optimization
- Tailored for VM constraints
- Reduced round complexity
- Efficient sum-check protocol

### 3. XMSS + zkVM Integration

Novel approach to post-quantum signatures:

```
Traditional XMSS:
• Each signature: ~2.5 KB
• 1000 signatures: ~2.5 MB
• Linear verification time

XMSS + Lean zkVM:
• 1000 signatures → 1 proof: ~450 KB
• Constant verification time
• Unbounded aggregation
• Post-quantum secure
```

### 4. Mobile-First Field Arithmetic

KoalaBear field chosen for:
- 32-bit mobile processors
- Single-instruction modular reduction
- Fast FFT operations
- Low memory footprint

## Current Status & Roadmap

### ✅ Completed (Phase 1)

- [x] Basic VM design and implementation
- [x] WHIR integration
- [x] SuperSpartan AIR prover
- [x] Poseidon2 benchmarks
- [x] XMSS aggregation (trivial encoding)
- [x] Fibonacci demo (2M steps)

### 🚧 In Progress (Phase 2)

- [ ] Full recursion program
- [ ] Merkle pruning for WHIR
- [ ] Proof size optimization (100-200 KiB reduction)
- [ ] Better AIR constraint encoding

### ⏳ Planned (Phase 3+)

- [ ] Public SDK release
- [ ] Provable security analysis
- [ ] Production hardening
- [ ] Advanced recursion
- [ ] Hardware acceleration (GPU)
- [ ] Formal verification

## Comparison with Other zkVMs

### vs. SP1 (Succinct)

| Feature | Lean zkVM | SP1 |
|---------|-----------|-----|
| Speed | 1.0-1.7 MHz | 0.5-1.0 MHz |
| Proof Size | 128-450 KiB | 256 B - 500 KiB |
| Recursion | In Progress | Full |
| Post-Quantum | Yes (XMSS) | No |
| Field | KoalaBear | BabyBear |
| Maturity | Early | Production |

### vs. Risc0

| Feature | Lean zkVM | Risc0 |
|---------|-----------|-------|
| Speed | 1.0-1.7 MHz | 0.3-0.5 MHz |
| Architecture | Cairo-inspired | RISC-V |
| Proof System | WHIR + SuperSpartan | FRI + STARK |
| Target | Mobile + PQ | General purpose |
| Maturity | Early | Production |

### vs. Cairo

| Feature | Lean zkVM | Cairo |
|---------|-----------|-------|
| Design | Inspired by Cairo | Original |
| Complexity | Minimal | Full-featured |
| Language | Rust guest | Cairo lang |
| Target | Fast proving | General ZK apps |
| Field | KoalaBear | Prime field |

## Use Cases

### 1. Post-Quantum Multisig

**Problem**: Traditional multisig vulnerable to quantum attacks

**Solution**: XMSS + zkVM aggregation

```
Benefits:
• Post-quantum secure
• Constant verification time
• Unbounded signatures
• Compact proof (~450 KB vs. 2.5 MB)
```

### 2. Mobile Proving

**Problem**: Most zkVMs too slow for mobile

**Solution**: KoalaBear field + minimal VM

```
Benefits:
• 32-bit optimized
• Fast on ARM processors
• Low memory footprint
• Battery efficient
```

### 3. High-Throughput Applications

**Problem**: Need fast proof generation

**Solution**: 1.0-1.7 MHz proving speed

```
Use cases:
• High-frequency trading proofs
• Real-time verification
• Streaming data proofs
• IoT device proving
```

## Technical Challenges

### 1. Recursion (In Progress)

**Challenge**: Efficient proof composition

**Current**: WHIR opening verification (25 variables, rate=1/4)

**Target**: Full recursive proof verification

### 2. Proof Size Optimization

**Challenge**: Reduce from 450 KB to 128-256 KB

**Approaches**:
- Merkle pruning (saves ~100 KB)
- Better encoding (saves ~50 KB)
- Lower rate (1/4 or 1/8, slower)

### 3. Provable Security

**Challenge**: Move from conjecture to proof

**Current**: Based on WHIR conjecture 4.12

**Target**: Formal security analysis

## Resources

### Papers & Documentation

- **minimal_zkVM.pdf**: In leanMultisig repository
- **WHIR**: Polynomial commitment scheme paper
- **SuperSpartan**: Multivariate AIR argument paper
- **W. Borgeaud**: "A simple multivariate AIR argument inspired by SuperSpartan"

### Dependencies

- **Plonky3**: Field arithmetic, Poseidon2
- **whir-p3**: Plonky3-compatible WHIR
- **Whirlaway**: Multilinear SNARK for AIR

### Community

- **GitHub**: https://github.com/leanEthereum/leanMultisig
- **Contributors**: 8 active developers
- **Commits**: 217 (active development)
- **Stars**: 50+

## Conclusion

Lean zkVM represents a **focused, minimal approach** to zero-knowledge virtual machines:

- **Fast**: 1.0-1.7 MHz proving
- **Compact**: Targeting 128-256 KiB proofs
- **Post-Quantum**: XMSS integration
- **Mobile-First**: KoalaBear field
- **Innovative**: SuperSpartan + WHIR stack

**Trade-offs**:
- Early stage (SDK not public)
- Recursion incomplete
- Security based on conjecture (for now)

**Best for**:
- Post-quantum applications
- Mobile proving
- High-throughput scenarios
- Research and experimentation

**Not yet ready for**:
- Production deployments
- Complex recursive proofs
- Applications requiring formal security proofs

---

*Last updated: Based on leanMultisig repository status as of November 2025*

