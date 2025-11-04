use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use hexodsp_daw::{
    HexoDSPEngine,
    ui::UiState,
};
use std::fs;
use bevy::render::RenderPlugin;
use bevy::asset::AssetPlugin;
use bevy_image::ImagePlugin;
use bevy::winit::WinitPlugin;
use bevy::window::WindowPlugin;
use bevy_a11y::AccessibilityPlugin;

const UI_STATE_PATH: &str = "ui_state.json";

// Define a custom WakeUp message for WinitPlugin
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Message, Default)]
struct WakeUp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Load persisted UI state
    let ui_state: UiState = fs::read_to_string(UI_STATE_PATH)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();

    // Build Bevy App
    App::new()
        .add_plugins((
            MinimalPlugins,
            AssetPlugin::default(),
            RenderPlugin::default(),
            ImagePlugin::default(),
            WindowPlugin::default(),
            AccessibilityPlugin,
            WinitPlugin::<WakeUp>::default(),
            EguiPlugin::default(),
        ))
        .insert_non_send_resource(ui_state)
        .insert_non_send_resource(HexoDSPEngine::new()?)
        .add_systems(Startup, setup_camera)
        .add_systems(EguiPrimaryContextPass, ui_system)
        .run();

    Ok(())
}

fn setup_camera(mut commands: Commands) {
    // bevy_egui requires at least one camera
    commands.spawn(Camera2d);
}

fn ui_system(mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();
    egui::Window::new("Hello").show(ctx, |ui| {
        ui.label("world");
    });
}
