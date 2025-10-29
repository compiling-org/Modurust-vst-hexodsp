//! Basic Modurust DAW Usage Example
//!
//! This example demonstrates how to create a simple DAW session,
//! add tracks, load audio, and perform basic operations.

use hexodsp_daw::{
    daw_core::{TrackDatabaseEntry, AudioContainerFormat},
    transport_sync::GlobalTransport,
    node_graph::NeuroNodeGraph,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎵 Modurust DAW - Basic Usage Example");

    // Initialize transport
    let mut transport = GlobalTransport::new(44100);
    transport.set_tempo(120.0);
    println!("🎼 Transport initialized with 120 BPM");

    // Create a node graph
    let mut graph = NeuroNodeGraph::new();
    println!("🔗 Node graph created");

    // Create a track database entry (placeholder for audio file)
    // In a real implementation, this would load actual audio
    let track_entry = TrackDatabaseEntry {
        id: 1,
        path: std::path::PathBuf::from("example.wav"),
        container: AudioContainerFormat::Wav,
        sample_rate: 44100,
        channels: 2,
        total_samples: 44100 * 4, // 4 seconds
        duration_seconds: 4.0,
        metadata: Default::default(),
        beatgrid: Default::default(),
        cues: vec![],
        markers: vec![],
        overview_waveform: Some(vec![]),
    };

    println!("📁 Created track entry for example.wav");

    // Start transport
    transport.start();
    println!("▶️  Transport started");

    // Simulate some processing time
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Stop transport
    transport.stop();
    println!("⏹️  Transport stopped");

    println!("✅ Basic DAW operations completed successfully!");
    Ok(())
}