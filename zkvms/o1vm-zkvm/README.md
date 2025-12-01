# o1vm zkVM Demo

This demo showcases the structure and concepts of [o1vm](https://github.com/o1-labs/proof-systems/tree/master/o1vm), a zkVM designed by O(1) Labs for proving MIPS program execution using the Kimchi proof system.

## Overview

**o1vm** is a zero-knowledge virtual machine that proves the correct execution of MIPS32 programs. It is part of the `proof-systems` repository developed by O(1) Labs for the Mina Protocol.

### Key Features

- **MIPS32 Architecture**: Proves execution of standard MIPS programs
- **Kimchi Proof System**: Uses PLONK-based proof system with custom gates
- **Pasta Curves**: Leverages Pallas/Vesta curves for efficient recursive proofs
- **Production-Ready**: Part of Mina Protocol's production infrastructure

## Project Structure

```
o1vm-zkvm/
├── o1vm-host/          # Rust host program (prover/verifier)
│   ├── src/
│   │   └── main.rs     # Main host implementation
│   └── Cargo.toml
├── o1vm-guest/         # MIPS guest programs (C/assembly)
│   ├── fibonacci.c     # Example Fibonacci program
│   └── Makefile        # Build system for MIPS programs
├── Cargo.toml          # Workspace configuration
├── rust-toolchain.toml
└── README.md
```

## Prerequisites

### For Host (Rust)

1. **Rust toolchain** (version 1.75+):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install 1.75
```

2. **proof-systems dependencies**: The project uses the o1-labs/proof-systems repository:
   - Kimchi: PLONK-based proof system
   - Polynomial commitment schemes
   - Mina curves (Pallas/Vesta)

### For Guest (MIPS Programs)

To compile the MIPS guest programs, you need a MIPS cross-compiler:

#### Ubuntu/Debian
```bash
sudo apt-get update
sudo apt-get install gcc-mips-linux-gnu binutils-mips-linux-gnu
```

#### macOS
MIPS cross-compilation on macOS requires either:
- **Docker**: Use a Linux container with MIPS toolchain
- **Homebrew with custom tap**: Limited support
- **Pre-compiled binaries**: Download from online sources

#### Verify Installation
```bash
mips-linux-gnu-gcc --version
```

## Building

### 1. Build the Guest Program (MIPS)

Navigate to the guest directory and compile:

```bash
cd o1vm-guest
make
```

This will generate:
- `fibonacci.elf` - MIPS ELF executable
- `fibonacci.bin` - Raw binary

### 2. Build the Host Program (Rust)

Navigate to the host directory:

```bash
cd o1vm-host
cargo build --release
```

## Running the Demo

### Quick Start

```bash
cd o1vm-host
cargo run --release
```

### Using the Run Script

```bash
./run_demo.sh
```

## Implementation Status

This demo provides a **conceptual framework** for o1vm. A complete implementation requires:

### ✅ Implemented
- [x] Project structure following zkvm-demos patterns
- [x] MIPS guest program (Fibonacci in C)
- [x] Basic host program structure
- [x] Documentation and build system

### ⚠️ Requires Full Integration
- [ ] MIPS interpreter/simulator
- [ ] Execution trace generation
- [ ] Witness generation from traces
- [ ] Kimchi circuit integration
- [ ] Polynomial commitment setup
- [ ] Proof generation
- [ ] Proof verification

## Understanding o1vm

### Architecture

```
┌─────────────────┐
│  MIPS Program   │  (C/Assembly code)
│   (Guest)       │
└────────┬────────┘
         │ Compile
         ▼
┌─────────────────┐
│  MIPS Binary    │  (ELF executable)
└────────┬────────┘
         │ Execute & Trace
         ▼
┌─────────────────┐
│ Execution Trace │  (Memory, registers, instructions)
└────────┬────────┘
         │ Convert to Witnesses
         ▼
┌─────────────────┐
│ Kimchi Circuit  │  (Polynomial constraints)
└────────┬────────┘
         │ Generate Proof
         ▼
┌─────────────────┐
│  ZK Proof       │  (Verifiable proof of execution)
└─────────────────┘
```

### Workflow

1. **Write MIPS Program**: Create C or assembly code
2. **Compile to MIPS**: Use cross-compiler to generate MIPS binary
3. **Execute & Trace**: Run program and record execution trace
4. **Generate Witnesses**: Convert trace to circuit witnesses
5. **Create Proof**: Use Kimchi to generate ZK proof
6. **Verify**: Verify the proof confirms correct execution

### Why MIPS?

- **Simplicity**: MIPS is a RISC architecture with clean instruction set
- **Standardization**: Well-documented and widely used in education
- **Efficiency**: Easier to create efficient circuits for simple instructions
- **Compatibility**: Can run standard C/C++ programs

## Comparison with Other zkVMs

| Feature | o1vm | RISC Zero | SP1 | Jolt |
|---------|------|-----------|-----|------|
| Architecture | MIPS32 | RISC-V | RISC-V | RISC-V |
| Proof System | Kimchi (PLONK) | STARK | STARK | Lookup-based |
| Curves | Pasta | BN254 | BN254 | BN254 |
| Recursion | Native | Yes | Yes | Limited |
| Production Use | Mina Protocol | Multiple | Growing | Research |

## Advanced Usage

### Custom MIPS Programs

Create your own MIPS programs in the `o1vm-guest` directory:

```c
// my_program.c
#include <stdint.h>

uint32_t my_function(uint32_t input) {
    // Your computation here
    return input * 2;
}

int main() {
    return my_function(42);
}
```

Compile with:
```bash
mips-linux-gnu-gcc -O2 -static -nostdlib -o my_program.elf my_program.c
```

### Docker Alternative

If you don't have MIPS toolchain installed, use Docker:

```bash
docker run --rm -v $(pwd):/work -w /work \
    ubuntu:22.04 bash -c \
    "apt-get update && apt-get install -y gcc-mips-linux-gnu && \
     cd o1vm-guest && make"
```

## Integration with Kimchi

To fully integrate o1vm with Kimchi, you would need to:

1. **Implement MIPS Interpreter**:
```rust
struct MipsInterpreter {
    registers: [u32; 32],
    memory: Vec<u8>,
    pc: u32,
}

impl MipsInterpreter {
    fn execute_instruction(&mut self, instr: u32) -> Trace {
        // Decode and execute MIPS instruction
        // Record execution trace
    }
}
```

2. **Generate Witnesses**:
```rust
fn generate_witnesses(trace: &ExecutionTrace) -> Vec<FieldElement> {
    // Convert execution trace to polynomial witnesses
}
```

3. **Create Kimchi Circuit**:
```rust
use kimchi::circuits::gate::CircuitGate;

fn create_mips_circuit(witnesses: &[FieldElement]) -> Vec<CircuitGate<F>> {
    // Define gates for MIPS operations
}
```

4. **Prove and Verify**:
```rust
// Generate proof
let proof = kimchi::prove(&prover_index, &witnesses)?;

// Verify proof
let is_valid = kimchi::verify(&verifier_index, &proof)?;
```

## Resources

### Official Documentation
- **proof-systems Repository**: https://github.com/o1-labs/proof-systems
- **o1vm Source Code**: https://github.com/o1-labs/proof-systems/tree/master/o1vm
- **Kimchi Documentation**: https://o1-labs.github.io/proof-systems/kimchi/overview.html
- **Online Book**: https://o1-labs.github.io/proof-systems/

### Learning Resources
- **Mina Protocol**: https://minaprotocol.com/
- **PLONK Paper**: [PLONK: Permutations over Lagrange-bases for Oecumenical Noninteractive arguments of Knowledge](https://eprint.iacr.org/2019/953)
- **Pasta Curves**: https://electriccoin.co/blog/the-pasta-curves-for-halo-2-and-beyond/

### Community
- **Discord**: Mina Protocol Discord
- **Forum**: https://forums.minaprotocol.com/

## Troubleshooting

### MIPS Toolchain Not Found

**Error**: `mips-linux-gnu-gcc: command not found`

**Solutions**:
1. Install the MIPS cross-compiler (see Prerequisites)
2. Use Docker to compile (see Docker Alternative)
3. Use pre-compiled MIPS binaries

### Dependency Build Errors

**Error**: Issues building `proof-systems` dependencies

**Solutions**:
1. Ensure Rust toolchain is up to date: `rustup update`
2. Check for system dependencies (OpenSSL, etc.)
3. Try using stable Rust instead of specific version
4. Check GitHub issues for known problems

### Missing Libraries

**Error**: `cannot find -lc` or similar linker errors

**Solutions**:
1. Install additional MIPS libraries: `apt-get install libc6-mips-cross`
2. Use `-static` flag in compilation
3. Adjust linker paths in Makefile

## Contributing

This demo is part of the `zkvm-demos` repository. Contributions are welcome!

1. Fork the repository
2. Create a feature branch
3. Implement your changes
4. Add tests and documentation
5. Submit a pull request

## Notes

- **Research Project**: o1vm is actively developed as part of Mina Protocol
- **Production Use**: Used in Mina blockchain for zkApp proofs
- **Performance**: Optimized for recursive proof composition
- **Complexity**: Full implementation requires deep understanding of:
  - MIPS architecture
  - Polynomial commitment schemes
  - PLONK protocol
  - Finite field arithmetic

## License

MIT OR Apache-2.0

## Acknowledgments

- **O(1) Labs**: For developing o1vm and the proof-systems framework
- **Mina Protocol**: For production use and continued development
- **zkVM Community**: For advancing zero-knowledge virtual machines

---

**Note**: This demo provides a structural and conceptual overview. For production use or research, refer to the official [proof-systems repository](https://github.com/o1-labs/proof-systems) and its comprehensive documentation.

