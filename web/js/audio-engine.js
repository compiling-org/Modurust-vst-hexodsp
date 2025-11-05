/**
 * Modurust DAW - Audio Engine
 * Web Audio API-based audio processing and synthesis
 */

class AudioEngine {
    static audioContext = null;
    static masterGain = null;
    static analyser = null;
    static analyserL = null;
    static analyserR = null;
    static splitter = null;
    static tracks = new Map();
    static effects = new Map();
    static isInitialized = false;

    // Initialize the audio engine
    static async init() {
        try {
            console.log('🎵 Initializing Audio Engine...');

            // Create audio context
            this.audioContext = new (window.AudioContext || window.webkitAudioContext)();

            // Create master gain node
            this.masterGain = this.audioContext.createGain();
            this.masterGain.gain.value = 0.8;
            this.masterGain.connect(this.audioContext.destination);

            // Create analyser(s) for visualization
            this.analyser = this.audioContext.createAnalyser();
            this.analyser.fftSize = 2048;
            this.analyser.smoothingTimeConstant = 0.8;
            this.masterGain.connect(this.analyser);

            // Create stereo analysers via a sidechain splitter
            this.splitter = this.audioContext.createChannelSplitter(2);
            this.analyserL = this.audioContext.createAnalyser();
            this.analyserR = this.audioContext.createAnalyser();
            this.analyserL.fftSize = 2048;
            this.analyserR.fftSize = 2048;
            this.analyserL.smoothingTimeConstant = 0.85;
            this.analyserR.smoothingTimeConstant = 0.85;
            // Tap the master output into the splitter solely for analysis
            this.masterGain.connect(this.splitter);
            this.splitter.connect(this.analyserL, 0);
            this.splitter.connect(this.analyserR, 1);

            // Initialize tracks
            this.initializeTracks();

            // Initialize effects
            this.initializeEffects();

            this.isInitialized = true;
            console.log('✅ Audio Engine initialized');

        } catch (error) {
            console.error('❌ Audio Engine initialization failed:', error);
            throw error;
        }
    }

    // Initialize default tracks
    static initializeTracks() {
        // Create 8 default tracks
        for (let i = 1; i <= 8; i++) {
            this.createTrack(i, `Track ${i}`);
        }
    }

    // Create a new track
    static createTrack(id, name) {
        const track = {
            id,
            name,
            gain: this.audioContext.createGain(),
            pan: this.audioContext.createStereoPanner(),
            mute: false,
            solo: false,
            volume: 0.8,
            panValue: 0,
            clips: [],
            effects: []
        };

        // Connect track to master
        track.gain.connect(track.pan);
        track.pan.connect(this.masterGain);

        this.tracks.set(id, track);
        return track;
    }

    // Initialize default effects
    static initializeEffects() {
        // Create master effects chain
        this.createEffect('reverb', 'Reverb');
        this.createEffect('delay', 'Delay');
        this.createEffect('filter', 'Filter');
    }

    // Create a new effect
    static createEffect(id, name) {
        let effectNode;

        switch (id) {
            case 'reverb':
                effectNode = this.createReverb();
                break;
            case 'delay':
                effectNode = this.createDelay();
                break;
            case 'filter':
                effectNode = this.createFilter();
                break;
            default:
                effectNode = this.audioContext.createGain();
        }

        const effect = {
            id,
            name,
            node: effectNode,
            enabled: false,
            parameters: new Map()
        };

        this.effects.set(id, effect);
        return effect;
    }

    // Create reverb effect
    static createReverb() {
        const convolver = this.audioContext.createConvolver();

        // Create simple reverb impulse response
        const length = this.audioContext.sampleRate * 2; // 2 seconds
        const impulse = this.audioContext.createBuffer(2, length, this.audioContext.sampleRate);

        for (let channel = 0; channel < 2; channel++) {
            const channelData = impulse.getChannelData(channel);
            for (let i = 0; i < length; i++) {
                channelData[i] = (Math.random() * 2 - 1) * Math.pow(1 - i / length, 2);
            }
        }

        convolver.buffer = impulse;
        return convolver;
    }

    // Create delay effect
    static createDelay() {
        const delay = this.audioContext.createDelay(2.0);
        const feedback = this.audioContext.createGain();
        const wet = this.audioContext.createGain();

        delay.delayTime.value = 0.3;
        feedback.gain.value = 0.4;
        wet.gain.value = 0.3;

        delay.connect(feedback);
        feedback.connect(delay);
        delay.connect(wet);

        // Return wet signal (dry signal handled separately)
        return wet;
    }

    // Create filter effect
    static createFilter() {
        const filter = this.audioContext.createBiquadFilter();
        filter.type = 'lowpass';
        filter.frequency.value = 1000;
        filter.Q.value = 1;
        return filter;
    }

    // Start playback
    static startPlayback() {
        if (!this.isInitialized) return;

        console.log('▶ Starting playback');
        // Implementation for starting playback
        // This would typically involve scheduling clips and starting transport
    }

    // Pause playback
    static pausePlayback() {
        if (!this.isInitialized) return;

        console.log('⏸ Pausing playback');
        // Implementation for pausing playback
    }

    // Stop playback
    static stopPlayback() {
        if (!this.isInitialized) return;

        console.log('⏹ Stopping playback');
        // Implementation for stopping playback
    }

    // Start recording
    static startRecording() {
        if (!this.isInitialized) return;

        console.log('● Starting recording');
        // Implementation for starting recording
    }

    // Stop recording
    static stopRecording() {
        if (!this.isInitialized) return;

        console.log('⬜ Stopping recording');
        // Implementation for stopping recording
    }

    // Seek to position
    static seekTo(time) {
        if (!this.isInitialized) return;

        console.log(`🔍 Seeking to ${time}s`);
        // Implementation for seeking
    }

    // Get frequency data for visualization
    static getFrequencyData() {
        if (!this.analyser) return new Uint8Array(0);

        const bufferLength = this.analyser.frequencyBinCount;
        const dataArray = new Uint8Array(bufferLength);
        this.analyser.getByteFrequencyData(dataArray);
        return dataArray;
    }

    // Get time domain data for waveform visualization
    static getTimeDomainData() {
        if (!this.analyser) return new Uint8Array(0);

        const bufferLength = this.analyser.fftSize;
        const dataArray = new Uint8Array(bufferLength);
        this.analyser.getByteTimeDomainData(dataArray);
        return dataArray;
    }

    // Get stereo RMS levels in dB for meters
    static getStereoLevels() {
        const levels = { left: -Infinity, right: -Infinity };

        if (!this.analyserL || !this.analyserR) {
            // Fallback to mono analyser
            const mono = this.getTimeDomainData();
            if (mono && mono.length) {
                const rms = Math.sqrt(mono.reduce((acc, v) => {
                    const centered = (v - 128) / 128; // byte time domain centered around 128
                    return acc + centered * centered;
                }, 0) / mono.length);
                const db = rms > 0 ? 20 * Math.log10(rms) : -Infinity;
                levels.left = db;
                levels.right = db;
            }
            return levels;
        }

        const bufL = new Float32Array(this.analyserL.fftSize);
        const bufR = new Float32Array(this.analyserR.fftSize);
        this.analyserL.getFloatTimeDomainData(bufL);
        this.analyserR.getFloatTimeDomainData(bufR);

        const rmsL = Math.sqrt(bufL.reduce((acc, v) => acc + v * v, 0) / bufL.length);
        const rmsR = Math.sqrt(bufR.reduce((acc, v) => acc + v * v, 0) / bufR.length);
        levels.left = rmsL > 0 ? 20 * Math.log10(rmsL) : -Infinity;
        levels.right = rmsR > 0 ? 20 * Math.log10(rmsR) : -Infinity;

        return levels;
    }

    // Create oscillator for synthesis
    static createOscillator(frequency = 440, type = 'sine') {
        const oscillator = this.audioContext.createOscillator();
        const gain = this.audioContext.createGain();

        oscillator.frequency.value = frequency;
        oscillator.type = type;
        gain.gain.value = 0.1;

        oscillator.connect(gain);
        gain.connect(this.masterGain);

        return { oscillator, gain };
    }

    // Create buffer source for sample playback
    static createBufferSource(buffer) {
        const source = this.audioContext.createBufferSource();
        source.buffer = buffer;
        source.connect(this.masterGain);
        return source;
    }

    // Load audio file
    static async loadAudioFile(file) {
        return new Promise((resolve, reject) => {
            const reader = new FileReader();
            reader.onload = async (e) => {
                try {
                    const arrayBuffer = e.target.result;
                    const audioBuffer = await this.audioContext.decodeAudioData(arrayBuffer);
                    resolve(audioBuffer);
                } catch (error) {
                    reject(error);
                }
            };
            reader.onerror = reject;
            reader.readAsArrayBuffer(file);
        });
    }

    // Get current time
    static getCurrentTime() {
        return this.audioContext ? this.audioContext.currentTime : 0;
    }

    // Set master volume
    static setMasterVolume(volume) {
        if (this.masterGain) {
            this.masterGain.gain.value = Math.max(0, Math.min(1, volume));
        }
    }

    // Get master volume
    static getMasterVolume() {
        return this.masterGain ? this.masterGain.gain.value : 0.8;
    }

    // Set track volume
    static setTrackVolume(trackId, volume) {
        const track = this.tracks.get(trackId);
        if (track) {
            track.volume = Math.max(0, Math.min(1, volume));
            track.gain.gain.value = track.volume;
        }
    }

    // Set track pan
    static setTrackPan(trackId, pan) {
        const track = this.tracks.get(trackId);
        if (track) {
            track.panValue = Math.max(-1, Math.min(1, pan));
            track.pan.pan.value = track.panValue;
        }
    }

    // Mute/unmute track
    static setTrackMute(trackId, mute) {
        const track = this.tracks.get(trackId);
        if (track) {
            track.mute = mute;
            track.gain.gain.value = track.mute ? 0 : track.volume;
        }
    }

    // Solo track
    static setTrackSolo(trackId, solo) {
        const track = this.tracks.get(trackId);
        if (track) {
            track.solo = solo;
            // Implementation for solo logic
            this.updateSoloState();
        }
    }

    // Update solo state for all tracks
    static updateSoloState() {
        const hasSolo = Array.from(this.tracks.values()).some(track => track.solo);

        this.tracks.forEach(track => {
            if (hasSolo) {
                track.gain.gain.value = track.solo ? track.volume : 0;
            } else {
                track.gain.gain.value = track.mute ? 0 : track.volume;
            }
        });
    }

    // Enable/disable effect
    static setEffectEnabled(effectId, enabled) {
        const effect = this.effects.get(effectId);
        if (effect) {
            effect.enabled = enabled;
            // Reconnect audio graph based on effect state
            this.updateEffectsChain();
        }
    }

    // Update effects chain
    static updateEffectsChain() {
        // Disconnect all effects first
        this.effects.forEach(effect => {
            effect.node.disconnect();
        });

        // Reconnect based on current state
        let currentNode = this.masterGain;

        this.effects.forEach(effect => {
            if (effect.enabled) {
                currentNode.disconnect();
                currentNode.connect(effect.node);
                currentNode = effect.node;
            }
        });

        // Connect final node to destination
        currentNode.connect(this.audioContext.destination);
    }

    // Get track info
    static getTrack(trackId) {
        return this.tracks.get(trackId);
    }

    // Get all tracks
    static getAllTracks() {
        return Array.from(this.tracks.values());
    }

    // Get effect info
    static getEffect(effectId) {
        return this.effects.get(effectId);
    }

    // Get all effects
    static getAllEffects() {
        return Array.from(this.effects.values());
    }

    // Clean up resources
    static dispose() {
        if (this.audioContext) {
            this.audioContext.close();
            this.audioContext = null;
        }

        this.tracks.clear();
        this.effects.clear();
        this.isInitialized = false;

        console.log('🗑️ Audio Engine disposed');
    }
}

// Export for use in other modules
window.AudioEngine = AudioEngine;