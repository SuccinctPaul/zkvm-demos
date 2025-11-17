# mise Quick Start - 5 Minutes Setup

Get started with mise for zkvm-demos in 5 minutes.

## What is mise?

mise automatically manages different tool versions (Rust, Python, etc.) per directory.
When you `cd` into a zkVM directory, the right tools are automatically activated.

## 1. Install mise

### macOS
```bash
brew install mise
```

### Linux
```bash
curl https://mise.run | sh
```

### Or use cargo
```bash
cargo install mise
```

## 2. Activate in Shell

Add to `~/.zshrc` (or `~/.bashrc` for bash):

```bash
eval "$(mise activate zsh)"  # or: bash, fish
```

Then restart terminal or:
```bash
source ~/.zshrc
```

## 3. Setup Project

```bash
cd zkvm-demos

# Trust mise configurations
mise trust

# Install all tools
mise install
```

## 4. Test It!

```bash
# Root directory - uses nightly
cd zkvm-demos
rustc --version
# → rustc 1.86.0-nightly (2025-06-05)

# Jolt - uses stable 1.88
cd jolt-zkvm
rustc --version
# → rustc 1.88.0

# Cairo - has Python
cd ../cairo-zkvm
python --version
# → Python 3.11.x
```

## 5. Try Tasks

```bash
# Check what's installed
mise run check-tools

# Go to any zkVM and see available tasks
cd sp1-zkvm
mise tasks

# Run a task
mise run install-sp1
mise run run-execute
```

## That's It!

- Tools switch automatically when you `cd`
- Each zkVM has its own configuration
- Native performance (no Docker overhead)
- Works with your IDE

## Common Commands

```bash
mise list              # Show installed tools
mise current           # Show active tools
mise run <task>        # Run a task
mise doctor            # Check setup
```

## Need Help?

- Full guide: [MISE_SETUP.md](MISE_SETUP.md) | [中文版](MISE_SETUP.zh-CN.md)
- Verify setup: `./scripts/verify_mise_setup.sh`
- Official docs: https://mise.jdx.dev/

## Troubleshooting

**Tools not switching?**
```bash
echo $MISE_SHELL  # Should show "zsh" or "bash"
# If empty, check that mise activate is in your shell config
```

**Wrong Rust version?**
```bash
mise current rust      # Check what's active
mise install rust@1.88 # Install specific version
```

**Want to use Docker instead?**

See [README.md](README.md) for Docker-based isolation.

---

**Happy coding! 🚀**

