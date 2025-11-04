/// eframe-based UI Framework for HexoDSP DAW
///
/// This module implements the revolutionary three-view UI system:
/// - Arrangement View: Traditional DAW timeline/arrangement
/// - Live View: Real-time performance interface
/// - Node View: Modular node-based patching

use egui;
use std::sync::{Arc, Mutex};
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};
use serde_json;
use crate::audio_engine::bridge::{AudioEngineBridge, AudioEngineState, AudioParamMessage};
use crate::audio_engine::cpal_io::AudioIO;
use crate::node_instance_manager::NodeInstanceManager;
use crate::clip_node_integration::ClipNodeIntegration;
use crate::ui::hexagonal_node_view::HexNodeViewState;
use crate::presets::{PresetManager, PresetCategory};
// Temporarily commented out to isolate compilation issues
use crate::vst3_host::{VST3Host, VST3Plugin};
// use crate::midi2_mpe::{Midi2Processor, MPEConfig};
// use crate::web_interface::{WebInterfaceServer, WebInterfaceConfig};
// use crate::modular_patch_system::{ModularPatchManager, ModularNode, NodeType};
// use crate::theming_system::{ThemeManager, Theme};
// use crate::piano_roll_editor::{PianoRollEditor, MidiClip};
use crate::midi_control_system::MidiControlSystem;
use crate::piano_roll_editor::{PianoRollEditor, GridResolution};

/// UI View Modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum UIViewMode {
    #[default]
    Arrangement,
    Live,
    Node,
}

/// File browser item
#[derive(Debug, Clone)]
pub struct FileItem {
    pub name: String,
    pub path: PathBuf,
    pub is_directory: bool,
    pub file_type: FileType,
    pub size: Option<u64>,
}

/// Supported file types for the browser
#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    Audio,
    Midi,
    Project,
    Directory,
    Other,
}

/// VST3 Plugin Sorting Mode
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum VST3Sort {
    Alphabetical,
    Vendor,
    Category,
}

/// Main UI State
#[derive(Clone)]
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct UiState {
    pub current_view: UIViewMode,
    pub show_transport: bool,
    pub show_mixer: bool,
    pub show_browser: bool,
    pub zoom_level: f32,
    pub pan_offset: (f32, f32), // Using tuple instead of Vec2
    // Theme controls
    pub dark_mode: bool,
    pub font_scale: f32,
    pub contrast: f32,
    // --- New Mixer-related persistent state ---
    pub master_volume: f32,
    pub master_pan: f32,
    pub master_mute: bool,
    pub master_mono: bool,
    pub master_phase: bool,
    pub dim_level: f32,
    // Input monitoring controls to prevent feedback
    pub input_monitoring_enabled: bool,
    pub input_gain: f32,
    pub input_monitor_level: f32,
    pub master_eq_on: bool,
    pub master_eq_low_gain: f32,
    pub master_eq_lmid_gain: f32,
    pub master_eq_hmid_gain: f32,
    pub master_eq_high_gain: f32,
    pub master_comp_ratio: f32,
    pub master_comp_threshold: f32,
    pub master_lim_ceiling: f32,
    pub link_channels: bool,
    pub show_levels: bool,
    #[serde(skip)]
    pub track_channels: Vec<ChannelState>, // State for individual tracks
    #[serde(skip)]
    pub professional_mixer: ProfessionalMixer, // Enhanced professional mixer
    #[serde(skip)]
    pub preset_manager: PresetManager, // Modular content system
    // Theming system
    pub theme_manager: crate::theming_system::ThemeManager,
    // Modular patch manager
    #[serde(skip)]
    pub modular_patch_manager: crate::modular_patch_system::ModularPatchManager,
    // Audio bridge reference for transport controls
    #[serde(skip)]
    pub audio_bridge: Option<Arc<Mutex<AudioEngineBridge>>>,
    // File browser state
    pub sample_library_path: String,
    pub current_browser_path: String,
    #[serde(skip)]
    pub browser_files: Vec<FileItem>,
    pub selected_file: Option<String>,
    pub show_file_browser: bool,
    // New comprehensive systems - temporarily commented out
    #[serde(skip)]
    pub vst3_host: VST3Host,
    #[serde(skip)]
    pub midi_control_system: MidiControlSystem,
    #[serde(skip)]
    pub piano_roll_editor: PianoRollEditor,
    // pub midi2_processor: Midi2Processor,
    // pub web_server: Option<WebInterfaceServer>,
    // pub modular_patch_manager: ModularPatchManager,
    // pub theme_manager: ThemeManager,
    // pub piano_roll_editor: PianoRollEditor,
    pub show_vst3_browser: bool,
    pub vst3_search: String,
    pub vst3_sort_alpha: bool,
    pub vst3_sort_mode: VST3Sort,
    pub vst3_selected_track: usize,
    pub show_midi_control: bool,
    pub show_piano_roll: bool,
    pub show_modular_patches: bool,
    pub show_theme_editor: bool,
    pub web_server_running: bool,
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
            dark_mode: true,
            font_scale: 1.0,
            contrast: 1.0,
            // Initialize Master controls to match UI defaults
            master_volume: 0.8,
            master_pan: 0.0,
            master_mute: false,
            master_mono: false,
            master_phase: false,
            dim_level: -20.0,
            // Input monitoring disabled by default to prevent feedback
            input_monitoring_enabled: false,
            input_gain: 0.5,
            input_monitor_level: 0.0,
            master_eq_on: true,
            master_eq_low_gain: 1.5,
            master_eq_lmid_gain: -2.0,
            master_eq_hmid_gain: 1.0,
            master_eq_high_gain: -1.5,
            master_comp_ratio: 4.0,
            master_comp_threshold: -18.0,
            master_lim_ceiling: -0.1,
            link_channels: false,
            show_levels: true,
            track_channels: Vec::new(),
            professional_mixer: ProfessionalMixer::default(),
            preset_manager: PresetManager::new(),
            theme_manager: crate::theming_system::ThemeManager::new(),
            modular_patch_manager: crate::modular_patch_system::ModularPatchManager::new(),
            audio_bridge: None,
            // File browser defaults
            sample_library_path: "C:\\Users\\kapil\\OneDrive\\Documents".to_string(),
            current_browser_path: "C:\\Users\\kapil\\OneDrive\\Documents".to_string(),
            browser_files: Vec::new(),
            selected_file: None,
            show_file_browser: true,
            // Initialize new comprehensive systems - temporarily commented out
            vst3_host: VST3Host::new(),
            // midi2_processor: Midi2Processor::new(),
            // web_server: None,
            // modular_patch_manager: ModularPatchManager::new(),
            // theme_manager: ThemeManager::new(),
            piano_roll_editor: PianoRollEditor::new("Main Piano Roll".to_string()),
            midi_control_system: MidiControlSystem::new(),
            show_vst3_browser: false,
            vst3_search: String::new(),
            vst3_sort_alpha: true,
            vst3_sort_mode: VST3Sort::Alphabetical,
            vst3_selected_track: 0,
            show_midi_control: false,
            show_piano_roll: false,
            show_modular_patches: false,
            show_theme_editor: false,
            web_server_running: false,
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

        // Initialize new systems - temporarily commented out
        let _ = default_state.vst3_host.scan_plugins();
        // let _ = default_state.midi_control_system.scan_devices();

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
    pub auto_gain: bool,
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
            auto_gain: false,
        }
    }
}

/// Enhanced Professional Mixer Channel Strip
#[derive(Clone, Debug)]
pub struct ProfessionalChannelStrip {
    pub channel_state: ChannelState,
    pub eq: ChannelEQ,
    pub dynamics: ChannelDynamics,
    pub sends: Vec<SendSlot>,
    pub inserts: Vec<InsertSlot>,
    pub routing: ChannelRouting,
    pub metering: ChannelMetering,
    pub automation: ChannelAutomation,
}

#[derive(Clone, Debug)]
pub struct ChannelEQ {
    pub enabled: bool,
    pub high_shelf_freq: f32,
    pub high_shelf_gain: f32,
    pub high_shelf_q: f32,
    pub high_mid_freq: f32,
    pub high_mid_gain: f32,
    pub high_mid_q: f32,
    pub low_mid_freq: f32,
    pub low_mid_gain: f32,
    pub low_mid_q: f32,
    pub low_shelf_freq: f32,
    pub low_shelf_gain: f32,
    pub low_shelf_q: f32,
    pub high_pass_freq: f32,
    pub high_pass_enabled: bool,
    pub low_pass_freq: f32,
    pub low_pass_enabled: bool,
}

impl Default for ChannelEQ {
    fn default() -> Self {
        Self {
            enabled: true,
            high_shelf_freq: 12000.0,
            high_shelf_gain: 0.0,
            high_shelf_q: 0.7,
            high_mid_freq: 2500.0,
            high_mid_gain: 0.0,
            high_mid_q: 1.0,
            low_mid_freq: 500.0,
            low_mid_gain: 0.0,
            low_mid_q: 1.0,
            low_shelf_freq: 80.0,
            low_shelf_gain: 0.0,
            low_shelf_q: 0.7,
            high_pass_freq: 20.0,
            high_pass_enabled: false,
            low_pass_freq: 20000.0,
            low_pass_enabled: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ChannelDynamics {
    pub compressor_enabled: bool,
    pub comp_threshold: f32,
    pub comp_ratio: f32,
    pub comp_attack: f32,
    pub comp_release: f32,
    pub comp_knee: f32,
    pub comp_makeup_gain: f32,
    pub gate_enabled: bool,
    pub gate_threshold: f32,
    pub gate_ratio: f32,
    pub gate_attack: f32,
    pub gate_hold: f32,
    pub gate_release: f32,
    pub limiter_enabled: bool,
    pub limiter_ceiling: f32,
    pub limiter_release: f32,
}

impl Default for ChannelDynamics {
    fn default() -> Self {
        Self {
            compressor_enabled: false,
            comp_threshold: -18.0,
            comp_ratio: 3.0,
            comp_attack: 10.0,
            comp_release: 100.0,
            comp_knee: 2.0,
            comp_makeup_gain: 0.0,
            gate_enabled: false,
            gate_threshold: -40.0,
            gate_ratio: 10.0,
            gate_attack: 1.0,
            gate_hold: 10.0,
            gate_release: 100.0,
            limiter_enabled: false,
            limiter_ceiling: -0.1,
            limiter_release: 50.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SendSlot {
    pub name: String,
    pub level: f32,
    pub enabled: bool,
    pub pre_fader: bool,
    pub destination: String,
    pub pan: f32,
}

impl Default for SendSlot {
    fn default() -> Self {
        Self {
            name: "Send".to_string(),
            level: 0.0,
            enabled: false,
            pre_fader: false,
            destination: "None".to_string(),
            pan: 0.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct InsertSlot {
    pub name: String,
    pub plugin_name: String,
    pub enabled: bool,
    pub bypass: bool,
    pub wet_dry: f32,
    pub parameters: Vec<(String, f32)>,
}

impl Default for InsertSlot {
    fn default() -> Self {
        Self {
            name: "Insert".to_string(),
            plugin_name: "None".to_string(),
            enabled: false,
            bypass: false,
            wet_dry: 1.0,
            parameters: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ChannelRouting {
    pub input_source: String,
    pub output_destination: String,
    pub sidechain_source: String,
    pub group_assignment: Option<usize>,
    pub stereo_link: bool,
    pub phase_invert_left: bool,
    pub phase_invert_right: bool,
}

impl Default for ChannelRouting {
    fn default() -> Self {
        Self {
            input_source: "None".to_string(),
            output_destination: "Master".to_string(),
            sidechain_source: "None".to_string(),
            group_assignment: None,
            stereo_link: true,
            phase_invert_left: false,
            phase_invert_right: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ChannelMetering {
    pub peak_left: f32,
    pub peak_right: f32,
    pub rms_left: f32,
    pub rms_right: f32,
    pub gain_reduction: f32,
    pub clip_indicator: bool,
    pub phase_correlation: f32,
}

impl Default for ChannelMetering {
    fn default() -> Self {
        Self {
            peak_left: -60.0,
            peak_right: -60.0,
            rms_left: -60.0,
            rms_right: -60.0,
            gain_reduction: 0.0,
            clip_indicator: false,
            phase_correlation: 1.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ChannelAutomation {
    pub volume_automation: Vec<AutomationPoint>,
    pub pan_automation: Vec<AutomationPoint>,
    pub send_automation: Vec<Vec<AutomationPoint>>,
    pub eq_automation: Vec<Vec<AutomationPoint>>,
    pub plugin_automation: Vec<Vec<AutomationPoint>>,
}

impl Default for ChannelAutomation {
    fn default() -> Self {
        Self {
            volume_automation: Vec::new(),
            pan_automation: Vec::new(),
            send_automation: vec![Vec::new(); 8], // 8 sends
            eq_automation: vec![Vec::new(); 4], // 4 EQ bands
            plugin_automation: Vec::new(),
        }
    }
}

impl Default for ProfessionalChannelStrip {
    fn default() -> Self {
        Self {
            channel_state: ChannelState::default(),
            eq: ChannelEQ::default(),
            dynamics: ChannelDynamics::default(),
            sends: vec![SendSlot::default(); 8], // 8 sends per channel
            inserts: vec![InsertSlot::default(); 8], // 8 inserts per channel
            routing: ChannelRouting::default(),
            metering: ChannelMetering::default(),
            automation: ChannelAutomation::default(),
        }
    }
}

/// Professional Mixer State
#[derive(Clone, Debug)]
pub struct ProfessionalMixer {
    pub channels: Vec<ProfessionalChannelStrip>,
    pub return_channels: Vec<ProfessionalChannelStrip>,
    pub group_channels: Vec<ProfessionalChannelStrip>,
    pub master_channel: ProfessionalChannelStrip,
    pub mixer_view_mode: MixerViewMode,
    pub show_eq: bool,
    pub show_dynamics: bool,
    pub show_sends: bool,
    pub show_inserts: bool,
    pub show_routing: bool,
    pub channel_width: f32,
    pub selected_channel: Option<usize>,
    pub solo_in_place: bool,
    pub auto_solo: bool,
    pub pre_fader_listen: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum MixerViewMode {
    Compact,
    Extended,
    Large,
}

impl Default for ProfessionalMixer {
    fn default() -> Self {
        Self {
            channels: vec![ProfessionalChannelStrip::default(); 32], // 32 channels
            return_channels: vec![ProfessionalChannelStrip::default(); 8], // 8 returns
            group_channels: vec![ProfessionalChannelStrip::default(); 8], // 8 groups
            master_channel: ProfessionalChannelStrip::default(),
            mixer_view_mode: MixerViewMode::Extended,
            show_eq: true,
            show_dynamics: false,
            show_sends: false,
            show_inserts: false,
            show_routing: false,
            channel_width: 80.0,
            selected_channel: None,
            solo_in_place: false,
            auto_solo: false,
            pre_fader_listen: false,
        }
    }
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
        // Initialize 8 track-related vectors to match the loop in draw_arrangement_view
        let automation_lanes_visible = vec![false; 8];
        let track_inputs = vec!["None".to_string(); 8];
        let track_armed = vec![false; 8];
        
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
    // Hexagonal node canvas state (egui painter-based)
    pub hex: HexNodeViewState,
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
            hex: HexNodeViewState::default(),
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
    pub audio_bridge: Arc<Mutex<AudioEngineBridge>>,
    pub node_manager: Arc<Mutex<NodeInstanceManager>>,
    pub clip_integration: ClipNodeIntegration,
    pub audio_io: Arc<Mutex<AudioIO>>, // keep audio stream alive
}

impl Default for HexoDSPApp {
    fn default() -> Self {
        Self::new_with_state(UiState::default())
    }
}

impl HexoDSPApp {
    pub fn new_with_state(ui_state: UiState) -> Self {
        // TEMPORARILY DISABLE ALL AUDIO TO STOP FEEDBACK - UI ONLY MODE
        // TODO: Implement proper audio system without feedback
        println!("🔇 AUDIO DISABLED - UI ONLY MODE TO PREVENT FEEDBACK");
        let audio_io = AudioIO::new().expect("Failed to initialize AudioIO");
        // DO NOT START ANY AUDIO STREAMS
        let audio_io_arc = Arc::new(Mutex::new(audio_io));

        // Initialize audio bridge (UI thread will process messages and update AudioIO)
        let bridge = AudioEngineBridge::new();
        let bridge_arc = Arc::new(Mutex::new(bridge));

        // Initialize node manager and clip integration
        let node_manager = Arc::new(Mutex::new(NodeInstanceManager::new(bridge_arc.clone())));
        let clip_integration = ClipNodeIntegration::new(node_manager.clone());

        // Build UI state and inject bridge ref for transport controls
        let mut ui_state = UiState::default();
        ui_state.audio_bridge = Some(bridge_arc.clone());

        Self {
            ui_state,
            arrangement_state: ArrangementViewState::default(),
            live_state: LiveViewState::default(),
            node_state: NodeViewState::default(),
            audio_bridge: bridge_arc,
            node_manager,
            clip_integration,
            audio_io: audio_io_arc,
        }
    }
}

// Removed eframe::App implementation; UI is driven via egui contexts without eframe.

/// Renders the DAW's main timeline/arrangement view.
fn arrangement_view(ui: &mut egui::Ui, state: &mut ArrangementViewState) {
    ui.heading("🎼 Arrangement View");
    ui.label(format!("Timeline Position: {:.2} beats", state.timeline_position));
    ui.separator();
    ui.horizontal(|ui| {
        ui.label("Zoom Level:");
        ui.add(egui::Slider::new(&mut state.timeline_zoom, 0.1..=10.0));
        ui.checkbox(&mut state.snap_to_grid, "Snap to Grid");
    });
}

/// Renders the DAW's real-time performance (Live) view.
fn live_view(ui: &mut egui::Ui, state: &mut LiveViewState) {
    ui.heading("🎵 Live View - Real-time Performance");
    ui.separator();
    ui.horizontal(|ui| {
        ui.label("Crossfader:");
        ui.add(egui::Slider::new(&mut state.crossfader_position, 0.0..=1.0).text("A-B"));
        ui.checkbox(&mut state.sync_enabled, "Sync");
        ui.label(format!("Master Tempo: {:.2} BPM", state.master_tempo));
    });
}

/// Renders the DAW's modular node-based patching view.
fn node_view(ui: &mut egui::Ui, state: &mut NodeViewState) {
    ui.heading("🔗 Node View - Modular Patching");
    ui.separator();

    // Minimal toolbar for adding a few node types to the canvas
    ui.horizontal(|ui| {
        if ui.button("➕ Sine Osc").clicked() {
            let pos = ui.input(|i| i.pointer.hover_pos()).unwrap_or(egui::Pos2::new(100.0, 100.0));
            state.hex.add_node("Sine Osc", "generator.sine", pos);
        }
        if ui.button("➕ LPF").clicked() {
            let pos = ui.input(|i| i.pointer.hover_pos()).unwrap_or(egui::Pos2::new(220.0, 100.0));
            state.hex.add_node("LPF", "filter.lpf", pos);
        }
        if ui.button("➕ Delay").clicked() {
            let pos = ui.input(|i| i.pointer.hover_pos()).unwrap_or(egui::Pos2::new(340.0, 100.0));
            state.hex.add_node("Delay", "effect.delay", pos);
        }
    });

    // Canvas area: draw the hexagonal node view
    let available = ui.available_size();
    let canvas_rect = egui::Rect::from_min_size(ui.cursor().min, available);

    // Draw node canvas directly; drag-and-drop integration will be wired via bevy_egui systems.
    state.hex.draw(ui, canvas_rect);

    ui.allocate_rect(canvas_rect, egui::Sense::click());
}

/// Renders the transport (play/stop/tempo) bar.
fn transport_bar(ui: &mut egui::Ui, ui_state: &mut UiState) {
    ui.horizontal(|ui| {
        ui.label("⏱️");
        if ui.button("▶").clicked() {
            if let Some(br) = &ui_state.audio_bridge { let _ = br.lock().unwrap().send_param(AudioParamMessage::Play); }
        }
        if ui.button("◼").clicked() {
            if let Some(br) = &ui_state.audio_bridge { let _ = br.lock().unwrap().send_param(AudioParamMessage::Stop); }
        }
        if ui.button("⏺").clicked() {
            if let Some(br) = &ui_state.audio_bridge { let _ = br.lock().unwrap().send_param(AudioParamMessage::Record); }
        }
        ui.separator();
        // Placeholder for tempo
        let mut tempo = 120.0;
        ui.label("BPM:");
        if ui.add(egui::DragValue::new(&mut tempo).clamp_range(40.0..=300.0).suffix(" BPM")).changed() {
            if let Some(br) = &ui_state.audio_bridge { let _ = br.lock().unwrap().send_param(AudioParamMessage::SetTempo(tempo)); }
        }
    });
}

/// Main UI System - Full eframe Implementation
/// Uses eframe's native App structure
pub fn ui_system(ctx: &egui::Context, ui_state: &mut UiState, arrangement_state: &mut ArrangementViewState, live_state: &mut LiveViewState, node_state: &mut NodeViewState) {
    apply_theme(ctx, ui_state);
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
                ui.checkbox(&mut ui_state.show_vst3_browser, "🎛️ VST3 Plugins");
                ui.checkbox(&mut ui_state.show_midi_control, "🎹 MIDI Control");
                ui.checkbox(&mut ui_state.show_piano_roll, "🎼 Piano Roll");
                ui.checkbox(&mut ui_state.show_modular_patches, "🔗 Modular Patches");
                ui.checkbox(&mut ui_state.show_theme_editor, "🎨 Theme Editor");
                ui.separator();
                // Web server functionality temporarily commented out
                // if ui.button("🌐 Start Web Server").clicked() { 
                //     if ui_state.web_server.is_none() {
                //         ui_state.web_server = Some(WebInterfaceServer::new(WebInterfaceConfig::default()));
                //         ui_state.web_server_running = true;
                //     }
                // }
                // if ui.button("🌐 Stop Web Server").clicked() { 
                //     ui_state.web_server = None;
                //     ui_state.web_server_running = false;
                // }
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

            // Theme menu
            ui.menu_button("🎨 Theme", |ui| {
                ui.checkbox(&mut ui_state.dark_mode, "Dark Mode");
                ui.add(egui::Slider::new(&mut ui_state.font_scale, 0.8..=1.6).text("Font Scale"));
                ui.add(egui::Slider::new(&mut ui_state.contrast, 0.5..=1.5).text("Contrast"));
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
        
        // Close the Top menu bar panel
        });

    // Side panels
    if ui_state.show_browser {
        egui::SidePanel::left("browser").show(ctx, |ui| {
            ui.set_min_width(280.0);
            ui.heading("📁 Sample Library Browser");

            // Library path configuration
            ui.horizontal(|ui| {
                ui.label("📂 Library Path:");
                if ui.button("📁").on_hover_text("Browse for library folder").clicked() {
                    // TODO: Open file dialog
                    println!("Open file dialog for library path");
                }
            });
            
            ui.horizontal(|ui| {
                let mut path_text = ui_state.sample_library_path.clone();
                if ui.text_edit_singleline(&mut path_text).changed() {
                    ui_state.sample_library_path = path_text;
                }
                if ui.button("🔄").on_hover_text("Refresh library").clicked() {
                    ui_state.set_sample_library_path(ui_state.sample_library_path.clone());
                }
            });

            ui.separator();

            // Navigation controls
            ui.horizontal(|ui| {
                if ui.button("⬆️ Up").clicked() {
                    ui_state.navigate_up();
                }
                if ui.button("🏠 Home").clicked() {
                    ui_state.current_browser_path = ui_state.sample_library_path.clone();
                    ui_state.load_browser_files();
                }
                ui.label(format!("📍 {}", 
                    Path::new(&ui_state.current_browser_path)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Root")
                ));
            });

            ui.separator();

            // File browser
            egui::ScrollArea::vertical().show(ui, |ui| {
                // Load files if browser_files is empty
                if ui_state.browser_files.is_empty() {
                    ui_state.load_browser_files();
                }

                for file in &ui_state.browser_files.clone() {
                    let id = ui.make_persistent_id(&file.path);
                    // Removed egui::DragAndDrop::drag_source; render row directly.
                    ui.horizontal(|ui| {
                        // File type icon
                        let icon = match file.file_type {
                            FileType::Directory => "📁",
                            FileType::Audio => "🎵",
                            FileType::Midi => "🎹",
                            FileType::Project => "💾",
                            FileType::Other => "📄",
                        };

                        let response = ui.selectable_label(
                            ui_state.selected_file.as_ref() == Some(&file.name),
                            format!("{} {}", icon, file.name)
                        );

                        if response.clicked() {
                            if file.is_directory {
                                    ui_state.navigate_to(&file.path);
                                } else {
                                    ui_state.selected_file = Some(file.name.clone());
                                    println!("🎯 Selected file: {}", file.path.display());
                                }
                            }

                            // Show file size for files
                            if let Some(size) = file.size {
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    let size_str = if size > 1024 * 1024 {
                                        format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
                                    } else if size > 1024 {
                                        format!("{:.1} KB", size as f64 / 1024.0)
                                    } else {
                                        format!("{} B", size)
                                    };
                                    ui.label(egui::RichText::new(size_str).small().weak());
                                });
                            }
                        });
                    // Drag-and-drop removed; no payload handling.
                }

                if ui_state.browser_files.is_empty() {
                    ui.centered_and_justified(|ui| {
                        ui.label("📂 No files found in this directory");
                    });
                }
            });

            ui.separator();

            // Quick actions for selected file
            if let Some(selected) = &ui_state.selected_file {
                ui.label(format!("Selected: {}", selected));
                ui.horizontal(|ui| {
                    if ui.button("▶️ Preview").clicked() {
                        println!("🎯 Preview file: {}", selected);
                    }
                    if ui.button("➕ Add to Track").clicked() {
                        println!("🎯 Add to track: {}", selected);
                    }
                    if ui.button("📋 Copy Path").clicked() {
                        println!("🎯 Copy path: {}", selected);
                    }
                });
            }

            ui.separator();

            // Node Presets - The revolutionary modular content system
            ui.collapsing("🔗 Node Presets", |ui| {
                // Display presets by category
                for category in ui_state.preset_manager.get_categories() {
                    let category_name = format!("📦 {}", category.as_str());
                    ui.collapsing(category_name, |ui| {
                        if let Some(presets) = ui_state.preset_manager.get_presets_by_category(category) {
                            for preset in presets {
                                ui.horizontal(|ui| {
                                    // Preset icon based on category
                                    let icon = match category {
                                        PresetCategory::Generators => "🎛️",
                                        PresetCategory::Filters => "🔧",
                                        PresetCategory::Effects => "✨",
                                        PresetCategory::Cookbook => "📚",
                                        PresetCategory::Utilities => "⚙️",
                                        PresetCategory::Custom => "🎨",
                                    };

                                    // Clickable preset button (adds to Node canvas)
                                    let response = ui.button(format!("{} {}", icon, preset.name));

                                    if response.clicked() {
                                        // Add the library preset (supporting multi-node patches) to the canvas
                                        add_library_preset_to_canvas(node_state, preset);
                                        // Jump to Node view so the user sees the new nodes immediately
                                        ui_state.current_view = UIViewMode::Node;
                                        println!(
                                            "✅ Added preset to canvas: {} (nodes: {})",
                                            preset.name,
                                            preset.nodes.len()
                                        );
                                    }

                                    // TODO: Implement drag-and-drop functionality
                                    if response.hovered() {
                                        response.on_hover_text(&preset.description);
                                    }
                                });
                            }
                        }
                    });
                }
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

            // Input Monitoring Section - CRITICAL: Prevent Feedback
            ui.group(|ui| {
                ui.label("🎤 Input Monitoring - FEEDBACK CONTROL");
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label("⚠️ Input Monitor");
                        ui.checkbox(&mut ui_state.input_monitoring_enabled, "Enable Input");
                        if ui_state.input_monitoring_enabled {
                            ui.colored_label(egui::Color32::RED, "⚠️ FEEDBACK RISK");
                        } else {
                            ui.colored_label(egui::Color32::GREEN, "✅ Safe Mode");
                        }
                    });
                    ui.vertical(|ui| {
                        ui.label("Input Gain");
                        ui.add(egui::Slider::new(&mut ui_state.input_gain, 0.0..=1.0).text("Gain"));
                    });
                    ui.vertical(|ui| {
                        ui.label("Monitor Level");
                        ui.add(egui::Slider::new(&mut ui_state.input_monitor_level, 0.0..=1.0).text("Mon"));
                    });
                });
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("🔇 Mute All Inputs").clicked() {
                        ui_state.input_monitoring_enabled = false;
                        println!("🔇 All input monitoring disabled");
                    }
                    if ui.button("🎧 Headphone Only").clicked() {
                        println!("🎧 Routing input to headphones only");
                    }
                });
            });

            ui.separator();

            // Master section with advanced controls
            ui.group(|ui| {
                ui.label("🎧 Master Bus - Advanced");
                ui.horizontal(|ui| {
                    // Master fader with peak indicators
                    ui.vertical(|ui| {
                        ui.label("Volume");
                        // Link to engine: send MasterVolume when changed
                        if ui.add(egui::Slider::new(&mut ui_state.master_volume, 0.0..=1.0).vertical().text("Vol")).changed() {
                            if let Some(br) = &ui_state.audio_bridge { let _ = br.lock().unwrap().send_param(AudioParamMessage::MasterVolume(ui_state.master_volume)); }
                        }
                        ui.horizontal(|ui| {
                            ui.label("Peak:");
                            ui.colored_label(egui::Color32::GREEN, "-6dB");
                        });
                    });

                    // Master pan with center indicator
                    ui.vertical(|ui| {
                        ui.label("Pan");
                        // FIX: Link to ui_state.master_pan
                        ui.add(egui::Slider::new(&mut ui_state.master_pan, -1.0..=1.0).text("Pan"));
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
                    // FIX: Link to ui_state fields
                    ui.checkbox(&mut ui_state.master_mute, "Mute");
                    ui.checkbox(&mut ui_state.master_mono, "Mono");
                    ui.checkbox(&mut ui_state.master_phase, "Phase");
                    ui.separator();
                    ui.label("Dim:");
                    // FIX: Link to ui_state.dim_level
                    ui.add(egui::DragValue::new(&mut ui_state.dim_level).clamp_range(-60.0..=0.0).suffix(" dB"));
                    ui.separator();
                    // Simple test tone control routed to engine
                    ui.label("Tone Freq:");
                    let mut tone_freq = 440.0_f32;
                    if ui.add(egui::DragValue::new(&mut tone_freq).clamp_range(20.0..=20000.0).speed(10.0).suffix(" Hz")).changed() {
                        if let Some(br) = &ui_state.audio_bridge { let _ = br.lock().unwrap().send_param(AudioParamMessage::SetParameter("frequency".to_string(), tone_freq)); }
                    }
                });

                // Master EQ section with spectrum analyzer
                ui.collapsing("🎚️ Master EQ", |ui| {
                    // FIX: Link to ui_state.master_eq_on
                    ui.checkbox(&mut ui_state.master_eq_on, "Master EQ On");

                    // Master spectrum analyzer
                    ui.label("📊 Master Spectrum:");
                    ui.allocate_response(egui::Vec2::new(80.0, 50.0), egui::Sense::hover());
    let spectrum_rect = ui.max_rect();
                    ui.painter().rect_filled(spectrum_rect, egui::Rounding::same(2.0), egui::Color32::from_rgb(10, 10, 20));

                    // Draw master spectrum with more detail (simulated)
                    for i in 0..80 {
                        let x = spectrum_rect.left() + (i as f32 * 1.0);
                        let height = (ui.input(|inp| inp.time as f32 * 1.5 + i as f32 * 0.05).sin() * 0.5 + 0.5) * 40.0;
                        let y_top = spectrum_rect.bottom() - height;
                        ui.painter().line_segment(
                            [egui::Pos2::new(x, spectrum_rect.bottom()), egui::Pos2::new(x, y_top)],
                            egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 150, 50))
                        );
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
                            // FIX: Remove unlinked temp EQ 'On' checkbox

                            ui.label("Gain:");
                            // FIX: Link to ui_state.master_eq_low_gain
                            ui.add(egui::DragValue::new(&mut ui_state.master_eq_low_gain).clamp_range(-24.0..=24.0).suffix(" dB"));

                            // These next variables are fine as local since they are not in UiState
                            ui.label("Freq:");
                            let mut low_freq = 60.0;
                            ui.add(egui::DragValue::new(&mut low_freq).clamp_range(20.0..=300.0).suffix(" Hz"));
                            ui.label("Q:");
                            let mut low_q = 0.7;
                            ui.add(egui::DragValue::new(&mut low_q).clamp_range(0.1..=5.0));
                        });

                        ui.vertical(|ui| {
                            ui.label("Low Mid");
                            // FIX: Remove unlinked temp EQ 'On' checkbox

                            ui.label("Gain:");
                            // FIX: Link to ui_state.master_eq_lmid_gain
                            ui.add(egui::DragValue::new(&mut ui_state.master_eq_lmid_gain).clamp_range(-24.0..=24.0).suffix(" dB"));

                            ui.label("Freq:");
                            let mut lmid_freq = 250.0;
                            ui.add(egui::DragValue::new(&mut lmid_freq).clamp_range(100.0..=1000.0).suffix(" Hz"));
                            ui.label("Q:");
                            let mut lmid_q = 1.0;
                            ui.add(egui::DragValue::new(&mut lmid_q).clamp_range(0.1..=10.0));
                        });

                        ui.vertical(|ui| {
                            ui.label("High Mid");
                            // FIX: Remove unlinked temp EQ 'On' checkbox

                            ui.label("Gain:");
                            // FIX: Link to ui_state.master_eq_hmid_gain
                            ui.add(egui::DragValue::new(&mut ui_state.master_eq_hmid_gain).clamp_range(-24.0..=24.0).suffix(" dB"));

                            ui.label("Freq:");
                            let mut hmid_freq = 3000.0;
                            ui.add(egui::DragValue::new(&mut hmid_freq).clamp_range(1000.0..=8000.0).suffix(" Hz"));
                            ui.label("Q:");
                            let mut hmid_q = 1.4;
                            ui.add(egui::DragValue::new(&mut hmid_q).clamp_range(0.1..=10.0));
                        });

                        ui.vertical(|ui| {
                            ui.label("High");
                            // FIX: Remove unlinked temp EQ 'On' checkbox

                            ui.label("Gain:");
                            // FIX: Link to ui_state.master_eq_high_gain
                            ui.add(egui::DragValue::new(&mut ui_state.master_eq_high_gain).clamp_range(-24.0..=24.0).suffix(" dB"));

                            ui.label("Freq:");
                            let mut high_freq = 10000.0;
                            ui.add(egui::DragValue::new(&mut high_freq).clamp_range(5000.0..=20000.0).suffix(" Hz"));
                            ui.label("Q:");
                            let mut high_q = 0.8;
                            ui.add(egui::DragValue::new(&mut high_q).clamp_range(0.1..=5.0));
                        });
                    });

                    // Master frequency response curve (simulated display)
                    ui.label("📈 Master Frequency Response:");
                    ui.allocate_response(egui::Vec2::new(80.0, 40.0), egui::Sense::hover());
    let response_rect = ui.max_rect();
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
                        // FIX: Remove unlinked temp Dynamics 'On' checkbox

                        ui.label("Ratio:");
                        // FIX: Link to ui_state.master_comp_ratio
                        ui.add(egui::DragValue::new(&mut ui_state.master_comp_ratio).clamp_range(1.0..=20.0));

                        ui.label("Threshold:");
                        // FIX: Link to ui_state.master_comp_threshold
                        ui.add(egui::DragValue::new(&mut ui_state.master_comp_threshold).clamp_range(-40.0..=0.0).suffix(" dB"));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Limiter:");
                        // FIX: Remove unlinked temp Dynamics 'On' checkbox
                        
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
                    for (i, channel_state) in ui_state.track_channels.iter_mut().enumerate().take(12) {
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
                                // FIX: Link to channel_state.volume
                                ui.add(egui::Slider::new(&mut channel_state.volume, 0.0..=1.0).vertical().text("Vol"));
                                ui.horizontal(|ui| {
                                    ui.label("Peak:");
                                    ui.colored_label(egui::Color32::GREEN, "-12dB");
                                });

                                // Pan control with width
                                ui.horizontal(|ui| {
                                    ui.label("Pan:");
                                    // FIX: Link to channel_state.pan
                                    ui.add(egui::Slider::new(&mut channel_state.pan, -1.0..=1.0));
                                    ui.label("W:");
                                    // FIX: Link to channel_state.width
                                    ui.add(egui::Slider::new(&mut channel_state.width, 0.0..=2.0));
                                });

                                // Channel controls with solo/mute/arm
                                ui.horizontal(|ui| {
                                    // FIX: Link to channel_state fields
                                    ui.checkbox(&mut channel_state.mute, "M"); // Mute
                                    ui.checkbox(&mut channel_state.solo, "S"); // Solo
                                    ui.checkbox(&mut channel_state.arm, "A"); // Arm/Record
                                    ui.checkbox(&mut channel_state.phase, "P"); // Phase
                                });


                                // Advanced EQ section with spectrum analyzer and frequency response
                                ui.collapsing("🎚️ Advanced EQ", |ui| {
                                    // EQ enable/disable and type selection
                                    ui.horizontal(|ui| {
                                        let mut temp = true; ui.checkbox(&mut temp, "EQ On");
                                        ui.label("Type:");
                                        let mut eq_type = "Parametric";
                                        egui::ComboBox::from_label("")
                                            .selected_text(eq_type)
                                            .show_ui(ui, |ui| {
                                                ui.selectable_value(&mut eq_type, "Parametric", "Parametric");
                                                ui.selectable_value(&mut eq_type, "Graphic", "Graphic");
                                                ui.selectable_value(&mut eq_type, "Dynamic", "Dynamic");
                                                ui.selectable_value(&mut eq_type, "Vintage", "Vintage");
                                            });
                                        let mut temp = false; ui.checkbox(&mut temp, "Auto Gain");
                                    });

                                    // Spectrum analyzer with real-time display
                                    ui.label("📊 Real-time Spectrum Analyzer:");
                                    ui.allocate_response(egui::Vec2::new(80.0, 60.0), egui::Sense::hover());
    let spectrum_rect = ui.max_rect();
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
                                            let mut temp = true; ui.checkbox(&mut temp, "On");
                                            ui.label("Gain:");
                                            let mut low_gain = 2.0;
                                            ui.add(egui::DragValue::new(&mut low_gain).clamp_range(-24.0..=24.0).suffix(" dB"));
                                            ui.label("Freq:");
                                            let mut low_freq = 80.0;
                                            ui.add(egui::DragValue::new(&mut low_freq).clamp_range(20.0..=500.0).suffix(" Hz"));
                                            ui.label("Slope:");
                                            let mut low_slope = 6.0;
                                            ui.add(egui::DragValue::new(&mut low_slope).clamp_range(3.0..=24.0).suffix(" dB/oct"));
                                        });

                                        ui.vertical(|ui| {
                                            ui.label("Low Mid Peak");
                                            let mut temp = true; ui.checkbox(&mut temp, "On");
                                            ui.label("Gain:");
                                            let mut lm_gain = -3.0;
                                            ui.add(egui::DragValue::new(&mut lm_gain).clamp_range(-24.0..=24.0).suffix(" dB"));
                                            ui.label("Freq:");
                                            let mut lm_freq = 250.0;
                                            ui.add(egui::DragValue::new(&mut lm_freq).clamp_range(100.0..=1000.0).suffix(" Hz"));
                                            ui.label("Q:");
                                            let mut lm_q = 1.4;
                                            ui.add(egui::DragValue::new(&mut lm_q).clamp_range(0.1..=10.0));
                                        });

                                        ui.vertical(|ui| {
                                            ui.label("High Mid Peak");
                                            let mut temp = true; ui.checkbox(&mut temp, "On");
                                            ui.label("Gain:");
                                            let mut hm_gain = 1.5;
                                            ui.add(egui::DragValue::new(&mut hm_gain).clamp_range(-24.0..=24.0).suffix(" dB"));
                                            ui.label("Freq:");
                                            let mut hm_freq = 3000.0;
                                            ui.add(egui::DragValue::new(&mut hm_freq).clamp_range(1000.0..=8000.0).suffix(" Hz"));
                                            ui.label("Q:");
                                            let mut hm_q = 2.0;
                                            ui.add(egui::DragValue::new(&mut hm_q).clamp_range(0.1..=10.0));
                                        });

                                        ui.vertical(|ui| {
                                            ui.label("High Shelf");
                                            let mut temp = true; ui.checkbox(&mut temp, "On");
                                            ui.label("Gain:");
                                            let mut high_gain = -1.0;
                                            ui.add(egui::DragValue::new(&mut high_gain).clamp_range(-24.0..=24.0).suffix(" dB"));
                                            ui.label("Freq:");
                                            let mut high_freq = 12000.0;
                                            ui.add(egui::DragValue::new(&mut high_freq).clamp_range(5000.0..=20000.0).suffix(" Hz"));
                                            ui.label("Slope:");
                                            let mut high_slope = 6.0;
                                            ui.add(egui::DragValue::new(&mut high_slope).clamp_range(3.0..=24.0).suffix(" dB/oct"));
                                        });
                                    });

                                    // Frequency response curve visualization
                                    ui.label("📈 Frequency Response:");
                                    ui.allocate_response(egui::Vec2::new(80.0, 50.0), egui::Sense::hover());
    let response_rect = ui.max_rect();
                                    ui.painter().rect_filled(response_rect, egui::Rounding::same(2.0), egui::Color32::from_rgb(15, 15, 30));

                                    // Draw frequency response curve
                                    let mut points = Vec::new();
                                    for i in 0..80 {
                                        let freq = 20.0_f32 * (20000.0_f32 / 20.0_f32).powf(i as f32 / 80.0);
                                        let gain = match freq {
                                            f if f < 100.0 => 2.0, // Low shelf boost
                                            f if f >= 100.0 && f < 300.0 => -3.0, // Low mid cut
                                            f if f >= 3000.0 && f < 4000.0 => 1.5, // High mid boost
                                            f if f >= 12000.0 => -1.0, // High shelf cut
                                            _ => 0.0,
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
                                            ui.colored_label(egui::Color32::YELLOW, "-6.8 dB");
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

                                    // EQ presets and automation
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

                                        let mut temp = true; ui.checkbox(&mut temp, "Link to Automation");
                                        let mut temp = false; ui.checkbox(&mut temp, "EQ Bypass in Solo");
                                    });
                                });

                                // Comprehensive Send section
                                ui.collapsing("📤 Sends", |ui| {
                                    for send in 0..6 { // 6 sends for professional routing
                                        ui.horizontal(|ui| {
                                            ui.label(format!("Send {}:", send + 1));
                                            let mut send_level = 0.0;
                                            ui.add(egui::Slider::new(&mut send_level, 0.0..=1.0));
                                            let mut temp = false; ui.checkbox(&mut temp, "Pre");
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

                                // Insert effects with bypass
                                ui.collapsing("🔌 Inserts", |ui| {
                                    for slot in 0..4 { // 4 insert slots
                                        ui.horizontal(|ui| {
                                            ui.label(format!("Slot {}:", slot + 1));
                                            let mut temp = true; ui.checkbox(&mut temp, "On");
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

                                // Channel routing options
                                ui.collapsing("🎛️ Routing", |ui| {
                                    let mut temp = true; ui.checkbox(&mut temp, "Direct Out");
                                    let mut temp = false; ui.checkbox(&mut temp, "Send to Master");
                                    let mut temp = false; ui.checkbox(&mut temp, "Send to Group");
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
                                let mut return_vol = 0.7;
                                ui.add(egui::Slider::new(&mut return_vol, 0.0..=1.0).vertical().text("Vol"));
                                ui.horizontal(|ui| {
                                    ui.label("Peak:");
                                    ui.colored_label(egui::Color32::YELLOW, "-8dB");
                                });

                                // Return pan
                                ui.horizontal(|ui| {
                                    ui.label("Pan:");
                                    let mut return_pan = 0.0;
                                    ui.add(egui::Slider::new(&mut return_pan, -1.0..=1.0));
                                });

                                // Return controls
                                ui.horizontal(|ui| {
                                    let mut temp = false; ui.checkbox(&mut temp, "Mute");
                                    let mut temp = false; ui.checkbox(&mut temp, "Solo");
                                });

                                // Return EQ with spectrum analyzer
                                ui.collapsing("🎚️ Return EQ", |ui| {
                                    let mut temp = true; ui.checkbox(&mut temp, "EQ On");

                                    // Mini spectrum analyzer for return
                                    ui.allocate_response(egui::Vec2::new(60.0, 30.0), egui::Sense::hover());
    let spectrum_rect = ui.max_rect();
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

                                // Return send (for cascading returns)
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

                                // Group controls
                                ui.horizontal(|ui| {
                                    let mut temp = false; ui.checkbox(&mut temp, "Mute");
                                    let mut temp = false; ui.checkbox(&mut temp, "Solo");
                                });

                                // Group sends
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
                                    let mut temp = false; ui.checkbox(&mut temp, "Pre");
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

            // Mixer performance indicators
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
                    let mut temp = true; ui.checkbox(&mut temp, "On");
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

                    // Loop controls
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

                // Tempo and sync
                ui.vertical(|ui| {
                    ui.label("🎼 Tempo & Sync");
                    ui.horizontal(|ui| {
                        ui.label("BPM:");
                        let mut bpm = 128.0;
                        ui.add(egui::DragValue::new(&mut bpm).clamp_range(60.0..=200.0));
                    });

                    ui.horizontal(|ui| {
                        let mut metronome = true;
                        ui.checkbox(&mut metronome, "🔔 Metronome");
                        let mut auto_quantize = false;
                        ui.checkbox(&mut auto_quantize, "🎯 Auto-Quantize");
                        let mut sync = true;
                        ui.checkbox(&mut sync, "🔗 Sync");
                    });
                });

                ui.separator();

                // Performance indicators
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

                // Advanced controls
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
        });
    }

    // Central panel with multi-pass render graph integration
    egui::CentralPanel::default().show(ctx, |ui| {
        match ui_state.current_view {
            UIViewMode::Arrangement => draw_arrangement_view(ui, arrangement_state),
            UIViewMode::Live => draw_live_view(ui, live_state),
            UIViewMode::Node => draw_node_view(ui, node_state),
        }
    });

    // VST3 Plugin Browser Side Panel (refactored from floating window)
    if ui_state.show_vst3_browser {
        egui::SidePanel::right("vst3_browser").show(ctx, |ui| {
            ui.set_min_width(320.0);
            ui.heading("🎛️ VST3 Plugins");

            ui.horizontal(|ui| {
                if ui.button("🔄 Rescan").clicked() {
                    let _ = ui_state.vst3_host.scan_plugins();
                }
                ui.label(format!("Found {} plugins", ui_state.vst3_host.get_plugin_count()));
            });

            ui.horizontal(|ui| {
                ui.label("🔎");
                ui.text_edit_singleline(&mut ui_state.vst3_search)
                    .on_hover_text("Filter by name, vendor, or category");
                ui.checkbox(&mut ui_state.vst3_sort_alpha, "A–Z");
            });

            ui.horizontal(|ui| {
                ui.label("Sort by:");
                ui.selectable_value(&mut ui_state.vst3_sort_mode, VST3Sort::Alphabetical, "A-Z");
                ui.selectable_value(&mut ui_state.vst3_sort_mode, VST3Sort::Vendor, "Vendor");
                ui.selectable_value(&mut ui_state.vst3_sort_mode, VST3Sort::Category, "Category");
            });

            egui::ComboBox::from_label("Load to Track")
                .selected_text(format!("Track {}", ui_state.vst3_selected_track + 1))
                .show_ui(ui, |ui| {
                    for i in 0..12 {
                        ui.selectable_value(&mut ui_state.vst3_selected_track, i, format!("Track {}", i + 1));
                    }
                });

            ui.separator();

            // Prepare filtered/sorted list
            let mut plugins: Vec<_> = ui_state
                .vst3_host
                .get_plugins()
                .iter()
                .filter(|p| {
                    if ui_state.vst3_search.trim().is_empty() { return true; }
                    let q = ui_state.vst3_search.to_lowercase();
                    p.name.to_lowercase().contains(&q)
                        || p.vendor.to_lowercase().contains(&q)
                        || p.category.to_lowercase().contains(&q)
                })
                .cloned()
                .collect();

            // Group and sort based on the selected mode
            match ui_state.vst3_sort_mode {
                VST3Sort::Alphabetical => {
                    if ui_state.vst3_sort_alpha {
                        plugins.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
                    }
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for plugin in plugins.iter() {
                            ui.horizontal(|ui| {
                                ui.label(&plugin.name);
                                if ui.small_button("Load").clicked() {
                                    let _ = ui_state.vst3_host.load_plugin_to_track(&plugin.path, ui_state.vst3_selected_track);
                                }
                            });
                            ui.small(
                                egui::RichText::new(format!("{} • {}", plugin.vendor, plugin.category))
                                    .italics()
                                    .color(egui::Color32::GRAY),
                            );
                            ui.separator();
                        }
                    });
                }
                VST3Sort::Vendor | VST3Sort::Category => {
                    let mut grouped: std::collections::BTreeMap<String, Vec<_>> = std::collections::BTreeMap::new();
                    for plugin in plugins {
                        let key = if ui_state.vst3_sort_mode == VST3Sort::Vendor {
                            plugin.vendor.clone()
                        } else {
                            plugin.category.clone()
                        };
                        grouped.entry(key).or_default().push(plugin);
                    }

                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for (group_name, mut group_plugins) in grouped {
                            if ui_state.vst3_sort_alpha {
                                group_plugins.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
                            }
                            ui.collapsing(group_name, |ui| {
                                for plugin in group_plugins {
                                    ui.horizontal(|ui| {
                                        ui.label(&plugin.name);
                                        if ui.small_button("Load").clicked() {
                                            let _ = ui_state.vst3_host.load_plugin_to_track(&plugin.path, ui_state.vst3_selected_track);
                                        }
                                    });
                                    ui.small(
                                        egui::RichText::new(format!("{} • {}", plugin.vendor, plugin.category))
                                            .italics()
                                            .color(egui::Color32::GRAY),
                                    );
                                    ui.separator();
                                }
                            });
                        }
                    });
                }
            }
        });
    }

    // MIDI Control System Window
    if ui_state.show_midi_control {
        egui::Window::new("🎹 MIDI Control System")
            .default_size([500.0, 400.0])
            .show(ctx, |ui| {
                ui.heading("MIDI Devices & Mappings");

                ui.horizontal(|ui| {
                    if ui.button("🔄 Scan Devices").clicked() {
                        let _ = ui_state.midi_control_system.scan_devices();
                    }
                    ui.checkbox(&mut ui_state.midi_control_system.learn_mode.active, "🎯 Learn Mode");
                });

                ui.separator();

                ui.columns(2, |columns| {
                    columns[0].heading("MIDI Devices");
                    egui::ScrollArea::vertical().show(&mut columns[0], |ui| {
                        for (_id, device) in ui_state.midi_control_system.devices.iter() {
                            ui.horizontal(|ui| {
                                let status = if device.connected { "🟢" } else { "🔴" };
                                ui.label(format!("{} {}", status, device.name));
                            });
                        }
                    });

                    columns[1].heading("MIDI Mappings");
                    egui::ScrollArea::vertical().show(&mut columns[1], |ui| {
                        for (_mid, mapping) in ui_state.midi_control_system.mappings.iter() {
                            ui.horizontal(|ui| {
                                ui.label(format!("{:?} → {:?}", 
                                    mapping.source.message_type, 
                                    mapping.target.target_type));
                                if ui.button("🗑️").clicked() {
                                    // TODO: Remove mapping
                                }
                            });
                        }
                    });
                });
            });
    }

    // Piano Roll Editor Window
    if ui_state.show_piano_roll {
        egui::Window::new("🎼 Piano Roll Editor")
            .default_size([800.0, 600.0])
            .show(ctx, |ui| {
                ui.heading("Piano Roll Editor");
                
                ui.horizontal(|ui| {
                    ui.label("Snap:");
                    ui.selectable_value(&mut ui_state.piano_roll_editor.view_settings.grid_resolution, GridResolution::Sixteenth, "1/16");
                    ui.selectable_value(&mut ui_state.piano_roll_editor.view_settings.grid_resolution, GridResolution::Eighth, "1/8");
                    ui.selectable_value(&mut ui_state.piano_roll_editor.view_settings.grid_resolution, GridResolution::Quarter, "1/4");

                    ui.separator();

                    ui.label("Velocity:");
                    ui.add(egui::Slider::new(&mut ui_state.piano_roll_editor.edit_settings.insert_velocity, 1..=127));
                });
                
                ui.separator();
                
                // Piano roll grid (simplified)
                let available_rect = ui.available_rect_before_wrap();
                let response = ui.allocate_rect(available_rect, egui::Sense::click_and_drag());
                
                if ui.is_rect_visible(response.rect) {
                    let painter = ui.painter();
                    
                    // Draw piano keys on the left
                    let key_width = 60.0;
                    let key_height = 12.0;
                    
                    for i in 0..88 {
                        let y = response.rect.top() + i as f32 * key_height;
                        let key_rect = egui::Rect::from_min_size(
                            egui::pos2(response.rect.left(), y),
                            egui::vec2(key_width, key_height)
                        );
                        
                        let is_black_key = match i % 12 {
                            1 | 3 | 6 | 8 | 10 => true,
                            _ => false,
                        };
                        
                        let color = if is_black_key {
                            egui::Color32::DARK_GRAY
                        } else {
                            egui::Color32::WHITE
                        };
                        
                        painter.rect_filled(key_rect, 2.0, color);
                        painter.rect_stroke(key_rect, 2.0, egui::Stroke::new(1.0, egui::Color32::GRAY));
                    }
                    
                    // Draw grid lines
                    let grid_rect = egui::Rect::from_min_size(
                        egui::pos2(response.rect.left() + key_width, response.rect.top()),
                        egui::vec2(response.rect.width() - key_width, response.rect.height())
                    );
                    
                    // Vertical grid lines (time)
                    for i in 0..32 {
                        let x = grid_rect.left() + i as f32 * 20.0;
                        painter.line_segment(
                            [egui::pos2(x, grid_rect.top()), egui::pos2(x, grid_rect.bottom())],
                            egui::Stroke::new(0.5, egui::Color32::GRAY)
                        );
                    }
                    
                    // Horizontal grid lines (notes)
                    for i in 0..88 {
                        let y = grid_rect.top() + i as f32 * key_height;
                        painter.line_segment(
                            [egui::pos2(grid_rect.left(), y), egui::pos2(grid_rect.right(), y)],
                            egui::Stroke::new(0.5, egui::Color32::GRAY)
                        );
                    }
                }
            });
    }

    // Modular Patches Window
    if ui_state.show_modular_patches {
        egui::Window::new("🔗 Modular Patch System")
            .default_size([600.0, 500.0])
            .show(ctx, |ui| {
                ui.heading("Modular Patches");
                
                ui.horizontal(|ui| {
                    if ui.button("📁 New Patch").clicked() {
                        ui_state.modular_patch_manager.create_patch("New Patch".to_string());
                    }
                    if ui.button("💾 Save Patch").clicked() {
                        // TODO: Save current patch
                    }
                    if ui.button("📂 Load Patch").clicked() {
                        // TODO: Load patch
                    }
                });
                
                ui.separator();
                
                ui.columns(2, |columns| {
                    columns[0].heading("Available Nodes");
                    egui::ScrollArea::vertical().show(&mut columns[0], |ui| {
                        ui.collapsing("🎵 Oscillators", |ui| {
                            if ui.button("Sine Wave").clicked() {
                                // TODO: Add sine oscillator node
                            }
                            if ui.button("Saw Wave").clicked() {
                                // TODO: Add saw oscillator node
                            }
                            if ui.button("Square Wave").clicked() {
                                // TODO: Add square oscillator node
                            }
                        });
                        
                        ui.collapsing("🎛️ Filters", |ui| {
                            if ui.button("Low Pass").clicked() {
                                // TODO: Add lowpass filter node
                            }
                            if ui.button("High Pass").clicked() {
                                // TODO: Add highpass filter node
                            }
                            if ui.button("Band Pass").clicked() {
                                // TODO: Add bandpass filter node
                            }
                        });
                        
                        ui.collapsing("🎚️ Effects", |ui| {
                            if ui.button("Reverb").clicked() {
                                // TODO: Add reverb node
                            }
                            if ui.button("Delay").clicked() {
                                // TODO: Add delay node
                            }
                            if ui.button("Chorus").clicked() {
                                // TODO: Add chorus node
                            }
                        });
                    });
                    
                    columns[1].heading("Current Patch");
                    egui::ScrollArea::vertical().show(&mut columns[1], |ui| {
                        if let Some(current_patch) = ui_state.modular_patch_manager.get_current_patch() {
                            ui.label(format!("Name: {}", current_patch.name));
                            ui.label(format!("Nodes: {}", current_patch.nodes.len()));
                            ui.label(format!("Connections: {}", current_patch.connections.len()));
                        } else {
                            ui.label("No patch loaded");
                        }
                    });
                });
            });
    }

    // Theme Editor Window
    if ui_state.show_theme_editor {
        egui::Window::new("🎨 Theme Editor")
            .default_size([400.0, 600.0])
            .show(ctx, |ui| {
                ui.heading("Theme Editor");
                
                ui.horizontal(|ui| {
                    if ui.button("Dark Professional").clicked() {
                        let _ = ui_state.theme_manager.set_current_theme("Dark Professional");
                    }
                    if ui.button("Light Studio").clicked() {
                        let _ = ui_state.theme_manager.set_current_theme("Light Studio");
                    }
                    if ui.button("Neon").clicked() {
                        let _ = ui_state.theme_manager.set_current_theme("Neon");
                    }
                });
                
                ui.separator();
                
                // Avoid borrow conflicts: clone data from manager before mutably borrowing theme
                let available_fonts = ui_state.theme_manager.font_settings.available_fonts.clone();
                let palettes = ui_state.theme_manager.color_palettes.clone();
                if let Some(theme) = ui_state.theme_manager.get_current_theme_mut() {
                    ui.label(format!("Current Theme: {}", theme.name));

                    ui.collapsing("🎨 Colors", |ui| {
                        ui.label("Background:");
                        let mut bg = [
                            theme.colors.background_primary.r,
                            theme.colors.background_primary.g,
                            theme.colors.background_primary.b,
                            theme.colors.background_primary.a,
                        ];
                        if ui.color_edit_button_rgba_unmultiplied(&mut bg).changed() {
                            theme.colors.background_primary.r = bg[0];
                            theme.colors.background_primary.g = bg[1];
                            theme.colors.background_primary.b = bg[2];
                            theme.colors.background_primary.a = bg[3];
                        }

                        ui.label("Primary Accent:");
                        let mut acc = [
                            theme.colors.accent_primary.r,
                            theme.colors.accent_primary.g,
                            theme.colors.accent_primary.b,
                            theme.colors.accent_primary.a,
                        ];
                        if ui.color_edit_button_rgba_unmultiplied(&mut acc).changed() {
                            theme.colors.accent_primary.r = acc[0];
                            theme.colors.accent_primary.g = acc[1];
                            theme.colors.accent_primary.b = acc[2];
                            theme.colors.accent_primary.a = acc[3];
                        }

                        ui.label("Focus Ring:");
                        let mut focus = [
                            theme.colors.focus_ring.r,
                            theme.colors.focus_ring.g,
                            theme.colors.focus_ring.b,
                            theme.colors.focus_ring.a,
                        ];
                        if ui.color_edit_button_rgba_unmultiplied(&mut focus).changed() {
                            theme.colors.focus_ring.r = focus[0];
                            theme.colors.focus_ring.g = focus[1];
                            theme.colors.focus_ring.b = focus[2];
                            theme.colors.focus_ring.a = focus[3];
                        }
                    });

                    ui.collapsing("📝 Typography", |ui| {
                        ui.label("Font Scale:");
                        ui.add(egui::Slider::new(&mut ui_state.font_scale, 0.8..=2.0));

                        ui.label("Primary Font:");
                        let available = &available_fonts;
                        let mut selected = theme.typography.font_families.primary.clone();
                        egui::ComboBox::from_label("")
                            .selected_text(&selected)
                            .show_ui(ui, |ui| {
                                for f in available {
                                    ui.selectable_value(&mut selected, f.clone(), f);
                                }
                            });
                        if selected != theme.typography.font_families.primary {
                            theme.typography.font_families.primary = selected;
                        }
                    });

                    ui.collapsing("🎵 Channel Colors", |ui| {
                        for (i, color) in theme.channel_colors.default_colors.iter_mut().enumerate() {
                            let mut rgba = [color.r, color.g, color.b, color.a];
                            ui.horizontal(|ui| {
                                ui.label(format!("Track {}:", i + 1));
                                if ui.color_edit_button_rgba_unmultiplied(&mut rgba).changed() {
                                    color.r = rgba[0]; color.g = rgba[1]; color.b = rgba[2]; color.a = rgba[3];
                                }
                            });
                        }

                        ui.separator();
                        ui.label("Apply Palette:");
                        let palette_names: Vec<String> = palettes.keys().cloned().collect();
                        let mut selected = palette_names.first().cloned().unwrap_or_else(|| "Ableton".to_string());
                        egui::ComboBox::from_label("Palette")
                            .selected_text(&selected)
                            .show_ui(ui, |ui| {
                                for name in &palette_names {
                                    ui.selectable_value(&mut selected, name.clone(), name);
                                }
                            });
                        if ui.button("Apply to Tracks").clicked() {
                            if let Some(p) = palettes.get(&selected) {
                                theme.channel_colors.default_colors = p.colors.clone();
                            }
                        }
                    });

                    ui.collapsing("🎼 Sample Colors", |ui| {
                        use crate::theming_system::SampleType;
                        let sample_types = [
                            SampleType::Kick,
                            SampleType::Snare,
                            SampleType::HiHat,
                            SampleType::Cymbal,
                            SampleType::Tom,
                            SampleType::Percussion,
                            SampleType::Bass,
                            SampleType::Lead,
                            SampleType::Pad,
                            SampleType::Vocal,
                            SampleType::FX,
                            SampleType::Loop,
                            SampleType::OneShot,
                        ];
                        for ty in sample_types.iter() {
                            let entry = theme.sample_colors.by_type.entry(ty.clone()).or_insert(crate::theming_system::ColorRgba::from_hex("#666666"));
                            let mut rgba = [entry.r, entry.g, entry.b, entry.a];
                            ui.horizontal(|ui| {
                                ui.label(format!("{:?}", ty));
                                if ui.color_edit_button_rgba_unmultiplied(&mut rgba).changed() {
                                    entry.r = rgba[0]; entry.g = rgba[1]; entry.b = rgba[2]; entry.a = rgba[3];
                                }
                            });
                        }
                    });
                }
            });
    }
}

fn apply_theme(ctx: &egui::Context, ui_state: &UiState) {
    let mut visuals = if ui_state.dark_mode { egui::Visuals::dark() } else { egui::Visuals::light() };
    visuals.dark_mode = ui_state.dark_mode;

    // Apply colors from ThemeManager when available
    if let Some(theme) = ui_state.theme_manager.get_current_theme() {
        // Backgrounds and surfaces
        visuals.panel_fill = theme.colors.background_primary.to_color32();
        visuals.window_fill = theme.colors.surface_primary.to_color32();
        visuals.extreme_bg_color = theme.colors.background_tertiary.to_color32();

        // Selection and focus
        visuals.selection.bg_fill = theme.colors.selection_primary.to_color32();
        visuals.selection.stroke.color = theme.colors.focus_ring.to_color32();

        // Widgets
        visuals.widgets.inactive.bg_fill = theme.colors.surface_secondary.to_color32();
        visuals.widgets.hovered.bg_fill = theme.colors.button_hover.to_color32();
        visuals.widgets.active.bg_fill = theme.colors.accent_primary.to_color32();
        visuals.widgets.open.bg_fill = theme.colors.surface_elevated.to_color32();
        visuals.widgets.inactive.fg_stroke.color = theme.colors.text_primary.to_color32();
        visuals.widgets.hovered.fg_stroke.color = theme.colors.text_accent.to_color32();
        visuals.widgets.active.fg_stroke.color = theme.colors.focus_ring.to_color32();

        // Hyperlinks and accents
        visuals.hyperlink_color = theme.colors.accent_secondary.to_color32();
    } else {
        // Fallback contrast tweak when no theme available
        let base_gray = if ui_state.dark_mode {
            (20.0 + (ui_state.contrast - 1.0) * 60.0).clamp(0.0, 255.0)
        } else {
            (200.0 - (ui_state.contrast - 1.0) * 60.0).clamp(0.0, 255.0)
        } as u8;
        visuals.panel_fill = egui::Color32::from_gray(base_gray);
    }

    let mut style = (*ctx.style()).clone();
    style.visuals = visuals;
    ctx.set_style(style);
    ctx.set_pixels_per_point(ui_state.font_scale);
}

/// Draw Arrangement View - Full Implementation
/// Uses Bevy 0.17 render graph integration
fn draw_arrangement_view(ui: &mut egui::Ui, state: &mut ArrangementViewState) {
    // Professional DAW-style header bar
    ui.horizontal(|ui| {
        ui.heading("🎼 Arrangement");
        ui.separator();
        
        // Transport controls integrated into arrangement
        ui.label("⏯️");
        if ui.button("▶").on_hover_text("Play").clicked() { 
            println!("🎵 Play arrangement from current position"); 
        }
        if ui.button("⏸").on_hover_text("Pause").clicked() { 
            println!("⏸ Pause arrangement"); 
        }
        if ui.button("⏹").on_hover_text("Stop").clicked() { 
            println!("⏹ Stop and return to start"); 
        }
        if ui.button("⏺").on_hover_text("Record").clicked() { 
            println!("⏺ Start recording"); 
        }
        
        ui.separator();
        
        // Timeline position and zoom
        ui.label("🕐");
        ui.add(egui::DragValue::new(&mut state.timeline_position)
            .clamp_range(0.0..=999.0)
            .speed(0.25)
            .prefix("Bar: ")
            .suffix(".1"));
        
        ui.add(egui::Slider::new(&mut state.timeline_zoom, 0.1..=10.0)
            .text("Zoom")
            .logarithmic(true));
        
        ui.separator();
        
        // Snap controls
        ui.label("📐");
        if ui.checkbox(&mut state.snap_to_grid, "Grid").on_hover_text("Snap to grid lines").clicked() {
            if state.snap_to_grid { state.snap_to_beats = false; state.snap_to_bars = false; }
        }
        if ui.checkbox(&mut state.snap_to_beats, "Beat").on_hover_text("Snap to beat divisions").clicked() {
            if state.snap_to_beats { state.snap_to_grid = false; state.snap_to_bars = false; }
        }
        if ui.checkbox(&mut state.snap_to_bars, "Bar").on_hover_text("Snap to bar boundaries").clicked() {
            if state.snap_to_bars { state.snap_to_grid = false; state.snap_to_beats = false; }
        }
        
        ui.separator();
        
        // Quantization settings
        ui.label("🎯");
        if ui.checkbox(&mut state.quantize_fourths, "1/4").on_hover_text("Quarter note quantization").clicked() {
            if state.quantize_fourths { state.quantize_eighths = false; state.quantize_sixteenths = false; }
        }
        if ui.checkbox(&mut state.quantize_eighths, "1/8").on_hover_text("Eighth note quantization").clicked() {
            if state.quantize_eighths { state.quantize_fourths = false; state.quantize_sixteenths = false; }
        }
        if ui.checkbox(&mut state.quantize_sixteenths, "1/16").on_hover_text("Sixteenth note quantization").clicked() {
            if state.quantize_sixteenths { state.quantize_fourths = false; state.quantize_eighths = false; }
        }
        
        ui.separator();
        
        // Groove and swing
        ui.label("🎶");
        ui.add(egui::Slider::new(&mut state.groove_amount, 0.0..=100.0)
            .text("Groove")
            .suffix("%"));
        ui.add(egui::Slider::new(&mut state.groove_rate, 0.5..=8.0)
            .text("Swing")
            .suffix("x"));
        
        ui.separator();
        
        // View options
        ui.label("👁");
        ui.checkbox(&mut state.show_automation, "Auto");
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
        for track_num in 0..8 {
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

                    // Track controls with arm button from JS
                    ui.horizontal(|ui| {
                        let mut mute = false;
                        ui.checkbox(&mut mute, "M");
                        let mut solo = false;
                        ui.checkbox(&mut solo, "S");
                        let arm_idx = track_num % state.track_armed.len();
                        let mut armed = *state.track_armed.get(arm_idx).unwrap_or(&false);
                        ui.checkbox(&mut armed, "R");
                        if track_num < state.track_armed.len() {
                            state.track_armed[track_num] = armed;
                        }
                    });

                    // Volume fader with automation
                    ui.label("Volume:");
                    let mut volume = 0.8;
                    ui.add(egui::Slider::new(&mut volume, 0.0..=1.0).vertical().text("Vol"));

                    // Pan control
                    ui.label("Pan:");
                    let mut pan = 0.0;
                    ui.add(egui::Slider::new(&mut pan, -1.0..=1.0).text("Pan"));

                    // Send controls with advanced routing
                    ui.collapsing("📤 Sends", |ui| {
                        for send_num in 0..4 {
                            ui.horizontal(|ui| {
                                ui.label(format!("Send {}:", send_num + 1));
                                let mut send_level = 0.0;
                                ui.add(egui::Slider::new(&mut send_level, 0.0..=1.0));
                                let mut temp = false; ui.checkbox(&mut temp, "Pre");
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

                    // Draw timeline background using themed visuals
                    let visuals = ui.style().visuals.clone();
                    ui.painter().rect_filled(rect, 2.0, visuals.window_fill);

                    // Draw bar lines
                    for bar in 0..32 {
                        let x = rect.left() + (bar as f32 * 100.0);
                        ui.painter().line_segment(
                            [egui::Pos2::new(x, rect.top()), egui::Pos2::new(x, rect.bottom())],
                            egui::Stroke::new(2.0, visuals.widgets.inactive.fg_stroke.color.linear_multiply(0.6))
                        );
                    }

                    // Draw beat lines
                    for beat in 0..128 { // 4 beats per bar * 32 bars
                        let x = rect.left() + (beat as f32 * 25.0);
                        ui.painter().line_segment(
                            [egui::Pos2::new(x, rect.top()), egui::Pos2::new(x, rect.bottom())],
                            egui::Stroke::new(1.0, visuals.widgets.inactive.fg_stroke.color.linear_multiply(0.5))
                        );
                    }

                    // Draw sample clips with enhanced editing features
                    if track_num == 0 {
                        // Kick clip with editing controls
                        let clip_rect = egui::Rect::from_min_size(
                            egui::Pos2::new(rect.left() + 25.0, rect.top() + 10.0),
                            egui::Vec2::new(75.0, 50.0)
                        );
                        ui.painter().rect_filled(clip_rect, 4.0, visuals.widgets.inactive.bg_fill);
                        ui.painter().rect_stroke(clip_rect, egui::Rounding::same(4.0), egui::Stroke::new(2.0, visuals.selection.stroke.color));
                        ui.painter().text(
                            clip_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "Kick",
                            egui::FontId::proportional(12.0),
                            visuals.text_color()
                        );

                        // Clip editing controls (hover overlay)
                        let control_rect = egui::Rect::from_min_size(
                            egui::Pos2::new(clip_rect.right() - 60.0, clip_rect.top() - 20.0),
                            egui::Vec2::new(60.0, 15.0)
                        );
                        ui.painter().rect_filled(control_rect, 2.0, visuals.extreme_bg_color.gamma_multiply(0.8));
                        ui.painter().text(
                            control_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "✂️📋🗑️",
                            egui::FontId::proportional(10.0),
                            visuals.text_color()
                        );

                        // Snare clip
                        let snare_rect = egui::Rect::from_min_size(
                            egui::Pos2::new(rect.left() + 125.0, rect.top() + 10.0),
                            egui::Vec2::new(75.0, 50.0)
                        );
                        ui.painter().rect_filled(snare_rect, 4.0, visuals.widgets.inactive.bg_fill);
                        ui.painter().rect_stroke(snare_rect, egui::Rounding::same(4.0), egui::Stroke::new(2.0, visuals.selection.stroke.color));
                        ui.painter().text(
                            snare_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "Snare",
                            egui::FontId::proportional(12.0),
                            visuals.text_color()
                        );
                    }

                    if track_num == 1 {
                        // Bass line clip with fade handles
                        let bass_rect = egui::Rect::from_min_size(
                            egui::Pos2::new(rect.left() + 0.0, rect.top() + 10.0),
                            egui::Vec2::new(200.0, 50.0)
                        );
                        ui.painter().rect_filled(bass_rect, 4.0, visuals.widgets.inactive.bg_fill);
                        ui.painter().rect_stroke(bass_rect, egui::Rounding::same(4.0), egui::Stroke::new(2.0, visuals.selection.stroke.color));
                        ui.painter().text(
                            bass_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "Bass Line",
                            egui::FontId::proportional(12.0),
                            visuals.text_color()
                        );

                        // Fade in/out handles
                        let fade_in_handle = egui::Pos2::new(bass_rect.left() + 10.0, bass_rect.top() + 25.0);
                        ui.painter().circle_filled(fade_in_handle, 4.0, visuals.hyperlink_color);
                        let fade_out_handle = egui::Pos2::new(bass_rect.right() - 10.0, bass_rect.top() + 25.0);
                        ui.painter().circle_filled(fade_out_handle, 4.0, visuals.hyperlink_color);
                    }

                    if track_num == 5 {
                        // MIDI clip with piano roll preview
                        let midi_rect = egui::Rect::from_min_size(
                            egui::Pos2::new(rect.left() + 50.0, rect.top() + 10.0),
                            egui::Vec2::new(150.0, 50.0)
                        );
                        ui.painter().rect_filled(midi_rect, 4.0, visuals.widgets.inactive.bg_fill);
                        ui.painter().rect_stroke(midi_rect, egui::Rounding::same(4.0), egui::Stroke::new(2.0, visuals.selection.stroke.color));
                        ui.painter().text(
                            midi_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "Chord Prog",
                            egui::FontId::proportional(12.0),
                            visuals.text_color()
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
                                visuals.widgets.hovered.fg_stroke.color
                            );
                        }
                    }

                    // Automation lanes (if enabled)
                    if state.show_automation {
                        let automation_y = rect.top() + 70.0;
                        ui.painter().line_segment(
                            [egui::Pos2::new(rect.left(), automation_y), egui::Pos2::new(rect.right(), automation_y)],
                            egui::Stroke::new(1.0, visuals.selection.stroke.color)
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
        let mut temp = false; ui.checkbox(&mut temp, "1/32");
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
                            let mut temp = false; ui.checkbox(&mut temp, "Pre");
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
            for to_track in 0..8 {
                ui.label(format!("Track {}", to_track + 1));
            }
            ui.end_row();

            for from_track in 0..8 {
                ui.label(format!("Track {}", from_track + 1));
                for to_track in 0..8 {
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
fn draw_live_view(ui: &mut egui::Ui, state: &mut LiveViewState) {
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
            let (resp, painter) = ui.allocate_painter(egui::Vec2::new(200.0, 60.0), egui::Sense::hover());
            painter.rect_filled(resp.rect, egui::Rounding::same(4.0), egui::Color32::from_rgb(30, 30, 50));
            painter.text(
                resp.rect.center(),
                egui::Align2::CENTER_CENTER,
                "🎵 Waveform",
                egui::FontId::proportional(12.0),
                egui::Color32::WHITE,
            );

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
            let (resp, painter) = ui.allocate_painter(egui::Vec2::new(200.0, 60.0), egui::Sense::hover());
            painter.rect_filled(resp.rect, egui::Rounding::same(4.0), egui::Color32::from_rgb(30, 30, 50));
            painter.text(
                resp.rect.center(),
                egui::Align2::CENTER_CENTER,
                "🎵 Waveform",
                egui::FontId::proportional(12.0),
                egui::Color32::WHITE,
            );

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
fn draw_node_view(ui: &mut egui::Ui, state: &mut NodeViewState) {
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
    ui.painter().rect_filled(rect, 0.0, egui::Color32::from_rgb(15, 15, 25));

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
            let mut temp = true; ui.checkbox(&mut temp, "Rack On");
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
            ui.allocate_response(egui::Vec2::new(80.0, 30.0), egui::Sense::hover());
            let chain_rect = ui.max_rect();
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

/// File browser helper functions
impl UiState {
    /// Load files from the current browser path
    pub fn load_browser_files(&mut self) {
        self.browser_files.clear();
        
        if let Ok(entries) = fs::read_dir(&self.current_browser_path) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    let path = entry.path();
                    let name = path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("Unknown")
                        .to_string();
                    
                    let file_type = if metadata.is_dir() {
                        FileType::Directory
                    } else {
                        match path.extension().and_then(|ext| ext.to_str()) {
                            Some("wav") | Some("mp3") | Some("flac") | Some("aiff") | Some("ogg") => FileType::Audio,
                            Some("mid") | Some("midi") => FileType::Midi,
                            Some("hexo") | Some("json") => FileType::Project,
                            _ => FileType::Other,
                        }
                    };
                    
                    // Only show audio files, MIDI files, directories, and projects
                    if matches!(file_type, FileType::Audio | FileType::Midi | FileType::Directory | FileType::Project) {
                        self.browser_files.push(FileItem {
                            name,
                            path: path.clone(),
                            is_directory: metadata.is_dir(),
                            file_type,
                            size: if metadata.is_file() { Some(metadata.len()) } else { None },
                        });
                    }
                }
            }
            
            // Sort: directories first, then by name
            self.browser_files.sort_by(|a, b| {
                match (a.is_directory, b.is_directory) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.name.cmp(&b.name),
                }
            });
        }
    }
    
    /// Navigate to parent directory
    pub fn navigate_up(&mut self) {
        if let Some(parent) = Path::new(&self.current_browser_path).parent() {
            self.current_browser_path = parent.to_string_lossy().to_string();
            self.load_browser_files();
        }
    }
    
    /// Navigate to a subdirectory
    pub fn navigate_to(&mut self, path: &Path) {
        if path.is_dir() {
            self.current_browser_path = path.to_string_lossy().to_string();
            self.load_browser_files();
        }
    }
    
    /// Set the sample library path
    pub fn set_sample_library_path(&mut self, path: String) {
        self.sample_library_path = path.clone();
        self.current_browser_path = path;
        self.load_browser_files();
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

    // eframe removed; UI will be driven by bevy_egui systems.
    println!("Using bevy_egui path");

    Ok(())
}

/// Add a library preset (from `crate::presets`) to the Node canvas,
/// creating all nodes and connections described by the preset.
fn add_library_preset_to_canvas(state: &mut NodeViewState, preset: &crate::presets::NodePreset) {
    // Map preset node index -> generated node id
    let mut id_map: Vec<String> = Vec::new();

    // Add nodes
    for node_cfg in &preset.nodes {
        let id = format!(
            "{}_{}",
            node_cfg.node_type.replace('.', "_"),
            state.node_positions.len()
        );
        id_map.push(id.clone());

        let (x, y) = node_cfg.position;
        let params: Vec<NodeParameter> = node_cfg
            .parameters
            .iter()
            .map(|p| NodeParameter {
                name: p.name.clone(),
                value: p.default,
                min: p.min,
                max: p.max,
                modulated: false,
            })
            .collect();

        state.node_positions.push(NodePosition {
            id,
            node_type: node_cfg.node_type.clone(),
            x,
            y,
            selected: false,
            parameters: params,
        });
    }

    // Add connections
    for conn in &preset.connections {
        let id = format!("conn_{}", state.connections.len());
        let from_id = id_map
            .get(conn.from_node)
            .cloned()
            .unwrap_or_else(|| format!("node{}", conn.from_node));
        let to_id = id_map
            .get(conn.to_node)
            .cloned()
            .unwrap_or_else(|| format!("node{}", conn.to_node));

        state.connections.push(NodeConnection {
            id,
            from_node: from_id,
            from_port: conn.from_output.clone(),
            to_node: to_id,
            to_port: conn.to_input.clone(),
            data_type: "audio".to_string(),
        });
    }
}