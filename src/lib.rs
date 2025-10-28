//! # HexoDSP DAW
//!
//! A revolutionary Rust-based Digital Audio Workstation with three UI views:
//! arrangement, live performance, and node-based modular synthesis.
//!
//! ## Features
//!
//! ### 🎛️ **Core DAW Primitives**
//! - **Track Database**: WAV/AIFF file management with metadata extraction
//! - **Beatgrid System**: Adjustable BPM-based beat markers for robust sync
//! - **Cue Points & Markers**: Colored timeline markers with extra data storage
//! - **Transport Engine**: Global transport with sample-accurate positioning
//!
//! ### 🔄 **Sync & Communication**
//! - **UDP Transport Sync**: JSON-based transport state broadcasting
//! - **HID/OSC Bindings**: Control surface mapping for hardware integration
//! - **Timecode Support**: LTC-style SMPTE timecode generation
//! - **Network Protocol**: Extensible UDP messaging for distributed setups
//!
//! ### 🧩 **Modular Architecture**
//! - **Node Graph System**: Composable processing pipelines
//! - **NUWE Integration**: Compatible with NUWE node ecosystem
//! - **Plugin Architecture**: Extensible module system
//! - **Real-time Processing**: Low-latency audio processing framework
//!
//! ### 🎹 **Advanced MIDI Features**
//! - **MPE Support**: Microtonal expression and per-note control
//! - **MIDI 2.0**: Enhanced protocol with higher resolution and new features
//! - **Controller Scripts**: Lua/Python scripting for custom control logic
//! - **Ultra-stable Clock**: High-precision BPM and timing synchronization
//!
//! ### 🤖 **AI-Powered Tools**
//! - **Stem Separation**: AI-based track isolation and extraction
//! - **Synthesis Tools**: ML-powered sound design and generation
//! - **MCP Server Integration**: Model Context Protocol for AI assistants
//! - **Smart Arrangement**: AI-assisted music composition and arrangement

// Core DAW modules
pub mod daw_core;
pub mod transport_sync;
pub mod player_backend;
pub mod hid_osc_bindings;
pub mod node_graph;
pub mod daw_nodes;
pub mod audio_nodes;
pub mod audio_backend;
pub mod midi2_mpe;
pub mod ui;
pub mod ai_audio;
pub mod sai_audio;
pub mod stream_diffusion_audio;
pub mod mcp_server;
pub mod web_interface;

// Re-exports for convenience
pub use daw_core::*;
pub use transport_sync::*;
pub use player_backend::*;
pub use hid_osc_bindings::*;
pub use node_graph::*;
pub use daw_nodes::*;
pub use audio_nodes::*;
pub use audio_backend::*;
pub use midi2_mpe::*;
pub use ui::*;
pub use ai_audio::*;
pub use sai_audio::*;
pub use stream_diffusion_audio::*;
pub use mcp_server::*;
pub use web_interface::*;