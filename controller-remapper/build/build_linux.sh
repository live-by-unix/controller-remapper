#!/usr/bin/env bash
#
# Controller Remapper - Linux Production Build
#

set -Eeuo pipefail

echo "========================================"
echo " Controller Remapper - Linux Build"
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

########################################
# Ensure Tauri CLI
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

if [[ ! -f src-tauri/tauri.conf.json && ! -f src-tauri/tauri.conf.json5 ]]; then
    echo "❌ This does not appear to be a Tauri project."
    exit 1
fi

mkdir -p build

########################################
# Helpful Dependency Info
########################################

echo
echo "Linux system packages may be required."

if command -v apt >/dev/null 2>&1; then
    echo
    echo "Ubuntu/Debian:"
    echo "sudo apt install \\"
    echo "    libwebkit2gtk-4.1-dev \\"
    echo "    libgtk-3-dev \\"
    echo "    libayatana-appindicator3-dev \\"
    echo "    librsvg2-dev"
elif command -v dnf >/dev/null 2>&1; then
    echo
    echo "Fedora:"
    echo "sudo dnf install \\"
    echo "    webkit2gtk4.1-devel \\"
    echo "    gtk3-devel \\"
    echo "    libappindicator-gtk3-devel \\"
    echo "    librsvg2-devel"
elif command -v pacman >/dev/null 2>&1; then
    echo
    echo "Arch Linux:"
    echo "sudo pacman -S \\"
    echo "    webkit2gtk \\"
    echo "    gtk3 \\"
    echo "    libappindicator-gtk3 \\"
    echo "    librsvg"
fi

########################################
# Build
########################################

echo
echo "Building production release..."
echo

if ! cargo tauri build; then
    echo
    echo "❌ Build failed."
    echo
    echo "Most Linux build failures are caused by missing system libraries."
    exit 1
fi

echo
echo "========================================"
echo "✅ Build completed successfully!"
echo "========================================"
echo

echo "Bundles generated:"

find src-tauri/target/release/bundle \
    -type f \
    \( \
        -name "*.AppImage" \
        -o -name "*.deb" \
        -o -name "*.rpm" \
        -o -name "*.tar.gz" \
    \)

echo
echo "Done!"
