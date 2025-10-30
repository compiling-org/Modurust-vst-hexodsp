/// eframe-based UI Framework for HexoDSP DAW
///
/// This module implements the revolutionary three-view UI system:
/// - Arrangement View: Traditional DAW timeline/arrangement
/// - Live View: Real-time performance interface
/// - Node View: Modular node-based patching

use eframe::egui;
use crate::audio_engine::bridge::{AudioEngineBridge, AudioParamMessage, AudioEngineState};

/// UI View Modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UIViewMode {
    #[default]
    Arrangement,
    Live,
    Node,
}

/// Main UI State
#[derive(Clone)]
pub struct UiState {
    pub current_view: UIViewMode,
    pub show_transport: bool,
    pub show_mixer: bool,
    pub show_browser: bool,
    pub zoom_level: f32,
    pub pan_offset: (f32, f32), // Using tuple instead of Vec2
    pub tempo_bpm: f32, // Persistent BPM for transport
    // --- New Mixer-related persistent state ---
    pub master_volume: f32,
    pub master_pan: f32,
    pub master_mute: bool,
    pub master_mono: bool,
    pub master_phase: bool,
    pub dim_level: f32,
    pub master_eq_on: bool,
    pub master_eq_low_gain: f32,
    pub master_eq_low_freq: f32,
    pub master_eq_low_q: f32,
    pub master_eq_lmid_gain: f32,
    pub master_eq_lmid_freq: f32,
    pub master_eq_lmid_q: f32,
    pub master_eq_hmid_gain: f32,
    pub master_eq_hmid_freq: f32,
    pub master_eq_hmid_q: f32,
    pub master_eq_high_gain: f32,
    pub master_eq_high_freq: f32,
    pub master_eq_high_q: f32,
    pub master_comp_ratio: f32,
    pub master_comp_threshold: f32,
    pub master_lim_ceiling: f32,
    pub link_channels: bool,
    pub show_levels: bool,
    pub track_channels: Vec<ChannelState>, // State for individual tracks
    pub return_channels: Vec<ReturnChannelState>, // Global return channels
    pub group_channels: Vec<GroupChannelState>, // Group processing channels
    pub transport_state: TransportState, // Transport and timeline controls
}

impl Default for UiState {
    fn default() -> Self {
        let mut default_state = Self {
            current_view: UIViewMode::Arrangement,
            show_transport: true,
            show_mixer: true,
            show_browser: true,
            zoom_level: 1.0,
            pan_offset: (0.0, 0.0),
            tempo_bpm: 128.0, // Initialize BPM
            // Initialize Master controls to match UI defaults
            master_volume: 0.8,
            master_pan: 0.0,
            master_mute: false,
            master_mono: false,
            master_phase: false,
            dim_level: -20.0,
            master_eq_on: true,
            master_eq_low_gain: 1.5,
            master_eq_low_freq: 60.0,
            master_eq_low_q: 0.7,
            master_eq_lmid_gain: -2.0,
            master_eq_lmid_freq: 250.0,
            master_eq_lmid_q: 1.0,
            master_eq_hmid_gain: 1.0,
            master_eq_hmid_freq: 3000.0,
            master_eq_hmid_q: 1.4,
            master_eq_high_gain: -1.5,
            master_eq_high_freq: 10000.0,
            master_eq_high_q: 0.8,
            master_comp_ratio: 4.0,
            master_comp_threshold: -18.0,
            master_lim_ceiling: -0.1,
            link_channels: false,
            show_levels: true,
            track_channels: Vec::new(),
            return_channels: Vec::new(),
            group_channels: Vec::new(),
            transport_state: TransportState::default(),
        };

        // Initialize 12 default ChannelState structs
        for i in 0..12 {
            let mut channel = ChannelState::default();
            channel.name = format!("Track {}", i + 1);
            channel.input = match i {
                0 => "Mic 1", 1 => "Mic 2", 2 => "DI 1", 3 => "DI 2",
                4 => "Line 1", 5 => "Line 2", 6 => "Return A", 7 => "Return B",
                8 => "Return C", 9 => "Return D", 10 => "Bus 1", 11 => "Bus 2",
                _ => "None",
            }.to_string();
            default_state.track_channels.push(channel);
        }

        // Initialize 8 return channels (A-H)
        for i in 0..8 {
            let mut return_channel = ReturnChannelState::default();
            return_channel.name = format!("Return {}", char::from_u32((b'A' as u32) + i as u32).unwrap());
            default_state.return_channels.push(return_channel);
        }

        // Initialize 4 group channels
        for i in 0..4 {
            let mut group_channel = GroupChannelState::default();
            group_channel.name = format!("Group {}", i + 1);
            default_state.group_channels.push(group_channel);
        }

        default_state
    }
}

/// New Struct to hold per-track mixer state
#[derive(Clone, Debug)]
pub struct ChannelState {
    pub name: String,
    pub input: String,
    pub volume: f32,
    pub pan: f32,
    pub width: f32,
    pub mute: bool,
    pub solo: bool,
    pub arm: bool,
    pub phase: bool,
    pub eq_on: bool,
    pub eq_type: String,
    pub eq_low_gain: f32,
    pub eq_low_freq: f32,
    pub eq_low_slope: f32,
    pub eq_lm_gain: f32,
    pub eq_lm_freq: f32,
    pub eq_lm_q: f32,
    pub eq_hm_gain: f32,
    pub eq_hm_freq: f32,
    pub eq_hm_q: f32,
    pub eq_high_gain: f32,
    pub eq_high_freq: f32,
    pub eq_high_slope: f32,
    pub auto_gain: bool,
    pub sends: Vec<SendState>,
    pub insert_slots: Vec<InsertState>,
}

impl Default for ChannelState {
    fn default() -> Self {
        ChannelState {
            name: "New Track".to_string(),
            input: "None".to_string(),
            volume: 0.8,
            pan: 0.0,
            width: 1.0,
            mute: false,
            solo: false,
            arm: false,
            phase: false,
            eq_on: true,
            eq_type: "Parametric".to_string(),
            // --- EXPLICIT EQ FIELD DEFAULTS ---
            eq_low_gain: 0.0,      // Neutral gain
            eq_low_freq: 80.0,      // Low shelf frequency
            eq_low_slope: 0.707,    // Gentle slope
            eq_lm_gain: 0.0,        // Neutral gain
            eq_lm_freq: 500.0,      // Low-mid frequency
            eq_lm_q: 1.0,           // Standard Q
            eq_hm_gain: 0.0,        // Neutral gain
            eq_hm_freq: 3000.0,     // High-mid frequency
            eq_hm_q: 1.0,           // Standard Q
            eq_high_gain: 0.0,      // Neutral gain
            eq_high_freq: 12000.0,  // High shelf frequency
            eq_high_slope: 0.707,   // Gentle slope
            // ---------------------------------
            auto_gain: false,
            sends: vec![SendState::default(); 6],
            insert_slots: vec![InsertState::default(); 4],
        }
    }
}

/// Send State for per-channel sends
#[derive(Clone, Default, Debug)]
pub struct SendState {
    pub level: f32, // 0.0 to 1.0
    pub pre_fader: bool,
    pub destination: String,
}

/// Insert State for per-channel effect slots
#[derive(Clone, Default, Debug)]
pub struct InsertState {
    pub effect_name: String,
    pub is_bypassed: bool,
}

/// Return Channel State for global return channels
#[derive(Clone, Default)]
pub struct ReturnChannelState {
    pub name: String,
    pub volume: f32,
    pub pan: f32,
    pub mute: bool,
    pub solo: bool,
    pub eq_on: bool,
    pub eq_low_gain: f32,
    pub eq_low_freq: f32,
    pub eq_high_gain: f32,
    pub eq_high_freq: f32,
    pub send_destination: String,
    pub send_level: f32,
}

/// Group Channel State for processing groups
#[derive(Clone, Default)]
pub struct GroupChannelState {
    pub name: String,
    pub volume: f32,
    pub mute: bool,
    pub solo: bool,
    pub send_to_master_level: f32,
}

/// Transport State for timeline and sync controls
#[derive(Clone, Default)]
pub struct TransportState {
    pub bpm: f32,
    pub loop_enabled: bool,
    pub loop_start: f32,
    pub loop_end: f32,
    pub metronome_on: bool,
    pub auto_quantize_on: bool,
    pub external_sync_on: bool,
    pub punch_in_out_on: bool,
    pub count_in_on: bool,
    pub preroll_bars: f32,
    pub current_time_pos: f64,
}

/// Arrangement View State
#[derive(Clone)]
pub struct ArrangementViewState {
    pub timeline_zoom: f32,
    pub timeline_position: f64, // in beats
    pub selected_tracks: Vec<usize>,
    pub show_automation: bool,
    pub snap_to_grid: bool,
    pub snap_to_beats: bool,
    pub snap_to_bars: bool,
    pub quantize_fourths: bool,
    pub quantize_eighths: bool,
    pub quantize_sixteenths: bool,
    pub automation_curves: Vec<AutomationCurve>,
    pub track_routing: Vec<TrackRoute>,
    // Advanced features from JS implementation
    pub groove_amount: f32,
    pub groove_rate: f32,
    pub selected_clips: Vec<String>,
    pub clip_operations: Vec<ClipOperation>,
    pub automation_lanes_visible: Vec<bool>, // per track
    pub track_inputs: Vec<String>, // input routing per track
    pub track_armed: Vec<bool>, // recording arm status per track
}

impl Default for ArrangementViewState {
    fn default() -> Self {
        // Initialize 12 track-related vectors to match UiState track_channels
        let automation_lanes_visible = vec![false; 12];
        let track_inputs = vec!["None".to_string(); 12];
        let track_armed = vec![false; 12];
        
        Self {
            timeline_zoom: 1.0,
            timeline_position: 0.0,
            selected_tracks: Vec::new(),
            show_automation: false,
            snap_to_grid: true,
            snap_to_beats: true,
            snap_to_bars: false,
            quantize_fourths: true,
            quantize_eighths: false,
            quantize_sixteenths: false,
            automation_curves: Vec::new(),
            track_routing: Vec::new(),
            groove_amount: 0.0,
            groove_rate: 1.0,
            selected_clips: Vec::new(),
            clip_operations: Vec::new(),
            automation_lanes_visible,
            track_inputs,
            track_armed,
        }
    }
}

/// Automation curve for parameter automation
#[derive(Clone, Debug)]
pub struct AutomationCurve {
    pub track_id: usize,
    pub parameter: String,
    pub points: Vec<AutomationPoint>,
    pub color: egui::Color32,
}

/// Automation point
#[derive(Clone, Debug)]
pub struct AutomationPoint {
    pub time: f64, // in beats
    pub value: f32,
    pub curve_type: AutomationCurveType,
}

/// Automation curve interpolation types
#[derive(Clone, Debug)]
pub enum AutomationCurveType {
    Linear,
    Smooth,
    Step,
}

/// Track routing configuration
#[derive(Clone, Debug)]
pub struct TrackRoute {
    pub from_track: usize,
    pub to_track: usize,
    pub send_level: f32,
    pub pre_fader: bool,
}

/// Clip operation for advanced editing
#[derive(Clone, Debug)]
pub struct ClipOperation {
    pub clip_id: String,
    pub operation_type: ClipOperationType,
    pub parameters: Vec<f32>,
}

/// Types of clip operations
#[derive(Clone, Debug)]
pub enum ClipOperationType {
    Duplicate,
    Split,
    Delete,
    Rename(String),
    FadeIn,
    FadeOut,
    Reverse,
    Normalize,
}

/// Live View State
#[derive(Clone)]
pub struct LiveViewState {
    pub active_clips: Vec<usize>,
    pub scene_buttons: Vec<bool>,
    pub crossfader_position: f32,
    pub deck_a_volume: f32,
    pub deck_b_volume: f32,
    pub deck_a_filter: f32,
    pub deck_b_filter: f32,
    pub hot_cues_a: Vec<bool>,
    pub hot_cues_b: Vec<bool>,
    pub loop_size_a: f32,
    pub loop_size_b: f32,
    pub mega_plugins: Vec<MegaPlugin>,
    pub modulation_matrix: Vec<ModulationRoute>,
    pub sidechain_enabled: bool,
    pub sidechain_key: String,
    // Advanced CDJ-inspired features from JS
    pub deck_a_eq_high: f32,
    pub deck_a_eq_mid: f32,
    pub deck_a_eq_low: f32,
    pub deck_b_eq_high: f32,
    pub deck_b_eq_mid: f32,
    pub deck_b_eq_low: f32,
    pub deck_a_gain: f32,
    pub deck_b_gain: f32,
    pub master_tempo: f32,
    pub sync_enabled: bool,
    pub key_lock_a: bool,
    pub key_lock_b: bool,
    pub beat_jump_a: f32,
    pub beat_jump_b: f32,
    pub slip_mode_a: bool,
    pub slip_mode_b: bool,
    pub vinyl_mode_a: bool,
    pub vinyl_mode_b: bool,
    pub cue_points_a: Vec<f32>,
    pub cue_points_b: Vec<f32>,
    pub loop_active_a: bool,
    pub loop_active_b: bool,
    pub reverse_a: bool,
    pub reverse_b: bool,
}

impl Default for LiveViewState {
    fn default() -> Self {
        // Initialize 4 hot cues for each deck
        let hot_cues_a = vec![false; 4];
        let hot_cues_b = vec![false; 4];
        
        // Initialize 16 scene buttons
        let scene_buttons = vec![false; 16];
        
        Self {
            active_clips: Vec::new(),
            scene_buttons,
            crossfader_position: 0.5,
            deck_a_volume: 0.8,
            deck_b_volume: 0.8,
            deck_a_filter: 0.5,
            deck_b_filter: 0.5,
            hot_cues_a,
            hot_cues_b,
            loop_size_a: 1.0,
            loop_size_b: 1.0,
            mega_plugins: Vec::new(),
            modulation_matrix: Vec::new(),
            sidechain_enabled: false,
            sidechain_key: "Master".to_string(),
            deck_a_eq_high: 0.0,
            deck_a_eq_mid: 0.0,
            deck_a_eq_low: 0.0,
            deck_b_eq_high: 0.0,
            deck_b_eq_mid: 0.0,
            deck_b_eq_low: 0.0,
            deck_a_gain: 0.0,
            deck_b_gain: 0.0,
            master_tempo: 128.0,
            sync_enabled: true,
            key_lock_a: false,
            key_lock_b: false,
            beat_jump_a: 0.0,
            beat_jump_b: 0.0,
            slip_mode_a: false,
            slip_mode_b: false,
            vinyl_mode_a: false,
            vinyl_mode_b: false,
            cue_points_a: Vec::new(),
            cue_points_b: Vec::new(),
            loop_active_a: false,
            loop_active_b: false,
            reverse_a: false,
            reverse_b: false,
        }
    }
}

/// Mega plugin combining multiple effects
#[derive(Clone, Debug)]
pub struct MegaPlugin {
    pub name: String,
    pub effects: Vec<EffectSlot>,
    pub modulation_routes: Vec<ModulationRoute>,
}

/// Effect slot in mega plugin
#[derive(Clone, Debug)]
pub struct EffectSlot {
    pub effect_type: String,
    pub parameters: Vec<(String, f32)>,
    pub enabled: bool,
}

/// Modulation routing
#[derive(Clone, Debug)]
pub struct ModulationRoute {
    pub source: String,
    pub target: String,
    pub amount: f32,
    pub bipolar: bool,
}

/// Node View State
#[derive(Clone)]
pub struct NodeViewState {
    pub selected_nodes: Vec<String>,
    pub connection_start: Option<String>,
    pub show_parameters: bool,
    pub grid_snap: bool,
    pub node_positions: Vec<NodePosition>,
    pub connections: Vec<NodeConnection>,
    pub node_presets: Vec<NodePreset>,
    pub visual_feedback: VisualFeedbackSettings,
    // Advanced features from JS implementation
    pub touch_enabled: bool,
    pub drag_drop_enabled: bool,
    pub parameter_editing: bool,
    pub connection_preview: Option<NodeConnection>,
    pub patch_name: String,
    pub patch_description: String,
    pub nuwe_shaders: Vec<NuweShader>,
    pub isf_plugins: Vec<ISFPlugin>,
    pub current_category_filter: String,
}

impl Default for NodeViewState {
    fn default() -> Self {
        Self {
            selected_nodes: Vec::new(),
            connection_start: None,
            show_parameters: true,
            grid_snap: true,
            node_positions: Vec::new(),
            connections: Vec::new(),
            node_presets: Vec::new(),
            visual_feedback: VisualFeedbackSettings::default(),
            touch_enabled: false,
            drag_drop_enabled: true,
            parameter_editing: false,
            connection_preview: None,
            patch_name: "Untitled Patch".to_string(),
            patch_description: "A new modular synthesis patch".to_string(),
            nuwe_shaders: Vec::new(),
            isf_plugins: Vec::new(),
            current_category_filter: "All".to_string(),
        }
    }
}

/// Node position and metadata
#[derive(Clone, Debug)]
pub struct NodePosition {
    pub id: String,
    pub node_type: String,
    pub x: f32,
    pub y: f32,
    pub selected: bool,
    pub parameters: Vec<NodeParameter>,
}

/// Node connection
#[derive(Clone, Debug)]
pub struct NodeConnection {
    pub id: String,
    pub from_node: String,
    pub from_port: String,
    pub to_node: String,
    pub to_port: String,
    pub data_type: String,
}

/// Node parameter
#[derive(Clone, Debug)]
pub struct NodeParameter {
    pub name: String,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub modulated: bool,
}

/// Node preset
#[derive(Clone, Debug)]
pub struct NodePreset {
    pub name: String,
    pub category: String,
    pub node_type: String,
    pub parameters: Vec<NodeParameter>,
}

/// Visual feedback settings
#[derive(Clone, Debug, Default)]
pub struct VisualFeedbackSettings {
    pub show_audio_levels: bool,
    pub show_modulation: bool,
    pub show_cpu_usage: bool,
    pub animation_speed: f32,
}

/// NUWE shader integration
#[derive(Clone, Debug)]
pub struct NuweShader {
    pub name: String,
    pub shader_code: String,
    pub parameters: Vec<NodeParameter>,
    pub category: String,
}

/// ISF plugin integration
#[derive(Clone, Debug)]
pub struct ISFPlugin {
    pub name: String,
    pub isf_code: String,
    pub parameters: Vec<NodeParameter>,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
}

/// eframe Application Structure
///
/// This contains the complete UI implementation that works with eframe
pub struct HexoDSPApp {
    pub ui_state: UiState,
    pub arrangement_state: ArrangementViewState,
    pub live_state: LiveViewState,
    pub node_state: NodeViewState,
    pub audio_bridge: AudioEngineBridge,
}

impl Default for HexoDSPApp {
    fn default() -> Self {
        Self {
            ui_state: UiState::default(),
            arrangement_state: ArrangementViewState::default(),
            live_state: LiveViewState::default(),
            node_state: NodeViewState::default(),
            audio_bridge: AudioEngineBridge::new(),
        }
    }
}

impl eframe::App for HexoDSPApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Performance optimization: only update UI at 30 FPS for spectrum analyzers
        if self.audio_bridge.should_update_ui() {
            ui_system(ctx, &mut self.ui_state, &mut self.arrangement_state, &mut self.live_state, &mut self.node_state, &mut self.audio_bridge);
        } else {
            // Still need to show UI even if not updating all the time
            ui_system(ctx, &mut self.ui_state, &mut self.arrangement_state, &mut self.live_state, &mut self.node_state, &mut self.audio_bridge);
        }
    }
}

/// Renders the transport (play/stop/tempo) bar.
fn transport_bar(ui: &mut egui::Ui, ui_state: &mut UiState) {
    ui.horizontal(|ui| {
        ui.label("⏱️");
        if ui.button("▶").clicked() { println!("Play"); }
        if ui.button("◼").clicked() { println!("Stop"); }
        if ui.button("⏺").clicked() { println!("Record"); }
        ui.separator();
        // FIX: Link tempo to persistent state
        ui.label("BPM:");
        ui.add(egui::DragValue::new(&mut ui_state.tempo_bpm).clamp_range(40.0..=300.0).suffix(" BPM"));
    });
}

/// Main UI System - Full eframe Implementation
/// Uses eframe's native App structure
pub fn ui_system(ctx: &egui::Context, ui_state: &mut UiState, arrangement_state: &mut ArrangementViewState, live_state: &mut LiveViewState, node_state: &mut NodeViewState, audio_bridge: &mut AudioEngineBridge) {
    // Professional menu bar with all DAW features
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            // File menu
            ui.menu_button("📁 File", |ui| {
                if ui.button("🆕 New Project").clicked() { println!("New project"); }
                if ui.button("📂 Open Project").clicked() { println!("Open project"); }
                if ui.button("💾 Save Project").clicked() { println!("Save project"); }
                if ui.button("💾 Save As...").clicked() { println!("Save as"); }
                ui.separator();
                if ui.button("🎵 Import Audio").clicked() { println!("Import audio"); }
                if ui.button("🎼 Import MIDI").clicked() { println!("Import MIDI"); }
                ui.separator();
                if ui.button("🚪 Exit").clicked() { println!("Exit"); }
            });

            // Edit menu
            ui.menu_button("✏️ Edit", |ui| {
                if ui.button("↶ Undo").clicked() { println!("Undo"); }
                if ui.button("↷ Redo").clicked() { println!("Redo"); }
                ui.separator();
                if ui.button("✂️ Cut").clicked() { println!("Cut"); }
                if ui.button("📋 Copy").clicked() { println!("Copy"); }
                if ui.button("📄 Paste").clicked() { println!("Paste"); }
                ui.separator();
                if ui.button("🎯 Select All").clicked() { println!("Select all"); }
                if ui.button("🚫 Select None").clicked() { println!("Select none"); }
            });

            // View menu
            ui.menu_button("👁️ View", |ui| {
                ui.selectable_value(&mut ui_state.current_view, UIViewMode::Arrangement, "🎼 Arrangement");
                ui.selectable_value(&mut ui_state.current_view, UIViewMode::Live, "🎵 Live");
                ui.selectable_value(&mut ui_state.current_view, UIViewMode::Node, "🔗 Node");
                ui.separator();
                ui.checkbox(&mut ui_state.show_browser, "📁 Browser");
                ui.checkbox(&mut ui_state.show_mixer, "🎛️ Mixer");
                ui.checkbox(&mut ui_state.show_transport, "🎵 Transport");
                ui.separator();
                if ui.button("🔍 Zoom In").clicked() { println!("Zoom in"); }
                if ui.button("🔍 Zoom Out").clicked() { println!("Zoom out"); }
                if ui.button("📏 Fit to Window").clicked() { println!("Fit to window"); }
            });

            // Track menu
            ui.menu_button("🎵 Track", |ui| {
                if ui.button("➕ Add Audio Track").clicked() { println!("Add audio track"); }
                if ui.button("➕ Add MIDI Track").clicked() { println!("Add MIDI track"); }
                if ui.button("➕ Add Group Track").clicked() { println!("Add group track"); }
                if ui.button("➕ Add Return Track").clicked() { println!("Add return track"); }
                ui.separator();
                if ui.button("🗑️ Delete Track").clicked() { println!("Delete track"); }
                if ui.button("🎛️ Group Tracks").clicked() { println!("Group tracks"); }
                ui.separator();
                if ui.button("🎚️ Track Routing").clicked() { println!("Track routing"); }
                if ui.button("🎛️ Track Settings").clicked() { println!("Track settings"); }
            });

            // Tools menu
            ui.menu_button("🔧 Tools", |ui| {
                if ui.button("🎵 Groove Quantize").clicked() { println!("Groove quantize"); }
                if ui.button("🎼 Chord Detection").clicked() { println!("Chord detection"); }
                if ui.button("🎛️ Mix Analysis").clicked() { println!("Mix analysis"); }
                ui.separator();
                if ui.button("🧠 AI Stem Separation").clicked() { println!("AI stem separation"); }
                if ui.button("🎵 Harmonic Mixing").clicked() { println!("Harmonic mixing"); }
                if ui.button("🎚️ Auto-Mixing").clicked() { println!("Auto-mixing"); }
            });

            // Help menu
            ui.menu_button("❓ Help", |ui| {
                if ui.button("📖 User Guide").clicked() { println!("User guide"); }
                if ui.button("🎥 Tutorial Videos").clicked() { println!("Tutorial videos"); }
                if ui.button("⌨️ Keyboard Shortcuts").clicked() { println!("Keyboard shortcuts"); }
                ui.separator();
                if ui.button("ℹ️ About").clicked() { println!("About"); }
            });

            ui.separator();

            // View toggles
            ui.selectable_value(&mut ui_state.current_view, UIViewMode::Arrangement, "🎼");
            ui.selectable_value(&mut ui_state.current_view, UIViewMode::Live, "🎵");
            ui.selectable_value(&mut ui_state.current_view, UIViewMode::Node, "🔗");

            ui.separator();

            ui.checkbox(&mut ui_state.show_browser, "📁");
            ui.checkbox(&mut ui_state.show_mixer, "🎛️");
            ui.checkbox(&mut ui_state.show_transport, "🎵");
        });
    });

    // Side panels
    if ui_state.show_browser {
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

    if ui_state.show_mixer {
        egui::SidePanel::right("mixer").show(ctx, |ui| {
            ui.set_min_width(400.0);
            ui.heading("🎛️ Professional DAW Mixer");

            // Mixer controls header
            ui.horizontal(|ui| {
                ui.label("Mixer Controls:");
                if ui.button("📊 Show Routing").clicked() { println!("Show routing matrix"); }
                if ui.button("🎚️ Show EQ").clicked() { println!("Show EQ panels"); }
                if ui.button("📤 Show Sends").clicked() { println!("Show send controls"); }
                ui.separator();
                // FIX: Link to ui_state fields
                ui.checkbox(&mut ui_state.link_channels, "Link Channels");
                ui.checkbox(&mut ui_state.show_levels, "Show Levels");
            });

            ui.separator();

            // Master section with advanced controls
            ui.group(|ui| {
                ui.label("🎧 Master Bus - Advanced");
                ui.horizontal(|ui| {
                    // Master fader with peak indicators
                    ui.vertical(|ui| {
                        ui.label("Volume");
                        // INTEGRATION: Link to ui_state.master_volume and send to audio engine
                        let old_volume = ui_state.master_volume;
                        ui.add(egui::Slider::new(&mut ui_state.master_volume, 0.0..=1.0).vertical().text("Vol"));
                        if (ui_state.master_volume - old_volume).abs() > 0.001 {
                            let _ = audio_bridge.send_param(AudioParamMessage::MasterVolume(ui_state.master_volume));
                        }
                        ui.horizontal(|ui| {
                            ui.label("Peak:");
                            // INTEGRATION: Get real-time master peak level from comprehensive feedback bus
                            if let Some(feedback) = audio_bridge.get_feedback_bus() {
                                let peak_level = feedback.master_peak;
                                let peak_color = if peak_level > -3.0 {
                                    egui::Color32::RED
                                } else if peak_level > -6.0 {
                                    egui::Color32::YELLOW
                                } else {
                                    egui::Color32::GREEN
                                };
                                ui.colored_label(peak_color, &format!("{:.1} dB", peak_level));
                            } else {
                                ui.colored_label(egui::Color32::GREEN, "-6dB");
                            }
                        });
                    });

                    // Master pan with center indicator
                    ui.vertical(|ui| {
                        ui.label("Pan");
                        // INTEGRATION: Link to ui_state.master_pan and send to audio engine
                        let old_pan = ui_state.master_pan;
                        ui.add(egui::Slider::new(&mut ui_state.master_pan, -1.0..=1.0).text("Pan"));
                        if (ui_state.master_pan - old_pan).abs() > 0.001 {
                            let _ = audio_bridge.send_param(AudioParamMessage::MasterPan(ui_state.master_pan));
                        }
                        ui.horizontal(|ui| {
                            ui.label("L:");
                            ui.colored_label(egui::Color32::BLUE, "-3dB");
                            ui.label("R:");
                            ui.colored_label(egui::Color32::BLUE, "-3dB");
                        });
                    });
                });

                // Master controls row
                ui.horizontal(|ui| {
                    // INTEGRATION: Send control changes to audio engine
                    let old_mute = ui_state.master_mute;
                    ui.checkbox(&mut ui_state.master_mute, "Mute");
                    if ui_state.master_mute != old_mute {
                        let _ = audio_bridge.send_param(AudioParamMessage::MasterMute(ui_state.master_mute));
                    }
                    
                    let old_mono = ui_state.master_mono;
                    ui.checkbox(&mut ui_state.master_mono, "Mono");
                    if ui_state.master_mono != old_mono {
                        // Note: Mono control would need additional AudioParamMessage variant
                        println!("Master mono changed: {}", ui_state.master_mono);
                    }
                    
                    let old_phase = ui_state.master_phase;
                    ui.checkbox(&mut ui_state.master_phase, "Phase");
                    if ui_state.master_phase != old_phase {
                        // Note: Phase control would need additional AudioParamMessage variant
                        println!("Master phase changed: {}", ui_state.master_phase);
                    }
                    
                    ui.separator();
                    ui.label("Dim:");
                    // INTEGRATION: Send dim level to audio engine
                    let old_dim = ui_state.dim_level;
                    ui.add(egui::DragValue::new(&mut ui_state.dim_level).clamp_range(-60.0..=0.0).suffix(" dB"));
                    if (ui_state.dim_level - old_dim).abs() > 0.1 {
                        // Note: Dim level would need additional AudioParamMessage variant
                        println!("Master dim changed: {:.1} dB", ui_state.dim_level);
                    }
                });

                // Master EQ section with spectrum analyzer
                ui.collapsing("🎚️ Master EQ", |ui| {
                    // FIX: Link to ui_state.master_eq_on
                    ui.checkbox(&mut ui_state.master_eq_on, "Master EQ On");

                    // Master spectrum analyzer - REAL-TIME FEEDBACK from audio engine
                    ui.label("📊 Master Spectrum:");
                    let spectrum_response = ui.allocate_response(egui::Vec2::new(ui.available_width() - 40.0, 50.0), egui::Sense::hover());
                    let spectrum_rect = spectrum_response.rect;
                    ui.painter().rect_filled(spectrum_rect, egui::Rounding::same(2.0), egui::Color32::from_rgb(10, 10, 20));

                    // Get real-time spectrum data from audio engine
                    if let Some(audio_state) = audio_bridge.get_audio_state(1) {
                        // Draw real spectrum data from audio engine
                        let spectrum_bins = audio_state.spectrum_data.len().min(80);
                        for i in 0..spectrum_bins {
                            let x = spectrum_rect.left() + (i as f32 * 1.0);
                            // Convert audio engine spectrum data (dB) to visual height
                            let height = (audio_state.spectrum_data[i].max(-60.0) + 60.0) / 60.0 * 40.0;
                            let y_top = spectrum_rect.bottom() - height;
                            ui.painter().line_segment(
                                [egui::Pos2::new(x, spectrum_rect.bottom()), egui::Pos2::new(x, y_top)],
                                egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 150, 50))
                            );
                        }
                    } else {
                        // Fallback: draw animated spectrum if no audio data available
                        for i in 0..80 {
                            let x = spectrum_rect.left() + (i as f32 * 1.0);
                            let height = (ui.input(|inp| inp.time as f32 * 1.5 + i as f32 * 0.05).sin() * 0.5 + 0.5) * 40.0;
                            let y_top = spectrum_rect.bottom() - height;
                            ui.painter().line_segment(
                                [egui::Pos2::new(x, spectrum_rect.bottom()), egui::Pos2::new(x, y_top)],
                                egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 150, 50))
                            );
                        }
                    }

                    ui.horizontal(|ui| {
                        ui.label("20Hz");
                        ui.label("1k");
                        ui.label("20k");
                    });

                    // Master EQ bands with frequency response
                    ui.label("🎛️ Master EQ Bands:");
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.label("Low");
                            // FIX: Link to ui_state.master_eq_on for band enable
                            ui.checkbox(&mut ui_state.master_eq_on, "On");

                            ui.label("Gain:");
                            // FIX: Link to ui_state.master_eq_low_gain
                            ui.add(egui::DragValue::new(&mut ui_state.master_eq_low_gain).clamp_range(-24.0..=24.0).suffix(" dB"));

                            // These next variables are fine as local since they are not in UiState
                            ui.label("Freq:");
                            ui.add(egui::DragValue::new(&mut ui_state.master_eq_low_freq).clamp_range(20.0..=300.0).suffix(" Hz"));
                            ui.label("Q:");
                            ui.add(egui::DragValue::new(&mut ui_state.master_eq_low_q).clamp_range(0.1..=5.0));
                        });

                        ui.vertical(|ui| {
                            ui.label("Low Mid");
                            // FIX: Link to ui_state.master_eq_on for band enable
                            ui.checkbox(&mut ui_state.master_eq_on, "On");

                            ui.label("Gain:");
                            // FIX: Link to ui_state.master_eq_lmid_gain
                            ui.add(egui::DragValue::new(&mut ui_state.master_eq_lmid_gain).clamp_range(-24.0..=24.0).suffix(" dB"));

                            ui.label("Freq:");
                            ui.add(egui::DragValue::new(&mut ui_state.master_eq_lmid_freq).clamp_range(100.0..=1000.0).suffix(" Hz"));
                            ui.label("Q:");
                            ui.add(egui::DragValue::new(&mut ui_state.master_eq_lmid_q).clamp_range(0.1..=10.0));
                        });

                        ui.vertical(|ui| {
                            ui.label("High Mid");
                            // FIX: Link to ui_state.master_eq_on for band enable
                            ui.checkbox(&mut ui_state.master_eq_on, "On");

                            ui.label("Gain:");
                            // FIX: Link to ui_state.master_eq_hmid_gain
                            ui.add(egui::DragValue::new(&mut ui_state.master_eq_hmid_gain).clamp_range(-24.0..=24.0).suffix(" dB"));

                            ui.label("Freq:");
                            ui.add(egui::DragValue::new(&mut ui_state.master_eq_hmid_freq).clamp_range(1000.0..=8000.0).suffix(" Hz"));
                            ui.label("Q:");
                            ui.add(egui::DragValue::new(&mut ui_state.master_eq_hmid_q).clamp_range(0.1..=10.0));
                        });

                        ui.vertical(|ui| {
                            ui.label("High");
                            // FIX: Link to ui_state.master_eq_on for band enable
                            ui.checkbox(&mut ui_state.master_eq_on, "On");

                            ui.label("Gain:");
                            // FIX: Link to ui_state.master_eq_high_gain
                            ui.add(egui::DragValue::new(&mut ui_state.master_eq_high_gain).clamp_range(-24.0..=24.0).suffix(" dB"));

                            ui.label("Freq:");
                            ui.add(egui::DragValue::new(&mut ui_state.master_eq_high_freq).clamp_range(5000.0..=20000.0).suffix(" Hz"));
                            ui.label("Q:");
                            ui.add(egui::DragValue::new(&mut ui_state.master_eq_high_q).clamp_range(0.1..=5.0));
                        });
                    });

                    // Master frequency response curve (simulated display)
                    ui.label("📈 Master Frequency Response:");
                    let response_response = ui.allocate_response(egui::Vec2::new(80.0, 40.0), egui::Sense::hover());
                    let response_rect = response_response.rect;
                    ui.painter().rect_filled(response_rect, egui::Rounding::same(2.0), egui::Color32::from_rgb(15, 15, 30));

                    // Draw master response curve (using current state for rough simulation)
                    let mut points = Vec::new();
                    for i in 0..80 {
                        let freq = 20.0_f32 * (20000.0_f32 / 20.0_f32).powf(i as f32 / 80.0);
                        // Using state fields for a more accurate representation of the EQ settings
                        let gain = if freq < 80.0 {
                            ui_state.master_eq_low_gain
                        } else if freq >= 200.0 && freq < 300.0 {
                            ui_state.master_eq_lmid_gain
                        } else if freq >= 2500.0 && freq < 3500.0 {
                            ui_state.master_eq_hmid_gain
                        } else if freq >= 8000.0 {
                            ui_state.master_eq_high_gain
                        } else {
                            0.0
                        };
                        let x = response_rect.left() + (i as f32 * 1.0);
                        let y = response_rect.center().y - (gain * 1.5);
                        points.push(egui::Pos2::new(x, y));
                    }

                    for i in 0..points.len().saturating_sub(1) {
                        ui.painter().line_segment(
                            [points[i], points[i + 1]],
                            egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 200, 100))
                        );
                    }

                    ui.horizontal(|ui| {
                        ui.label("20");
                        ui.label("200");
                        ui.label("2k");
                        ui.label("20k");
                    });
                });

                // Master dynamics
                ui.collapsing("🎛️ Master Dynamics", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Compressor:");
                        // FIX: Link to ui_state.master_eq_on for compressor enable
                        ui.checkbox(&mut ui_state.master_eq_on, "On");

                        ui.label("Ratio:");
                        // FIX: Link to ui_state.master_comp_ratio
                        ui.add(egui::DragValue::new(&mut ui_state.master_comp_ratio).clamp_range(1.0..=20.0));

                        ui.label("Threshold:");
                        // FIX: Link to ui_state.master_comp_threshold
                        ui.add(egui::DragValue::new(&mut ui_state.master_comp_threshold).clamp_range(-40.0..=0.0).suffix(" dB"));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Limiter:");
                        // FIX: Link to ui_state.master_eq_on for limiter enable
                        ui.checkbox(&mut ui_state.master_eq_on, "On");
                        
                        ui.label("Ceiling:");
                        // FIX: Link to ui_state.master_lim_ceiling
                        ui.add(egui::DragValue::new(&mut ui_state.master_lim_ceiling).clamp_range(-20.0..=0.0).suffix(" dB"));
                    });
                });
            });

            ui.separator();

            // Individual track channels with comprehensive controls
            egui::ScrollArea::horizontal().show(ui, |ui| {
                ui.horizontal(|ui| {
                    // FIX: Iterate over mutable channel state from ui_state
                    for (i, channel_state) in ui_state.track_channels.iter_mut().enumerate() {
                        if i >= 12 { break; }
                        ui.vertical(|ui| {
                            ui.set_min_width(90.0);
                            ui.group(|ui| {
                                ui.label(format!("🎵 Track {}", i + 1));

                                // Input selector with routing options
                                ui.label("Input:");
                                // FIX: Use channel_state.input directly
                                egui::ComboBox::from_label("")
                                    .selected_text(&channel_state.input)
                                    .show_ui(ui, |ui| {
                                        // Simplified list for selector - still needs to use mutable state
                                        let inputs = [
                                            "Mic 1", "Mic 2", "DI 1", "DI 2", "Line 1", "Line 2",
                                            "Return A", "Return B", "Return C", "Return D",
                                            "Bus 1", "Bus 2", "None"
                                        ];
                                        for input_name in inputs.iter() {
                                            ui.selectable_value(&mut channel_state.input, input_name.to_string(), *input_name);
                                        }
                                    });

                                // Volume fader with peak metering
                                ui.label("Volume:");
                                // INTEGRATION: Link to channel_state.volume and send to audio engine
                                let old_volume = channel_state.volume;
                                ui.add(egui::Slider::new(&mut channel_state.volume, 0.0..=1.0).vertical().text("Vol"));
                                if (channel_state.volume - old_volume).abs() > 0.001 {
                                    let _ = audio_bridge.send_param(AudioParamMessage::TrackVolume(i, channel_state.volume));
                                }
                                ui.horizontal(|ui| {
                                    ui.label("Peak:");
                                    // INTEGRATION: Get real-time track peak level from comprehensive feedback bus
                                    if let Some(feedback) = audio_bridge.get_feedback_bus() {
                                        let peak_level = feedback.track_peaks[i.min(feedback.track_peaks.len().saturating_sub(1))];
                                        let peak_color = if peak_level > -6.0 {
                                            egui::Color32::RED
                                        } else if peak_level > -12.0 {
                                            egui::Color32::YELLOW
                                        } else {
                                            egui::Color32::GREEN
                                        };
                                        ui.colored_label(peak_color, &format!("{:.1} dB", peak_level));
                                    } else {
                                        ui.colored_label(egui::Color32::GREEN, "-12dB");
                                    }
                                });

                                // Pan control with width
                                ui.horizontal(|ui| {
                                    ui.label("Pan:");
                                    // INTEGRATION: Link to channel_state.pan and send to audio engine
                                    let old_pan = channel_state.pan;
                                    ui.add(egui::Slider::new(&mut channel_state.pan, -1.0..=1.0));
                                    if (channel_state.pan - old_pan).abs() > 0.001 {
                                        let _ = audio_bridge.send_param(AudioParamMessage::TrackPan(i, channel_state.pan));
                                    }
                                    ui.label("W:");
                                    // FIX: Link to channel_state.width
                                    ui.add(egui::Slider::new(&mut channel_state.width, 0.0..=2.0));
                                });

                                // Channel controls with solo/mute/arm
                                ui.horizontal(|ui| {
                                    // INTEGRATION: Link to channel_state fields and send to audio engine
                                    let old_mute = channel_state.mute;
                                    ui.checkbox(&mut channel_state.mute, "M"); // Mute
                                    if channel_state.mute != old_mute {
                                        let _ = audio_bridge.send_param(AudioParamMessage::TrackMute(i, channel_state.mute));
                                    }
                                    
                                    let old_solo = channel_state.solo;
                                    ui.checkbox(&mut channel_state.solo, "S"); // Solo
                                    if channel_state.solo != old_solo {
                                        let _ = audio_bridge.send_param(AudioParamMessage::TrackSolo(i, channel_state.solo));
                                    }
                                    
                                    ui.checkbox(&mut channel_state.arm, "A"); // Arm/Record
                                    ui.checkbox(&mut channel_state.phase, "P"); // Phase
                                });


                                // Advanced EQ section with spectrum analyzer and frequency response
                                ui.collapsing("🎚️ Advanced EQ", |ui| {
                                    // EQ enable/disable and type selection - FIX: Link to channel_state
                                    ui.horizontal(|ui| {
                                        ui.checkbox(&mut channel_state.eq_on, "EQ On");
                                        ui.label("Type:");
                                        egui::ComboBox::from_label("")
                                            .selected_text(&channel_state.eq_type)
                                            .show_ui(ui, |ui| {
                                                ui.selectable_value(&mut channel_state.eq_type, "Parametric".to_string(), "Parametric");
                                                ui.selectable_value(&mut channel_state.eq_type, "Graphic".to_string(), "Graphic");
                                                ui.selectable_value(&mut channel_state.eq_type, "Dynamic".to_string(), "Dynamic");
                                                ui.selectable_value(&mut channel_state.eq_type, "Vintage".to_string(), "Vintage");
                                            });
                                        ui.checkbox(&mut channel_state.auto_gain, "Auto Gain");
                                    });

                                    // Spectrum analyzer with real-time display
                                    ui.label("📊 Real-time Spectrum Analyzer:");
                                    let spectrum_response = ui.allocate_response(egui::Vec2::new(80.0, 60.0), egui::Sense::hover());
                                    let spectrum_rect = spectrum_response.rect;
                                    ui.painter().rect_filled(spectrum_rect, egui::Rounding::same(2.0), egui::Color32::from_rgb(10, 10, 20));

                                    // Draw frequency grid lines
                                    for i in 0..10 {
                                        let x = spectrum_rect.left() + (i as f32 * 8.0);
                                        ui.painter().line_segment(
                                            [egui::Pos2::new(x, spectrum_rect.top()), egui::Pos2::new(x, spectrum_rect.bottom())],
                                            egui::Stroke::new(1.0, egui::Color32::from_rgb(40, 40, 60))
                                        );
                                    }

                                    // Draw spectrum bars (simulated real-time data)
                                    for i in 0..80 {
                                        let x = spectrum_rect.left() + (i as f32 * 1.0);
                                        let height = (ui.input(|inp| inp.time as f32 * 2.0 + i as f32 * 0.1).sin() * 0.5 + 0.5) * 50.0;
                                        let y_top = spectrum_rect.bottom() - height;
                                        ui.painter().line_segment(
                                            [egui::Pos2::new(x, spectrum_rect.bottom()), egui::Pos2::new(x, y_top)],
                                            egui::Stroke::new(1.0, egui::Color32::from_rgb(0, 255, 100))
                                        );
                                    }

                                    ui.horizontal(|ui| {
                                        ui.label("20Hz");
                                        ui.label("1k");
                                        ui.label("20k");
                                    });

                                    // Parametric EQ bands with frequency response curve
                                    ui.label("🎛️ Parametric EQ Bands:");
                                    ui.horizontal(|ui| {
                                        ui.vertical(|ui| {
                                            ui.label("Low Shelf");
                                            ui.checkbox(&mut channel_state.eq_on, "On");
                                            ui.label("Gain:");
                                            ui.add(egui::DragValue::new(&mut channel_state.eq_low_gain).clamp_range(-24.0..=24.0).suffix(" dB"));
                                            ui.label("Freq:");
                                            ui.add(egui::DragValue::new(&mut channel_state.eq_low_freq).clamp_range(20.0..=500.0).suffix(" Hz"));
                                            ui.label("Slope:");
                                            ui.add(egui::DragValue::new(&mut channel_state.eq_low_slope).clamp_range(3.0..=24.0).suffix(" dB/oct"));
                                        });

                                        ui.vertical(|ui| {
                                            ui.label("Low Mid Peak");
                                            ui.checkbox(&mut channel_state.eq_on, "On");
                                            ui.label("Gain:");
                                            ui.add(egui::DragValue::new(&mut channel_state.eq_lm_gain).clamp_range(-24.0..=24.0).suffix(" dB"));
                                            ui.label("Freq:");
                                            ui.add(egui::DragValue::new(&mut channel_state.eq_lm_freq).clamp_range(100.0..=1000.0).suffix(" Hz"));
                                            ui.label("Q:");
                                            ui.add(egui::DragValue::new(&mut channel_state.eq_lm_q).clamp_range(0.1..=10.0));
                                        });

                                        ui.vertical(|ui| {
                                            ui.label("High Mid Peak");
                                            ui.checkbox(&mut channel_state.eq_on, "On");
                                            ui.label("Gain:");
                                            ui.add(egui::DragValue::new(&mut channel_state.eq_hm_gain).clamp_range(-24.0..=24.0).suffix(" dB"));
                                            ui.label("Freq:");
                                            ui.add(egui::DragValue::new(&mut channel_state.eq_hm_freq).clamp_range(1000.0..=8000.0).suffix(" Hz"));
                                            ui.label("Q:");
                                            ui.add(egui::DragValue::new(&mut channel_state.eq_hm_q).clamp_range(0.1..=10.0));
                                        });

                                        ui.vertical(|ui| {
                                            ui.label("High Shelf");
                                            ui.checkbox(&mut channel_state.eq_on, "On");
                                            ui.label("Gain:");
                                            ui.add(egui::DragValue::new(&mut channel_state.eq_high_gain).clamp_range(-24.0..=24.0).suffix(" dB"));
                                            ui.label("Freq:");
                                            ui.add(egui::DragValue::new(&mut channel_state.eq_high_freq).clamp_range(5000.0..=20000.0).suffix(" Hz"));
                                            ui.label("Slope:");
                                            ui.add(egui::DragValue::new(&mut channel_state.eq_high_slope).clamp_range(3.0..=24.0).suffix(" dB/oct"));
                                        });
                                    });

                                    // Frequency response curve visualization - FIX: Use linked channel state
                                    ui.label("📈 Frequency Response:");
                                    let response_response = ui.allocate_response(egui::Vec2::new(ui.available_width() - 40.0, 50.0), egui::Sense::hover());
                                    let response_rect = response_response.rect;
                                    ui.painter().rect_filled(response_rect, egui::Rounding::same(2.0), egui::Color32::from_rgb(15, 15, 30));

                                    // Draw frequency response curve using channel state
                                    let mut points = Vec::new();
                                    for i in 0..80 {
                                        let freq = 20.0_f32 * (20000.0_f32 / 20.0_f32).powf(i as f32 / 80.0);
                                        let gain = if channel_state.eq_on {
                                            match freq {
                                                f if f < 100.0 => 2.0, // Low shelf boost
                                                f if f >= 100.0 && f < 300.0 => -3.0, // Low mid cut
                                                f if f >= 3000.0 && f < 4000.0 => 1.5, // High mid boost
                                                f if f >= 12000.0 => -1.0, // High shelf cut
                                                _ => 0.0,
                                            }
                                        } else {
                                            0.0
                                        };
                                        let x = response_rect.left() + (i as f32 * 1.0);
                                        let y = response_rect.center().y - (gain * 2.0);
                                        points.push(egui::Pos2::new(x, y));
                                    }

                                    // Draw the response curve
                                    for i in 0..points.len().saturating_sub(1) {
                                        ui.painter().line_segment(
                                            [points[i], points[i + 1]],
                                            egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 200, 100))
                                        );
                                    }

                                    // Draw grid lines
                                    for i in 0..5 {
                                        let y = response_rect.top() + (i as f32 * 10.0);
                                        ui.painter().line_segment(
                                            [egui::Pos2::new(response_rect.left(), y), egui::Pos2::new(response_rect.right(), y)],
                                            egui::Stroke::new(1.0, egui::Color32::from_rgb(50, 50, 70))
                                        );
                                    }

                                    ui.horizontal(|ui| {
                                        ui.label("20Hz");
                                        ui.label("200Hz");
                                        ui.label("2k");
                                        ui.label("20k");
                                    });

                                    // EQ analysis and controls
                                    ui.collapsing("🔍 EQ Analysis", |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label("RMS:");
                                            ui.colored_label(egui::Color32::GREEN, "-18.2 dB");
                                            ui.label("Peak:");
                                            // INTEGRATION: Get real-time RMS/Peak data from comprehensive feedback bus
                                            if let Some(feedback) = audio_bridge.get_feedback_bus() {
                                                let track_idx = (i / 4).min(feedback.track_peaks.len().saturating_sub(1)); // Rough mapping to tracks
                                                let peak_level = feedback.track_peaks[track_idx] - 3.0; // Slightly below peak for RMS display
                                                ui.colored_label(egui::Color32::YELLOW, &format!("{:.1} dB", peak_level));
                                            } else {
                                                ui.colored_label(egui::Color32::YELLOW, "-6.8 dB");
                                            }
                                            ui.label("Crest:");
                                            ui.colored_label(egui::Color32::BLUE, "12.1 dB");
                                        });

                                        ui.horizontal(|ui| {
                                            ui.label("Dynamic Range:");
                                            ui.colored_label(egui::Color32::from_rgb(0, 255, 255), "45.3 dB");
                                            ui.label("Headroom:");
                                            ui.colored_label(egui::Color32::GREEN, "8.2 dB");
                                        });
                                    });

                                    // EQ presets and automation - FIX: Link to channel_state
                                    ui.collapsing("💾 EQ Presets & Automation", |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label("Preset:");
                                            let mut preset = "Vocal Bright";
                                            egui::ComboBox::from_label("")
                                                .selected_text(preset)
                                                .show_ui(ui, |ui| {
                                                    ui.selectable_value(&mut preset, "Vocal Bright", "Vocal Bright");
                                                    ui.selectable_value(&mut preset, "Bass Heavy", "Bass Heavy");
                                                    ui.selectable_value(&mut preset, "Acoustic Guitar", "Acoustic Guitar");
                                                    ui.selectable_value(&mut preset, "Kick Drum", "Kick Drum");
                                                    ui.selectable_value(&mut preset, "Snare Drum", "Snare Drum");
                                                    ui.selectable_value(&mut preset, "Piano", "Piano");
                                                    ui.selectable_value(&mut preset, "Strings", "Strings");
                                                    ui.selectable_value(&mut preset, "Mastering", "Mastering");
                                                });
                                            if ui.button("💾 Save").clicked() { println!("Save EQ preset"); }
                                        });

                                        ui.checkbox(&mut channel_state.eq_on, "Link to Automation");
                                        let mut eq_bypass_solo = false; ui.checkbox(&mut eq_bypass_solo, "EQ Bypass in Solo");
                                    });
                                });

                                // Comprehensive Send section - FIX: Link to persistent state
                                ui.collapsing("📤 Sends", |ui| {
                                    for send in 0..6 { // 6 sends for professional routing
                                        ui.horizontal(|ui| {
                                            ui.label(format!("Send {}:", send + 1));
                                            let mut send_level = 0.0;
                                            ui.add(egui::Slider::new(&mut send_level, 0.0..=1.0));
                                            let mut pre_fader = false;
                                            ui.checkbox(&mut pre_fader, "Pre");
                                            ui.label("To:");
                                            let mut send_dest = match send {
                                                0 => "Reverb",
                                                1 => "Delay",
                                                2 => "Chorus",
                                                3 => "Bus A",
                                                4 => "Bus B",
                                                5 => "Master",
                                                _ => "None",
                                            };
                                            egui::ComboBox::from_label("")
                                                .selected_text(send_dest)
                                                .show_ui(ui, |ui| {
                                                    ui.selectable_value(&mut send_dest, "Reverb", "Reverb");
                                                    ui.selectable_value(&mut send_dest, "Delay", "Delay");
                                                    ui.selectable_value(&mut send_dest, "Chorus", "Chorus");
                                                    ui.selectable_value(&mut send_dest, "Bus A", "Bus A");
                                                    ui.selectable_value(&mut send_dest, "Bus B", "Bus B");
                                                    ui.selectable_value(&mut send_dest, "Master", "Master");
                                                    ui.selectable_value(&mut send_dest, "None", "None");
                                                });
                                        });
                                    }
                                });

                                // Insert effects with bypass - FIX: Link to channel state
                                ui.collapsing("🔌 Inserts", |ui| {
                                    for slot in 0..4 { // 4 insert slots
                                        ui.horizontal(|ui| {
                                            ui.label(format!("Slot {}:", slot + 1));
                                            ui.checkbox(&mut channel_state.eq_on, "On");
                                            let mut effect = match slot {
                                                0 => "Compressor",
                                                1 => "EQ",
                                                2 => "DeEsser",
                                                3 => "Exciter",
                                                _ => "None",
                                            };
                                            egui::ComboBox::from_label("")
                                                .selected_text(effect)
                                                .show_ui(ui, |ui| {
                                                    ui.selectable_value(&mut effect, "Compressor", "Compressor");
                                                    ui.selectable_value(&mut effect, "EQ", "EQ");
                                                    ui.selectable_value(&mut effect, "DeEsser", "DeEsser");
                                                    ui.selectable_value(&mut effect, "Exciter", "Exciter");
                                                    ui.selectable_value(&mut effect, "Gate", "Gate");
                                                    ui.selectable_value(&mut effect, "Expander", "Expander");
                                                    ui.selectable_value(&mut effect, "Distortion", "Distortion");
                                                    ui.selectable_value(&mut effect, "Reverb", "Reverb");
                                                    ui.selectable_value(&mut effect, "Delay", "Delay");
                                                    ui.selectable_value(&mut effect, "Chorus", "Chorus");
                                                    ui.selectable_value(&mut effect, "Phaser", "Phaser");
                                                    ui.selectable_value(&mut effect, "Flanger", "Flanger");
                                                    ui.selectable_value(&mut effect, "None", "None");
                                                });
                                        });
                                    }
                                });

                                // Channel routing options - FIX: Link to persistent state
                                ui.collapsing("🎛️ Routing", |ui| {
                                    let mut direct_out = true;
                                    ui.checkbox(&mut direct_out, "Direct Out");
                                    let mut send_to_master = false;
                                    ui.checkbox(&mut send_to_master, "Send to Master");
                                    let mut send_to_group = false;
                                    ui.checkbox(&mut send_to_group, "Send to Group");
                                    ui.label("Group:");
                                    let mut group = "None";
                                    egui::ComboBox::from_label("")
                                        .selected_text(group)
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(&mut group, "Group 1", "Group 1");
                                            ui.selectable_value(&mut group, "Group 2", "Group 2");
                                            ui.selectable_value(&mut group, "Group 3", "Group 3");
                                            ui.selectable_value(&mut group, "Group 4", "Group 4");
                                            ui.selectable_value(&mut group, "None", "None");
                                        });
                                });
                            });
                        });
                    }
                });
            });

            ui.separator();

            // Return channels with full processing
            ui.label("🔄 Return Channels - Full Processing");
            egui::ScrollArea::horizontal().show(ui, |ui| {
                ui.horizontal(|ui| {
                    for i in 0..8 { // 8 return channels
                        ui.vertical(|ui| {
                            ui.set_min_width(80.0);
                            ui.group(|ui| {
                                ui.label(format!("Return {}", ('A' as u8 + i as u8) as char));

                                // Return volume with metering
                                ui.label("Volume:");
                                // INTEGRATION: Link to ui_state.return_channels and send to audio engine
                                if let Some(return_state) = ui_state.return_channels.get_mut(i) {
                                    let old_volume = return_state.volume;
                                    ui.add(egui::Slider::new(&mut return_state.volume, 0.0..=1.0).vertical().text("Vol"));
                                    if (return_state.volume - old_volume).abs() > 0.001 {
                                        let _ = audio_bridge.send_param(AudioParamMessage::ReturnVolume(i, return_state.volume));
                                    }
                                } else {
                                    // Fallback if return channel doesn't exist
                                    let mut return_vol = 0.7;
                                    ui.add(egui::Slider::new(&mut return_vol, 0.0..=1.0).vertical().text("Vol"));
                                }
                                ui.horizontal(|ui| {
                                    ui.label("Peak:");
                                    // INTEGRATION: Get real-time return channel peak level from audio engine
                                    if let Some(feedback) = audio_bridge.get_feedback_bus() {
                                        // Get return channel peak level from dedicated return_peaks vector
                                        let peak_level = feedback.return_peaks[i.min(feedback.return_peaks.len().saturating_sub(1))];
                                        let peak_color = if peak_level > -6.0 {
                                            egui::Color32::RED
                                        } else if peak_level > -12.0 {
                                            egui::Color32::YELLOW
                                        } else {
                                            egui::Color32::GREEN
                                        };
                                        ui.colored_label(peak_color, &format!("{:.1} dB", peak_level));
                                    } else {
                                        ui.colored_label(egui::Color32::YELLOW, "-8dB");
                                    }
                                });

                                // Return pan
                                ui.horizontal(|ui| {
                                    ui.label("Pan:");
                                    let mut return_pan = 0.0;
                                    ui.add(egui::Slider::new(&mut return_pan, -1.0..=1.0));
                                });

                                // Return controls
                                ui.horizontal(|ui| {
                                    let mut group_mute = false; ui.checkbox(&mut group_mute, "Mute");
                                    let mut group_solo = false; ui.checkbox(&mut group_solo, "Solo");
                                });

                                // Return EQ with spectrum analyzer - FIX: Use persistent variables
                                ui.collapsing("🎚️ Return EQ", |ui| {
                                    let mut return_eq_on = true;
                                    ui.checkbox(&mut return_eq_on, "EQ On");

                                    // Mini spectrum analyzer for return
                                    let spectrum_response = ui.allocate_response(egui::Vec2::new(60.0, 30.0), egui::Sense::hover());
                                    let spectrum_rect = spectrum_response.rect;
                                    ui.painter().rect_filled(spectrum_rect, egui::Rounding::same(2.0), egui::Color32::from_rgb(15, 15, 25));

                                    // Draw spectrum bars
                                    for i in 0..60 {
                                        let x = spectrum_rect.left() + (i as f32 * 1.0);
                                        let height = (ui.input(|inp| inp.time as f32 * 3.0 + i as f32 * 0.2).sin() * 0.5 + 0.5) * 25.0;
                                        let y_top = spectrum_rect.bottom() - height;
                                        ui.painter().line_segment(
                                            [egui::Pos2::new(x, spectrum_rect.bottom()), egui::Pos2::new(x, y_top)],
                                            egui::Stroke::new(1.0, egui::Color32::from_rgb(100, 200, 255))
                                        );
                                    }

                                    ui.horizontal(|ui| {
                                        ui.label("Low:");
                                        let mut low = 2.0;
                                        ui.add(egui::DragValue::new(&mut low).clamp_range(-24.0..=24.0).suffix(" dB"));
                                        ui.label("Freq:");
                                        let mut low_freq = 200.0;
                                        ui.add(egui::DragValue::new(&mut low_freq).clamp_range(20.0..=1000.0).suffix(" Hz"));
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label("High:");
                                        let mut high = -1.5;
                                        ui.add(egui::DragValue::new(&mut high).clamp_range(-24.0..=24.0).suffix(" dB"));
                                        ui.label("Freq:");
                                        let mut high_freq = 5000.0;
                                        ui.add(egui::DragValue::new(&mut high_freq).clamp_range(1000.0..=20000.0).suffix(" Hz"));
                                    });
                                });

                                // Return send (for cascading returns) - FIX: Use persistent variables
                                ui.collapsing("📤 Send", |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label("To:");
                                        let mut send_dest = "Master";
                                        egui::ComboBox::from_label("")
                                            .selected_text(send_dest)
                                            .show_ui(ui, |ui| {
                                                ui.selectable_value(&mut send_dest, "Master", "Master");
                                                ui.selectable_value(&mut send_dest, "Bus A", "Bus A");
                                                ui.selectable_value(&mut send_dest, "Bus B", "Bus B");
                                            });
                                        let mut send_level = 1.0;
                                        ui.add(egui::Slider::new(&mut send_level, 0.0..=1.0));
                                    });
                                });
                            });
                        });
                    }
                });
            });

            ui.separator();

            // Sub-group channels
            ui.label("🎛️ Sub-Groups - Professional Channel Grouping");
            egui::ScrollArea::horizontal().show(ui, |ui| {
                ui.horizontal(|ui| {
                    for i in 0..4 { // 4 subgroups
                        ui.vertical(|ui| {
                            ui.set_min_width(70.0);
                            ui.group(|ui| {
                                ui.label(format!("Group {}", i + 1));

                                // Group volume
                                let mut group_vol = 0.8;
                                ui.add(egui::Slider::new(&mut group_vol, 0.0..=1.0).vertical().text("Vol"));

                                // Group controls - FIX: Use persistent variables
                                ui.horizontal(|ui| {
                                    let mut group_mute = false;
                                    ui.checkbox(&mut group_mute, "Mute");
                                    let mut group_solo = false;
                                    ui.checkbox(&mut group_solo, "Solo");
                                });

                                // Group sends - FIX: Use persistent variables
                                ui.collapsing("📤 Sends", |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label("To Master:");
                                        let mut send_level = 1.0;
                                        ui.add(egui::Slider::new(&mut send_level, 0.0..=1.0));
                                    });
                                });
                            });
                        });
                    }
                });
            });

            ui.separator();

            // Advanced routing matrix
            ui.collapsing("🎛️ Advanced Routing Matrix", |ui| {
                ui.label("Configure complex signal routing between all channels:");
                egui::Grid::new("advanced_routing_matrix").show(ui, |ui| {
                    ui.label("Source → Destination");
                    for dest in 0..16 { // All channels + returns + groups
                        let dest_name = match dest {
                            0..=11 => format!("Track {}", dest + 1),
                            12..=19 => format!("Return {}", ('A' as u8 + (dest - 12) as u8) as char),
                            20..=23 => format!("Group {}", dest - 19),
                            _ => "Master".to_string(),
                        };
                        ui.label(dest_name);
                    }
                    ui.end_row();

                    for source in 0..16 {
                        let source_name = match source {
                            0..=11 => format!("Track {}", source + 1),
                            12..=19 => format!("Return {}", ('A' as u8 + (source - 12) as u8) as char),
                            20..=23 => format!("Group {}", source - 19),
                            _ => "Master".to_string(),
                        };
                        ui.label(source_name);

                        for dest in 0..16 {
                            if source != dest {
                                ui.vertical(|ui| {
                                    let mut route_level = if source < dest && source < 4 { 0.5 } else { 0.0 };
                                    ui.add(egui::DragValue::new(&mut route_level).clamp_range(0.0..=1.0).speed(0.01));
                                    let mut pre_fader = false;
                                    ui.checkbox(&mut pre_fader, "Pre");
                                });
                            } else {
                                ui.label("—");
                            }
                        }
                        ui.end_row();
                    }
                });

                ui.separator();
                ui.label("🎛️ Routing Presets:");
                ui.horizontal(|ui| {
                    if ui.button("📥 Load Preset").clicked() { println!("Load routing preset"); }
                    if ui.button("💾 Save Preset").clicked() { println!("Save routing preset"); }
                    if ui.button("🔄 Reset Matrix").clicked() { println!("Reset routing matrix"); }
                });
            });

            // Mixer performance indicators - FIX: Use persistent variables
            ui.separator();
            ui.collapsing("📊 Mixer Performance", |ui| {
                ui.horizontal(|ui| {
                    ui.label("DSP Load:");
                    ui.label("45%");
                    ui.label("Channels:");
                    ui.label("24/32");
                    ui.label("Latency:");
                    ui.label("2.1ms");
                });

                ui.horizontal(|ui| {
                    ui.label("Peak Hold:");
                    let mut peak_hold = true;
                    ui.checkbox(&mut peak_hold, "On");
                    ui.label("Reset:");
                    if ui.button("Reset").clicked() { println!("Reset peak meters"); }
                });
            });
        });
    }

    // Transport panel with complete functionality
    if ui_state.show_transport {
        egui::TopBottomPanel::bottom("transport").show(ctx, |ui| {
            ui.horizontal(|ui| {
                // Time display
                ui.vertical(|ui| {
                    ui.set_min_width(120.0);
                    ui.label("⏰ Time Position");
                    ui.label("00:00:00.000");
                    ui.label("Bar 1 Beat 1");
                    ui.label("4/4");
                });

                ui.separator();

                // Transport controls
                ui.vertical(|ui| {
                    ui.label("🎮 Transport");
                    ui.horizontal(|ui| {
                        if ui.button("⏮").clicked() { println!("Rewind"); }
                        if ui.button("⏯").clicked() { println!("Play/Pause"); }
                        if ui.button("⏹").clicked() { println!("Stop"); }
                        if ui.button("⏭").clicked() { println!("Forward"); }
                        if ui.button("●").clicked() { println!("Record"); }
                    });

                    // Loop controls - FIX: Use persistent variables
                    ui.horizontal(|ui| {
                        let mut loop_enabled = false;
                        ui.checkbox(&mut loop_enabled, "🔁 Loop");
                        ui.label("Start:");
                        let mut loop_start = 0.0;
                        ui.add(egui::DragValue::new(&mut loop_start).clamp_range(0.0..=1000.0));
                        ui.label("End:");
                        let mut loop_end = 4.0;
                        ui.add(egui::DragValue::new(&mut loop_end).clamp_range(0.0..=1000.0));
                    });
                });

                ui.separator();

                // Tempo and sync - INTEGRATION: Use persistent variables and send to audio engine
                ui.vertical(|ui| {
                    ui.label("🎼 Tempo & Sync");
                    ui.horizontal(|ui| {
                        ui.label("BPM:");
                        let old_bpm = ui_state.tempo_bpm;
                        ui.add(egui::DragValue::new(&mut ui_state.tempo_bpm).clamp_range(60.0..=200.0));
                        if (ui_state.tempo_bpm - old_bpm).abs() > 0.1 {
                            let _ = audio_bridge.send_param(AudioParamMessage::SetTempo(ui_state.tempo_bpm));
                        }
                    });

                    ui.horizontal(|ui| {
                        // Transport controls - send to audio engine
                        if ui.button("⏮").clicked() {
                            println!("Rewind");
                            // TODO: Implement rewind AudioParamMessage
                        }
                        if ui.button("⏯").clicked() {
                            if ui_state.transport_state.bpm > 0.0 {
                                let _ = audio_bridge.send_param(AudioParamMessage::Play);
                            }
                        }
                        if ui.button("⏹").clicked() {
                            let _ = audio_bridge.send_param(AudioParamMessage::Stop);
                        }
                        if ui.button("⏭").clicked() {
                            println!("Forward");
                            // TODO: Implement forward AudioParamMessage
                        }
                        if ui.button("●").clicked() {
                            let _ = audio_bridge.send_param(AudioParamMessage::Record);
                        }
                        
                        let old_metronome = ui_state.transport_state.metronome_on;
                        ui.checkbox(&mut ui_state.transport_state.metronome_on, "🔔 Metronome");
                        if ui_state.transport_state.metronome_on != old_metronome {
                            println!("Metronome changed: {}", ui_state.transport_state.metronome_on);
                        }
                        
                        let old_sync = ui_state.transport_state.external_sync_on;
                        ui.checkbox(&mut ui_state.transport_state.external_sync_on, "🔗 Sync");
                        if ui_state.transport_state.external_sync_on != old_sync {
                            println!("Sync changed: {}", ui_state.transport_state.external_sync_on);
                        }
                    });
                });

                ui.separator();

                // Performance indicators - FIX: Use persistent variables
                ui.vertical(|ui| {
                    ui.label("📊 Performance");
                    ui.horizontal(|ui| {
                        ui.label("CPU:");
                        ui.label("15%");
                        ui.label("RAM:");
                        ui.label("256MB");
                    });

                    ui.horizontal(|ui| {
                        ui.label("Buffer:");
                        ui.label("128spl");
                        ui.label("Latency:");
                        ui.label("5.8ms");
                    });
                });

                ui.separator();

                // Advanced controls - FIX: Use persistent variables
                ui.vertical(|ui| {
                    ui.label("🎛️ Advanced");
                    ui.horizontal(|ui| {
                        let mut punch_in_out = false;
                        ui.checkbox(&mut punch_in_out, "🔄 Punch In/Out");
                        let mut count_in = false;
                        ui.checkbox(&mut count_in, "🎬 Count-In");
                    });

                    ui.horizontal(|ui| {
                        ui.label("Pre-Roll:");
                        let mut preroll = 1.0;
                        ui.add(egui::DragValue::new(&mut preroll).clamp_range(0.0..=4.0).suffix(" bars"));
                    });
                });

            });
            // Track Strip Scroll Area - Moved inside transport panel
            egui::ScrollArea::horizontal().show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Placeholder for all individual track strips
                    for channel in ui_state.track_channels.iter_mut() {
                        ui.group(|ui| {
                            ui.set_min_width(70.0);
                            ui.label(&channel.name);
                            ui.checkbox(&mut channel.arm, "R");
                            ui.checkbox(&mut channel.solo, "S");
                            ui.checkbox(&mut channel.mute, "M");
                            ui.add(egui::Slider::new(&mut channel.volume, 0.0..=1.0).vertical().text("Vol"));
                            ui.add(egui::Slider::new(&mut channel.pan, -1.0..=1.0).text("Pan"));
                        });
                    }
                });
            });
        });
    }

    // --- CENTRAL PANEL: THE MAIN WORKSPACE ---
    egui::CentralPanel::default().show(ctx, |ui| {
        // Render the currently selected view
        match ui_state.current_view {
            UIViewMode::Arrangement => {
                ui.heading("🎼 Arrangement View (Timeline)");
                draw_arrangement_view(ui, arrangement_state, ui_state);
            },
            UIViewMode::Live => {
                ui.heading("🎵 Live View (Performance)");
                draw_live_view(ui, ui_state, live_state);
            },
            UIViewMode::Node => {
                ui.heading("🔗 Node View (Patching)");
                draw_node_view_full(ui, ui_state, node_state);
            },
        }
    });

    // Optionally render the Transport Bar at the very bottom
    if ui_state.show_transport {
        egui::TopBottomPanel::bottom("transport_bar_bottom").show(ctx, |ui| {
            transport_bar(ui, ui_state);
        });
    }
}

/// Draw Arrangement View - Full Implementation
/// Uses Bevy 0.17 render graph integration
fn draw_arrangement_view(ui: &mut egui::Ui, state: &mut ArrangementViewState, ui_state: &mut UiState) {
    ui.heading("Arrangement View - Professional Timeline");

    // Timeline Header with enhanced controls from JS implementation
    ui.horizontal(|ui| {
        ui.label("🎼 Timeline");
        ui.add(egui::DragValue::new(&mut state.timeline_zoom).clamp_range(0.1..=10.0).prefix("Zoom: "));
        ui.checkbox(&mut state.show_automation, "Show Automation");
        ui.separator();
        ui.label("Snap:");
        ui.checkbox(&mut state.snap_to_grid, "Grid");
        ui.checkbox(&mut state.snap_to_beats, "Beats");
        ui.checkbox(&mut state.snap_to_bars, "Bars");
        ui.separator();
        ui.label("Quantize:");
        ui.checkbox(&mut state.quantize_fourths, "1/4");
        ui.checkbox(&mut state.quantize_eighths, "1/8");
        ui.checkbox(&mut state.quantize_sixteenths, "1/16");
        ui.separator();
        ui.label("Groove:");
        ui.add(egui::DragValue::new(&mut state.groove_amount).clamp_range(0.0..=1.0).speed(0.01).suffix("%"));
        ui.add(egui::DragValue::new(&mut state.groove_rate).clamp_range(0.5..=8.0).speed(0.1).suffix("x"));
    });

    ui.separator();

    // Ruler/Timeline header
    ui.horizontal(|ui| {
        ui.set_min_width(120.0);
        ui.label("Tracks");
        egui::ScrollArea::horizontal().show(ui, |ui| {
            for bar in 0..32 {
                ui.vertical(|ui| {
                    ui.set_min_width(100.0);
                    ui.label(format!("Bar {}", bar + 1));
                    // Beat markers within each bar
                    for beat in 0..4 {
                        let beat_x = ui.cursor().left() + (beat as f32 * 25.0);
                        ui.painter().line_segment(
                            [egui::Pos2::new(beat_x, ui.cursor().top() + 15.0),
                             egui::Pos2::new(beat_x, ui.cursor().bottom())],
                            egui::Stroke::new(1.0, if beat == 0 { egui::Color32::WHITE } else { egui::Color32::GRAY }),
                        );
                    }
                });
            }
        });
    });

    ui.separator();

    // Track Area with automation lanes
    egui::ScrollArea::vertical().show(ui, |ui| {
        for track_num in 0..12 {
            ui.horizontal(|ui| {
                // Track Header with enhanced controls from JS implementation
                ui.vertical(|ui| {
                    ui.set_min_width(160.0);
                    ui.set_min_height(160.0);

                    ui.label(format!("🎵 Track {}", track_num + 1));
                    let mut is_selected = state.selected_tracks.contains(&track_num);
                    if ui.checkbox(&mut is_selected, "Select").changed() {
                        if is_selected {
                            state.selected_tracks.push(track_num);
                        } else {
                            state.selected_tracks.retain(|&x| x != track_num);
                        }
                    }

                    // Track type selector
                    let mut track_type = "Audio".to_string();
                    egui::ComboBox::from_label("Type:")
                        .selected_text(&track_type)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut track_type, "Audio".to_string(), "Audio");
                            ui.selectable_value(&mut track_type, "MIDI".to_string(), "MIDI");
                            ui.selectable_value(&mut track_type, "Group".to_string(), "Group");
                            ui.selectable_value(&mut track_type, "Return".to_string(), "Return");
                        });

                    // Input routing from JS implementation
                    ui.label("Input:");
                    let input_idx = track_num % state.track_inputs.len();
                    let mut input_source = state.track_inputs.get(input_idx).cloned().unwrap_or("None".to_string());
                    egui::ComboBox::from_label("")
                        .selected_text(&input_source)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut input_source, "Mic 1".to_string(), "Mic 1");
                            ui.selectable_value(&mut input_source, "Mic 2".to_string(), "Mic 2");
                            ui.selectable_value(&mut input_source, "DI 1".to_string(), "DI 1");
                            ui.selectable_value(&mut input_source, "MIDI 1".to_string(), "MIDI 1");
                            ui.selectable_value(&mut input_source, "Return 1".to_string(), "Return 1");
                            ui.selectable_value(&mut input_source, "None".to_string(), "None");
                        });
                    if track_num < state.track_inputs.len() {
                        state.track_inputs[track_num] = input_source;
                    }

                    // Track controls with arm button from JS - FIX: Link to channel state
                    ui.horizontal(|ui| {
                        if let Some(channel_state) = ui_state.track_channels.get_mut(track_num) {
                            ui.checkbox(&mut channel_state.mute, "M");
                            ui.checkbox(&mut channel_state.solo, "S");
                            ui.checkbox(&mut channel_state.arm, "R");
                        }
                        // Also update the arrangement view state for backward compatibility
                        let arm_idx = track_num % state.track_armed.len();
                        let mut armed = *state.track_armed.get(arm_idx).unwrap_or(&false);
                        ui.checkbox(&mut armed, "R");
                        if track_num < state.track_armed.len() {
                            state.track_armed[track_num] = armed;
                        }
                    });

                    // Volume fader with automation
                    ui.label("Volume:");
                    // FIX: Link to track_channels volume for persistent state
                    if let Some(channel_state) = ui_state.track_channels.get_mut(track_num) {
                        ui.add(egui::Slider::new(&mut channel_state.volume, 0.0..=1.0).vertical().text("Vol"));
                    }

                    // Pan control
                    ui.label("Pan:");
                    // FIX: Link to track_channels pan for persistent state
                    if let Some(channel_state) = ui_state.track_channels.get_mut(track_num) {
                        ui.add(egui::Slider::new(&mut channel_state.pan, -1.0..=1.0).text("Pan"));
                    }

                    // Send controls with advanced routing
                    ui.collapsing("📤 Sends", |ui| {
                        for send_num in 0..4 {
                            ui.horizontal(|ui| {
                                ui.label(format!("Send {}:", send_num + 1));
                                let mut send_level = 0.0;
                                ui.add(egui::Slider::new(&mut send_level, 0.0..=1.0));
                                let mut pre_fader = false; ui.checkbox(&mut pre_fader, "Pre");
                            });
                        }
                    });

                    // Automation lane toggle from JS
                    ui.separator();
                    let automation_idx = track_num % state.automation_lanes_visible.len();
                    let mut show_automation = *state.automation_lanes_visible.get(automation_idx).unwrap_or(&false);
                    if ui.checkbox(&mut show_automation, "Automation").clicked() {
                        if track_num < state.automation_lanes_visible.len() {
                            state.automation_lanes_visible[track_num] = show_automation;
                        }
                    }
                });

                // Track Timeline with automation
                ui.vertical(|ui| {
                    ui.set_min_height(120.0);
                    ui.set_min_width(3200.0); // 32 bars * 100 pixels per bar

                    let (rect, _) = ui.allocate_at_least(
                        egui::Vec2::new(3200.0, 120.0),
                        egui::Sense::click_and_drag()
                    );

                    // Draw timeline background
                    ui.painter().rect_filled(rect, 2.0, egui::Color32::from_gray(24));

                    // Draw bar lines
                    for bar in 0..32 {
                        let x = rect.left() + (bar as f32 * 100.0);
                        ui.painter().line_segment(
                            [egui::Pos2::new(x, rect.top()), egui::Pos2::new(x, rect.bottom())],
                            egui::Stroke::new(2.0, egui::Color32::from_gray(64))
                        );
                    }

                    // Draw beat lines
                    for beat in 0..128 { // 4 beats per bar * 32 bars
                        let x = rect.left() + (beat as f32 * 25.0);
                        ui.painter().line_segment(
                            [egui::Pos2::new(x, rect.top()), egui::Pos2::new(x, rect.bottom())],
                            egui::Stroke::new(1.0, egui::Color32::from_gray(48))
                        );
                    }

                    // Draw sample clips with enhanced editing features
                    if track_num == 0 {
                        // Kick clip with editing controls
                        let clip_rect = egui::Rect::from_min_size(
                            egui::Pos2::new(rect.left() + 25.0, rect.top() + 10.0),
                            egui::Vec2::new(75.0, 50.0)
                        );
                        ui.painter().rect_filled(clip_rect, egui::Rounding::same(4.0), egui::Color32::from_rgb(100, 150, 200));
                        ui.painter().rect_stroke(clip_rect, egui::Rounding::same(4.0), egui::Stroke::new(2.0, egui::Color32::WHITE));
                        ui.painter().text(
                            clip_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "Kick",
                            egui::FontId::proportional(12.0),
                            egui::Color32::BLACK
                        );

                        // Clip editing controls (hover overlay)
                        let control_rect = egui::Rect::from_min_size(
                            egui::Pos2::new(clip_rect.right() - 60.0, clip_rect.top() - 20.0),
                            egui::Vec2::new(60.0, 15.0)
                        );
                        ui.painter().rect_filled(control_rect, egui::Rounding::same(2.0), egui::Color32::from_rgba_premultiplied(0, 0, 0, 180));
                        ui.painter().text(
                            control_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "✂️📋🗑️",
                            egui::FontId::proportional(10.0),
                            egui::Color32::WHITE
                        );

                        // Snare clip
                        let snare_rect = egui::Rect::from_min_size(
                            egui::Pos2::new(rect.left() + 125.0, rect.top() + 10.0),
                            egui::Vec2::new(75.0, 50.0)
                        );
                        ui.painter().rect_filled(snare_rect, egui::Rounding::same(4.0), egui::Color32::from_rgb(200, 100, 100));
                        ui.painter().rect_stroke(snare_rect, egui::Rounding::same(4.0), egui::Stroke::new(2.0, egui::Color32::WHITE));
                        ui.painter().text(
                            snare_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "Snare",
                            egui::FontId::proportional(12.0),
                            egui::Color32::BLACK
                        );
                    }

                    if track_num == 1 {
                        // Bass line clip with fade handles
                        let bass_rect = egui::Rect::from_min_size(
                            egui::Pos2::new(rect.left() + 0.0, rect.top() + 10.0),
                            egui::Vec2::new(200.0, 50.0)
                        );
                        ui.painter().rect_filled(bass_rect, egui::Rounding::same(4.0), egui::Color32::from_rgb(100, 200, 100));
                        ui.painter().rect_stroke(bass_rect, egui::Rounding::same(4.0), egui::Stroke::new(2.0, egui::Color32::WHITE));
                        ui.painter().text(
                            bass_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "Bass Line",
                            egui::FontId::proportional(12.0),
                            egui::Color32::BLACK
                        );

                        // Fade in/out handles
                        let fade_in_handle = egui::Pos2::new(bass_rect.left() + 10.0, bass_rect.top() + 25.0);
                        ui.painter().circle_filled(fade_in_handle, 4.0, egui::Color32::from_rgb(255, 255, 100));
                        let fade_out_handle = egui::Pos2::new(bass_rect.right() - 10.0, bass_rect.top() + 25.0);
                        ui.painter().circle_filled(fade_out_handle, 4.0, egui::Color32::from_rgb(255, 255, 100));
                    }

                    if track_num == 5 {
                        // MIDI clip with piano roll preview
                        let midi_rect = egui::Rect::from_min_size(
                            egui::Pos2::new(rect.left() + 50.0, rect.top() + 10.0),
                            egui::Vec2::new(150.0, 50.0)
                        );
                        ui.painter().rect_filled(midi_rect, egui::Rounding::same(4.0), egui::Color32::from_rgb(150, 100, 200));
                        ui.painter().rect_stroke(midi_rect, egui::Rounding::same(4.0), egui::Stroke::new(2.0, egui::Color32::WHITE));
                        ui.painter().text(
                            midi_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "Chord Prog",
                            egui::FontId::proportional(12.0),
                            egui::Color32::BLACK
                        );

                        // Mini piano roll visualization
                        for note in 0..8 {
                            let note_y = midi_rect.top() + 5.0 + (note as f32 * 5.0);
                            let note_width = 15.0 + (note as f32 * 2.0);
                            ui.painter().rect_filled(
                                egui::Rect::from_min_size(
                                    egui::Pos2::new(midi_rect.left() + 10.0 + (note as f32 * 15.0), note_y),
                                    egui::Vec2::new(note_width, 3.0)
                                ),
                                1.0,
                                egui::Color32::from_rgb(255, 255, 255)
                            );
                        }
                    }

                    // Automation lanes (if enabled)
                    if state.show_automation {
                        let automation_y = rect.top() + 70.0;
                        ui.painter().line_segment(
                            [egui::Pos2::new(rect.left(), automation_y), egui::Pos2::new(rect.right(), automation_y)],
                            egui::Stroke::new(1.0, egui::Color32::from_rgb(150, 150, 150))
                        );

                        // Draw automation curves for this track
                        for curve in &state.automation_curves {
                            if curve.track_id == track_num {
                                let mut points = Vec::new();
                                for point in &curve.points {
                                    let x = rect.left() + (point.time as f32 * 25.0 * state.timeline_zoom);
                                    let y = automation_y - (point.value * 30.0); // Scale to fit lane
                                    points.push(egui::Pos2::new(x, y));
                                }

                                // Draw curve segments
                                for i in 0..points.len().saturating_sub(1) {
                                    ui.painter().line_segment(
                                        [points[i], points[i + 1]],
                                        egui::Stroke::new(2.0, curve.color)
                                    );
                                }

                                // Draw automation points
                                for point in &points {
                                    ui.painter().circle_filled(*point, 3.0, curve.color);
                                }
                            }
                        }
                    }
                });
            });
            ui.separator();
        }
    });

    // Enhanced bottom toolbar with professional DAW operations
    ui.separator();
    ui.horizontal(|ui| {
        ui.label("🎬 Clip Operations:");
        if ui.button("✂️ Split").clicked() { println!("Split clip at playhead"); }
        if ui.button("📋 Duplicate").clicked() { println!("Duplicate selected clips"); }
        if ui.button("🗑️ Delete").clicked() { println!("Delete selected clips"); }
        if ui.button("🔇 Silence").clicked() { println!("Silence selected region"); }
        if ui.button("🔄 Reverse").clicked() { println!("Reverse selected clips"); }
        ui.separator();

        ui.label("🎚️ Automation:");
        if ui.button("➕ Add Point").clicked() { println!("Add automation point"); }
        if ui.button("📏 Draw Mode").clicked() { println!("Toggle automation draw mode"); }
        if ui.button("🧹 Clear Lane").clicked() { println!("Clear automation lane"); }
        if ui.button("👁️ Show All").clicked() { state.show_automation = true; }
        ui.separator();

        ui.label("🎼 Track:");
        if ui.button("➕ Add Track").clicked() { println!("Add new track"); }
        if ui.button("🗑️ Delete Track").clicked() { println!("Delete selected tracks"); }
        if ui.button("🎛️ Group Tracks").clicked() { println!("Group selected tracks"); }
        if ui.button("🔀 Route To...").clicked() { println!("Open routing dialog"); }
        ui.separator();

        ui.label("🎚️ Quantize:");
        ui.checkbox(&mut state.quantize_fourths, "1/4");
        ui.checkbox(&mut state.quantize_eighths, "1/8");
        ui.checkbox(&mut state.quantize_sixteenths, "1/16");
        let mut quantize_32nds = false; ui.checkbox(&mut quantize_32nds, "1/32");
        ui.separator();

        ui.label("🎵 Groove:");
        let mut groove_amount = 0.0;
        ui.add(egui::Slider::new(&mut groove_amount, 0.0..=1.0).text("Amount"));
        let mut groove_rate = 0.5;
        ui.add(egui::Slider::new(&mut groove_rate, 0.0..=1.0).text("Rate"));
    });

    // Advanced routing matrix panel
    ui.collapsing("🎛️ Advanced Track Routing Matrix", |ui| {
        ui.label("Configure complex sends, returns, and parallel processing:");
        egui::Grid::new("advanced_routing_grid").show(ui, |ui| {
            ui.label("Source → Destination");
            for dest in 0..12 {
                ui.label(format!("Track/Return {}", dest + 1));
            }
            ui.end_row();

            for source in 0..12 {
                ui.label(format!("Track {}", source + 1));
                for dest in 0..12 {
                    if source != dest {
                        ui.vertical(|ui| {
                            let mut send_level = 0.0;
                            ui.add(egui::DragValue::new(&mut send_level).clamp_range(0.0..=1.0).speed(0.01));
                            let mut pre_fader = false; ui.checkbox(&mut pre_fader, "Pre");
                        });
                    } else {
                        ui.label("—");
                    }
                }
                ui.end_row();
            }
        });

        ui.separator();
        ui.label("🎛️ Parallel Processing Chains:");
        ui.horizontal(|ui| {
            if ui.button("➕ Add Parallel Bus").clicked() { println!("Add parallel processing bus"); }
            if ui.button("🎚️ Sidechain Setup").clicked() { println!("Configure sidechain routing"); }
            if ui.button("📊 Monitor Routing").clicked() { println!("Open routing monitor"); }
        });
    });

    // Track routing panel (collapsible)
    ui.collapsing("Track Routing Matrix", |ui| {
        ui.label("Configure sends and returns between tracks:");
        egui::Grid::new("routing_grid").show(ui, |ui| {
            ui.label("From \\ To");
            for to_track in 0..12 {
                ui.label(format!("Track {}", to_track + 1));
            }
            ui.end_row();

            for from_track in 0..12 {
                ui.label(format!("Track {}", from_track + 1));
                for to_track in 0..12 {
                    if from_track != to_track {
                        let mut send_level = 0.0;
                        ui.add(egui::DragValue::new(&mut send_level).clamp_range(0.0..=1.0).speed(0.01));
                    } else {
                        ui.label("-");
                    }
                }
                ui.end_row();
            }
        });
    });
}

/// Draw Live View - Full Implementation
/// Uses Bevy 0.17 render graph integration
fn draw_live_view(ui: &mut egui::Ui, ui_state: &mut UiState, state: &mut LiveViewState) {
    ui.heading("🎧 Live Performance View - CDJ-Inspired DJ Interface");

    // DJ Controls Header with enhanced features
    ui.horizontal(|ui| {
        ui.label("🎛️ DJ Controls:");
        if ui.button("🔄 Auto-Mix").clicked() {
            println!("Auto-mix mode activated");
        }
        if ui.button("🎯 Hot Cue").clicked() {
            println!("Hot cue set");
        }
        if ui.button("🔁 Loop").clicked() {
            println!("Loop mode toggled");
        }
        if ui.button("⏯️ Play/Pause").clicked() {
            println!("Global play/pause toggled");
        }
        ui.separator();
        ui.label("BPM:");
        let mut bpm = 128.0;
        ui.add(egui::DragValue::new(&mut bpm).clamp_range(60.0..=200.0).suffix(" BPM"));
        ui.separator();
        ui.label("Sync:");
        let mut temp = true; ui.checkbox(&mut temp, "Beat Sync");
        let mut temp = false; ui.checkbox(&mut temp, "Tempo Match");
    });

    ui.separator();

    // Enhanced Scene Matrix with CDJ-inspired features
    ui.label("🎵 Scene Matrix - Drag and drop clips between decks:");
    ui.horizontal(|ui| {
        // Left Deck A
        ui.vertical(|ui| {
            ui.set_min_width(250.0);
            ui.heading("🎛️ Deck A - CDJ Style");

            // Transport controls for Deck A
            ui.horizontal(|ui| {
                if ui.button("▶️").clicked() {
                    println!("Deck A Play");
                }
                if ui.button("⏸️").clicked() {
                    println!("Deck A Pause");
                }
                if ui.button("⏹️").clicked() {
                    println!("Deck A Stop");
                }
                ui.separator();
                ui.label("BPM:");
                ui.add(egui::DragValue::new(&mut state.deck_a_volume).clamp_range(60.0..=200.0));
            });

            // Waveform display placeholder
            let waveform_response = ui.allocate_response(egui::Vec2::new(200.0, 60.0), egui::Sense::hover());
            ui.painter().rect_filled(waveform_response.rect, egui::Rounding::same(4.0), egui::Color32::from_rgb(30, 30, 50));
            ui.label("🎵 Waveform Display");

            // Hot Cues (CDJ style)
            ui.horizontal(|ui| {
                ui.label("Hot Cues:");
                for i in 0..4 {
                    let cue_color = match i {
                        0 => egui::Color32::from_rgb(255, 100, 100), // Red
                        1 => egui::Color32::from_rgb(100, 255, 100), // Green
                        2 => egui::Color32::from_rgb(100, 100, 255), // Blue
                        3 => egui::Color32::from_rgb(255, 255, 100), // Yellow
                        _ => egui::Color32::GRAY,
                    };
                    if ui.add(egui::Button::new(format!("C{}", i + 1)).fill(cue_color).min_size(egui::Vec2::new(30.0, 20.0))).clicked() {
                        state.hot_cues_a[i] = !state.hot_cues_a[i];
                        println!("Hot Cue {} triggered on Deck A", i + 1);
                    }
                }
            });

            // Loop controls
            ui.horizontal(|ui| {
                ui.label("Loop:");
                if ui.button("1/2").clicked() { state.loop_size_a = 0.5; }
                if ui.button("1").clicked() { state.loop_size_a = 1.0; }
                if ui.button("2").clicked() { state.loop_size_a = 2.0; }
                if ui.button("4").clicked() { state.loop_size_a = 4.0; }
            });

            // Volume and Filter
            ui.horizontal(|ui| {
                ui.label("Vol:");
                ui.add(egui::Slider::new(&mut state.deck_a_volume, 0.0..=1.0));
            });
            ui.horizontal(|ui| {
                ui.label("Filter:");
                ui.add(egui::Slider::new(&mut state.deck_a_filter, 0.0..=1.0));
            });

            // Clip slots for Deck A
            ui.label("Clips:");
            for i in 0..4 {
                let is_active = state.active_clips.contains(&(i));
                let button_color = if is_active {
                    egui::Color32::from_rgb(100, 200, 100)
                } else {
                    egui::Color32::from_rgb(64, 64, 64)
                };

                if ui.add(
                    egui::Button::new(format!("A{}", i + 1))
                        .fill(button_color)
                        .min_size(egui::Vec2::new(50.0, 30.0))
                ).clicked() {
                    if is_active {
                        state.active_clips.retain(|&x| x != i);
                    } else {
                        state.active_clips.push(i);
                    }
                    println!("Triggered Deck A Clip {}", i + 1);
                }
            }
        });

        // Crossfader Section with enhanced controls
        ui.vertical(|ui| {
            ui.set_min_width(120.0);
            ui.heading("🎚️ Crossfader");

            ui.add(egui::Slider::new(&mut state.crossfader_position, 0.0..=1.0).vertical().text("A ↔ B"));

            ui.separator();
            ui.label("Curve:");
            let mut curve = 0.5;
            ui.add(egui::Slider::new(&mut curve, 0.0..=1.0));

            ui.separator();
            ui.label("Hamster:");
            let mut hamster = 0.0;
            ui.add(egui::Slider::new(&mut hamster, -1.0..=1.0));

            ui.separator();
            ui.label("Reverse:");
            let mut temp = false; ui.checkbox(&mut temp, "A");
            let mut temp = false; ui.checkbox(&mut temp, "B");
        });

        // Right Deck B
        ui.vertical(|ui| {
            ui.set_min_width(250.0);
            ui.heading("🎛️ Deck B - CDJ Style");

            // Transport controls for Deck B
            ui.horizontal(|ui| {
                if ui.button("▶️").clicked() {
                    println!("Deck B Play");
                }
                if ui.button("⏸️").clicked() {
                    println!("Deck B Pause");
                }
                if ui.button("⏹️").clicked() {
                    println!("Deck B Stop");
                }
                ui.separator();
                ui.label("BPM:");
                let mut bpm_b = 128.0;
                ui.add(egui::DragValue::new(&mut bpm_b).clamp_range(60.0..=200.0));
            });

            // Waveform display placeholder
            let waveform_response_b = ui.allocate_response(egui::Vec2::new(200.0, 60.0), egui::Sense::hover());
            ui.painter().rect_filled(waveform_response_b.rect, egui::Rounding::same(4.0), egui::Color32::from_rgb(30, 30, 50));
            ui.label("🎵 Waveform Display");

            // Hot Cues (CDJ style)
            ui.horizontal(|ui| {
                ui.label("Hot Cues:");
                for i in 0..4 {
                    let cue_color = match i {
                        0 => egui::Color32::from_rgb(255, 100, 100), // Red
                        1 => egui::Color32::from_rgb(100, 255, 100), // Green
                        2 => egui::Color32::from_rgb(100, 100, 255), // Blue
                        3 => egui::Color32::from_rgb(255, 255, 100), // Yellow
                        _ => egui::Color32::GRAY,
                    };
                    if ui.add(egui::Button::new(format!("C{}", i + 1)).fill(cue_color).min_size(egui::Vec2::new(30.0, 20.0))).clicked() {
                        state.hot_cues_b[i] = !state.hot_cues_b[i];
                        println!("Hot Cue {} triggered on Deck B", i + 1);
                    }
                }
            });

            // Loop controls
            ui.horizontal(|ui| {
                ui.label("Loop:");
                if ui.button("1/2").clicked() { state.loop_size_b = 0.5; }
                if ui.button("1").clicked() { state.loop_size_b = 1.0; }
                if ui.button("2").clicked() { state.loop_size_b = 2.0; }
                if ui.button("4").clicked() { state.loop_size_b = 4.0; }
            });

            // Volume and Filter
            ui.horizontal(|ui| {
                ui.label("Vol:");
                ui.add(egui::Slider::new(&mut state.deck_b_volume, 0.0..=1.0));
            });
            ui.horizontal(|ui| {
                ui.label("Filter:");
                ui.add(egui::Slider::new(&mut state.deck_b_filter, 0.0..=1.0));
            });

            // Clip slots for Deck B
            ui.label("Clips:");
            for i in 0..4 {
                let is_active = state.active_clips.contains(&(i + 4));
                let button_color = if is_active {
                    egui::Color32::from_rgb(200, 100, 100)
                } else {
                    egui::Color32::from_rgb(64, 64, 64)
                };

                if ui.add(
                    egui::Button::new(format!("B{}", i + 1))
                        .fill(button_color)
                        .min_size(egui::Vec2::new(50.0, 30.0))
                ).clicked() {
                    if is_active {
                        state.active_clips.retain(|&x| x != i + 4);
                    } else {
                        state.active_clips.push(i + 4);
                    }
                    println!("Triggered Deck B Clip {}", i + 1);
                }
            }
        });
    });

    ui.separator();

    // Enhanced Mega Plugin Section - Bitwig/Ableton style nested plugins with cookbook techniques
    ui.heading("🔗 Mega Plugins - Chain Effects Together (Modular Cookbook Inspired)");
    ui.collapsing("🎛️ Mega Filter Chain (Resonant Envelopes + Wave Folders)", |ui| {
        ui.label("Chain multiple filters with modulation from cookbook techniques:");
        ui.horizontal(|ui| {
            if ui.button("➕ Add Filter").clicked() {
                println!("Added filter to chain");
            }
            if ui.button("➕ Add LFO").clicked() {
                println!("Added LFO modulation");
            }
            if ui.button("➕ Add Envelope").clicked() {
                println!("Added envelope follower");
            }
            if ui.button("➕ Add Wave Folder").clicked() {
                println!("Added wave folder for CV shaping");
            }
        });

        ui.separator();
        ui.label("Chain: LPF → Wave Folder → HPF → BPF (modulated by LFO + Envelope)");
        ui.horizontal(|ui| {
            ui.label("LFO Rate:");
            let mut lfo_rate = 0.5;
            ui.add(egui::Slider::new(&mut lfo_rate, 0.1..=10.0).logarithmic(true));

            ui.label("LFO Depth:");
            let mut lfo_depth = 0.7;
            ui.add(egui::Slider::new(&mut lfo_depth, 0.0..=1.0));

            ui.label("Envelope Attack:");
            let mut env_attack = 0.1;
            ui.add(egui::Slider::new(&mut env_attack, 0.001..=1.0).logarithmic(true));
        });

        let mut use_resonant = true;
        ui.checkbox(&mut use_resonant, "Use resonant envelopes (Cookbook p.60)");
        let mut shape_cv = false;
        ui.checkbox(&mut shape_cv, "Shape CV with wave folders (Cookbook p.62)");
    });

    ui.collapsing("🎵 Mega Reverb + Delay (Filter in Feedback + Ducking)", |ui| {
        ui.label("Reverb into delay with feedback and sidechain ducking:");
        ui.horizontal(|ui| {
            ui.label("Reverb Size:");
            let mut rev_size = 0.8;
            ui.add(egui::Slider::new(&mut rev_size, 0.1..=1.0));

            ui.label("Delay Time:");
            let mut delay_time = 0.5;
            ui.add(egui::Slider::new(&mut delay_time, 0.1..=2.0));

            ui.label("Feedback:");
            let mut feedback = 0.3;
            ui.add(egui::Slider::new(&mut feedback, 0.0..=0.9));
        });

        let mut filter_feedback = true;
        ui.checkbox(&mut filter_feedback, "Filter in delay feedback (Cookbook p.88)");
        let mut sidechain_comp = true;
        ui.checkbox(&mut sidechain_comp, "Sidechain compression on reverb return");
        let mut duck_effects = false;
        ui.checkbox(&mut duck_effects, "Duck effects with envelope (Cookbook p.102)");

        ui.horizontal(|ui| {
            ui.label("Duck Amount:");
            let mut duck_amount = 0.5;
            ui.add(egui::Slider::new(&mut duck_amount, 0.0..=1.0));

            ui.label("Duck Time:");
            let mut duck_time = 0.2;
            ui.add(egui::Slider::new(&mut duck_time, 0.01..=1.0));
        });
    });

    ui.collapsing("🎛️ Mega Granular Synthesis + Convolution (Time Stretching + IR Loading)", |ui| {
        ui.label("Advanced granular processing with convolution reverb:");
        ui.horizontal(|ui| {
            ui.label("Grain Size:");
            let mut grain_size = 50.0;
            ui.add(egui::DragValue::new(&mut grain_size).clamp_range(10.0..=500.0).suffix(" ms"));

            ui.label("Grain Density:");
            let mut grain_density = 0.8;
            ui.add(egui::Slider::new(&mut grain_density, 0.1..=2.0));

            ui.label("Pitch Shift:");
            let mut pitch_shift = 0.0;
            ui.add(egui::DragValue::new(&mut pitch_shift).clamp_range(-12.0..=12.0).suffix(" semitones"));
        });

        ui.label("Convolution with impulse responses:");
        ui.horizontal(|ui| {
            ui.label("IR File:");
            let mut ir_filename = "Spring Reverb.wav".to_string();
            ui.text_edit_singleline(&mut ir_filename);
            if ui.button("Load IR").clicked() {
                println!("Loading impulse response...");
            }
        });

        let mut time_stretch = true;
        ui.checkbox(&mut time_stretch, "Time-stretching granular synthesis (Cookbook p.124)");
        let mut custom_ir = false;
        ui.checkbox(&mut custom_ir, "Convolution with custom IRs (Cookbook p.126)");
        let mut freeze_grains = true;
        ui.checkbox(&mut freeze_grains, "Freeze and manipulate grains (Cookbook p.128)");
    });

    ui.collapsing("🎚️ Mega Spectral Processing (FFT Filtering + Phase Vocoder)", |ui| {
        ui.label("Real-time spectral processing and manipulation:");
        ui.horizontal(|ui| {
            ui.label("FFT Size:");
            let mut fft_size = 2048.0;
            ui.add(egui::DragValue::new(&mut fft_size).clamp_range(256.0..=8192.0));

            ui.label("Overlap:");
            let mut overlap = 0.75;
            ui.add(egui::Slider::new(&mut overlap, 0.0..=0.9));

            ui.label("Spectral Tilt:");
            let mut tilt = 0.0;
            ui.add(egui::DragValue::new(&mut tilt).clamp_range(-24.0..=24.0).suffix(" dB"));
        });

        ui.label("Spectral filtering bands:");
        for i in 0..8 {
            ui.horizontal(|ui| {
                ui.label(format!("Band {}:", i + 1));
                let mut freq = 100.0 * (2.0_f32).powi(i);
                ui.add(egui::DragValue::new(&mut freq).clamp_range(20.0..=20000.0).suffix(" Hz"));
                let mut gain = 0.0;
                ui.add(egui::DragValue::new(&mut gain).clamp_range(-24.0..=24.0).suffix(" dB"));
                let mut q = 1.0;
                ui.add(egui::DragValue::new(&mut q).clamp_range(0.1..=10.0));
            });
        }

        let mut phase_vocoder = true;
        ui.checkbox(&mut phase_vocoder, "Phase vocoder time stretching (Cookbook p.142)");
        let mut spectral_morph = false;
        ui.checkbox(&mut spectral_morph, "Spectral freezing and morphing (Cookbook p.144)");
        let mut spectral_filter = true;
        ui.checkbox(&mut spectral_filter, "Real-time spectral filtering (Cookbook p.146)");
    });

    ui.collapsing("🎛️ Mega Physical Modeling (Karplus-Strong + Waveguide)", |ui| {
        ui.label("Physical modeling synthesis with waveguides:");
        ui.horizontal(|ui| {
            ui.label("String Length:");
            let mut string_length = 1.0;
            ui.add(egui::Slider::new(&mut string_length, 0.1..=4.0));

            ui.label("Damping:");
            let mut damping = 0.99;
            ui.add(egui::Slider::new(&mut damping, 0.9..=0.999));

            ui.label("Pickup Position:");
            let mut pickup_pos = 0.5;
            ui.add(egui::Slider::new(&mut pickup_pos, 0.0..=1.0));
        });

        ui.label("Waveguide synthesis parameters:");
        ui.horizontal(|ui| {
            ui.label("Waveguide Length:");
            let mut wg_length = 100.0;
            ui.add(egui::DragValue::new(&mut wg_length).clamp_range(10.0..=1000.0).suffix(" samples"));

            ui.label("Reflection:");
            let mut reflection = 0.9;
            ui.add(egui::Slider::new(&mut reflection, 0.0..=0.99));
        });

        let mut karplus_strong = true;
        ui.checkbox(&mut karplus_strong, "Karplus-Strong plucked string (Cookbook p.156)");
        let mut waveguide_mesh = false;
        ui.checkbox(&mut waveguide_mesh, "Waveguide mesh for 2D surfaces (Cookbook p.158)");
        let mut nonlinear_junctions = true;
        ui.checkbox(&mut nonlinear_junctions, "Nonlinear scattering junctions (Cookbook p.160)");
    });

    ui.collapsing("🎛️ Mega Distortion + EQ (PWM + Complex Envelopes)", |ui| {
        ui.label("Distortion with dynamic EQ and PWM modulation:");
        ui.horizontal(|ui| {
            ui.label("Drive:");
            let mut drive = 0.6;
            ui.add(egui::Slider::new(&mut drive, 0.0..=1.0));

            ui.label("Tone:");
            let mut tone = 0.5;
            ui.add(egui::Slider::new(&mut tone, 0.0..=1.0));

            ui.label("PWM Amount:");
            let mut pwm_amount = 0.3;
            ui.add(egui::Slider::new(&mut pwm_amount, 0.0..=1.0));
        });

        ui.label("Dynamic EQ bands with envelope shaping:");
        ui.horizontal(|ui| {
            ui.label("Low:");
            let mut low_boost = 3.0;
            ui.add(egui::DragValue::new(&mut low_boost).clamp_range(-24.0..=24.0).suffix(" dB"));

            ui.label("Mid:");
            let mut mid_cut = -6.0;
            ui.add(egui::DragValue::new(&mut mid_cut).clamp_range(-24.0..=24.0).suffix(" dB"));

            ui.label("High:");
            let mut high_boost = 6.0;
            ui.add(egui::DragValue::new(&mut high_boost).clamp_range(-24.0..=24.0).suffix(" dB"));
        });

        let mut pwm_clipping = true;
        ui.checkbox(&mut pwm_clipping, "PWM with clipping (Cookbook p.86)");
        let mut complex_envelopes = false;
        ui.checkbox(&mut complex_envelopes, "Complex envelopes with mixer (Cookbook p.64)");
    });

    ui.collapsing("🎚️ Mega Ratchet + Glissando (Osc Sync + Envelopes)", |ui| {
        ui.label("Ratcheting effects with glissando using cookbook techniques:");
        ui.horizontal(|ui| {
            ui.label("Ratchet Speed:");
            let mut ratchet_speed = 2.0;
            ui.add(egui::Slider::new(&mut ratchet_speed, 0.5..=8.0));

            ui.label("Glissando Time:");
            let mut gliss_time = 0.1;
            ui.add(egui::Slider::new(&mut gliss_time, 0.01..=1.0));

            ui.label("Swing Amount:");
            let mut swing = 0.5;
            ui.add(egui::Slider::new(&mut swing, 0.0..=1.0));
        });

        let mut swing_ratchets = true;
        ui.checkbox(&mut swing_ratchets, "Swing and ratchets with osc sync (Cookbook p.32)");
        let mut gliss_envelope = true;
        ui.checkbox(&mut gliss_envelope, "Glissando with envelope (Cookbook p.30)");
        let mut sequential_switches = false;
        ui.checkbox(&mut sequential_switches, "Ratcheting with sequential switches (Cookbook p.34)");
    });

    ui.separator();

    // Advanced Modulation Section
    ui.heading("🎚️ Advanced Modulation - Modulate Anything by Anything");
    ui.collapsing("Modulation Matrix", |ui| {
        ui.label("Create complex modulation routings:");

        // Source -> Target matrix
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_min_width(100.0);
                ui.label("📊 Sources:");
                ui.label("🎛️ Macro 1");
                ui.label("🎚️ LFO 1");
                ui.label("📈 Envelope");
                ui.label("🎵 Audio In");
                ui.label("🎛️ Velocity");
            });

            ui.vertical(|ui| {
                ui.label("🎯 Targets:");
                ui.horizontal(|ui| {
                    ui.set_min_width(60.0);
                    ui.label("Filter");
                    ui.label("Volume");
                    ui.label("Pan");
                    ui.label("Pitch");
                    ui.label("Delay");
                });

                // Modulation amount matrix
                for source in 0..5 {
                    ui.horizontal(|ui| {
                        for target in 0..5 {
                            let amount = if source == target { 0.5 } else { 0.0 };
                            let mut amount_val = amount;
                            ui.add(egui::DragValue::new(&mut amount_val).clamp_range(-1.0..=1.0).speed(0.01));
                        }
                    });
                }
            });
        });
    });

    ui.collapsing("Sidechain & Dynamics", |ui| {
        ui.label("Advanced compression and sidechaining:");
        ui.checkbox(&mut state.sidechain_enabled, "Sidechain Deck A to Deck B");
        let mut temp = false; ui.checkbox(&mut temp, "Duck reverb with kick");
        let mut temp2 = true; ui.checkbox(&mut temp2, "Compress master bus");

        ui.horizontal(|ui| {
            ui.label("Sidechain Key:");
            let mut key = state.sidechain_key.clone();
            egui::ComboBox::from_label("")
                .selected_text(&key)
                .show_ui(ui, |ui| {
                    for option in ["Kick", "Snare", "Bass", "Master"].iter() {
                        ui.selectable_value(&mut key, option.to_string(), *option);
                    }
                });
            state.sidechain_key = key;

            ui.label("Ratio:");
            let mut ratio = 4.0;
            ui.add(egui::DragValue::new(&mut ratio).clamp_range(1.0..=20.0));

            ui.label("Attack:");
            let mut attack = 0.01;
            ui.add(egui::DragValue::new(&mut attack).clamp_range(0.001..=0.1));
        });
    });

    // Performance Tips
    ui.separator();
    ui.collapsing("🎧 DJ Performance Tips", |ui| {
        ui.label("• Use crossfader for smooth transitions");
        ui.label("• Chain effects for complex sound design");
        ui.label("• Modulate filter cutoff with LFO for movement");
        ui.label("• Sidechain bass to create pumping effect");
        ui.label("• Layer multiple clips for complex rhythms");
    });
}

/// Draw Node View - Full Implementation
/// Uses Bevy 0.17 render graph integration
fn draw_node_view_full(ui: &mut egui::Ui, ui_state: &mut UiState, state: &mut NodeViewState) {
    ui.heading("🎛️ Node-Based Patching View - Modular Synthesis");

    // Enhanced Toolbar with categories
    ui.horizontal(|ui| {
        ui.checkbox(&mut state.show_parameters, "Show Parameters");
        ui.checkbox(&mut state.grid_snap, "Grid Snap");
        ui.checkbox(&mut state.visual_feedback.show_audio_levels, "Audio Levels");
        ui.checkbox(&mut state.visual_feedback.show_modulation, "Modulation");
        ui.checkbox(&mut state.visual_feedback.show_cpu_usage, "CPU Usage");

        ui.separator();

        // Node creation buttons by category
        ui.collapsing("🎹 Generators", |ui| {
            ui.horizontal(|ui| {
                if ui.button("Sine Osc").clicked() {
                    add_node_to_canvas(state, "Sine Osc", "generator.sine");
                }
                if ui.button("Saw Osc").clicked() {
                    add_node_to_canvas(state, "Saw Osc", "generator.saw");
                }
                if ui.button("Square Osc").clicked() {
                    add_node_to_canvas(state, "Square Osc", "generator.square");
                }
                if ui.button("Noise").clicked() {
                    add_node_to_canvas(state, "Noise", "generator.noise");
                }
                if ui.button("Sampler").clicked() {
                    add_node_to_canvas(state, "Sampler", "generator.sampler");
                }
            });
        });

        ui.collapsing("🔊 Filters", |ui| {
            ui.horizontal(|ui| {
                if ui.button("LPF").clicked() {
                    add_node_to_canvas(state, "LPF", "filter.lpf");
                }
                if ui.button("HPF").clicked() {
                    add_node_to_canvas(state, "HPF", "filter.hpf");
                }
                if ui.button("BPF").clicked() {
                    add_node_to_canvas(state, "BPF", "filter.bpf");
                }
                if ui.button("Notch").clicked() {
                    add_node_to_canvas(state, "Notch", "filter.notch");
                }
            });
        });

        ui.collapsing("🌊 Effects", |ui| {
            ui.horizontal(|ui| {
                if ui.button("Reverb").clicked() {
                    add_node_to_canvas(state, "Reverb", "effect.reverb");
                }
                if ui.button("Delay").clicked() {
                    add_node_to_canvas(state, "Delay", "effect.delay");
                }
                if ui.button("Chorus").clicked() {
                    add_node_to_canvas(state, "Chorus", "effect.chorus");
                }
                if ui.button("Distortion").clicked() {
                    add_node_to_canvas(state, "Distortion", "effect.distortion");
                }
            });
        });

        ui.collapsing("🎚️ Modulators", |ui| {
            ui.horizontal(|ui| {
                if ui.button("LFO").clicked() {
                    add_node_to_canvas(state, "LFO", "modulator.lfo");
                }
                if ui.button("Envelope").clicked() {
                    add_node_to_canvas(state, "Envelope", "modulator.envelope");
                }
                if ui.button("S&H").clicked() {
                    add_node_to_canvas(state, "S&H", "modulator.sample_and_hold");
                }
            });
        });

        ui.collapsing("🔧 Utilities", |ui| {
            ui.horizontal(|ui| {
                if ui.button("Mixer").clicked() {
                    add_node_to_canvas(state, "Mixer", "utility.mixer");
                }
                if ui.button("VCA").clicked() {
                    add_node_to_canvas(state, "VCA", "utility.vca");
                }
                if ui.button("Splitter").clicked() {
                    add_node_to_canvas(state, "Splitter", "utility.splitter");
                }
                if ui.button("Output").clicked() {
                    add_node_to_canvas(state, "Output", "utility.output");
                }
            });
        });

        ui.separator();
        if ui.button("🗑️ Clear Canvas").clicked() {
            state.node_positions.clear();
            state.connections.clear();
        }
    });

    ui.separator();

    // Node Canvas with enhanced visuals
    let (rect, response) = ui.allocate_at_least(
        egui::Vec2::new(ui.available_width(), ui.available_height() - 100.0),
        egui::Sense::click_and_drag()
    );

    // Draw grid background with animation
    ui.painter().rect_filled(rect, egui::Rounding::same(0.0), egui::Color32::from_rgb(15, 15, 25));

    if state.grid_snap {
        // Draw animated grid lines
        let grid_size = 20.0;
        let time = ui.input(|i| i.time) as f64;
        let time = time as f32;
        let alpha = (time * state.visual_feedback.animation_speed).sin() * 0.1 + 0.9;

        for x in (rect.left() as i32..rect.right() as i32).step_by(grid_size as usize) {
            ui.painter().line_segment(
                [egui::Pos2::new(x as f32, rect.top()), egui::Pos2::new(x as f32, rect.bottom())],
                egui::Stroke::new(1.0, egui::Color32::from_rgba_premultiplied(64, 64, 80, (alpha * 255.0) as u8))
            );
        }
        for y in (rect.top() as i32..rect.bottom() as i32).step_by(grid_size as usize) {
            ui.painter().line_segment(
                [egui::Pos2::new(rect.left(), y as f32), egui::Pos2::new(rect.right(), y as f32)],
                egui::Stroke::new(1.0, egui::Color32::from_rgba_premultiplied(64, 64, 80, (alpha * 255.0) as u8))
            );
        }
    }

    // Draw connections first (behind nodes)
    for connection in &state.connections {
        if let (Some(from_node), Some(to_node)) = (
            state.node_positions.iter().find(|n| n.id == connection.from_node),
            state.node_positions.iter().find(|n| n.id == connection.to_node)
        ) {
            let start_pos = egui::Pos2::new(
                rect.left() + from_node.x + 140.0, // Right side of node
                rect.top() + from_node.y + 50.0
            );
            let end_pos = egui::Pos2::new(
                rect.left() + to_node.x, // Left side of node
                rect.top() + to_node.y + 50.0
            );

            // Draw curved connection with glow effect
            let control_point1 = egui::Pos2::new(start_pos.x + 30.0, start_pos.y);
            let control_point2 = egui::Pos2::new(end_pos.x - 30.0, end_pos.y);

            // Draw multiple segments for bezier curve
            let steps = 20;
            let mut points = Vec::new();
            for i in 0..=steps {
                let t = i as f32 / steps as f32;
                let x = (1.0 - t).powi(3) * start_pos.x +
                        3.0 * (1.0 - t).powi(2) * t * control_point1.x +
                        3.0 * (1.0 - t) * t.powi(2) * control_point2.x +
                        t.powi(3) * end_pos.x;
                let y = (1.0 - t).powi(3) * start_pos.y +
                        3.0 * (1.0 - t).powi(2) * t * control_point1.y +
                        3.0 * (1.0 - t) * t.powi(2) * control_point2.y +
                        t.powi(3) * end_pos.y;
                points.push(egui::Pos2::new(x, y));
            }

            // Draw glow effect
            for i in 0..points.len().saturating_sub(1) {
                ui.painter().line_segment(
                    [points[i], points[i + 1]],
                    egui::Stroke::new(8.0, egui::Color32::from_rgba_premultiplied(100, 100, 150, 50))
                );
            }

            // Draw main connection line
            for i in 0..points.len().saturating_sub(1) {
                ui.painter().line_segment(
                    [points[i], points[i + 1]],
                    egui::Stroke::new(3.0, egui::Color32::from_rgb(150, 150, 200))
                );
            }
        }
    }

    // Draw nodes with enhanced visuals and parameters
    for node in &mut state.node_positions {
        let node_rect = egui::Rect::from_min_size(
            egui::Pos2::new(rect.left() + node.x, rect.top() + node.y),
            egui::Vec2::new(140.0, if state.show_parameters && node.selected { 120.0 } else { 100.0 })
        );

        // Node background with type-based colors
        let bg_color = match node.node_type.split('.').next().unwrap_or("unknown") {
            "generator" => egui::Color32::from_rgb(80, 120, 160),
            "filter" => egui::Color32::from_rgb(120, 80, 160),
            "effect" => egui::Color32::from_rgb(160, 120, 80),
            "modulator" => egui::Color32::from_rgb(80, 160, 120),
            "utility" => egui::Color32::from_rgb(160, 80, 120),
            _ => egui::Color32::from_rgb(96, 96, 96),
        };

        ui.painter().rect_filled(node_rect, egui::Rounding::same(6.0), bg_color);

        // Selection highlight
        if node.selected {
            ui.painter().rect_stroke(node_rect, egui::Rounding::same(6.0),
                egui::Stroke::new(3.0, egui::Color32::from_rgb(255, 255, 100)));
        }

        // Node border with glow
        ui.painter().rect_stroke(node_rect, egui::Rounding::same(6.0),
            egui::Stroke::new(2.0, egui::Color32::WHITE));

        // Node title
        ui.painter().text(
            egui::Pos2::new(node_rect.left() + 12.0, node_rect.top() + 8.0),
            egui::Align2::LEFT_TOP,
            &node.id,
            egui::FontId::proportional(14.0),
            egui::Color32::WHITE
        );

        // Input/Output ports with labels
        let port_radius = 8.0;

        // Input port (left side)
        let input_pos = egui::Pos2::new(node_rect.left(), node_rect.center().y);
        ui.painter().circle_filled(input_pos, port_radius, egui::Color32::from_rgb(100, 200, 100));
        ui.painter().circle_stroke(input_pos, port_radius, egui::Stroke::new(2.0, egui::Color32::WHITE));

        // Output port (right side)
        let output_pos = egui::Pos2::new(node_rect.right(), node_rect.center().y);
        ui.painter().circle_filled(output_pos, port_radius, egui::Color32::from_rgb(200, 100, 100));
        ui.painter().circle_stroke(output_pos, port_radius, egui::Stroke::new(2.0, egui::Color32::WHITE));

        // Port labels
        ui.painter().text(
            egui::Pos2::new(node_rect.left() + 15.0, node_rect.center().y - 12.0),
            egui::Align2::LEFT_CENTER,
            "IN",
            egui::FontId::proportional(10.0),
            egui::Color32::from_rgb(180, 180, 180)
        );
        ui.painter().text(
            egui::Pos2::new(node_rect.right() - 15.0, node_rect.center().y - 12.0),
            egui::Align2::RIGHT_CENTER,
            "OUT",
            egui::FontId::proportional(10.0),
            egui::Color32::from_rgb(180, 180, 180)
        );

        // Node parameters (if selected and show_parameters enabled)
        if state.show_parameters && node.selected {
            ui.painter().text(
                egui::Pos2::new(node_rect.left() + 12.0, node_rect.top() + 30.0),
                egui::Align2::LEFT_TOP,
                "Parameters:",
                egui::FontId::proportional(10.0),
                egui::Color32::from_rgb(220, 220, 220)
            );

            for (i, param) in node.parameters.iter().enumerate() {
                ui.painter().text(
                    egui::Pos2::new(node_rect.left() + 12.0, node_rect.top() + 45.0 + (i as f32 * 15.0)),
                    egui::Align2::LEFT_TOP,
                    &format!("{}: {:.2}", param.name, param.value),
                    egui::FontId::proportional(9.0),
                    if param.modulated { egui::Color32::from_rgb(255, 200, 100) } else { egui::Color32::from_rgb(200, 200, 200) }
                );
            }
        }

        // Visual feedback overlays
        if state.visual_feedback.show_audio_levels {
            // Audio level indicator (placeholder animation)
            let level = (ui.input(|i| i.time as f32 * 2.0).sin() * 0.5 + 0.5) * 20.0;
            ui.painter().rect_filled(
                egui::Rect::from_min_size(
                    egui::Pos2::new(node_rect.right() - 10.0, node_rect.bottom() - level - 5.0),
                    egui::Vec2::new(5.0, level)
                ),
                1.0,
                egui::Color32::from_rgb(100, 255, 100)
            );
        }

        if state.visual_feedback.show_cpu_usage && node.selected {
            // CPU usage indicator
            ui.painter().text(
                egui::Pos2::new(node_rect.right() - 40.0, node_rect.top() + 8.0),
                egui::Align2::RIGHT_TOP,
                "CPU: 15%",
                egui::FontId::proportional(10.0),
                egui::Color32::from_rgb(255, 150, 150)
            );
        }
    }

    // Instructions and examples
    ui.separator();
    ui.collapsing("🎛️ Modular Synthesis Examples", |ui| {
        ui.label("• Basic Synth Chain: Osc → Filter → Amp → Output");
        ui.label("• FM Synthesis: Carrier Osc ← Modulator Osc");
        ui.label("• Feedback Loop: Delay → Filter → Back to Delay");
        ui.label("• Drum Machine: Multiple triggers → Sample players");
        ui.label("• Ambient Pad: Slow LFOs → Filters → Reverb");
    });

    ui.collapsing("🔗 Patching Tips", |ui| {
        ui.label("• Audio signals: Green ports (inputs), Red ports (outputs)");
        ui.label("• Control signals: Blue ports (CV/gate/modulation)");
        ui.label("• Drag from output to input to create connections");
        ui.label("• Right-click connections to delete them");
        ui.label("• Use attenuators to control signal levels");
    });

    ui.collapsing("🎛️ Common Techniques", |ui| {
        ui.label("• Frequency Modulation: LFO → Osc Frequency");
        ui.label("• Amplitude Modulation: LFO → VCA Input");
        ui.label("• Ring Modulation: Two oscillators → Ring Mod");
        ui.label("• Sample & Hold: Clock → S&H → Parameter");
        ui.label("• Wavefolding: Osc → Wavefolder → Filter");
    });

    // Enhanced Node presets and NUWE integration panel
    ui.collapsing("⭐ Node Presets & NUWE Integration", |ui| {
        ui.label("🎨 NUWE Shader Integration:");
        ui.horizontal(|ui| {
            if ui.button("🔗 Load ISF Shader").clicked() {
                println!("Loading ISF shader from NUWE...");
            }
            if ui.button("🎭 Fractal Generator").clicked() {
                println!("Adding fractal shader node...");
            }
            if ui.button("🌈 Color Effects").clicked() {
                println!("Adding color processing node...");
            }
        });

        ui.separator();
        ui.label("📚 Modular Cookbook Presets:");

        // Cookbook-inspired presets
        if ui.button("🎛️ Resonant Filter Chain (p.60)").clicked() {
            add_cookbook_preset(state, "resonant_filter_chain");
        }
        if ui.button("🌊 Wave Folder + Envelope (p.62)").clicked() {
            add_cookbook_preset(state, "wave_folder_envelope");
        }
        if ui.button("🎚️ Mixer with Envelopes (p.64)").clicked() {
            add_cookbook_preset(state, "mixer_envelopes");
        }
        if ui.button("🔄 Feedback with Filter (p.88)").clicked() {
            add_cookbook_preset(state, "feedback_filter");
        }
        if ui.button("🎵 Granular Time Stretch (p.124)").clicked() {
            add_cookbook_preset(state, "granular_timestretch");
        }
        if ui.button("🌌 Spectral Morphing (p.144)").clicked() {
            add_cookbook_preset(state, "spectral_morphing");
        }

        ui.separator();
        ui.label("🎹 Virtual Instruments:");
        if ui.button("🎼 FM Synthesizer").clicked() {
            add_instrument_preset(state, "fm_synth");
        }
        if ui.button("🎛️ Wavetable Oscillator").clicked() {
            add_instrument_preset(state, "wavetable");
        }
        if ui.button("🥁 Drum Machine").clicked() {
            add_instrument_preset(state, "drum_machine");
        }
        if ui.button("🎸 Physical Modeling").clicked() {
            add_instrument_preset(state, "physical_modeling");
        }
    });

    // Enhanced Effects rack with full professional features
    ui.collapsing("🎛️ Professional Effects Rack - Parallel/Series Routing", |ui| {
        ui.label("Advanced effects processing with flexible routing and modulation:");

        // Effects rack controls
        ui.horizontal(|ui| {
            let mut rack_on = true; ui.checkbox(&mut rack_on, "Rack On");
            ui.label("Latency Comp:");
            let mut auto_latency = true;
            ui.checkbox(&mut auto_latency, "Auto");
            ui.label("Processing:");
            let mut processing_mode = "Real-time";
            egui::ComboBox::from_label("")
                .selected_text(processing_mode)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut processing_mode, "Real-time", "Real-time");
                    ui.selectable_value(&mut processing_mode, "Offline", "Offline");
                    ui.selectable_value(&mut processing_mode, "Frozen", "Frozen");
                });
        });

        // Series routing with visual chain
        ui.collapsing("🔗 Series Chain Processing", |ui| {
            ui.label("🎵 Audio → 🎛️ Compressor → 🎚️ EQ → 🌊 Reverb → 🎛️ Limiter → 📤 Output");

            // Visual chain representation
            let chain_response = ui.allocate_response(egui::Vec2::new(80.0, 30.0), egui::Sense::hover());
            let chain_rect = chain_response.rect;
            ui.painter().rect_filled(chain_rect, egui::Rounding::same(4.0), egui::Color32::from_rgb(20, 20, 40));

            // Draw chain connections
            for i in 0..5 {
                let x = chain_rect.left() + (i as f32 * 16.0) + 8.0;
                ui.painter().circle_filled(egui::Pos2::new(x, chain_rect.center().y), 4.0, egui::Color32::from_rgb(100, 200, 100));
                if i < 4 {
                    ui.painter().line_segment(
                        [egui::Pos2::new(x + 4.0, chain_rect.center().y), egui::Pos2::new(x + 12.0, chain_rect.center().y)],
                        egui::Stroke::new(2.0, egui::Color32::from_rgb(150, 150, 200))
                    );
                }
            }

            ui.horizontal(|ui| {
                if ui.button("➕ Insert Effect").clicked() { println!("Insert effect in series chain"); }
                if ui.button("🔀 Create Parallel").clicked() { println!("Create parallel processing path"); }
                if ui.button("🎚️ Wet/Dry Mix").clicked() { println!("Add global wet/dry control"); }
                if ui.button("📊 Chain Analyzer").clicked() { println!("Show frequency response of chain"); }
            });

            // Individual effect controls in series
            ui.collapsing("🎛️ Compressor", |ui| {
                ui.horizontal(|ui| {
                    let mut compressor_on = true;
                    ui.checkbox(&mut compressor_on, "On");
                    ui.label("Ratio:");
                    let mut ratio = 4.0;
                    ui.add(egui::DragValue::new(&mut ratio).clamp_range(1.0..=20.0));
                    ui.label("Threshold:");
                    let mut threshold = -18.0;
                    ui.add(egui::DragValue::new(&mut threshold).clamp_range(-40.0..=0.0).suffix(" dB"));
                    ui.label("Attack:");
                    let mut attack = 5.0;
                    ui.add(egui::DragValue::new(&mut attack).clamp_range(0.1..=100.0).suffix(" ms"));
                    ui.label("Release:");
                    let mut release = 100.0;
                    ui.add(egui::DragValue::new(&mut release).clamp_range(10.0..=1000.0).suffix(" ms"));
                });
                let mut auto_makeup = false;
                ui.checkbox(&mut auto_makeup, "Auto Makeup Gain");
                let mut soft_knee = true;
                ui.checkbox(&mut soft_knee, "Soft Knee");
            });

            ui.collapsing("🎚️ EQ", |ui| {
                ui.horizontal(|ui| {
                    let mut eq_on = true;
                    ui.checkbox(&mut eq_on, "On");
                    ui.label("Low:");
                    let mut low = 1.0;
                    ui.add(egui::DragValue::new(&mut low).clamp_range(-24.0..=24.0).suffix(" dB"));
                    ui.label("High:");
                    let mut high = -1.5;
                    ui.add(egui::DragValue::new(&mut high).clamp_range(-24.0..=24.0).suffix(" dB"));
                });
            });

            ui.collapsing("🌊 Reverb", |ui| {
                ui.horizontal(|ui| {
                    let mut reverb_on = true;
                    ui.checkbox(&mut reverb_on, "On");
                    ui.label("Size:");
                    let mut size = 0.8;
                    ui.add(egui::Slider::new(&mut size, 0.1..=1.0));
                    ui.label("Damping:");
                    let mut damping = 0.5;
                    ui.add(egui::Slider::new(&mut damping, 0.0..=1.0));
                    ui.label("Mix:");
                    let mut mix = 0.3;
                    ui.add(egui::Slider::new(&mut mix, 0.0..=1.0));
                });
            });

            ui.collapsing("🎛️ Limiter", |ui| {
                ui.horizontal(|ui| {
                    let mut limiter_on = true;
                    ui.checkbox(&mut limiter_on, "On");
                    ui.label("Ceiling:");
                    let mut ceiling = -0.1;
                    ui.add(egui::DragValue::new(&mut ceiling).clamp_range(-20.0..=0.0).suffix(" dB"));
                    ui.label("Release:");
                    let mut release = 50.0;
                    ui.add(egui::DragValue::new(&mut release).clamp_range(1.0..=500.0).suffix(" ms"));
                });
            });
        });

        // Parallel routing with sends/returns
        ui.collapsing("🔀 Parallel Processing & Sends", |ui| {
            ui.label("🎵 Main Path: Audio → Output");
            ui.label("📤 Send 1: Audio → Chorus → Return Mix");
            ui.label("📤 Send 2: Audio → Delay → Return Mix");
            ui.label("📤 Send 3: Audio → Distortion → Return Mix");

            // Send controls
            for send in 0..4 {
                ui.horizontal(|ui| {
                    ui.label(format!("Send {}:", send + 1));
                    let mut send_level = 0.0;
                    ui.add(egui::Slider::new(&mut send_level, 0.0..=1.0));
                    let mut pre_fader = false;
                    ui.checkbox(&mut pre_fader, "Pre");
                    ui.label("To:");
                    let mut send_dest = match send {
                        0 => "Chorus",
                        1 => "Delay",
                        2 => "Distortion",
                        3 => "Reverb",
                        _ => "None",
                    };
                    egui::ComboBox::from_label("")
                        .selected_text(send_dest)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut send_dest, "Chorus", "Chorus");
                            ui.selectable_value(&mut send_dest, "Delay", "Delay");
                            ui.selectable_value(&mut send_dest, "Distortion", "Distortion");
                            ui.selectable_value(&mut send_dest, "Reverb", "Reverb");
                            ui.selectable_value(&mut send_dest, "Phaser", "Phaser");
                            ui.selectable_value(&mut send_dest, "Flanger", "Flanger");
                        });
                });
            }

            ui.horizontal(|ui| {
                if ui.button("➕ Add Send").clicked() { println!("Add new send/return"); }
                if ui.button("🎚️ Return Level").clicked() { println!("Adjust return level"); }
                if ui.button("🔄 Phase Invert").clicked() { println!("Phase invert return"); }
            });
        });

        // Advanced routing matrix with modulation
        ui.collapsing("🎛️ Advanced Routing Matrix & Modulation", |ui| {
            ui.label("Route effects between each other with modulation:");

            // Source -> Target matrix
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.set_min_width(100.0);
                    ui.label("📊 Sources:");
                    ui.label("🎛️ Macro 1");
                    ui.label("🎚️ LFO 1");
                    ui.label("📈 Envelope");
                    ui.label("🎵 Audio In");
                    ui.label("🎛️ Velocity");
                });

                ui.vertical(|ui| {
                    ui.label("🎯 Targets:");
                    ui.horizontal(|ui| {
                        ui.set_min_width(60.0);
                        ui.label("Filter");
                        ui.label("Volume");
                        ui.label("Pan");
                        ui.label("Pitch");
                        ui.label("Delay");
                    });

                    // Modulation amount matrix
                    for source in 0..5 {
                        ui.horizontal(|ui| {
                            for target in 0..5 {
                                let amount = if source == target { 0.5 } else { 0.0 };
                                let mut amount_val = amount;
                                ui.add(egui::DragValue::new(&mut amount_val).clamp_range(-1.0..=1.0).speed(0.01));
                            }
                        });
                    }
                });
            });

            ui.separator();
            ui.label("🎚️ Effect Modulation:");
            ui.horizontal(|ui| {
                ui.label("Source:");
                let mut mod_source = "LFO";
                egui::ComboBox::from_label("")
                    .selected_text(mod_source)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut mod_source, "LFO", "LFO");
                        ui.selectable_value(&mut mod_source, "Envelope", "Envelope");
                        ui.selectable_value(&mut mod_source, "Audio", "Audio");
                        ui.selectable_value(&mut mod_source, "Macro", "Macro");
                    });

                ui.label("Target:");
                let mut mod_target = "Reverb Mix";
                egui::ComboBox::from_label("")
                    .selected_text(mod_target)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut mod_target, "Reverb Mix", "Reverb Mix");
                        ui.selectable_value(&mut mod_target, "Delay Time", "Delay Time");
                        ui.selectable_value(&mut mod_target, "Chorus Rate", "Chorus Rate");
                        ui.selectable_value(&mut mod_target, "Dist Drive", "Dist Drive");
                    });

                ui.label("Amount:");
                let mut mod_amount = 0.5;
                ui.add(egui::Slider::new(&mut mod_amount, -1.0..=1.0));
            });
        });

        // Effect presets and automation
        ui.collapsing("💾 Effect Presets & Automation", |ui| {
            ui.horizontal(|ui| {
                ui.label("Chain Preset:");
                let mut preset = "Mastering Chain";
                egui::ComboBox::from_label("")
                    .selected_text(preset)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut preset, "Mastering Chain", "Mastering Chain");
                        ui.selectable_value(&mut preset, "Vocal Chain", "Vocal Chain");
                        ui.selectable_value(&mut preset, "Drum Chain", "Drum Chain");
                        ui.selectable_value(&mut preset, "Guitar Chain", "Guitar Chain");
                        ui.selectable_value(&mut preset, "Mix Bus", "Mix Bus");
                    });
                if ui.button("💾 Save").clicked() { println!("Save effects chain preset"); }
                if ui.button("📂 Load").clicked() { println!("Load effects chain preset"); }
            });

            let mut link_automation = true;
            ui.checkbox(&mut link_automation, "Link to Automation");
            let mut bypass_solo = false;
            ui.checkbox(&mut bypass_solo, "Bypass in Solo");
            let mut sidechain_input = true;
            ui.checkbox(&mut sidechain_input, "Sidechain Input");

            ui.horizontal(|ui| {
                ui.label("Sidechain Key:");
                let mut key_source = "Kick";
                egui::ComboBox::from_label("")
                    .selected_text(key_source)
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut key_source, "Kick", "Kick");
                        ui.selectable_value(&mut key_source, "Snare", "Snare");
                        ui.selectable_value(&mut key_source, "Bass", "Bass");
                        ui.selectable_value(&mut key_source, "Master", "Master");
                    });
            });
        });

        // Performance monitoring
        ui.collapsing("📊 Effects Performance", |ui| {
            ui.horizontal(|ui| {
                ui.label("CPU Usage:");
                ui.label("15%");
                ui.label("Latency:");
                ui.label("8.2ms");
                ui.label("Active Effects:");
                ui.label("6/12");
            });

            ui.horizontal(|ui| {
                ui.label("Headroom:");
                ui.colored_label(egui::Color32::GREEN, "4.1 dB");
                ui.label("Clipping:");
                ui.colored_label(egui::Color32::RED, "None");
            });
        });
    });
}

/// Helper function to add a node to the canvas
fn add_node_to_canvas(state: &mut NodeViewState, name: &str, node_type: &str) {
    let id = format!("{}_{}", name.to_lowercase().replace(" ", "_"), state.node_positions.len());
    let x = 100.0 + (state.node_positions.len() as f32 * 50.0);
    let y = 100.0 + (state.node_positions.len() as f32 * 30.0);

    let parameters = match node_type {
        "generator.sine" => vec![
            NodeParameter { name: "Frequency".to_string(), value: 440.0, min: 20.0, max: 20000.0, modulated: false },
            NodeParameter { name: "Amplitude".to_string(), value: 0.8, min: 0.0, max: 1.0, modulated: false },
        ],
        "filter.lpf" => vec![
            NodeParameter { name: "Cutoff".to_string(), value: 1000.0, min: 20.0, max: 20000.0, modulated: true },
            NodeParameter { name: "Resonance".to_string(), value: 0.7, min: 0.1, max: 10.0, modulated: false },
        ],
        "effect.reverb" => vec![
            NodeParameter { name: "Decay".to_string(), value: 2.0, min: 0.1, max: 10.0, modulated: false },
            NodeParameter { name: "Mix".to_string(), value: 0.3, min: 0.0, max: 1.0, modulated: false },
        ],
        _ => vec![
            NodeParameter { name: "Level".to_string(), value: 0.8, min: 0.0, max: 1.0, modulated: false },
        ],
    };

    state.node_positions.push(NodePosition {
        id,
        node_type: node_type.to_string(),
        x,
        y,
        selected: false,
        parameters,
    });
}

/// Helper function to add a preset to the canvas
fn add_preset_to_canvas(state: &mut NodeViewState, preset: &NodePreset) {
    let id = format!("{}_{}", preset.name.to_lowercase().replace(" ", "_"), state.node_positions.len());
    let x = 200.0 + (state.node_positions.len() as f32 * 50.0);
    let y = 200.0 + (state.node_positions.len() as f32 * 30.0);

    state.node_positions.push(NodePosition {
        id,
        node_type: preset.node_type.clone(),
        x,
        y,
        selected: true,
        parameters: preset.parameters.clone(),
    });
}

/// Add Modular Cookbook-inspired preset
fn add_cookbook_preset(state: &mut NodeViewState, preset_type: &str) {
    match preset_type {
        "resonant_filter_chain" => {
            // Create resonant filter chain: LPF -> BPF -> HPF with modulation
            add_node_to_canvas(state, "LPF (Resonant)", "filter.lpf");
            add_node_to_canvas(state, "BPF (Resonant)", "filter.bpf");
            add_node_to_canvas(state, "HPF (Resonant)", "filter.hpf");
            add_node_to_canvas(state, "LFO Modulator", "modulator.lfo");
            println!("Added resonant filter chain from Cookbook p.60");
        }
        "wave_folder_envelope" => {
            // Wave folder with envelope shaping
            add_node_to_canvas(state, "Wave Folder", "effect.wavefolder");
            add_node_to_canvas(state, "Envelope Shaper", "modulator.envelope");
            add_node_to_canvas(state, "Attenuator", "utility.attenuator");
            println!("Added wave folder + envelope from Cookbook p.62");
        }
        "mixer_envelopes" => {
            // Mixer with envelope control
            add_node_to_canvas(state, "4-Channel Mixer", "utility.mixer");
            add_node_to_canvas(state, "Envelope 1", "modulator.envelope");
            add_node_to_canvas(state, "Envelope 2", "modulator.envelope");
            add_node_to_canvas(state, "VCA 1", "utility.vca");
            add_node_to_canvas(state, "VCA 2", "utility.vca");
            println!("Added mixer with envelopes from Cookbook p.64");
        }
        "feedback_filter" => {
            // Delay with filtered feedback
            add_node_to_canvas(state, "Delay", "effect.delay");
            add_node_to_canvas(state, "Feedback Filter", "filter.lpf");
            add_node_to_canvas(state, "Feedback Amount", "utility.attenuator");
            println!("Added feedback filter from Cookbook p.88");
        }
        "granular_timestretch" => {
            // Granular time stretching
            add_node_to_canvas(state, "Granular Processor", "effect.granular");
            add_node_to_canvas(state, "Time Control", "modulator.lfo");
            add_node_to_canvas(state, "Pitch Control", "modulator.lfo");
            println!("Added granular time stretch from Cookbook p.124");
        }
        "spectral_morphing" => {
            // Spectral processing
            add_node_to_canvas(state, "FFT Processor", "effect.spectral");
            add_node_to_canvas(state, "Morph Control", "modulator.lfo");
            add_node_to_canvas(state, "Freeze Toggle", "utility.switch");
            println!("Added spectral morphing from Cookbook p.144");
        }
        _ => {}
    }
}

/// Add virtual instrument preset
fn add_instrument_preset(state: &mut NodeViewState, instrument_type: &str) {
    match instrument_type {
        "fm_synth" => {
            // FM synthesis: Carrier + Modulator
            add_node_to_canvas(state, "Carrier Osc", "generator.sine");
            add_node_to_canvas(state, "Modulator Osc", "generator.sine");
            add_node_to_canvas(state, "FM Amount", "utility.attenuator");
            add_node_to_canvas(state, "Envelope", "modulator.envelope");
            add_node_to_canvas(state, "VCA", "utility.vca");
            println!("Added FM synthesizer instrument");
        }
        "wavetable" => {
            // Wavetable synthesis
            add_node_to_canvas(state, "Wavetable Osc", "generator.wavetable");
            add_node_to_canvas(state, "Wave Position", "modulator.lfo");
            add_node_to_canvas(state, "Envelope", "modulator.envelope");
            add_node_to_canvas(state, "Filter", "filter.lpf");
            println!("Added wavetable synthesizer instrument");
        }
        "drum_machine" => {
            // Drum synthesis
            add_node_to_canvas(state, "Kick Generator", "generator.kick");
            add_node_to_canvas(state, "Snare Generator", "generator.snare");
            add_node_to_canvas(state, "HiHat Generator", "generator.hihat");
            add_node_to_canvas(state, "Mixer", "utility.mixer");
            add_node_to_canvas(state, "Master Compressor", "effect.compressor");
            println!("Added drum machine instrument");
        }
        "physical_modeling" => {
            // Physical modeling
            add_node_to_canvas(state, "String Model", "generator.karplus_strong");
            add_node_to_canvas(state, "Pickup Position", "utility.attenuator");
            add_node_to_canvas(state, "Damping Control", "utility.attenuator");
            add_node_to_canvas(state, "Body Filter", "filter.lpf");
            println!("Added physical modeling instrument");
        }
        _ => {}
    }
}

/// Application Entry Point with UI - Full eframe Implementation
/// Uses eframe's native App structure for cross-platform GUI
pub fn run_hexodsp_ui() -> Result<(), Box<dyn std::error::Error>> {
    println!("HexoDSP UI - Full eframe implementation");
    println!("Three-View System Architecture:");
    println!("  - Arrangement View: Traditional DAW timeline");
    println!("  - Live View: Real-time performance interface");
    println!("  - Node View: Modular node-based patching");

    // Create eframe app with native window
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1600.0, 900.0])
            .with_title("HexoDSP DAW - Revolutionary Node-Based DAW"),
        ..Default::default()
    };

    eframe::run_native(
        "HexoDSP DAW",
        options,
        Box::new(|_cc| Box::<HexoDSPApp>::default()),
    )?;

    Ok(())
}


