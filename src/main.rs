/// HexoDSP DAW - Main Entry Point
/// 
/// This is the main entry point for the HexoDSP Digital Audio Workstation,
/// integrating the eframe UI with the real-time audio engine.

use hexodsp_daw::run_hexodsp_ui;
use hexodsp_daw::audio_engine::HexoDSPEngine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎵 Starting HexoDSP DAW - Revolutionary Node-Based Audio Workstation");
    println!("==========================================");
    
    // Initialize the audio engine (but don't start it)
    println!("🎛️ Initializing audio engine...");
    let mut audio_engine = HexoDSPEngine::new()?;
    
    // Don't start audio automatically - user can start it via UI
    // audio_engine.start()?;
    
    // Create a shared reference to the audio engine for the UI
    // Note: In a production implementation, this would be managed more carefully
    // to ensure thread safety between the UI and audio threads
    
    println!("🖥️ Starting eframe UI...");
    
    // Run the eframe UI application
    run_hexodsp_ui()?;
    
    // Clean shutdown
    println!("🛑 Shutting down...");
    audio_engine.stop()?;
    
    println!("✅ HexoDSP DAW shutdown complete");
    Ok(())
}
