# zkvm-demos

## Nexus zkvm

### Resources

* https://docs.nexus.xyz/zkvm/nexus-zkvm
* https://github.com/nexus-xyz/nexus-zkvm

### how to run the Nexus demo

* cd to the target demo directory

```bash
cd nexus-zkvm/nexus-host
```

* run the Nexus demo

```bash
cargo run -r -- --nocapture
```

## Risc0 zkvm

### Resources

* https://dev.risczero.com/api/zkvm/quickstart
* https://github.com/risc0/risc0

### how to run the Risc0 demo

* cd to the target demo directory

```bash
cd risc0/risc0-host
```

* run in dev mode

```bash
RISC0_DEV_MODE=1 RUST_LOG=info RISC0_INFO=1 cargo run --release
```

* run in production mode

```bash
RISC0_DEV_MODE=0 RUST_LOG=info RISC0_INFO=1 cargo run --release
```

## Sp1 zkvm

### Resources

* https://docs.succinct.xyz/docs/sp1/getting-started/quickstart
* https://github.com/succinctlabs/sp1

### how to run the Sp1 demo

* cd to the target demo directory

```bash
cd sp1-zkvm/sp1-host
```

* run in dev mode

```bash
RUST_LOG=info cargo run --release -- --execute
```

* run in production mode

```bash
RUST_LOG=info cargo run --release -- --prove
```




## ZKM zkvm

### Resources

* https://docs.zkm.io/introduction/quickstart.html
* https://github.com/ProjectZKM/Ziren

### how to run

* cd to the target demo directory
```bash
cd zkm-zkvm/zkm-host
```

* run in dev mode

```bash
RUST_LOG=info cargo run --release -- --execute
```

* run in production mode

```bash
cargo run --release -- --<PROOF_TYPE> // for core and compressed proofs 
cargo run --release --bin evm -- --system <PROOF_TYPE>  // for EVM-compatible proofs
```

* core proof
```bash
RUST_LOG=info cargo run --release -- --core
```

* compressed proof
```bash
RUST_LOG=info cargo run --release -- --compressed
```

* evm proof
```bash
cargo run --release --bin evm -- --system groth16

cargo run --release --bin evm -- --system plonk
```
