//! UI Module for HexoDSP DAW
//!
//! This module exports the revolutionary three-view UI system:
//! - Arrangement View: Traditional DAW timeline/arrangement
//! - Live View: Real-time performance interface
//! - Node View: Modular node-based patching

// Declare the eframe_ui submodule - contains the complete 3000+ line implementation
mod eframe_ui;

// Re-export everything from eframe_ui (which contains ALL your sophisticated code)
pub use eframe_ui::*;

// Re-export all the sophisticated UI components and state management
pub use eframe_ui::UIViewMode;
pub use eframe_ui::UiState;
pub use eframe_ui::ArrangementViewState;
pub use eframe_ui::LiveViewState;
pub use eframe_ui::NodeViewState;
pub use eframe_ui::VisualFeedbackSettings;
pub use eframe_ui::AutomationCurve;
pub use eframe_ui::AutomationPoint;
pub use eframe_ui::TrackRoute;
pub use eframe_ui::ClipOperation;
pub use eframe_ui::MegaPlugin;
pub use eframe_ui::ModulationRoute;
pub use eframe_ui::NodePosition;
pub use eframe_ui::NodeConnection;
pub use eframe_ui::NodeParameter;
pub use eframe_ui::NodePreset;
pub use eframe_ui::NuweShader;
pub use eframe_ui::ISFPlugin;
