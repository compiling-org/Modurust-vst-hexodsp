/// HexoDSP Audio Engine - Real-time audio processing core
/// 
/// This module provides the foundation for low-latency, real-time audio processing
/// with proper thread safety and modern DAW capabilities.

pub mod cpal_io;
pub mod dsp_core;
pub mod node_graph;
pub mod transport;
pub mod bridge;

use cpal_io::AudioIO;
use node_graph::NodeGraph;
use transport::Transport;
use bridge::{AudioEngineBridge, AudioParamMessage, AudioEngineState};

/// Main audio engine structure that coordinates all audio processing
pub struct HexoDSPEngine {
    /// Real-time audio I/O system
    pub audio_io: AudioIO,
    
    /// DSP processing core
    pub dsp_core: dsp_core::DSPCore,
    
    /// Node-based audio processing graph
    pub node_graph: NodeGraph,
    
    /// Transport and timing system
    pub transport: Transport,
    
    /// Bridge for UI communication
    pub bridge: AudioEngineBridge,
    
    /// Audio processing state
    pub processing_state: AudioEngineState,
}

impl HexoDSPEngine {
    /// Create a new audio engine with default settings
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let audio_io = AudioIO::new()?;
        let dsp_core = dsp_core::DSPCore::new(audio_io.sample_rate(), audio_io.buffer_size());
        let node_graph = NodeGraph::new();
        let transport = Transport::new(audio_io.sample_rate());
        let bridge = AudioEngineBridge::new();
        
        Ok(Self {
            audio_io,
            dsp_core,
            node_graph,
            transport,
            bridge,
            processing_state: AudioEngineState::default(),
        })
    }
    
    /// Start the audio engine with real-time processing
    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🎵 Starting HexoDSP Audio Engine...");
        println!("Sample Rate: {} Hz", self.audio_io.sample_rate());
        println!("Buffer Size: {} samples", self.audio_io.buffer_size());
        
        // Initialize audio processing pipeline
        self.setup_default_graph()?;
        
        // Start the audio I/O system with a dummy callback
        // TODO: Implement proper audio processing callback that uses process_audio()
        self.audio_io.start(|_data, _info| {
            // Placeholder callback - actual processing will be integrated later
        })?;
        
        println!("✅ HexoDSP Audio Engine started successfully");
        Ok(())
    }
    
    /// Stop the audio engine
    pub fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🛑 Stopping HexoDSP Audio Engine...");
        self.audio_io.stop()?;
        Ok(())
    }
    
    /// Set up the default audio processing graph
    fn setup_default_graph(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Create default nodes: Oscillator -> Filter -> Output
        let osc_id = self.node_graph.add_oscillator();
        let filter_id = self.node_graph.add_filter();
        let output_id = self.node_graph.add_output();
        
        // Connect oscillator to filter
        self.node_graph.connect(osc_id, filter_id, "audio_out", "audio_in")?;
        
        // Connect filter to output
        self.node_graph.connect(filter_id, output_id, "audio_out", "audio_in")?;
        
        println!("🎛️ Default audio graph initialized");
        Ok(())
    }
    
    /// Process audio buffer (called from real-time audio thread)
    pub fn process_audio(&mut self, input_buffer: &[f32], output_buffer: &mut [f32]) {
        // Update transport/timing
        self.transport.update();
        
        // Process UI messages
        while let Some(message) = self.bridge.receive_param() {
            self.handle_param_message(message);
        }
        
        // Process the node graph
        self.node_graph.process(input_buffer, output_buffer);
        
        // Update audio state for UI feedback
        self.update_audio_state();
        
        // Send state back to UI
        if self.bridge.should_send_feedback() {
            let _ = self.bridge.send_feedback(self.processing_state.clone());
        }
    }
    
    /// Handle parameter changes from UI
    fn handle_param_message(&mut self, message: AudioParamMessage) {
        match message {
            AudioParamMessage::SetTempo(bpm) => {
                self.transport.set_tempo(bpm);
            }
            AudioParamMessage::Play => {
                self.transport.play();
            }
            AudioParamMessage::Stop => {
                self.transport.stop();
            }
            AudioParamMessage::Record => {
                self.transport.record();
            }
            AudioParamMessage::MasterVolume(volume) => {
                self.node_graph.set_master_volume(volume);
            }
            AudioParamMessage::TrackVolume(track, volume) => {
                self.node_graph.set_track_volume(track, volume);
            }
            _ => {
                println!("🔧 Received parameter: {:?}", message);
            }
        }
    }
    
    /// Update audio processing state for UI feedback
    fn update_audio_state(&mut self) {
        // Update master peak level
        self.processing_state.master_peak = self.node_graph.get_master_peak();
        
        // Update track peak levels
        self.processing_state.track_peaks = self.node_graph.get_track_peaks();
        
        // Update spectrum data
        self.processing_state.spectrum_data = self.node_graph.get_spectrum_data();
        
        // Update transport state
        self.processing_state.playing = self.transport.is_playing();
        self.processing_state.bpm = self.transport.bpm();
        self.processing_state.time_position = self.transport.time_position();
    }
}

impl Default for HexoDSPEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create audio engine")
    }
}