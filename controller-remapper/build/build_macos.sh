#!/usr/bin/env bash
#
# Controller Remapper - macOS Production Build Script
#

set -Eeuo pipefail

echo "========================================"
echo " Controller Remapper - macOS Build"
echo "========================================"
echo

PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$PROJECT_ROOT"

########################################
# Dependency Checks
########################################

if ! command -v rustc >/dev/null 2>&1; then
    echo "❌ Rust is not installed."
    echo "Install it from:"
    echo "https://rustup.rs/"
    exit 1
fi

if ! command -v cargo >/dev/null 2>&1; then
    echo "❌ Cargo not found."
    exit 1
fi

if ! command -v brew >/dev/null 2>&1; then
    echo "❌ Homebrew not found."
    echo "https://brew.sh/"
    exit 1
fi

########################################
# Ensure pkg-config exists
########################################

if ! brew list pkg-config >/dev/null 2>&1; then
    echo "Installing pkg-config..."
    brew install pkg-config
else
    echo "✓ pkg-config already installed"
fi

########################################
# Ensure Tauri CLI exists
########################################

if ! cargo install --list | grep -q "^tauri-cli "; then
    echo "Installing Tauri CLI..."
    cargo install tauri-cli
else
    echo "✓ Tauri CLI already installed"
fi

########################################
# Validate Project
########################################

if [[ ! -f "src-tauri/tauri.conf.json" && ! -f "src-tauri/tauri.conf.json5" ]]; then
    echo "❌ Not a Tauri project."
    exit 1
fi

mkdir -p build

########################################
# Build
########################################

echo
echo "Building production release..."
echo

cargo tauri build

echo
echo "========================================"
echo "✅ Build completed successfully!"
echo "========================================"
echo

echo "Bundles are located in:"
echo "src-tauri/target/release/bundle/"
echo

find src-tauri/target/release/bundle -maxdepth 2 -type f \
    \( -name "*.app" -o -name "*.dmg" -o -name "*.pkg" \)

echo
echo "Done!"
