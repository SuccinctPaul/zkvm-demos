# mise Toolchain Management Guide

This project uses [mise](https://mise.jdx.dev/) (formerly rtx) to manage multiple toolchains across different zkVM implementations.

## Why mise?

This repository contains 20+ zkVM implementations, each with different requirements:
- **Multiple Rust versions**: Jolt (1.88 stable) vs SP1/Risc0 (nightly)
- **Non-Rust dependencies**: Cairo (Python/Scarb), Valida (LLVM 18+)
- **zkVM-specific tools**: Different CLI tools for each zkVM
- **Automatic switching**: Change directories → tools switch automatically

mise solves the toolchain conflicts mentioned in the main README while providing native performance (unlike Docker).

## Quick Start

### 1. Install mise

```bash
# macOS
brew install mise

# Linux
curl https://mise.run | sh

# Or with cargo
cargo install mise
```

Add to your shell profile (`~/.zshrc` or `~/.bashrc`):

```bash
# mise activation
eval "$(mise activate zsh)"  # or bash/fish
```

Restart your terminal or run `source ~/.zshrc`.

### 2. Trust and Activate

Navigate to the project root:

```bash
cd zkvm-demos

# Trust the mise configuration
mise trust

# Install all tools defined in .mise.toml
mise install

# Verify installation
mise doctor
```

### 3. Check Installed Tools

```bash
# Check what tools are available
mise list

# Run the check-tools task
mise run check-tools

# Check current environment
mise current
```

## Usage

### Automatic Toolchain Switching

mise automatically switches toolchains when you `cd` into directories:

```bash
# Root directory uses nightly-2025-06-05
cd zkvm-demos
rustc --version  # 1.86.0-nightly (2025-06-05)

# Jolt uses stable 1.88
cd jolt-zkvm
rustc --version  # 1.88.0

# Cairo environment has Python + Scarb
cd ../cairo-zkvm
python --version  # 3.11.x
```

### Running Tasks

mise provides convenient tasks for each zkVM:

```bash
# Root-level tasks
mise run check-tools        # Check what's installed
mise run install-rust-tools # Install Rust targets
mise run list-zkvms         # List all zkVM projects

# Jolt-specific tasks (from jolt-zkvm/)
cd jolt-zkvm
mise run install-jolt       # Install Jolt toolchain
mise run run                # Run Jolt demo
mise run build-guest        # Build guest program

# Cairo-specific tasks (from cairo-zkvm/)
cd cairo-zkvm
mise run install-scarb      # Install Scarb
mise run build              # Build Cairo project
mise run test               # Run tests
mise run run                # Run Cairo program

# Risc0-specific tasks (from risc0-zkvm/)
cd risc0-zkvm
mise run install-risc0      # Install Risc0
mise run run-dev            # Run in dev mode
mise run run-prod           # Run in production mode

# SP1-specific tasks (from sp1-zkvm/)
cd sp1-zkvm
mise run install-sp1        # Install SP1
mise run run-execute        # Execute only
mise run run-prove          # Generate proof

# Valida-specific tasks (from valida-zkvm/)
cd valida-zkvm
mise run check-llvm         # Check LLVM version
mise run install-valida     # Install Valida
mise run compile-c          # Compile C to ELF
mise run prove              # Generate proof
```

### Local Overrides

Create a local configuration for your machine:

```bash
# Copy the example
cp .mise.local.toml.example .mise.local.toml

# Edit with your preferences
vim .mise.local.toml
```

Example `.mise.local.toml`:

```toml
[tools]
rust = "nightly-2025-06-15"  # Use a newer nightly

[env]
RUST_LOG = "debug"
RISC0_DEV_MODE = "1"
LLVM_SYS_180_PREFIX = "/usr/local/opt/llvm@18"  # Your LLVM path
```

## Project Structure

```
zkvm-demos/
├── .mise.toml                    # Root configuration (default tools)
├── .mise.local.toml              # Your local overrides (gitignored)
├── jolt-zkvm/
│   └── .mise.toml                # Jolt-specific (Rust 1.88)
├── cairo-zkvm/
│   └── .mise.toml                # Cairo-specific (Python + Scarb)
├── risc0-zkvm/
│   └── .mise.toml                # Risc0-specific (nightly)
├── sp1-zkvm/
│   └── .mise.toml                # SP1-specific (nightly)
└── valida-zkvm/
    └── .mise.toml                # Valida-specific (Rust + LLVM)
```

## Integration with Existing Tools

### With rust-toolchain.toml

mise works alongside `rust-toolchain.toml` files:
- mise provides the Rust version
- `rust-toolchain.toml` provides targets and components
- Both are respected by cargo

### With Docker

mise is complementary to Docker:
- **Development**: Use mise for native performance and IDE integration
- **CI/CD**: Use Docker for reproducible builds
- **Team**: Some members use mise, others use Docker

### With direnv

If you already use direnv, mise integrates seamlessly:

```bash
# .envrc
eval "$(mise activate bash --shims)"
```

## Troubleshooting

### Tools not switching automatically

```bash
# Make sure mise is activated in your shell
echo $MISE_SHELL  # Should show your shell

# Re-source your shell config
source ~/.zshrc

# Check mise hook is installed
mise doctor
```

### Rust version mismatch

```bash
# Clear mise cache
mise cache clear

# Reinstall Rust versions
mise install rust@1.88
mise install rust@nightly-2025-06-05

# Check what's active
mise current rust
```

### LLVM not found (Valida)

mise doesn't manage LLVM directly. Install manually:

```bash
# macOS
brew install llvm@18
echo 'export LLVM_SYS_180_PREFIX="/opt/homebrew/opt/llvm@18"' >> ~/.zshrc

# Ubuntu
wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh
sudo ./llvm.sh 18
```

Then add to your `.mise.local.toml`:

```toml
[env]
LLVM_SYS_180_PREFIX = "/path/to/llvm-18"
```

### Python version conflicts (Cairo)

```bash
# Check Python version
mise current python

# Install specific version
mise install python@3.11

# Use it globally or locally
mise use -g python@3.11
```

### Installation fails for a tool

```bash
# Try manual installation
mise install rust@1.88 --verbose

# Check logs
mise doctor

# Fall back to manual installation
rustup install 1.88
```

## Advanced Usage

### Pin versions per directory

```bash
# In any zkVM directory
cd sp1-zkvm

# Pin to specific versions
mise use rust@nightly-2025-06-05
mise use python@3.11

# This creates/updates .mise.toml in current directory
```

### List all available versions

```bash
# See all available Rust versions
mise ls-remote rust

# See all Python versions
mise ls-remote python

# See all installed versions
mise list
```

### Upgrade tools

```bash
# Upgrade all tools to latest versions
mise upgrade

# Upgrade specific tool
mise upgrade rust

# Install latest version
mise install rust@latest
```

### Environment variables

mise can manage environment variables:

```bash
# Set for current directory
mise set RUST_LOG=debug
mise set FIBONACCI_N=20

# Check environment
mise env

# Export to shell
eval "$(mise env)"
```

## Comparison with Other Solutions

| Feature | mise | Docker | direnv | asdf |
|---------|------|--------|--------|------|
| **Setup Time** | 5 min | 10 min | 2 min | 5 min |
| **Performance** | Native | -10% overhead | Native | Native |
| **Auto-switch** | ✅ Yes | ❌ Manual | ✅ Yes | ✅ Yes |
| **Multi-language** | ✅ Yes | ✅ Yes | ⚠️ Manual | ✅ Yes |
| **Learning curve** | Easy | Medium | Easy | Easy |
| **IDE integration** | ✅ Seamless | ⚠️ Setup needed | ✅ Good | ✅ Good |
| **CI/CD** | ✅ Yes | ✅ Best | ❌ No | ✅ Yes |
| **Isolation** | Good | Excellent | None | Good |
| **Speed** | Fast | Slower | Fast | Medium |

**Recommendation:**
- **Solo dev, frequent builds**: mise (this guide)
- **Team collaboration, CI/CD**: Docker (see [ISOLATION-DOCKER.md](docs/ISOLATION-DOCKER.md))
- **Just environment vars**: direnv
- **Familiar with asdf**: mise (same syntax, faster)

## Resources

- **mise Documentation**: https://mise.jdx.dev/
- **GitHub**: https://github.com/jdx/mise
- **Discord**: https://discord.gg/mise
- **Comparison with asdf**: https://mise.jdx.dev/comparison-to-asdf.html

## Getting Help

1. **Check mise docs**: `mise help` or https://mise.jdx.dev/
2. **Run diagnostics**: `mise doctor`
3. **Check zkVM-specific README**: Each zkVM directory has detailed instructions
4. **Fall back to Docker**: See root [README.md](../README.md) for Docker setup

## Contributing

When adding a new zkVM:

1. Create `.mise.toml` in the zkVM directory
2. Specify required tools (Rust version, etc.)
3. Add convenience tasks (install, build, run)
4. Update this guide with new requirements
5. Test automatic switching: `cd new-zkvm && mise current`

Example template:

```toml
# new-zkvm/.mise.toml
[tools]
rust = "1.85"

[env]
RUST_LOG = "info"

[tasks.install]
description = "Install this zkVM's toolchain"
run = "../scripts/sdk_installers/install_new_zkvm.sh"

[tasks.run]
description = "Run demo"
run = "cargo run --release"
```

---

**Happy ZK proving with mise! 🚀**

For Docker-based isolation, see [ISOLATION-DOCKER.md](docs/ISOLATION-DOCKER.md).

