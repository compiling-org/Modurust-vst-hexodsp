# Developer Setup Guide

## Prerequisites

- **Rust**: 1.75 or later
- **Cargo**: Latest version
- **Git**: For version control

## Platform-Specific Setup

### Windows
```bash
# Install Rust using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

### macOS
```bash
# Install Rust using Homebrew
brew install rust

# Or using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Linux
```bash
# Install Rust using package manager
# Ubuntu/Debian
sudo apt install rustc cargo

# Or using rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Building the Project

### Clone the Repository
```bash
git clone https://github.com/compiling-org/Modurust-vst-hexodsp.git
cd Modurust-vst-hexodsp
```

### Build in Debug Mode
```bash
cargo build
```

### Build in Release Mode
```bash
cargo build --release
```

### Run the DAW
```bash
cargo run
```

## Development Workflow

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

### Testing
```bash
cargo test
```

### Documentation
```bash
cargo doc --open
```

## Audio System Setup

### Windows (WASAPI)
No additional setup required - uses system default audio device.

### macOS (CoreAudio)
No additional setup required - uses system default audio device.

### Linux (ALSA/JACK)
```bash
# Install ALSA development headers
sudo apt install libasound2-dev

# Optional: Install JACK for low-latency audio
sudo apt install qjackctl jackd2
```

## MIDI Setup

### Windows
Uses Windows MIDI API - no additional setup required.

### macOS
Uses CoreMIDI - no additional setup required.

### Linux
```bash
# Install ALSA MIDI utilities
sudo apt install alsa-utils
```

## IDE Setup

### VS Code
Install the following extensions:
- `rust-lang.rust-analyzer`
- `vadimcn.vscode-lldb`

### Configuration
Add to `.vscode/settings.json`:
```json
{
    "rust-analyzer.checkOnSave.command": "clippy"
}
```

## Troubleshooting

### Audio Issues
- Check that audio devices are not in use by other applications
- Verify audio device permissions on macOS/Linux
- Try different sample rates in the DAW settings

### MIDI Issues
- Check MIDI device connections
- Verify MIDI device permissions
- Test with different MIDI devices

### Build Issues
- Ensure Rust toolchain is up to date: `rustup update`
- Clear cargo cache: `cargo clean`
- Check for conflicting dependencies

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Format code: `cargo fmt`
6. Submit a pull request