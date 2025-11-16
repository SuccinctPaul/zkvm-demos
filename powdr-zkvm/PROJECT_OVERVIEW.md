# Powdr zkVM Project Overview

## What is Powdr?

Powdr is a modular toolkit for building custom zero-knowledge virtual machines (zkVMs). Unlike monolithic zkVM solutions like RISC Zero, SP1, or Nexus, Powdr provides a flexible framework where developers can:

1. **Choose their frontend**: Support for RISC-V, WASM, or custom instruction sets
2. **Select their backend**: Choose from Halo2, Plonky2, STARK, or implement custom proving systems
3. **Define custom constraints**: Use PIL (Polynomial Identity Language) to specify circuit constraints
4. **Optimize for specific use cases**: Build application-specific zkVMs with optimal performance

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                   Powdr Toolkit                     │
├─────────────────────────────────────────────────────┤
│                                                     │
│  Frontend Layer                                     │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐         │
│  │ RISC-V   │  │   WASM   │  │  Custom  │         │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘         │
│       └─────────────┼─────────────┘                │
│                     │                               │
│  ┌──────────────────▼──────────────────┐           │
│  │  PIL (Polynomial Identity Language) │           │
│  │     Circuit Definition Layer         │           │
│  └──────────────────┬──────────────────┘           │
│                     │                               │
│       ┌─────────────┼─────────────┐                │
│       │             │             │                 │
│  ┌────▼────┐  ┌────▼────┐  ┌────▼────┐            │
│  │  Halo2  │  │ Plonky2 │  │  STARK  │            │
│  └─────────┘  └─────────┘  └─────────┘            │
│                Backend Layer                        │
└─────────────────────────────────────────────────────┘
```

## Key Components

### 1. PIL (Polynomial Identity Language)

PIL is Powdr's constraint language for defining arithmetic circuits. It allows you to:

- Define polynomial constraints
- Specify lookup tables
- Create custom gates
- Optimize circuit structure

Example:
```pil
// Simple Fibonacci constraint
pol commit a, b, c;
pol constant STEP;

// Constraint: c = a + b
c = a + b;

// Next step: b' = c, a' = b
b' = c;
a' = b;
```

### 2. Frontend Compilers

Powdr supports multiple frontends:

- **RISC-V**: Compile standard Rust code to RISC-V, then to Powdr circuit
- **WASM**: Compile from WebAssembly to Powdr circuit
- **Custom**: Define your own instruction set and compiler

### 3. Backend Proof Systems

Choose the proving system that fits your needs:

- **Halo2**: Recursive SNARKs with no trusted setup
- **Plonky2**: Fast recursive proofs with FRI
- **STARK**: Transparent setup with larger proof sizes
- **Custom**: Implement your own proving system

## Comparison with zkvm-benchmarks Implementation

The [zkvm-benchmarks repository](https://github.com/kkrt-labs/zkvm-benchmarks) contains a reference implementation of Powdr. Key aspects:

### Fibonacci Implementation

In zkvm-benchmarks, Powdr uses:

1. **Native field arithmetic**: Optimize Fibonacci using field operations
2. **Custom PIL constraints**: Define Fibonacci recurrence directly in PIL
3. **Lookup tables**: Precompute common values for faster proving
4. **Backend selection**: Compare Halo2 vs STARK backends

Example structure from benchmarks:
```
powdr/
├── guests/                    # Guest programs
│   └── fibonacci.rs
├── circuits/                  # PIL circuit definitions
│   └── fibonacci.pil
├── src/
│   └── main.rs               # Host program
└── Cargo.toml
```

### Performance Characteristics

From zkvm-benchmarks data:

| Input Size | Compile Time | Prove Time | Proof Size | Backend |
|-----------|--------------|------------|------------|---------|
| fib(10)   | ~1s         | ~2s        | ~2KB       | Halo2   |
| fib(100)  | ~1s         | ~5s        | ~2KB       | Halo2   |
| fib(1000) | ~2s         | ~15s       | ~2KB       | Halo2   |

| Input Size | Compile Time | Prove Time | Proof Size | Backend |
|-----------|--------------|------------|------------|---------|
| fib(10)   | ~1s         | ~1s        | ~100KB     | STARK   |
| fib(100)  | ~1s         | ~3s        | ~120KB     | STARK   |
| fib(1000) | ~2s         | ~8s        | ~150KB     | STARK   |

## Use Cases

Powdr is ideal for:

1. **Research**: Experiment with new proving techniques
2. **Custom zkVMs**: Build application-specific virtual machines
3. **Protocol Development**: Create specialized proving systems
4. **Performance Optimization**: Fine-tune circuits for specific workloads
5. **Educational**: Learn about zkVM internals and circuit design

## Development Workflow

### 1. Define Your Circuit

Write PIL constraints for your computation:

```pil
// fibonacci.pil
namespace Fibonacci;

pol commit input, a, b, result;
pol constant STEP;

// Initial conditions
a[0] = 0;
b[0] = 1;

// Recurrence relation
a' = b;
b' = a + b;

// Track steps
STEP' = STEP + 1;
```

### 2. Write Guest Program

Implement the computation logic:

```rust
// guest/src/main.rs
#![no_main]
#![no_std]

#[no_mangle]
pub extern "C" fn main() {
    let n = powdr_read_u32();
    let result = fibonacci(n);
    powdr_write_u32(result);
}
```

### 3. Compile to Circuit

Use Powdr CLI to compile:

```bash
powdr compile --frontend riscv --backend halo2 guest/src/main.rs
```

### 4. Generate Proof

Run the host program:

```rust
let proof = prover.prove(&input)?;
let is_valid = verifier.verify(&proof)?;
```

## Advanced Features

### Custom Instructions

Extend the instruction set with application-specific operations:

```pil
// Custom instruction for Fibonacci step
instr fib_step a, b -> c {
    c = a + b,
}
```

### Lookup Tables

Use precomputed tables for expensive operations:

```pil
// Lookup table for small Fibonacci values
pol constant FIB_TABLE;
lookup [input] in [FIB_TABLE];
```

### Batch Proving

Generate proofs for multiple executions efficiently:

```rust
let proofs = prover.prove_batch(&inputs)?;
```

### Circuit Optimization

Powdr provides tools to analyze and optimize circuits:

```bash
# Analyze circuit complexity
powdr analyze fibonacci.pil

# Optimize circuit structure
powdr optimize fibonacci.pil --output fibonacci_opt.pil
```

## Limitations

- **Complexity**: Requires understanding of circuit design
- **Setup Time**: More initial configuration than monolithic zkVMs
- **Documentation**: Still evolving as project develops
- **Ecosystem**: Smaller community compared to established zkVMs

## Future Directions

1. **More Backends**: Support for additional proving systems
2. **Improved Tooling**: Better debugging and profiling tools
3. **Standard Library**: Common PIL patterns and optimizations
4. **Cross-VM Compilation**: Easier porting between zkVMs
5. **Hardware Acceleration**: GPU support for proving

## Resources

- **GitHub**: https://github.com/powdr-labs/powdr
- **Documentation**: https://docs.powdr.org/
- **Examples**: https://github.com/powdr-labs/powdr/tree/main/examples
- **PIL Specification**: https://docs.powdr.org/pil/
- **zkVM Benchmarks**: https://github.com/kkrt-labs/zkvm-benchmarks

## Contributing

This demo implementation can be improved by:

1. Implementing actual Powdr SDK integration (when available)
2. Adding more circuit examples
3. Comparing different backend performance
4. Creating PIL optimizations
5. Adding comprehensive benchmarks

## References

1. Powdr GitHub Repository
2. zkvm-benchmarks Powdr implementation
3. PIL Language Specification
4. Zero-Knowledge Proof Systems literature
5. RISC-V and WASM specifications

## Version History

- v0.1.0: Initial reference implementation
  - Basic project structure
  - Placeholder for SDK integration
  - Documentation and examples

---

**Note**: This is a reference implementation demonstrating the expected workflow for Powdr zkVM. Actual SDK integration will be added when officially released.

