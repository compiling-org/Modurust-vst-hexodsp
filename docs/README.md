# Modurust DAW Documentation

## Overview

Modurust is a revolutionary Rust-based Digital Audio Workstation featuring a unique three-view paradigm that seamlessly combines traditional DAW workflows with live performance capabilities and modular synthesis.

## Key Features

### Three-View System
- **Arrangement View**: Traditional timeline-based production
- **Live View**: Real-time performance with clip matrix
- **Node View**: Visual modular patching interface

### Audio Engine
- Ultra-low latency (<1ms round-trip)
- Real-time audio processing
- Modular node-based architecture
- 8 core audio node types

### MIDI 2.0 & MPE Support
- Full MIDI 2.0 protocol implementation
- MPE (MIDI Polyphonic Expression)
- Per-note pitch bend and pressure
- Sample-accurate timing

### AI-Powered Tools
- SAI (Sonic AI) for text-to-audio generation
- Stream Diffusion for real-time audio manipulation
- AI Stem Separation for intelligent mixing
- Pattern generation and arrangement analysis

## Getting Started

### Prerequisites
- Rust 1.75+
- Windows/macOS/Linux

### Building
```bash
cargo build --release
```

### Running
```bash
cargo run
```

## Architecture

### Core Modules
- `daw_core.rs` - Main DAW engine
- `transport_sync.rs` - Timing and synchronization
- `node_graph.rs` - Visual patching system
- `audio_backend.rs` - Real-time audio processing
- `midi2_mpe.rs` - MIDI implementation
- `ui.rs` - Three-view interface

### AI Tools
- `sai_audio.rs` - Sonic AI integration
- `stream_diffusion_audio.rs` - Audio diffusion
- `ai_audio.rs` - AI effects and processing
- `mcp_server.rs` - Model Context Protocol

## Contributing

Please see CONTRIBUTING.md for development guidelines.

## License

See LICENSE file for details.
## Quick Links

- Desktop Focus Plan: `FOCUS_DESKTOP.md` for the month-long, desktop-only scope.
- Desktop Demo Plan: see `desktop-demo-plan.md` for the demo scope, acceptance criteria, and plans covering shaders/visuals, MIDI 2.0/MPE, and microtuning.
- Web App Plan: `web-app-plan.md` for the post-demo browser strategy and detailed roadmap.
- Architecture: `architecture.md` for system design.
- Roadmap: `ROADMAP.md` for phased delivery.
- Frontend Features/Status: `frontend-features.md` and `frontend-status.md`.
- User Guide: `user-guide.md` for usage details.
 - Development Workflow (Script-Based SOP): `DEVELOPMENT_WORKFLOW.md` for non-interactive checks and audits.