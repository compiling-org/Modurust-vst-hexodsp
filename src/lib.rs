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

// Audio engine module - real-time audio processing and DSP
pub mod audio_engine;

// Presets module - modular content system for the Three-View workflow
pub mod presets;

// Node instance manager - bridge between UI nodes and audio processing
pub mod node_instance_manager;

// Clip-to-node integration - connects arrangement clips to audio nodes
pub mod clip_node_integration;

 // VST3 host implementation - plugin loading and management
 pub mod vst3_host;

// MIDI 2.0 and MPE support - advanced MIDI processing - temporarily commented out
// pub mod midi2_mpe;

// Web interface - browser-based remote control - temporarily commented out
// pub mod web_interface;

// Modular patch system - NoiseCraft/Pure Data inspired visual programming - temporarily commented out
// pub mod modular_patch_system;

// Theming system - extensive customization options - temporarily commented out
// pub mod theming_system;

// Piano roll editor - comprehensive MIDI editing
pub mod piano_roll_editor;

// MIDI control system - hardware controller integration
pub mod midi_control_system;

use eframe::egui;

/// Run the HexoDSP UI application
pub fn run_hexodsp_ui() -> Result<(), eframe::Error> {
    println!("HexoDSP UI - Full eframe implementation");
    println!("Three-View System Architecture:");
    println!("  - Arrangement View: Traditional DAW timeline");
    println!("  - Live View: Real-time performance interface");
    println!("  - Node View: Modular node-based patching");

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1600.0, 900.0])
            .with_title("HexoDSP DAW - Revolutionary Node-Based DAW"),
        ..Default::default()
    };
    
    eframe::run_native(
        "HexoDSP DAW",
        options,
        Box::new(|_cc| Box::<ui::HexoDSPApp>::default())
    )
}

// Re-exports for convenience
pub use ui::*;

// Re-export audio engine components
pub use audio_engine::*;