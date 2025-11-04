# ZKVM Docker Environment

Isolated Docker environments for multiple ZKVMs (SP1, Nexus, Risc0, ZKM) to avoid toolchain conflicts.

## 🚀 Quick Start

### First Time Setup
```bash
cd docker/scripts

# Build base image (required once)
./build-base.sh

# Build your ZKVM toolchain
./development-manager.sh build sp1
```

### Development Mode (Recommended)
```bash
cd docker/scripts

# SP1 - Execute & Prove
./development-manager.sh run sp1 --execute    # Execution without Proof
./development-manager.sh run sp1 --prove      # Generate proof

# Nexus - Auto prove
./development-manager.sh run nexus --nocapture

# Risc0 - Auto prove
./development-manager.sh run risc0

# ZKM - Auto prove
./development-manager.sh run zkm --execute    # Execution without Proof
./development-manager.sh run zkm --prove      # Generate proof

# Interactive debugging
./development-manager.sh shell sp1
```

## 📊 Development Mode

Development mode provides a flexible environment with real-time code changes and interactive debugging capabilities.

## 🔵 SP1 Commands

```bash
# Development mode
./development-manager.sh run sp1 --execute    # No proof
./development-manager.sh run sp1 --prove      # With proof
./development-manager.sh shell sp1            # Interactive
```

## 🟢 Nexus Commands

```bash
# Development mode
./development-manager.sh run nexus --nocapture
./development-manager.sh shell nexus
```

## 🔴 Risc0 Commands

```bash
# Development mode
./development-manager.sh run risc0
./development-manager.sh shell risc0
```

## 🟡 ZKM Commands

```bash
# Development mode
./development-manager.sh run zkm
./development-manager.sh shell zkm
```

## 🎯 Common Workflows

### Workflow 1: Rapid Development
```bash
# Edit code in your IDE
# Then run immediately (no rebuild)
./development-manager.sh run sp1 --execute
```

### Workflow 2: Full Testing
```bash
# Test all ZKVMs
for zkvm in sp1 nexus risc0 zkm; do
    ./development-manager.sh build $zkvm
    ./development-manager.sh run $zkvm
done
```

### Workflow 3: Debug Issues
```bash
# Enter container
./development-manager.sh shell sp1

# Inside container
cd sp1-zkvm/sp1-host
cargo build --release
RUST_LOG=debug cargo run --release -- --execute
```

## 🛠️ Management Commands

### Development Manager
```bash
./development-manager.sh build <zkvm>        # Build toolchain image
./development-manager.sh run <zkvm> [args]   # Run with args
./development-manager.sh shell <zkvm>        # Interactive shell
./development-manager.sh logs <zkvm>         # View logs
./development-manager.sh clean-cache         # Free space
```

## 🔍 Troubleshooting

### Error: "zkvm-base:latest not found"
**Auto-fix:** Scripts detect and build automatically
```bash
./development-manager.sh build sp1  # Will build base if needed
```

### Slow First Run?
**Normal:** First compilation takes 5-10 mins. Cached runs: 30s-2mins.

### Code Changes Not Applied?
**Solution:** Development mode automatically picks up code changes
```bash
# Development: automatic ✅
./development-manager.sh run sp1 --execute
```

### Out of Disk Space?
```bash
# Clean caches
./development-manager.sh clean-cache

# Clean everything
docker system prune -a
```

## 📁 Directory Structure

```
docker/
├── base/                    # Shared base image
├── development/             # Toolchain images  
├── scripts/                 # Management scripts
└── docs/                    # Detailed guides
    ├── DOCKER-USAGE-EXAMPLES.md  # Complete examples
    └── USAGE-GUIDE.md            # Detailed guide
```

## 📚 Documentation

- **[DOCKER-USAGE-EXAMPLES.md](docs/DOCKER-USAGE-EXAMPLES.md)** - Complete examples for all ZKVMs
- **[USAGE-GUIDE.md](docs/USAGE-GUIDE.md)** - Detailed usage patterns and best practices

## 💡 Tips

1. **Development mode** provides real-time code sync - no rebuilds needed
2. **Interactive shell** is great for debugging (`shell` command)
3. **First build is slow**, subsequent builds are cached
4. **Clean cache** when disk space is low

## 🎯 ZKVM Feature Matrix

| ZKVM | Execute Mode | Prove Mode | Interactive Shell |
|------|--------------|------------|-------------------|
| SP1 | `--execute` ✅ | `--prove` ✅ | ✅ |
| Nexus | Auto ⚡ | Auto ⚡ | ✅ |
| Risc0 | Auto ⚡ | Auto ⚡ | ✅ |
| ZKM | Auto ⚡ | Auto ⚡ | ✅ |

## ⚡ One-Line Commands

```bash
# Quick test SP1
cd docker/scripts && ./development-manager.sh build sp1 && ./development-manager.sh run sp1 --execute

# Build all ZKVMs
cd docker/scripts && for z in sp1 nexus risc0 zkm; do ./development-manager.sh build $z; done

# Interactive SP1 debug
cd docker/scripts && ./development-manager.sh shell sp1
```

## 🆘 Need Help?

1. Check [DOCKER-USAGE-EXAMPLES.md](docs/DOCKER-USAGE-EXAMPLES.md) for specific examples
2. Try the interactive menu: `./quick-start.sh`
3. View logs: `./development-manager.sh logs <zkvm>`
4. Enter container: `./development-manager.sh shell <zkvm>`

---

**Choose your mode, run your ZKVM, done! 🚀**
