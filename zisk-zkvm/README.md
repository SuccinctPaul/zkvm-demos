> NOTE: Generate Proof not support Macos yet.

## Install

2. **ZisK Toolchain**: Install using ziskup
   ```bash
   curl https://raw.githubusercontent.com/0xPolygonHermez/zisk/main/ziskup/install.sh | bash
   ```

   Or use the provided script:
   ```bash
   cd /path/to/zkvm-demos
   ./scripts/sdk_installers/install_zisk_sdk.sh
   ```

## How to Run

* build

```json
cargo-zisk build --release
```

* execution

```json
$ cargo-zisk run --release -i build/input.bin 
```