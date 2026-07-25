@echo off
setlocal

echo ========================================
echo  Controller Remapper - Windows Build
echo ========================================
echo.

REM -------------------------------------------------
REM Move to project root
REM -------------------------------------------------

cd /d "%~dp0.."

REM -------------------------------------------------
REM Dependency Checks
REM -------------------------------------------------

where rustc >nul 2>nul
if errorlevel 1 (
    echo ERROR: Rust is not installed.
    echo Install it from:
    echo https://rustup.rs/
    exit /b 1
)

where cargo >nul 2>nul
if errorlevel 1 (
    echo ERROR: Cargo not found.
    exit /b 1
)

REM -------------------------------------------------
REM Ensure Tauri CLI
REM -------------------------------------------------

cargo install --list | findstr /B "tauri-cli" >nul
if errorlevel 1 (
    echo Installing Tauri CLI...
    cargo install tauri-cli
)

REM -------------------------------------------------
REM Validate Project
REM -------------------------------------------------

if not exist src-tauri\tauri.conf.json (
    if not exist src-tauri\tauri.conf.json5 (
        echo ERROR: This does not appear to be a Tauri project.
        exit /b 1
    )
)

if not exist build mkdir build

REM -------------------------------------------------
REM Build
REM -------------------------------------------------

echo.
echo Building production release...
echo.

cargo tauri build

if errorlevel 1 (
    echo.
    echo ERROR: Build failed.
    exit /b 1
)

echo.
echo ========================================
echo Build completed successfully!
echo ========================================
echo.

echo Generated bundles:
dir /b /s src-tauri\target\release\bundle\*.msi 2>nul
dir /b /s src-tauri\target\release\bundle\*.exe 2>nul

echo.
echo Done!

endlocal
exit /b 0
