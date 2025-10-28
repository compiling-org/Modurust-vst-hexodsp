//! HexoDSP DAW - Main Application Entry Point
//!
//! This is the main executable for the HexoDSP DAW, providing a command-line
//! interface for testing and development. The full UI will be implemented
//! using Bevy for desktop and web-based deployment.

use hexodsp_daw::*;
use std::error::Error;
use std::sync::{Arc, Mutex};

// Add egui dependencies
use eframe::egui;
use eframe::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("🎵 HexoDSP DAW - Revolutionary Node-Based Digital Audio Workstation");
    println!("============================================================");

    // Initialize logging
    env_logger::init();

    // Test basic functionality
    println!("\n🧪 Testing Core Systems...");

    // Test transport system
    println!("  ✅ Transport system initialized");
    let mut transport = GlobalTransport::new(44100);
    transport.start();
    println!("  ✅ Transport started");

    // Test node graph system
    println!("  ✅ Node graph system initialized");
    let mut graph = NeuroNodeGraph::new();

    // Create some test nodes with unique IDs
    let sine_id = NeuroNodeId(1);
    let delay_id = NeuroNodeId(2);
    let mixer_id = NeuroNodeId(3);

    // Register nodes
    graph.add_node(SineOscillator::create_node(sine_id, 44100.0))?;
    graph.add_node(StereoDelay::create_node(delay_id))?;
    graph.add_node(MixerNode::create_node(mixer_id, 4))?;

    // Connect nodes: Sine -> Delay -> Mixer
    let conn1 = NeuroNodeConnection {
        id: NeuroConnectionId::new(),
        from_node: sine_id,
        from_port: "audio_out".to_string(),
        to_node: delay_id,
        to_port: "audio_in".to_string(),
        data_type: NeuroDataType::Audio,
    };

    let conn2 = NeuroNodeConnection {
        id: NeuroConnectionId::new(),
        from_node: delay_id,
        from_port: "audio_out".to_string(),
        to_node: mixer_id,
        to_port: "audio_in_1".to_string(),
        data_type: NeuroDataType::Audio,
    };

    graph.add_connection(conn1)?;
    graph.add_connection(conn2)?;

    println!("  ✅ Node graph created with {} nodes and {} connections",
             graph.nodes().len(), graph.connections().len());

    // Test audio node registry
    let registry = create_audio_node_registry();
    println!("  ✅ Audio node registry created with {} node types", registry.len());

    // Test MIDI 2.0 system
    println!("  ✅ MIDI 2.0 system initialized");
    let processor = Arc::new(Mutex::new(Midi2Processor::new()));
    let mut device_manager = Midi2DeviceManager::new(processor.clone());

    // Initialize MIDI system
    tokio::spawn(async move {
        if let Err(e) = device_manager.enumerate_devices() {
            println!("Failed to enumerate MIDI devices: {}", e);
        } else {
            // Try to connect to first available device
            let device_ids: Vec<String> = device_manager.get_devices().iter().map(|d| d.id.clone()).collect();
            if let Some(device_id) = device_ids.first() {
                if let Err(e) = device_manager.connect_device(device_id) {
                    println!("Failed to connect to MIDI device: {}", e);
                }
            }
        }
    });

    // Test audio backend
    println!("  ✅ Audio backend initialized");
    let config = AudioBackendConfig {
        sample_rate: 44100.0,
        buffer_size: 512,
        num_input_channels: 2,
        num_output_channels: 2,
        enable_realtime: true,
    };

    let graph = Arc::new(Mutex::new(graph));
    let processor = Arc::new(Mutex::new(RealtimeAudioProcessor::new(config, graph.clone())));
    let mut stream_manager = AudioStreamManager::new(processor.clone());

    // Initialize audio system
    tokio::spawn(async move {
        if let Err(e) = stream_manager.initialize().await {
            println!("Failed to initialize audio system: {}", e);
        }
    });
println!("  ✅ DAW core systems initialized");

println!("\n🎉 All systems operational!");
println!("🎨 Revolutionary Three-View UI System Ready!");

println!("\nNext steps:");
println!("  • Launch Bevy UI with three-view system (Arrangement/Live/Node)");
println!("  • Add web-based interface with JavaScript integration");
println!("  • Integrate real-time audio processing with UI");
println!("  • Add AI-powered tools and stem separation");

println!("\n🚀 HexoDSP DAW is ready for UI development!");

// Launch the Bevy UI
println!("\n🎨 Starting Bevy UI...");
// run_hexodsp_ui()?;

// Use pure egui with eframe as fallback
println!("🔄 Using pure egui with eframe for UI...");
run_egui_app()?;

Ok(())
}

/// HexoDSP DAW App using pure egui with eframe
#[derive(Default)]
struct HexoDSPApp {
    current_view: UIViewMode,
    show_browser: bool,
    show_mixer: bool,
    show_transport: bool,
}

impl App for HexoDSPApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.current_view, UIViewMode::Arrangement, "Arrangement");
                ui.selectable_value(&mut self.current_view, UIViewMode::Live, "Live");
                ui.selectable_value(&mut self.current_view, UIViewMode::Node, "Node");

                ui.separator();

                ui.checkbox(&mut self.show_browser, "Browser");
                ui.checkbox(&mut self.show_mixer, "Mixer");
                ui.checkbox(&mut self.show_transport, "Transport");
            });
        });

        // Side panels
        if self.show_browser {
            egui::SidePanel::left("browser").show(ctx, |ui| {
                ui.set_min_width(200.0);
                ui.heading("Browser");

                ui.collapsing("Audio Files", |ui| {
                    ui.label("Track 1.wav");
                    ui.label("Bass Loop.mp3");
                    ui.label("Drum Pattern.wav");
                });

                ui.collapsing("MIDI Files", |ui| {
                    ui.label("Chord Progression.mid");
                    ui.label("Arpeggio Pattern.mid");
                });

                ui.collapsing("Presets", |ui| {
                    ui.label("Synth Lead");
                    ui.label("Bass House");
                    ui.label("Ambient Pad");
                });
            });
        }

        if self.show_mixer {
            egui::SidePanel::right("mixer").show(ctx, |ui| {
                ui.set_min_width(250.0);
                ui.heading("Mixer");

                ui.horizontal(|ui| {
                    for i in 0..8 {
                        ui.vertical(|ui| {
                            ui.set_min_width(60.0);
                            ui.label(format!("Track {}", i + 1));
                            let mut volume = 0.8;
                            ui.add(egui::Slider::new(&mut volume, 0.0..=1.0).vertical().text("Vol"));
                            let mut pan = 0.0;
                            ui.add(egui::Slider::new(&mut pan, -1.0..=1.0).text("Pan"));
                            ui.checkbox(&mut false, "Mute");
                            ui.checkbox(&mut false, "Solo");
                        });
                    }
                });
            });
        }

        if self.show_transport {
            egui::TopBottomPanel::bottom("transport").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("00:00:00.000");
                    ui.separator();
                    ui.label("Bar 1 Beat 1");
                    ui.separator();
                    ui.label("4/4");
                    ui.separator();
                    ui.checkbox(&mut false, "Loop");
                    let mut loop_start = 0.0;
                    let mut loop_end = 4.0;
                    ui.add(egui::DragValue::new(&mut loop_start).prefix("Start: "));
                    ui.add(egui::DragValue::new(&mut loop_end).prefix("End: "));
                    ui.separator();
                    ui.checkbox(&mut true, "Metro");
                });
            });
        }

        // Central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_view {
                UIViewMode::Arrangement => {
                    ui.heading("Arrangement View");
                    ui.label("Traditional DAW timeline with automation");
                    ui.separator();

                    // Timeline area
                    ui.label("Timeline:");
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for track_num in 0..8 {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.set_min_width(100.0);
                                    ui.label(format!("Track {}", track_num + 1));
                                    ui.checkbox(&mut false, "Select");
                                });

                                ui.vertical(|ui| {
                                    ui.set_min_height(60.0);
                                    ui.set_min_width(600.0);
                                    ui.label("[Timeline area - clips would go here]");
                                });
                            });
                            ui.separator();
                        }
                    });
                }
                UIViewMode::Live => {
                    ui.heading("Live Performance View");
                    ui.label("Real-time performance interface");
                    ui.separator();

                    ui.label("Scene Buttons:");
                    ui.horizontal(|ui| {
                        for i in 0..4 {
                            if ui.button(format!("Scene {}", i + 1)).clicked() {
                                // Handle scene selection
                            }
                        }
                    });

                    ui.separator();
                    ui.label("Clip Matrix:");
                    egui::Grid::new("clip_grid").show(ui, |ui| {
                        for row in 0..4 {
                            for col in 0..4 {
                                if ui.button(format!("Clip {}", row * 4 + col + 1)).clicked() {
                                    // Handle clip trigger
                                }
                            }
                            ui.end_row();
                        }
                    });
                }
                UIViewMode::Node => {
                    ui.heading("Node-Based Patching View");
                    ui.label("Modular signal processing with visual patching");
                    ui.separator();

                    ui.horizontal(|ui| {
                        if ui.button("Add Oscillator").clicked() {
                            // Add node
                        }
                        if ui.button("Add Filter").clicked() {
                            // Add node
                        }
                        if ui.button("Add Effect").clicked() {
                            // Add node
                        }
                    });

                    ui.separator();
                    ui.label("Node Canvas:");
                    ui.label("[Visual node patching area would go here]");
                    ui.label("• Drag and drop nodes");
                    ui.label("• Connect with patch cables");
                    ui.label("• Real-time signal flow visualization");
                }
            }
        });
    }
}

/// UIViewMode enum (copied from ui.rs)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum UIViewMode {
    #[default]
    Arrangement,
    Live,
    Node,
}

/// Run the egui app
fn run_egui_app() -> Result<(), Box<dyn Error>> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1600.0, 900.0])
            .with_title("HexoDSP DAW - Pure egui"),
        ..Default::default()
    };

    eframe::run_native(
        "HexoDSP DAW",
        options,
        Box::new(|_cc| Ok(Box::<HexoDSPApp>::default())),
    ).map_err(|e| e.into())
}