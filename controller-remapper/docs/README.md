# Controller Remapper

A production-ready Steam utility for mapping controller buttons to keyboard keys with full Steam Workshop integration, cloud sync, and cross-platform support.

## Overview

Controller Remapper is a comprehensive application that allows gamers to:
- **Detect Bluetooth and wired controllers** across Windows, macOS, and Linux
- **Map controller buttons and axes** to keyboard keys with customizable deadzones and sensitivity
- **Create and save profiles** per game with JSON-based configuration
- **Share profiles via Steam Workshop** with the community
- **Sync profiles to Steam Cloud** for access across devices
- **Use Steam Overlay** for in-game profile switching

The application uses a modern Tauri-based frontend with Rust backend, ensuring performance and reliability while providing a clean, intuitive user interface.

## Features

### Controller Detection
- **Bluetooth Support**: BlueZ (Linux), CoreBluetooth (macOS), Windows.Gaming.Input (Windows)
- **HID Support**: Direct HID device detection via hidapi
- **SDL2 Integration**: Reliable controller detection and input handling
- **Automatic Scanning**: Continuous background detection of connected controllers

### Button Mapping
- **Visual Controller Layout**: Interactive UI showing controller buttons
- **Drag-and-Drop Mapping**: Click controller buttons and press keyboard keys to map
- **Axis Mapping**: Map analog sticks to keyboard keys with configurable deadzones
- **Trigger Mapping**: Map triggers to keyboard keys with sensitivity controls
- **Real-time Testing**: Test mappings immediately as you configure them

### Profile Management
- **JSON-based Profiles**: Human-readable profile files for easy sharing
- **Per-Game Profiles**: Create separate profiles for different games
- **Profile Duplication**: Quickly create variations of existing profiles
- **Import/Export**: Share profiles via file import/export
- **Search and Filter**: Quickly find profiles by game, name, or tags

### Steam Integration
- **Steam Workshop**: Publish and download community profiles
- **Cloud Sync**: Automatic backup of profiles to Steam Cloud
- **Steam Overlay**: In-game profile switching without leaving the game
- **Steam User Integration**: Display Steam user information and stats

## Installation

### Prerequisites

#### Common Requirements
- **Rust**: Install from [rustup.rs](https://rustup.rs/)
- **Node.js**: Install from [nodejs.org](https://nodejs.org/) (for frontend development)
- **Steam Client**: Required for Steam features

#### Platform-Specific Requirements

**Windows:**
- Microsoft Visual C++ Build Tools
- Windows 10 or later

**macOS:**
- Xcode Command Line Tools
- macOS 10.15 (Catalina) or later

**Linux:**
- Build tools: `sudo apt-get install build-essential`
- WebKitGTK: `sudo apt-get install libwebkit2gtk-4.0-dev`
- SSL dev: `sudo apt-get install libssl-dev`
- HID support: `sudo apt-get install libudev-dev`
- SDL2: `sudo apt-get install libsdl2-dev`
- BlueZ (for Bluetooth): `sudo apt-get install bluez libbluetooth-dev`

### Building from Source

#### Windows (MSVC)
```batch
cd build
build_windows.bat
```

#### macOS (clang)
```bash
cd build
chmod +x build_macos.sh
./build_macos.sh
```

#### Linux (gcc)
```bash
cd build
chmod +x build_linux.sh
./build_linux.sh
```

### Installing Pre-built Binaries

Pre-built binaries are available in the `build/` directory after running the build scripts. Install them according to your platform:

**Windows:** Run the `.msi` installer
**macOS:** Open the `.dmg` file and drag to Applications
**Linux:** Install the `.deb` package: `sudo dpkg -i controller-remapper.deb`

## How to Build for Steam

### 1. Obtain Steam App ID

To build for Steam, you need a Steam App ID. Contact Valve through [Steamworks](https://partner.steamgames.com/) to obtain an App ID for your application.

### 2. Configure Steamworks SDK

The Steamworks SDK should be placed in the `sdk/` directory relative to the project root. The SDK structure should match:

```
sdk/
├── public/
├── redistributable_bin/
└── ...
```

### 3. Set steam_appid.txt

Create a `steam_appid.txt` file in the project root with your Steam App ID:

```
480
```

For testing, you can use App ID 480 (Spacewar), which is Valve's test app.

### 4. Build with Steam Integration

Run the appropriate build script for your platform. The build process will:
- Link against the Steamworks SDK
- Include Steam API initialization
- Package the application with Steam integration

### 5. Test with Steam Client

Before building for release, test with the Steam client:

1. Ensure Steam is running
2. Place `steam_appid.txt` in the build output directory
3. Run the application
4. Verify Steam features work (Workshop, Cloud, Overlay)

### Platform-Specific Steam Build Notes

**Windows:**
- The build script automatically links against Steamworks libraries
- Output includes required Steam DLLs in the redistributable package

**macOS:**
- Ensure code signing is configured for Steam distribution
- The build script handles framework linking

**Linux:**
- The build script includes Steam runtime libraries
- Package includes Steam depot upload scripts

## How to Run Tests

### Unit Tests

Run unit tests for the mapping logic:

```bash
cargo test --lib
```

### Integration Tests

Run integration tests with virtual controllers:

```bash
cargo test --test '*'
```

### Specific Test Suites

Run only mapping tests:
```bash
cargo test mapping_tests
```

Run only controller tests:
```bash
cargo test controller_tests
```

### Test Coverage

Generate test coverage report:
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --out Html
```

## How to Publish Profiles to Steam Workshop

### 1. Create a Profile

1. Open Controller Remapper
2. Go to the "Profiles" section
3. Click "Create Profile"
4. Configure your button and axis mappings
5. Save the profile

### 2. Prepare for Workshop

1. Ensure your profile has:
   - A descriptive name
   - A clear description
   - Appropriate tags (e.g., "fps", "racing", "platformer")
   - The game name it's designed for

2. Test the profile thoroughly in-game to ensure mappings work correctly

### 3. Publish to Workshop

1. Go to the "Workshop" section
2. Click "Upload" on your profile
3. Add a preview image (optional but recommended)
4. Add a detailed description
5. Set appropriate tags
6. Click "Publish"

### 4. Manage Workshop Items

- **Update**: Modify your profile and re-upload to update the Workshop item
- **Subscribe**: Browse and subscribe to community profiles
- **Rate**: Upvote/downvote profiles to help the community
- **Share**: Share Workshop item links with others

## Troubleshooting

### Controller Not Detected

**Problem**: Controller not showing up in the application

**Solutions**:
1. Click "Scan for Controllers" to refresh the list
2. Ensure the controller is properly connected via USB or Bluetooth
3. Check if the controller works in other applications
4. On Linux, ensure your user is in the `input` group: `sudo usermod -a -G input $USER`
5. On Windows, ensure Xbox controller drivers are installed
6. Try a different USB port or cable

**Linux-specific**:
```bash
# Check if controller is detected by system
ls /dev/input/js*
# Check permissions
sudo chmod 666 /dev/input/js*
```

### Bluetooth Detection Issues

**Problem**: Bluetooth controllers not detected

**Solutions**:
1. Ensure Bluetooth is enabled on your system
2. On Linux, ensure BlueZ is running: `sudo systemctl status bluetooth`
3. Pair the controller with your system first
4. Check Bluetooth logs for errors

**Linux Bluetooth troubleshooting**:
```bash
# Restart Bluetooth service
sudo systemctl restart bluetooth

# Check Bluetooth devices
bluetoothctl devices

# Pair controller interactively
bluetoothctl
scan on
pair <controller-mac>
connect <controller-mac>
trust <controller-mac>
```

### Steam Features Not Working

**Problem**: Steam Workshop, Cloud, or Overlay not working

**Solutions**:
1. Ensure Steam client is running
2. Check that `steam_appid.txt` exists in the application directory
3. Verify the Steam App ID is correct
4. Check Steam logs for errors
5. Ensure you're logged into Steam

**Steam App ID issues**:
- For development/testing, use App ID 480 (Spacewar)
- For production, use your assigned Steam App ID
- Ensure `steam_appid.txt` contains only the App ID number

### Mapping Not Working In-Game

**Problem**: Controller buttons not registering as keyboard keys in games

**Solutions**:
1. Ensure the profile is loaded (click "Load" on the profile)
2. Check that mappings are enabled (not disabled in the profile)
3. Verify the game is accepting keyboard input
4. Try running the application as administrator (Windows)
5. Check for conflicts with other input mapping software

**Testing mappings**:
1. Load the profile in Controller Remapper
2. Open a text editor
3. Press controller buttons
4. Verify keyboard keys are being registered

### Build Errors

**Problem**: Build script fails with errors

**Common solutions**:

**Missing dependencies**:
```bash
# Linux
sudo apt-get install build-essential libwebkit2gtk-4.0-dev libssl-dev libudev-dev libsdl2-dev

# macOS
xcode-select --install
```

**Rust/Tauri issues**:
```bash
# Update Rust
rustup update

# Reinstall Tauri CLI
cargo install tauri-cli --force
```

**Windows-specific**:
- Ensure Microsoft Visual C++ Build Tools are installed
- Check that MSVC is in your PATH
- Run build script from Developer Command Prompt

### Profile Import/Export Issues

**Problem**: Cannot import or export profiles

**Solutions**:
1. Ensure the profile JSON is valid (check syntax)
2. Verify the profile format matches the current version
3. Check file permissions on the profiles directory
4. Ensure the profile directory exists: `~/.config/controller-remapper/profiles`

**Profile directory location**:
- Linux: `~/.config/controller-remapper/profiles`
- macOS: `~/Library/Application Support/controller-remapper/profiles`
- Windows: `%APPDATA%\controller-remapper\profiles`

### Cloud Sync Issues

**Problem**: Profiles not syncing to Steam Cloud

**Solutions**:
1. Ensure Steam Cloud is enabled for the app
2. Check your Steam Cloud storage quota
3. Verify internet connection
4. Manually trigger sync with "Sync Now" button
5. Check Steam logs for sync errors

**Steam Cloud quota**:
- Check quota in the "Cloud Sync" section
- Free up space by removing unused profiles
- Contact Steam support if you need more quota

## Development

### Project Structure

```
controller-remapper/
├── src/
│   ├── core/              # Bluetooth + HID logic
│   │   ├── mod.rs
│   │   ├── controller_manager.rs
│   │   ├── bluetooth_detector.rs
│   │   ├── hid_handler.rs
│   │   ├── input_mapper.rs
│   │   └── controller_types.rs
│   ├── profiles/          # JSON config management
│   │   ├── mod.rs
│   │   ├── profile_manager.rs
│   │   └── profile.rs
│   ├── steamworks/        # Steam API integration
│   │   ├── mod.rs
│   │   ├── steam_integration.rs
│   │   ├── workshop.rs
│   │   ├── remote_storage.rs
│   │   └── overlay.rs
│   ├── ui/                # Frontend commands
│   │   ├── mod.rs
│   │   └── commands.rs
│   └── main.rs
├── ui/                    # Frontend assets
│   ├── index.html
│   ├── styles.css
│   └── app.js
├── assets/
│   ├── icons/
│   ├── profiles/          # Example profiles
│   └── workshop/
├── tests/
│   ├── unit/              # Unit tests
│   └── integration/       # Integration tests
├── docs/
│   └── README.md
├── build/                 # Build scripts
│   ├── build_windows.bat
│   ├── build_macos.sh
│   └── build_linux.sh
├── Cargo.toml
├── tauri.conf.json
├── build.rs
└── steam_appid.txt
```

### Adding New Features

1. **Backend (Rust)**: Add modules in `src/core/` or `src/steamworks/`
2. **Frontend (JavaScript)**: Add UI components in `ui/`
3. **Tauri Commands**: Register new commands in `src/ui/commands.rs`
4. **Tests**: Add tests in `tests/unit/` or `tests/integration/`

### Code Style

- **Rust**: Follow standard Rust formatting (`cargo fmt`)
- **JavaScript**: Use modern ES6+ syntax
- **Documentation**: Document public functions with rustdoc comments

## License

MIT License - See LICENSE file for details

## Support

For issues, questions, or contributions:
- GitHub Issues: [Report bugs and request features]
- Steam Workshop: [Share and download profiles]
- Discord: [Join our community Discord]

## Acknowledgments

- Valve Corporation for the Steamworks SDK
- The SDL2 project for cross-platform input handling
- The Tauri team for the excellent framework
- The Rust community for excellent tooling
