# Controller Remapper

A production-ready Steam utility for mapping controller buttons to keyboard keys with full Steam Workshop integration, cloud sync, and cross-platform support.

## Quick Start

### Prerequisites
- Rust (install from https://rustup.rs/)
- Node.js (for frontend development)
- Steam Client (for Steam features)

### Building

**Windows:**
```batch
cd build
build_windows.bat
```

**macOS:**
```bash
cd build
chmod +x build_macos.sh
./build_macos.sh
```

**Linux:**
```bash
cd build
chmod +x build_linux.sh
./build_linux.sh
```

### Running Tests

```bash
# Unit tests
cargo test --lib

# Integration tests
cargo test --test '*'
```

## Features

- **Cross-platform controller detection** (Windows, macOS, Linux)
- **Bluetooth and HID device support**
- **Visual button-to-keyboard mapping**
- **Profile management with JSON storage**
- **Steam Workshop integration**
- **Steam Cloud sync**
- **Steam Overlay support**

## Documentation

For detailed documentation, see [docs/README.md](docs/README.md)

## Project Structure

```
controller-remapper/
├── src/
│   ├── core/              # Bluetooth + HID logic
│   ├── profiles/          # Profile management
│   ├── steamworks/        # Steam API integration
│   ├── ui/                # Frontend commands
│   └── main.rs
├── ui/                    # Frontend assets
├── assets/                # Example profiles
├── tests/                 # Unit and integration tests
├── docs/                  # Documentation
├── build/                 # Build scripts
└── steam_appid.txt        # Steam App ID (480 for testing)
```

## License

MIT License
