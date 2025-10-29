use hexodsp_daw::*;
use std::error::Error;

// Real-time visualization
use hexodsp_daw::ui::eframe_ui::{self, run_hexodsp_ui};

// File I/O capabilities
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    println!("🎵 HexoDSP DAW - Revolutionary Node-Based Digital Audio Workstation");
    println!("============================================================");
    println!("🎨 Three-View Interface: Arrangement | Live | Node");
    println!("🎛️  Features: Real-time Audio | EEG Control | Motion Capture | Fractal Shaders");
    println!();

    // Initialize logging
    env_logger::init();

    // Launch the eframe UI (blocking)
    run_hexodsp_ui()?;

    Ok(())
}

// No additional types needed - all defined in UI modules
