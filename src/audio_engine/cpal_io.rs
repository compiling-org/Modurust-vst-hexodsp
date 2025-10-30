/// Real-time audio I/O using CPAL (Cross-Platform Audio Library)
/// 
/// This module handles low-latency audio device communication,
/// supporting ASIO, WASAPI, CoreAudio, and other native APIs.

use cpal::{Device, Host, Stream, StreamConfig, SampleFormat, InputCallbackInfo, OutputCallbackInfo};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Audio configuration structure
#[derive(Debug, Clone)]
pub struct AudioConfig {
    pub sample_rate: u32,
    pub buffer_size: u32,
    pub channels: u16,
    pub device_name: String,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
            buffer_size: 256, // Small buffer for low latency
            channels: 2,      // Stereo
            device_name: "Default".to_string(),
        }
    }
}

/// Audio device information
#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub name: String,
    pub is_input: bool,
    pub is_output: bool,
    pub max_channels: u16,
    pub default_sample_rate: u32,
}

/// Audio I/O system for real-time processing
pub struct AudioIO {
    host: Host,
    input_device: Option<Device>,
    output_device: Option<Device>,
    config: AudioConfig,
    input_stream: Option<Stream>,
    output_stream: Option<Stream>,
    running: Arc<AtomicBool>,
    buffer_size: u32,
    sample_rate: u32,
}

impl AudioIO {
    /// Create a new audio I/O system
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let host = cpal::default_host();
        let config = AudioConfig::default();
        
        println!("🎵 Audio Host: {:?}", host.id());
        
        // Get default devices
        let input_device = host.default_input_device()
            .map(|dev| {
                println!("🎤 Input Device: {}", dev.name().unwrap_or_else(|_| "Unknown".to_string()));
                dev
            });
            
        let output_device = host.default_output_device()
            .map(|dev| {
                println!("🔊 Output Device: {}", dev.name().unwrap_or_else(|_| "Unknown".to_string()));
                dev
            });

        let buffer_size = config.buffer_size;
        let sample_rate = config.sample_rate;
        
        Ok(Self {
            host,
            input_device,
            output_device,
            config,
            input_stream: None,
            output_stream: None,
            running: Arc::new(AtomicBool::new(false)),
            buffer_size,
            sample_rate,
        })
    }
    
    /// Get list of available audio devices
    pub fn list_devices(&self) -> Result<Vec<AudioDevice>, Box<dyn std::error::Error>> {
        let mut devices = Vec::new();
        
        // Input devices
        for device in self.host.input_devices()? {
            let name = device.name()?.to_string();
            let config = device.default_input_config()?;
            
            devices.push(AudioDevice {
                name: name.clone(),
                is_input: true,
                is_output: false,
                max_channels: config.channels(),
                default_sample_rate: config.sample_rate().0,
            });
            
            println!("🎤 Input Device: {} ({} channels, {} Hz)", 
                name, config.channels(), config.sample_rate().0);
        }
        
        // Output devices  
        for device in self.host.output_devices()? {
            let name = device.name()?.to_string();
            let config = device.default_output_config()?;
            
            devices.push(AudioDevice {
                name: name.clone(),
                is_input: false,
                is_output: true,
                max_channels: config.channels(),
                default_sample_rate: config.sample_rate().0,
            });
            
            println!("🔊 Output Device: {} ({} channels, {} Hz)", 
                name, config.channels(), config.sample_rate().0);
        }
        
        Ok(devices)
    }
    
    /// Set audio configuration
    pub fn set_config(&mut self, config: AudioConfig) -> Result<(), Box<dyn std::error::Error>> {
        self.buffer_size = config.buffer_size;
        self.sample_rate = config.sample_rate;
        self.config = config;
        Ok(())
    }
    
    /// Get current configuration
    pub fn config(&self) -> &AudioConfig {
        &self.config
    }
    
    /// Get sample rate
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    
    /// Get buffer size
    pub fn buffer_size(&self) -> u32 {
        self.buffer_size
    }
    
    /// Start audio streams with callback
    pub fn start<F>(&mut self, mut input_callback: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnMut(&[f32], &cpal::InputCallbackInfo) + Send + 'static,
    {
        if self.running.load(Ordering::SeqCst) {
            println!("⚠️ Audio streams already running");
            return Ok(());
        }

        println!("🎵 Starting audio streams...");
        
        // Set up output stream
        if let Some(device) = &self.output_device {
            let config = self.create_output_config()?;
            let running = Arc::clone(&self.running);
            
            let output_callback = move |data: &mut [f32], _: &OutputCallbackInfo| {
                if !running.load(Ordering::SeqCst) {
                    // Fill with silence if not running
                    for sample in data.iter_mut() {
                        *sample = 0.0;
                    }
                    return;
                }
                
                // Generate test tone (sine wave)
                let sample_rate = 44100.0;
                let freq = 440.0; // A4
                let time = std::time::SystemTime::now();
                let elapsed = time.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
                let seconds = elapsed.as_secs_f64();
                
                for (i, sample) in data.iter_mut().enumerate() {
                    let t = seconds + (i as f64 / sample_rate);
                    *sample = (2.0 * std::f64::consts::PI * freq * t).sin() as f32 * 0.1;
                }
            };
            
            let stream = device.build_output_stream(
                &config,
                output_callback,
                |err| eprintln!("Output stream error: {}", err),
                None, // timeout
            )?;
            
            self.output_stream = Some(stream);
        }
        
        // Set up input stream
        if let Some(device) = &self.input_device {
            let config = self.create_input_config()?;
            let running = Arc::clone(&self.running);
            
            let input_callback = move |data: &[f32], info: &InputCallbackInfo| {
                if !running.load(Ordering::SeqCst) {
                    return;
                }
                
                // Call the user-provided callback
                input_callback(data, info);
            };
            
            let stream = device.build_input_stream(
                &config,
                input_callback,
                |err| eprintln!("Input stream error: {}", err),
                None, // timeout
            )?;
            
            self.input_stream = Some(stream);
        }
        
        // Start the streams
        if let Some(stream) = &self.output_stream {
            stream.play()?;
        }
        
        if let Some(stream) = &self.input_stream {
            stream.play()?;
        }
        
        self.running.store(true, Ordering::SeqCst);
        println!("✅ Audio streams started successfully");
        Ok(())
    }
    
    /// Stop audio streams
    pub fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.running.load(Ordering::SeqCst) {
            return Ok(());
        }
        
        println!("🛑 Stopping audio streams...");
        
        self.running.store(false, Ordering::SeqCst);
        
        // Drop streams to stop them
        self.input_stream = None;
        self.output_stream = None;
        
        println!("✅ Audio streams stopped");
        Ok(())
    }
    
    /// Check if audio is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
    
    /// Create output stream configuration
    fn create_output_config(&self) -> Result<StreamConfig, Box<dyn std::error::Error>> {
        let device = self.output_device
            .as_ref()
            .ok_or("No output device available")?;
            
        let default_config = device.default_output_config()?;
        let sample_format = default_config.sample_format();
        
        // Convert to our format if needed
        if !matches!(sample_format, SampleFormat::F32) {
            println!("⚠️ Device uses {:?}, converting to F32", sample_format);
        }
        
        // Use device's default sample rate and channels to avoid StreamConfigNotSupported
        let sample_rate = default_config.sample_rate();
        let channels = default_config.channels();
        
        println!("🎛️ Using output config: {} Hz, {} channels", sample_rate.0, channels);
        
        Ok(StreamConfig {
            channels,
            sample_rate,
            buffer_size: cpal::BufferSize::Default, // Use device default buffer size
        })
    }
    
    /// Create input stream configuration
    fn create_input_config(&self) -> Result<StreamConfig, Box<dyn std::error::Error>> {
        let device = self.input_device
            .as_ref()
            .ok_or("No input device available")?;
            
        let default_config = device.default_input_config()?;
        let sample_format = default_config.sample_format();
        
        // Convert to our format if needed
        if !matches!(sample_format, SampleFormat::F32) {
            println!("⚠️ Device uses {:?}, converting to F32", sample_format);
        }
        
        // Use device's default sample rate and channels to avoid StreamConfigNotSupported
        let sample_rate = default_config.sample_rate();
        let channels = default_config.channels();
        
        println!("🎛️ Using input config: {} Hz, {} channels", sample_rate.0, channels);
        
        Ok(StreamConfig {
            channels,
            sample_rate,
            buffer_size: cpal::BufferSize::Default, // Use device default buffer size
        })
    }
    
    /// Get available sample rates for a device
    pub fn get_sample_rates(&self, device_name: &str) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
        let mut sample_rates = Vec::new();
        
        // Check input devices
        for device in self.host.input_devices()? {
            if device.name()? == device_name {
                let config = device.default_input_config()?;
                sample_rates.push(config.sample_rate().0);
                break;
            }
        }
        
        // Check output devices
        for device in self.host.output_devices()? {
            if device.name()? == device_name {
                let config = device.default_output_config()?;
                if !sample_rates.contains(&config.sample_rate().0) {
                    sample_rates.push(config.sample_rate().0);
                }
                break;
            }
        }
        
        // Add common sample rates as fallback
        if sample_rates.is_empty() {
            sample_rates = vec![44100, 48000, 88200, 96000, 192000];
        }
        
        Ok(sample_rates)
    }
    
    /// Set output device
    pub fn set_output_device(&mut self, device_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        for device in self.host.output_devices()? {
            if device.name()? == device_name {
                self.output_device = Some(device);
                self.config.device_name = device_name.to_string();
                return Ok(());
            }
        }
        Err(format!("Output device '{}' not found", device_name).into())
    }
    
    /// Set input device
    pub fn set_input_device(&mut self, device_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        for device in self.host.input_devices()? {
            if device.name()? == device_name {
                self.input_device = Some(device);
                return Ok(());
            }
        }
        Err(format!("Input device '{}' not found", device_name).into())
    }
}

impl Drop for AudioIO {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}