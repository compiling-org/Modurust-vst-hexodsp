//! Node Graph Demo
//!
//! This example shows how to create and manipulate a node graph
//! for modular audio processing.

use modurust_daw::{
    node_graph::{NodeGraph, NodeId, Connection},
    audio_nodes::{OscillatorNode, FilterNode, OutputNode},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔗 Modurust DAW - Node Graph Demo");

    // Create a new node graph
    let mut graph = NodeGraph::new();

    // Add nodes
    let osc_id = graph.add_node(Box::new(OscillatorNode::new_sine(440.0)));
    let filter_id = graph.add_node(Box::new(FilterNode::new_lowpass(1000.0)));
    let output_id = graph.add_node(Box::new(OutputNode::new()));

    println!("📦 Added nodes: Oscillator({}), Filter({}), Output({})",
             osc_id, filter_id, output_id);

    // Connect nodes: Oscillator -> Filter -> Output
    graph.add_connection(Connection {
        from_node: osc_id,
        from_port: 0, // Audio output
        to_node: filter_id,
        to_port: 0,   // Audio input
    })?;

    graph.add_connection(Connection {
        from_node: filter_id,
        from_port: 0, // Filtered output
        to_node: output_id,
        to_port: 0,   // Audio input
    })?;

    println!("🔗 Connected: Osc → Filter → Output");

    // Process audio for a short time
    println!("🎵 Processing audio...");
    for _ in 0..1000 {
        graph.process_block()?;
    }

    println!("✅ Node graph processing completed!");
    Ok(())
}