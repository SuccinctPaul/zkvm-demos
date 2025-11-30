# ZKsync Airbender zkVM Demo

High-Performance RISC-V Zero-Knowledge Prover from Matter Labs.

## SDK Integration

Implementation based on [ere project's Airbender SDK](https://github.com/eth-act/ere/blob/master/crates/zkvm/airbender/src/zkvm/sdk.rs).

### Dependencies

```toml
airbender_execution_utils = { 
    git = "https://github.com/matter-labs/zksync-airbender.git", 
    package = "execution_utils", 
    tag = "v0.5.0" 
}
```

## Usage

### With Pre-compiled Guest Binary (Full SDK)

```bash
# Set path to pre-compiled guest binary
export AIRBENDER_GUEST_BIN=./guest.bin

# Optional: Enable GPU acceleration
export AIRBENDER_GPU=true

# Run
./target/release/airbender-host
```

### Without Guest Binary (Native Fallback)

```bash
# Runs native execution fallback
./target/release/airbender-host
```

## Building Guest Binary

```bash
# 1. Install RISC-V toolchain
rustup target add riscv32im-unknown-none-elf

# 2. Build guest program
cd airbender-guest
cargo build --release --target riscv32im-unknown-none-elf

# 3. Convert to binary
rust-objcopy -O binary \
    target/riscv32im-unknown-none-elf/release/airbender-guest \
    guest.bin
```

## SDK API

Based on [ere/airbender/sdk.rs](https://github.com/eth-act/ere/blob/master/crates/zkvm/airbender/src/zkvm/sdk.rs):

```rust
pub struct AirbenderSdk {
    bin: Vec<u8>,           // Pre-compiled guest binary
    vk_hash_chain: [u32; 8], // VK hash chain
    gpu: bool,              // Use GPU acceleration
}

impl AirbenderSdk {
    // Initialize SDK with guest binary
    pub fn new(bin: &[u8], gpu: bool) -> Self;
    
    // Execute and return (public_values, cycles)
    pub fn execute(&self, input: &[u8]) -> Result<(Vec<u8>, u64)>;
    
    // Generate proof
    pub fn prove(&self, input: &[u8]) -> Result<(Vec<u8>, ProgramProof)>;
    
    // Verify proof
    pub fn verify(&self, proof: &ProgramProof) -> Result<Vec<u8>>;
}
```

## CLI Commands

The SDK internally uses `airbender-cli`:

```bash
# Execute
airbender-cli run --bin guest.bin --input-file input.hex --cycles MAX

# Prove (1st recursion layer)
airbender-cli prove --bin guest.bin --input-file input.hex \
    --output-dir output/ --until final-recursion

# Prove (2nd recursion layer)  
airbender-cli prove-final --input-file recursion_program_proof.json \
    --output-dir output/
```

## References

- [ZKsync Airbender](https://github.com/matter-labs/zksync-airbender)
- [ERE Airbender Integration](https://github.com/eth-act/ere/tree/master/crates/zkvm/airbender)
- [basic_fibonacci Example](https://github.com/matter-labs/zksync-airbender/tree/main/examples/basic_fibonacci)

## License

MIT OR Apache-2.0
