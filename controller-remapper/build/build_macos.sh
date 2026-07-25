#!/bin/bash
# Controller Remapper - macOS Build Script
# This script builds the application for macOS with production settings

set -e

echo "========================================"
echo "Controller Remapper - macOS Production Build"
echo "========================================"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "ERROR: Rust/Cargo not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check if Homebrew is installed
if ! command -v brew &> /dev/null; then
    echo "ERROR: Homebrew not found. Please install Homebrew from https://brew.sh/"
    exit 1
fi

# Install system dependencies
echo "Installing system dependencies..."
brew install pkg-config

# Install Tauri CLI if not present
if ! cargo tauri --version &> /dev/null; then
    echo "Installing Tauri CLI..."
    cargo install tauri-cli --version "^2.0"
fi

# Navigate to project root
cd "$(dirname "$0")/.."

# Create build directory
mkdir -p build

echo "Building release version with production optimizations..."
cargo tauri build --release

if [ $? -ne 0 ]; then
    echo "ERROR: Build failed"
    exit 1
fi

echo ""
echo "========================================"
echo "Build completed successfully!"
echo "========================================"
echo ""
echo "Output location: src-tauri/target/release/bundle/macos/"
echo ""

echo "Done!"
exit 0
