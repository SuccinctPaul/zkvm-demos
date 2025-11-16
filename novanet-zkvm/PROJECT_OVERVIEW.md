# Novanet zkVM - Project Overview

## Introduction

Novanet zkVM is a demonstration implementation showcasing how to build a zero-knowledge virtual machine based on the Nova proof system. This project serves as both an educational resource and a practical template for developers interested in Nova's recursive SNARK technology.

## What is Nova?

Nova is a groundbreaking proof system that introduces a new approach to recursive SNARKs through "folding schemes." Published in 2021, it offers several advantages over traditional SNARK constructions:

### Key Advantages

1. **No Trusted Setup**
   - Unlike Groth16 or PLONK, Nova doesn't require a trusted setup ceremony
   - Reduces security assumptions
   - Simplifies deployment

2. **Native Recursion**
   - Recursion is built into the protocol, not added on
   - Efficient proof composition
   - Constant-size proofs regardless of computation depth

3. **Incrementally Verifiable Computation (IVC)**
   - Perfect for iterative algorithms
   - Each step builds on the previous proof
   - Efficient for long-running computations

4. **Folding Schemes**
   - Novel approach to proof compression
   - More efficient than traditional recursion
   - Lower prover overhead

## Architecture

### High-Level Design

```
┌─────────────────────────────────────────────────────────┐
│                    Novanet zkVM                         │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  ┌─────────────┐              ┌──────────────┐        │
│  │   Guest     │              │     Host     │        │
│  │  Program    │──(input)──▶  │   Program    │        │
│  │             │              │              │        │
│  │ Computation │              │ ┌──────────┐ │        │
│  │  to Prove   │              │ │ Compiler │ │        │
│  └─────────────┘              │ └────┬─────┘ │        │
│                                │      │       │        │
│                                │ ┌────▼─────┐ │        │
│                                │ │  Prover  │ │        │
│                                │ └────┬─────┘ │        │
│                                │      │       │        │
│                                │ ┌────▼─────┐ │        │
│                                │ │ Verifier │ │        │
│                                │ └──────────┘ │        │
│                                └──────────────┘        │
│                                                         │
│              ┌──────────────────────────┐              │
│              │  Nova Proof System       │              │
│              │  (Simulated in Demo)     │              │
│              └──────────────────────────┘              │
└─────────────────────────────────────────────────────────┘
```

### Component Breakdown

#### 1. Guest Program (novanet-guest)

**Responsibilities:**
- Defines the computation to be proven
- Implements business logic
- Declares input/output structures

**Example:**
```rust
pub fn compute_fibonacci(input: FibInput) -> FibOutput {
    let result = fib::fibonacci(input.n);
    FibOutput { result }
}
```

**Key Points:**
- Runs in isolated environment (in production)
- No I/O operations during proving
- Deterministic computation

#### 2. Host Program (novanet-host)

**Responsibilities:**
- Orchestrates the proving process
- Compiles guest program
- Generates and verifies proofs
- Manages cryptographic operations

**Workflow:**
```rust
// 1. Compile guest
let prover = NovanetProver::compile_guest()?;

// 2. Generate proof
let proof = prover.prove(input)?;

// 3. Verify proof
let is_valid = prover.verify(&proof)?;
```

#### 3. Proof System (Simulated)

In a production implementation, this would include:
- Circuit compilation
- Nova folding scheme implementation
- Cryptographic operations
- Proof serialization/deserialization

## Technical Deep Dive

### Nova's Folding Scheme

Nova introduces a novel approach called "folding":

```
Traditional Recursion:
┌──────┐    ┌──────┐    ┌──────┐
│Proof1│───▶│Proof2│───▶│Proof3│
└──────┘    └──────┘    └──────┘
  Each proof verifies the previous one

Nova Folding:
┌──────────┐  fold  ┌──────────┐  fold  ┌──────────┐
│ Instance1│───────▶│Instance2 │───────▶│Instance3 │
└──────────┘        └──────────┘        └──────────┘
  Instances are "folded" together
```

**Benefits:**
- Lower computational overhead
- Constant-size intermediate representations
- More efficient than full verification

### Incrementally Verifiable Computation (IVC)

IVC allows proving long computations incrementally:

```
Step 1: Compute F(x₀) = x₁, Prove π₁
Step 2: Compute F(x₁) = x₂, Prove π₂ (includes π₁)
Step 3: Compute F(x₂) = x₃, Prove π₃ (includes π₂)
...
Final: Single proof πₙ proves entire computation
```

**Use Cases:**
- Blockchain state transitions
- Long-running computations
- Iterative algorithms
- Multi-round protocols

## Performance Characteristics

### Nova Performance Model

| Metric | Complexity | Notes |
|--------|-----------|-------|
| Prover Time | O(n) | Linear in computation size |
| Verifier Time | O(1) | Constant, very fast |
| Proof Size | O(1) | Constant, ~1-2 KB |
| Setup Time | O(n) | One-time, no trusted setup |
| Memory | O(n) | Linear in circuit size |

### Comparison with Other Systems

| System | Trusted Setup | Recursion | Prover Time | Proof Size |
|--------|--------------|-----------|-------------|------------|
| Groth16 | Yes | Hard | Fast | Small (~200B) |
| PLONK | Universal | Possible | Medium | Medium (~400B) |
| STARK | No | Possible | Fast | Large (~100KB) |
| Nova | No | Native | Medium | Small (~1-2KB) |
| Novanet | No | Native | Medium | Small (~1-2KB) |

## Use Cases

### Ideal Applications

1. **Blockchain State Transitions**
   ```
   Block₁ → Block₂ → Block₃ → ... → Blockₙ
   Single proof: "State transition is valid"
   ```

2. **Batch Processing**
   ```
   Txₙ₁, Txₙ₂, ..., Txₙₖ → Single proof
   Efficient verification of many transactions
   ```

3. **Iterative Algorithms**
   ```
   Loop iterations → Folded into single proof
   Perfect for Fibonacci, factorization, etc.
   ```

4. **Proof Aggregation**
   ```
   Multiple proofs → Single aggregated proof
   Reduce verification cost
   ```

### Not Ideal For

- One-shot computations (STARK may be better)
- Extremely small circuits (overhead not worth it)
- Non-iterative, tree-like computations

## Implementation Status

### What's Implemented ✅

- ✅ Project structure and build system
- ✅ Guest program template
- ✅ Host program orchestration
- ✅ Proof workflow simulation
- ✅ Performance tracking
- ✅ Comprehensive documentation
- ✅ Test suite
- ✅ Run scripts and tooling

### What's Simulated ⚠️

- ⚠️ Circuit compilation
- ⚠️ Nova folding operations
- ⚠️ Cryptographic proof generation
- ⚠️ Proof verification

### Production Requirements 🔧

For a production implementation, you would need:

1. **Nova Library Integration**
   ```toml
   [dependencies]
   nova-snark = "0.x"
   bellperson = "0.x"  # or arkworks
   ```

2. **Circuit Compiler**
   - RISC-V to R1CS conversion
   - Circuit optimization
   - Constraint generation

3. **Crypto Backend**
   - Elliptic curve operations
   - Polynomial commitments
   - Hash functions

4. **Toolchain**
   - Guest program compiler
   - Proof serialization
   - Verification key management

## Development Guide

### Adding Custom Computations

1. **Modify Guest Program**
   ```rust
   // novanet-guest/src/lib.rs
   pub fn your_computation(input: YourInput) -> YourOutput {
       // Your logic here
       YourOutput { result }
   }
   ```

2. **Update Host Program**
   ```rust
   // novanet-host/src/main.rs
   let input = YourInput { ... };
   let proof = prover.prove(input)?;
   ```

3. **Add Tests**
   ```rust
   #[test]
   fn test_your_computation() {
       // Test logic
   }
   ```

### Extending the System

#### Add Proof Composition
```rust
pub fn compose_proofs(
    proof1: NovanetProof,
    proof2: NovanetProof,
) -> Result<NovanetProof> {
    // Proof composition logic
}
```

#### Add Custom Circuits
```rust
pub struct CustomCircuit {
    // Circuit definition
}

impl Circuit for CustomCircuit {
    // Circuit implementation
}
```

## Security Considerations

### Simulation vs Production

⚠️ **Important**: This demo simulates proof generation. In production:

1. **Soundness**: Ensure cryptographic assumptions are met
2. **Completeness**: All valid proofs must verify
3. **Zero-Knowledge**: No information leakage
4. **Setup**: Proper parameter generation

### Best Practices

1. **Input Validation**
   - Validate all inputs before proving
   - Check bounds and constraints
   - Prevent overflow/underflow

2. **Circuit Design**
   - Minimize constraint count
   - Avoid complex operations
   - Optimize for prover time

3. **Proof Management**
   - Secure storage of verification keys
   - Proper serialization
   - Version compatibility

## Future Directions

### Near Term

- [ ] Integrate actual Nova implementation
- [ ] Add more example computations
- [ ] Performance benchmarking
- [ ] Circuit optimization tools

### Long Term

- [ ] Full RISC-V support
- [ ] Hardware acceleration (GPU)
- [ ] Proof composition library
- [ ] Production deployment guide

## Resources

### Papers and Research

- [Nova: Recursive Zero-Knowledge Arguments from Folding Schemes](https://eprint.iacr.org/2021/370)
- [SuperNova: Proving NIVC with folding](https://eprint.iacr.org/2022/1758)
- [HyperNova: Recursive arguments for customizable constraint systems](https://eprint.iacr.org/2023/573)

### Implementations

- [Microsoft Nova](https://github.com/microsoft/nova) - Original implementation
- [Lurk](https://github.com/lurk-lang/lurk-rs) - Language using Nova
- [zkVM Benchmarks](https://github.com/kkrt-labs/zkvm-benchmarks) - Performance comparisons

### Learning Resources

- [ZK Whiteboard Sessions](https://zkhack.dev/whiteboard/) - Video lectures
- [ZK Docs](https://www.zkdocs.com/) - Community documentation
- [Awesome Zero Knowledge](https://github.com/matter-labs/awesome-zero-knowledge-proofs)

## Contributing

This project welcomes contributions! Areas for improvement:

1. **Documentation**: Expand guides and tutorials
2. **Examples**: Add more use cases
3. **Integration**: Connect to actual Nova implementation
4. **Testing**: Expand test coverage
5. **Tooling**: Better development tools

## Acknowledgments

This implementation was inspired by:
- Nova paper by Kothapalli, Setty, and Tzialla
- Jolt zkVM project structure
- Nexus zkVM API design
- zkVM Benchmarks methodology

## License

MIT OR Apache-2.0

---

**Built with ❤️ for the ZK community**

