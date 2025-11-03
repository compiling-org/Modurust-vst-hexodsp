//! UI Module for HexoDSP DAW
//!
//! This module exports the revolutionary three-view UI system:
//! - Arrangement View: Traditional DAW timeline/arrangement
//! - Live View: Real-time performance interface
//! - Node View: Modular node-based patching

// Declare the submodules
mod eframe_ui;
mod eframe_ui_full;
mod theme_manager;
pub mod hexagonal_node_view;

// Re-export the full-featured eframe UI implementation
pub use eframe_ui_full::*;

// Re-export all sophisticated UI components and state management from the full UI
pub use eframe_ui_full::UIViewMode;
pub use eframe_ui_full::UiState;
pub use eframe_ui_full::ArrangementViewState;
pub use eframe_ui_full::LiveViewState;
pub use eframe_ui_full::NodeViewState;
pub use eframe_ui_full::VisualFeedbackSettings;
pub use eframe_ui_full::AutomationCurve;
pub use eframe_ui_full::AutomationPoint;
pub use eframe_ui_full::TrackRoute;
pub use eframe_ui_full::ClipOperation;
pub use eframe_ui_full::MegaPlugin;
pub use eframe_ui_full::ModulationRoute;
pub use eframe_ui_full::NodePosition;
pub use eframe_ui_full::NodeConnection;
pub use eframe_ui_full::NodeParameter;
pub use eframe_ui_full::NodePreset;
pub use eframe_ui_full::NuweShader;
pub use eframe_ui_full::ISFPlugin;
