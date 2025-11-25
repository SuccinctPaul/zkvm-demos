# zkVM SDK Installers

This directory contains installation scripts for various Zero-Knowledge Virtual Machine (zkVM) SDKs.

## Available Installers

- `install_airbender_sdk.sh` - Airbender zkVM (zkSync)
- `install_cairo_sdk.sh` - Cairo zkVM (StarkWare)
- `install_cairo_m_sdk.sh` - Cairo-M zkVM (KKRT Labs, M31 field, Stwo prover)
- `install_ceno_sdk.sh` - CENO zkVM (Scroll, using Nexus as implementation)
- `install_jolt_sdk.sh` - Jolt zkVM
- `install_miden_sdk.sh` - Miden zkVM
- `install_nexus_sdk.sh` - Nexus zkVM
- `install_novanet_sdk.sh` - Novanet zkVM
- `install_openvm_sdk.sh` - OpenVM zkVM
- `install_pico_sdk.sh` - Pico zkVM
- `install_powdr_sdk.sh` - Powdr zkVM Toolkit
- `install_risc0_sdk.sh` - Risc0 zkVM
- `install_sp1_sdk.sh` - SP1 zkVM
- `install_valida_sdk.sh` - Valida zkVM (LLVM-based, C/C++ support)
- `install_zisk_sdk.sh` - Zisk zkVM
- `install_zkm_sdk.sh` - ZKM zkVM
- `install_zkwasm_sdk.sh` - zkWasm zkVM

## Usage

To install a specific zkVM SDK, run the corresponding script:

```bash
# Example: Install Cairo SDK
./scripts/sdk_installers/install_cairo_sdk.sh

# Example: Install SP1 SDK
./scripts/sdk_installers/install_sp1_sdk.sh
```

Make sure the scripts are executable:

```bash
chmod +x scripts/sdk_installers/*.sh
```

## Notes

- Some installers may require sudo privileges
- Installation may take several minutes depending on your system and network speed
- Cairo uses Python/pip while other zkVMs typically use Rust/Cargo
- Follow the on-screen instructions during installation

### Special Notes for Specific zkVMs

#### Cairo-M zkVM
- Mobile-first zkVM using M31 (Mersenne 31) field
- Requires Rust nightly toolchain (installed automatically)
- Installs: `cairo-m-compiler`, `cairo-m-runner`, `cairo-m-prover`, `cargo-cairo-m`
- MacOS users: Requires LLVM and LLD (installed via Homebrew if needed)
- Installation directory: `~/.cairo-m`
- See `cairo-m-zkvm/README.md` for more details

#### CENO zkVM
- Currently uses Nexus zkVM as the implementation
- Once Scroll releases the official CENO SDK, the project can be migrated
- See `ceno-zkvm/README.md` for more details

#### Valida zkVM
- Supports two installation methods: Docker (recommended) and local installation
- Local installation requires LLVM 18.1.7+ and is officially supported on Ubuntu 24.04 and Arch Linux
- Docker method is recommended for other operating systems
- Example: `docker pull lita-xyz/valida`

## Reference

Original reference: https://github.com/eth-act/ere/tree/bd37fb0/scripts/sdk_installers