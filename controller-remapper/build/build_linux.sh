#!/bin/bash
# Controller Remapper - Linux Build Script
# This script builds the application for Linux with production settings
# Note: Some system dependencies may need to be installed without sudo

set -e

echo "========================================"
echo "Controller Remapper - Linux Production Build"
echo "========================================"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "ERROR: Rust/Cargo not found. Please install Rust from https://rustup.rs/"
    exit 1
fi

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
echo "Note: If build fails due to missing system dependencies, you may need to install:"
echo "  - libwebkit2gtk-4.1-dev"
echo "  - libgtk-3-dev"
echo "  - libayatana-appindicator3-dev"
echo "  - librsvg2-dev"
echo ""

cargo tauri build --release

if [ $? -ne 0 ]; then
    echo "ERROR: Build failed"
    echo ""
    echo "If the error is about missing system libraries, try installing them:"
    echo "  sudo apt-get install libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev"
    echo ""
    echo "Or use a container-based build (Docker/Podman) if you don't have sudo access."
    exit 1
fi

echo ""
echo "========================================"
echo "Build completed successfully!"
echo "========================================"
echo ""
echo "Output location: src-tauri/target/release/bundle/"
echo ""

echo "Done!"
exit 0
