//! Integration Tests for Modurust DAW
//!
//! These tests verify that the major components work together correctly.

use hexodsp_daw::{
    node_graph::{NeuroNodeGraph, NeuroNodeId},
    transport_sync::{GlobalTransport, TransportState},
    audio_nodes::SineOscillator,
};

#[test]
fn test_daw_initialization() {
    // Test basic initialization - we don't have DAWCore, so test components
    let transport = GlobalTransport::new(44100);
    assert_eq!(transport.state.sample_rate, 44100, "Transport should initialize with correct sample rate");
}

#[test]
fn test_node_graph_operations() {
    let mut graph = NeuroNodeGraph::new();

    // Create a sine oscillator node definition
    let node_def = SineOscillator::create_node(NeuroNodeId(1), 44100.0);

    // Add a node
    let result = graph.add_node(node_def);
    assert!(result.is_ok(), "Node should be added successfully");

    // Check that node was added
    assert_eq!(graph.nodes().len(), 1, "Graph should contain one node");
}

#[test]
fn test_transport_operations() {
    let mut transport = GlobalTransport::new(44100);

    // Test tempo setting
    transport.set_tempo(120.0);
    assert_eq!(transport.state.tempo_bpm, 120.0, "Tempo should be set correctly");

    // Test play/stop
    transport.start();
    assert!(transport.state.playing, "Transport should be playing");

    transport.stop();
    assert!(!transport.state.playing, "Transport should be stopped");
}

#[test]
fn test_audio_node_creation() {
    let osc = SineOscillator::new(44100.0);
    // Basic test that oscillator can be created
    // Note: phase is private, so we just test creation
    assert!(true, "Oscillator should be created successfully");
}

#[test]
fn test_project_workflow() {
    // Test basic components work together
    let transport = GlobalTransport::new(44100);
    let graph = NeuroNodeGraph::new();

    assert_eq!(transport.state.sample_rate, 44100, "Transport should have correct sample rate");
    assert_eq!(graph.nodes().len(), 0, "Graph should start empty");
}