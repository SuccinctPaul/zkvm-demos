# zkVM SDK Installers

This directory contains installation scripts for various Zero-Knowledge Virtual Machine (zkVM) SDKs.

## Available Installers

- `install_cairo_sdk.sh` - Cairo zkVM (StarkWare)
- `install_jolt_sdk.sh` - Jolt zkVM
- `install_nexus_sdk.sh` - Nexus zkVM
- `install_openvm_sdk.sh` - OpenVM zkVM
- `install_pico_sdk.sh` - Pico zkVM
- `install_risc0_sdk.sh` - Risc0 zkVM
- `install_sp1_sdk.sh` - SP1 zkVM
- `install_zisk_sdk.sh` - Zisk zkVM
- `install_zkm_sdk.sh` - ZKM zkVM

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

## Reference

Original reference: https://github.com/eth-act/ere/tree/bd37fb0/scripts/sdk_installers