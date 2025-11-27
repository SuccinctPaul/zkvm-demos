# ZKsync Airbender zkVM Demo

High-Performance RISC-V Zero-Knowledge Prover from Matter Labs.

## SDK Integration

This project integrates the official [ZKsync Airbender](https://github.com/matter-labs/zksync-airbender) SDK (v0.5.0).

### References

- **Official Repository**: https://github.com/matter-labs/zksync-airbender
- **Example (basic_fibonacci)**: https://github.com/matter-labs/zksync-airbender/tree/main/examples/basic_fibonacci
- **ERE Integration Reference**: https://github.com/eth-act/ere/blob/master/crates/zkvm/airbender

### SDK Dependencies

```toml
# From workspace Cargo.toml
airbender_execution_utils = { 
    git = "https://github.com/matter-labs/zksync-airbender.git", 
    package = "execution_utils", 
    tag = "v0.5.0" 
}
airbender_riscv_common = { 
    git = "https://github.com/matter-labs/zksync-airbender.git", 
    package = "riscv_common", 
    tag = "v0.5.0" 
}
```

## Project Structure

```
airbender-zkvm/
├── airbender-guest/     # RISC-V guest program (for zkVM execution)
│   └── src/
│       ├── main.rs      # Guest program entry point
│       └── asm_reduced.S # Assembly entry point
├── airbender-host/      # Host program (benchmark runner)
│   └── src/
│       └── main.rs      # Uses execution_utils for VK computation
├── Cargo.toml           # Workspace configuration
└── rust-toolchain.toml  # Nightly Rust required
```

## Requirements

### For Host Program (Benchmarking)

- Rust nightly toolchain
- ZKsync Airbender SDK (automatically fetched via Cargo)

### For Full Proving

1. **airbender-cli**: Install from https://github.com/matter-labs/zksync-airbender
2. **GPU Support**: CUDA for GPU-accelerated proving
3. **Pre-compiled Guest Binary**: RISC-V ELF compiled to `.bin` format

## Usage

### Running Benchmarks

```bash
cd airbender-zkvm

# Build host program
cargo build --release -p airbender-host

# Run benchmark (native fallback)
PROGRAM_ID=0 SCALE=50 ./target/release/airbender-host
```

### Full Proving (with airbender-cli)

```bash
# 1. Compile guest to RISC-V
cargo build --release -p airbender-guest --target riscv32im-unknown-none-elf

# 2. Convert to binary format (requires objcopy)
rust-objcopy --output-target binary target/riscv32im-unknown-none-elf/release/airbender-guest guest.bin

# 3. Run with airbender-cli
airbender-cli run --bin guest.bin --input-file input.hex --cycles 1000000

# 4. Generate proof
airbender-cli prove --bin guest.bin --input-file input.hex --output-dir output/
```

## SDK API Reference

### Host Side (execution_utils)

```rust
use airbender_execution_utils::{
    Machine,                          // Machine type (Standard, etc.)
    ProgramProof,                     // Proof structure
    compute_chain_encoding,           // VK hash chain computation
    generate_params_for_binary,       // Generate VK params from binary
    universal_circuit_verifier_vk,    // Universal verifier VK
    verify_recursion_log_23_layer,    // Proof verification
};
```

### Guest Side (riscv_common)

```rust
use airbender_riscv_common::{
    zksync_os_finish_success,         // Signal successful execution
    zksync_os_finish_error,           // Signal error
    csr_read_word,                    // Read from CSR
    csr_write_word,                   // Write to CSR
};
```

## Output Convention

Airbender uses registers 10-25 for output:
- **x10-x17**: Public values (8 x u32 = 256 bits)
- **x18-x25**: VK hash chain for recursion

## CLI Output Format

When running with `airbender-cli run`:
```
Result: {v0}, {v1}, {v2}, {v3}, {v4}, {v5}, {v6}, {v7}
Took {cycles} cycles to finish
```

## Performance

Airbender targets:
- ~21.8 MHz proving speed on H100 GPU
- Full RISC-V ISA compatibility
- Optimized for Ethereum state transitions

## License

MIT OR Apache-2.0

## Links

- [ZKsync Airbender](https://github.com/matter-labs/zksync-airbender)
- [ZKsync](https://zksync.io/)
- [Matter Labs](https://matter-labs.io/)
