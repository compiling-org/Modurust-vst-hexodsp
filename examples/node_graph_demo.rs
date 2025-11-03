//! Node Graph Demo
//!
//! This example shows how to create and manipulate a node graph
//! for modular audio processing.

use hexodsp_daw::audio_engine::node_graph::NodeGraph;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔗 Modurust DAW - Node Graph Demo");

    // Create a new node graph
    let mut graph = NodeGraph::new();

    // Add nodes
    let osc_id = graph.add_node("oscillator");
    let filter_id = graph.add_node("filter");
    let output_id = graph.add_node("output");

    println!("📦 Added nodes: Oscillator({}), Filter({}), Output({})",
             osc_id, filter_id, output_id);

    // Connect nodes: Oscillator -> Filter -> Output
    graph.connect(osc_id, filter_id, "audio_out", "audio_in")?;
    graph.connect(filter_id, output_id, "audio_out", "audio_in")?;

    println!("🔗 Connected: Osc → Filter → Output");

    // Process audio for a short time
    println!("🎵 Processing audio...");
    
    // Create input and output buffers
    let input_buffer = vec![0.0; 1024];
    let mut output_buffer = vec![0.0; 1024];
    
    for _ in 0..10 {
        graph.process(&input_buffer, &mut output_buffer);
    }

    println!("✅ Node graph processing completed!");
    Ok(())
}