# zkvm-demos

## Risc0 zkvm

### Resources

* https://dev.risczero.com/api/zkvm/quickstart
* https://github.com/risc0/risc0

### how to run the Risc0 demo

* cd to the Risc0 demo directory

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

## Nexus zkvm

### Resources

* https://docs.nexus.xyz/zkvm/nexus-zkvm
* https://github.com/nexus-xyz/nexus-zkvm

### how to run the Nexus demo

* cd to the Nexus demo directory

```bash
cd nexus-zkvm/nexus-host
```

* run the Nexus demo

```bash
cargo run -r -- --nocapture
```

