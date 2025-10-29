//! # HexoDSP DAW
//!
//! A revolutionary Rust-based Digital Audio Workstation with three UI views:
//! arrangement, live performance, and node-based modular synthesis.
//!
//! ## Features
//!
//! ### 🎨 **Three-View UI System**
//! - **Arrangement View**: Traditional DAW timeline/arrangement
//! - **Live View**: Real-time performance interface
//! - **Node View**: Modular node-based patching
//!
//! ### 🎛️ **Professional DAW Features**
//! - **Advanced Mixer**: 12-track professional mixer with EQ, sends, and inserts
//! - **Effects Rack**: Comprehensive effects processing with parallel/series routing
//! - **Modulation Matrix**: Advanced modulation routing system
//! - **Real-time Visualizations**: Spectrum analyzers and audio level meters

// UI module - contains the complete eframe implementation
pub mod ui;

// Re-exports for convenience
pub use ui::*;