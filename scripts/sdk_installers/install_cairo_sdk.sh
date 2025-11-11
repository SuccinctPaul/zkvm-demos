#!/bin/bash

# Cairo SDK Installation Script
# This script installs the Cairo zkVM toolchain

set -e

echo "======================================"
echo "Cairo zkVM SDK Installation"
echo "======================================"
echo ""

# Check if Python is installed
if ! command -v python3 &> /dev/null; then
    echo "❌ Error: Python 3 is required but not installed."
    echo "Please install Python 3.7 or higher first."
    exit 1
fi

PYTHON_VERSION=$(python3 --version | cut -d' ' -f2 | cut -d'.' -f1,2)
echo "✓ Found Python version: $PYTHON_VERSION"

# Check Python version (needs 3.7+)
REQUIRED_VERSION="3.7"
if [ "$(printf '%s\n' "$REQUIRED_VERSION" "$PYTHON_VERSION" | sort -V | head -n1)" != "$REQUIRED_VERSION" ]; then
    echo "❌ Error: Python $REQUIRED_VERSION or higher is required."
    echo "Current version: $PYTHON_VERSION"
    exit 1
fi

# Create virtual environment (optional but recommended)
read -p "Do you want to create a virtual environment for Cairo? (recommended) [Y/n]: " create_venv
create_venv=${create_venv:-Y}

if [[ $create_venv =~ ^[Yy]$ ]]; then
    VENV_DIR="$HOME/.cairo-venv"
    
    if [ -d "$VENV_DIR" ]; then
        echo "Virtual environment already exists at $VENV_DIR"
    else
        echo "Creating virtual environment at $VENV_DIR..."
        python3 -m venv "$VENV_DIR"
    fi
    
    echo "Activating virtual environment..."
    source "$VENV_DIR/bin/activate"
    
    echo ""
    echo "Note: To use Cairo in the future, activate the environment with:"
    echo "  source $VENV_DIR/bin/activate"
    echo ""
fi

# Install Cairo
echo "Installing cairo-lang..."
pip install --upgrade pip
pip install cairo-lang

# Verify installation
echo ""
echo "Verifying Cairo installation..."
if command -v cairo-compile &> /dev/null; then
    CAIRO_VERSION=$(cairo-compile --version)
    echo "✓ Cairo installed successfully!"
    echo "  Version: $CAIRO_VERSION"
else
    echo "❌ Cairo installation verification failed."
    echo "Please check the installation manually."
    exit 1
fi

# Test Cairo
echo ""
echo "Testing Cairo installation..."
if command -v cairo-run &> /dev/null; then
    echo "✓ cairo-run command is available"
else
    echo "❌ cairo-run command not found"
    exit 1
fi

echo ""
echo "======================================"
echo "Cairo SDK Installation Complete!"
echo "======================================"
echo ""
echo "Quick Start:"
echo "  cd cairo-zkvm"
echo "  cairo-run --program=src/fib_simple.cairo --print_output --layout=small"
echo ""

if [[ $create_venv =~ ^[Yy]$ ]]; then
    echo "Remember to activate the virtual environment before using Cairo:"
    echo "  source $VENV_DIR/bin/activate"
    echo ""
fi

echo "Resources:"
echo "  • Documentation: https://www.cairo-lang.org/docs/"
echo "  • Cairo Book: https://book.cairo-lang.org/"
echo "  • Playground: https://www.cairo-lang.org/playground/"
echo ""

