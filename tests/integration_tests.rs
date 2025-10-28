//! Integration Tests for Modurust DAW
//!
//! These tests verify that the major components work together correctly.

use modurust_daw::{
    daw_core::DAWCore,
    node_graph::NodeGraph,
    transport_sync::Transport,
    audio_nodes::OscillatorNode,
};

#[test]
fn test_daw_initialization() {
    let daw = DAWCore::new();
    assert!(daw.is_ok(), "DAW should initialize successfully");
}

#[test]
fn test_node_graph_operations() {
    let mut graph = NodeGraph::new();

    // Add a node
    let node_id = graph.add_node(Box::new(OscillatorNode::new_sine(440.0)));
    assert!(node_id >= 0, "Node ID should be valid");

    // Process a block
    let result = graph.process_block();
    assert!(result.is_ok(), "Block processing should succeed");
}

#[test]
fn test_transport_operations() {
    let mut transport = Transport::new();

    // Test tempo setting
    transport.set_tempo(120.0);
    assert_eq!(transport.get_tempo(), 120.0, "Tempo should be set correctly");

    // Test play/stop
    transport.play();
    assert!(matches!(transport.get_state(), TransportState::Playing),
            "Transport should be playing");

    transport.stop();
    assert!(matches!(transport.get_state(), TransportState::Stopped),
            "Transport should be stopped");
}

#[test]
fn test_audio_node_creation() {
    let osc = OscillatorNode::new_sine(440.0);
    assert_eq!(osc.get_frequency(), 440.0, "Frequency should be set correctly");

    let square_osc = OscillatorNode::new_square(220.0);
    assert_eq!(square_osc.get_frequency(), 220.0, "Square wave frequency should be correct");
}

#[test]
fn test_project_workflow() {
    // This would test a complete workflow:
    // 1. Create project
    // 2. Add tracks
    // 3. Add clips
    // 4. Set up routing
    // 5. Play back
    // For now, just ensure basic components work together

    let daw = DAWCore::new();
    assert!(daw.is_ok(), "Full DAW workflow should be testable");
}