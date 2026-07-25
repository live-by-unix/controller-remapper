@echo off
REM Controller Remapper - Windows Build Script
REM This script builds the application for Windows with production settings

echo ========================================
echo Controller Remapper - Windows Production Build
echo ========================================
echo.

REM Check if Rust is installed
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Rust/Cargo not found. Please install Rust from https://rustup.rs/
    exit /b 1
)

REM Install Tauri CLI if not present
cargo tauri --version >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo Installing Tauri CLI...
    cargo install tauri-cli --version "^2.0"
)

REM Navigate to project root
cd /d "%~dp0.."

REM Create build directory
if not exist build mkdir build

echo Building release version with production optimizations...
cargo tauri build --release

if %ERRORLEVEL% NEQ 0 (
    echo ERROR: Build failed
    exit /b 1
)

echo.
echo ========================================
echo Build completed successfully!
echo ========================================
echo.
echo Output location: src-tauri\target\release\bundle\
echo.

echo Done!
exit /b 0
