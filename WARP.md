# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

**Modurust DAW** (also known as hexodsp-daw) is a revolutionary Rust-based Digital Audio Workstation featuring a three-view paradigm that combines traditional DAW workflows with live performance and modular synthesis. Built with Rust for ultra-low latency audio processing (<1ms round-trip).

### Key Technologies
- **Language**: Rust 1.75+
- **UI Framework**: eframe + egui 0.27 (immediate mode GUI)
- **Audio Backend**: CPAL (Cross-Platform Audio Library)
- **MIDI**: midir + wmidi for MIDI 2.0/MPE support
- **Async Runtime**: Tokio with multi-threaded runtime

## Essential Commands

### Building and Running
```bash
# Build the desktop application (debug)
cargo build

# Run the DAW (desktop with eframe + egui)
cargo run

# Build for release (optimized)
cargo build --release

# Run release build
cargo run --release
```

### Testing
```bash
# Run all tests
cargo test

# Run tests in release mode
cargo test --release

# Run integration tests
cargo test --test integration_tests
```

### Code Quality
```bash
# Format code (always run before committing)
cargo fmt

# Run linter
cargo clippy

# Generate documentation
cargo doc --open
```

### Web Build (WebAssembly)
```bash
# Install wasm-pack if needed
cargo install wasm-pack

# Build for web target
wasm-pack build --target web --out-dir web/pkg

# Release build for web
wasm-pack build --target web --release --out-dir web/pkg

# Serve locally (from project root)
cd web && python3 -m http.server 8000
# Or on Windows PowerShell:
# cd web; python -m http.server 8000
```

### Release Build Script
```bash
# Run the complete release build (Linux/macOS)
bash scripts/build_release.sh
```

## Architecture Overview

### Module Structure

The codebase follows a layered modular architecture:

```
hexodsp-daw/
├── src/
│   ├── lib.rs                    # Library entry point
│   ├── main.rs                   # Desktop application entry point
│   ├── ui/                       # UI layer
│   │   ├── mod.rs               # UI module exports
│   │   ├── eframe_ui.rs         # 3000+ line eframe/egui implementation
│   │   ├── professional_daw_ui.rs
│   │   ├── bevy_web_ui.rs       # Bevy WebAssembly integration
│   │   └── audio_engine_bridge.rs
│   ├── audio_engine/            # Real-time audio processing core
│   │   ├── mod.rs               # Engine coordination
│   │   ├── cpal_io.rs           # Audio I/O using CPAL
│   │   ├── dsp_core.rs          # DSP algorithms
│   │   ├── node_graph.rs        # Audio processing graph
│   │   ├── transport.rs         # Timing and sync
│   │   └── bridge.rs            # UI-to-audio thread communication
│   ├── audio_nodes.rs           # Audio node types
│   ├── daw_nodes.rs             # DAW-specific nodes
│   ├── midi2_mpe.rs             # MIDI 2.0/MPE support
│   └── (AI/ML modules)          # AI audio tools
├── web/                         # Web interface assets
│   ├── index.html
│   ├── css/
│   └── js/
└── docs/                        # Comprehensive documentation
```

### Three-View UI System

The UI implements three distinct views accessible from a single codebase:

1. **Arrangement View**: Traditional timeline-based DAW with automation
2. **Live View**: Real-time clip matrix and crossfader for performance
3. **Node View**: Visual modular patching with drag-and-drop connections

View state is managed through `UIViewMode` enum in `src/ui/eframe_ui.rs`.

### Audio Processing Pipeline

**Threading Model:**
- **Main Thread**: UI updates, user input, project management
- **Audio Thread**: Real-time processing (<1ms latency), MIDI processing
- **Worker Threads**: AI processing, file I/O, network communication

**Data Flow:**
```
Input Device → AudioIO (CPAL) → Node Graph → DSP Core → Output Device
                    ↓
              MIDI 2.0/MPE → Controllers
                    ↓
              AudioEngineBridge → UI Updates
```

**Critical Components:**
- `HexoDSPEngine` (audio_engine/mod.rs): Main audio engine coordinator
- `NodeGraph` (audio_engine/node_graph.rs): Audio routing and processing graph
- `AudioEngineBridge` (audio_engine/bridge.rs): Lock-free UI↔Audio communication
- `HexoDSPApp` (ui/eframe_ui.rs): Main UI application state

### UI State Management

The UI uses `UiState` struct (in eframe_ui.rs) to maintain persistent state:
- Current view mode (Arrangement/Live/Node)
- Transport state (playing, BPM, time position)
- Mixer state (12 track channels, 8 return channels, 4 group channels)
- Master channel settings (volume, pan, EQ, compression, limiting)
- Visual feedback settings (audio levels, CPU usage, animations)

### Audio Backend (CPAL)

Platform-specific audio backends are abstracted through CPAL:
- **Windows**: WASAPI
- **macOS**: CoreAudio
- **Linux**: ALSA/JACK
- **Web**: Web Audio API

Default configuration: 44.1kHz sample rate, 256 sample buffer (low latency).

## Development Guidelines

### UI Development

The primary UI is a 3000+ line eframe/egui implementation that was recently fixed (see FIXES_SUMMARY.md). Key points:

- **Never remove or exclude `ui_backup.rs`** - it's intentionally excluded from compilation
- **egui API Compatibility**: Using egui 0.27, ensure compatibility when updating
- **Common egui patterns in this codebase:**
  - Use `ui.selectable_value(current, target, label)` with `&str` not `String`
  - Use `egui::Rounding::same()` for corner radius (not deprecated `Rounding::same()`)
  - Separate `ui.allocate_response()` from `ui.painter()` calls to avoid borrow conflicts
  - `rect_filled()` parameter order: `(rect, corner_radius, color)`

### Audio Engine Development

- **Real-time safety**: Audio thread code must be lock-free, allocation-free
- **Communication**: Use `AudioEngineBridge` for UI↔Audio thread messages
- **Node Graph**: Add new audio nodes via `NodeGraph::add_*()` methods
- **DSP modules**: Implement `DSPModule` trait for new processors

### Adding Audio Nodes

1. Define node type in `audio_nodes.rs` or `daw_nodes.rs`
2. Implement `DSPModule` trait with `process()` method
3. Add constructor method to `NodeGraph` (e.g., `add_oscillator()`)
4. Connect nodes using `NodeGraph::connect()`

### Testing Strategy

- **Unit tests**: Test individual DSP modules and components
- **Integration tests**: Located in `tests/integration_tests.rs`
- **No test mocking by default**: Check README/docs for testing approach
- **Audio testing**: Verify latency, buffer handling, sample accuracy

### Error Handling

- Use `anyhow::Result` for general error handling
- Use `thiserror` for custom error types
- Audio thread errors: Log but never panic
- Graceful degradation: Continue with reduced functionality on non-critical errors

### Platform-Specific Notes

**Windows (PowerShell):**
- Use `Get-ChildItem` instead of `ls`
- Audio backend: WASAPI (no additional setup)
- MIDI: Windows MIDI API (automatic)

**Linux:**
- May need ALSA dev headers: `sudo apt install libasound2-dev`
- Optional JACK for pro audio: `sudo apt install qjackctl jackd2`

**macOS:**
- CoreAudio and CoreMIDI (automatic)
- May need Xcode command-line tools

## Common Patterns

### UI Update Pattern
```rust
// Update UI state
ui_state.master_volume = new_value;

// Send to audio thread
bridge.send_param(AudioParamMessage::MasterVolume(new_value));
```

### Node Graph Processing
```rust
// Create nodes
let osc_id = node_graph.add_oscillator();
let filter_id = node_graph.add_filter();
let output_id = node_graph.add_output();

// Connect them
node_graph.connect(osc_id, filter_id, "audio_out", "audio_in")?;
node_graph.connect(filter_id, output_id, "audio_out", "audio_in")?;
```

### Conditional Compilation (Desktop vs Web)
```rust
#[cfg(not(target_arch = "wasm32"))]
fn desktop_specific() { /* eframe/egui code */ }

#[cfg(target_arch = "wasm32")]
fn web_specific() { /* WebAssembly code */ }
```

## Important Files

- **src/ui/eframe_ui.rs**: Main UI implementation (3000+ lines, recently fixed)
- **src/audio_engine/mod.rs**: Audio engine entry point and coordinator
- **src/audio_engine/node_graph.rs**: Audio processing graph
- **src/audio_engine/bridge.rs**: Thread-safe UI↔Audio communication
- **FIXES_SUMMARY.md**: Recent compilation error fixes (174+ errors resolved)
- **docs/architecture.md**: Detailed architecture documentation
- **docs/developer-setup.md**: Platform-specific setup instructions

## Known Issues and Gotchas

1. **egui version**: Locked to 0.27 - verify compatibility when updating dependencies
2. **Audio latency**: Default 256 sample buffer is tuned for low latency; increase if glitches occur
3. **WASM build**: Requires wasm-pack; web target uses different audio/MIDI APIs
4. **Recent fixes**: See FIXES_SUMMARY.md for 174+ compilation errors that were recently resolved

## Resources

- User Guide: docs/user-guide.md
- Architecture: docs/architecture.md  
- Developer Setup: docs/developer-setup.md
- Changes & Features: docs/changes-features.md
- Frontend Documentation: docs/FRONTEND_DOCUMENTATION.md
