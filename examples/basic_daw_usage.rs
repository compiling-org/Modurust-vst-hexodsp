//! Basic Modurust DAW Usage Example
//!
//! This example demonstrates how to create a simple DAW session,
//! add tracks, load audio, and perform basic operations.

use modurust_daw::{
    daw_core::{DAWCore, Project, Track, Clip},
    audio_backend::AudioBackend,
    transport_sync::{Transport, TransportState},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎵 Modurust DAW - Basic Usage Example");

    // Initialize the DAW core
    let mut daw = DAWCore::new()?;

    // Create a new project
    let mut project = Project::new("My First Project".to_string());

    // Add an audio track
    let track = Track::new_audio_track("Main Track".to_string(), 0);
    project.add_track(track);

    // Load an audio file (placeholder - would load actual audio)
    // let audio_data = load_audio_file("path/to/audio.wav")?;
    // let clip = Clip::new_audio_clip("My Clip".to_string(), audio_data, 0.0);
    // project.add_clip_to_track(0, clip)?;

    // Set up transport
    let mut transport = Transport::new();
    transport.set_tempo(120.0);
    transport.set_time_signature(4, 4);

    // Start playback (placeholder - would actually play audio)
    transport.play();
    println!("▶️  Started playback at 120 BPM");

    // Simulate some time passing
    std::thread::sleep(std::time::Duration::from_secs(2));

    // Stop playback
    transport.stop();
    println!("⏹️  Stopped playback");

    println!("✅ Basic DAW operations completed successfully!");
    Ok(())
}