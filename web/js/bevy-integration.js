/**
 * Modurust DAW - Bevy Engine Integration
 * Seamless integration between Bevy (Rust) and JavaScript frontend
 */

class BevyIntegration {
    static isInitialized = false;
    static bevyInstance = null;
    static messageQueue = [];
    static eventListeners = new Map();
    static sharedMemory = null;
    static wasmModule = null;

    // Initialize Bevy integration
    static async init() {
        console.log('🔗 Initializing Bevy Integration...');

        try {
            // Load WebAssembly module
            await this.loadWasmModule();

            // Initialize shared memory for data exchange
            this.initializeSharedMemory();

            // Setup message passing system
            this.setupMessagePassing();

            // Initialize Bevy instance
            await this.initializeBevy();

            // Setup event synchronization
            this.setupEventSync();

            this.isInitialized = true;
            console.log('✅ Bevy Integration initialized');

        } catch (error) {
            console.error('Failed to initialize Bevy integration:', error);
            // Fallback to JavaScript-only mode
            this.fallbackToJS();
        }
    }

    // Load WebAssembly module
    static async loadWasmModule() {
        try {
            // Load the Bevy WebAssembly module
            const response = await fetch('/wasm/bevy-daw.wasm');
            const wasmBuffer = await response.arrayBuffer();

            // Instantiate the WebAssembly module
            const wasmModule = await WebAssembly.instantiate(wasmBuffer, {
                env: {
                    // Import functions from JavaScript to Rust
                    js_log: (ptr, len) => this.handleRustLog(ptr, len),
                    js_request_animation_frame: (callback) => requestAnimationFrame(callback),
                    js_performance_now: () => performance.now(),
                    js_send_message: (type, dataPtr, dataLen) => this.receiveMessageFromRust(type, dataPtr, dataLen),

                    // Memory management
                    memory: new WebAssembly.Memory({ initial: 256, maximum: 512 }),
                },
                // Additional imports for Bevy
                bevy: {
                    // Bevy-specific imports
                    create_entity: () => this.createEntity(),
                    destroy_entity: (id) => this.destroyEntity(id),
                    update_transform: (id, x, y, z) => this.updateTransform(id, x, y, z),
                    render_frame: () => this.renderFrame(),
                }
            });

            this.wasmModule = wasmModule.instance;
            this.sharedMemory = wasmModule.instance.exports.memory;

            console.log('📦 WebAssembly module loaded');

        } catch (error) {
            console.error('Failed to load WebAssembly module:', error);
            throw error;
        }
    }

    // Initialize shared memory buffer
    static initializeSharedMemory() {
        // Create a SharedArrayBuffer for zero-copy data exchange
        if (typeof SharedArrayBuffer !== 'undefined') {
            this.sharedMemory = new SharedArrayBuffer(1024 * 1024); // 1MB shared buffer
        } else {
            // Fallback for browsers without SharedArrayBuffer support
            this.sharedMemory = new ArrayBuffer(1024 * 1024);
        }

        // Initialize memory layout
        this.memoryLayout = {
            messageQueue: 0,      // Start of message queue
            audioBuffer: 1024,    // Audio data buffer
            eegBuffer: 2048,      // EEG data buffer
            poseBuffer: 4096,     // Motion capture buffer
            uiState: 8192,        // UI state synchronization
        };
    }

    // Setup message passing system
    static setupMessagePassing() {
        // Setup bidirectional communication
        this.messageHandlers = new Map([
            ['audio_data', this.handleAudioData.bind(this)],
            ['eeg_data', this.handleEEGData.bind(this)],
            ['pose_data', this.handlePoseData.bind(this)],
            ['ui_update', this.handleUIUpdate.bind(this)],
            ['render_command', this.handleRenderCommand.bind(this)],
        ]);

        // Start message processing loop
        this.processMessages();
    }

    // Initialize Bevy instance
    static async initializeBevy() {
        // Call Bevy initialization function from WebAssembly
        if (this.wasmModule && this.wasmModule.exports.init_bevy) {
            const result = this.wasmModule.exports.init_bevy();

            if (result === 0) { // Success
                console.log('🎮 Bevy engine initialized');

                // Setup Bevy world
                this.setupBevyWorld();

            } else {
                throw new Error('Bevy initialization failed');
            }
        }
    }

    // Setup Bevy world and systems
    static setupBevyWorld() {
        // Create the main Bevy app with necessary plugins
        this.sendMessageToRust('setup_world', {
            plugins: [
                'CorePlugin',
                'TransformPlugin',
                'HierarchyPlugin',
                'DiagnosticPlugin',
                'AudioPlugin',
                'RenderPlugin',
                'WindowPlugin',
                'WinitPlugin',
                'EguiPlugin'
            ],
            resources: {
                clear_color: [0.1, 0.1, 0.1, 1.0],
                window_size: [window.innerWidth, window.innerHeight]
            }
        });

        // Setup custom systems for DAW functionality
        this.setupDAWSystems();
    }

    // Setup DAW-specific systems
    static setupDAWSystems() {
        // Audio processing system
        this.sendMessageToRust('add_system', {
            name: 'audio_system',
            stage: 'Update',
            system: 'process_audio'
        });

        // EEG processing system
        this.sendMessageToRust('add_system', {
            name: 'eeg_system',
            stage: 'Update',
            system: 'process_eeg'
        });

        // UI synchronization system
        this.sendMessageToRust('add_system', {
            name: 'ui_sync_system',
            stage: 'Update',
            system: 'sync_ui_state'
        });

        // Rendering system for visualizations
        this.sendMessageToRust('add_system', {
            name: 'render_system',
            stage: 'PostUpdate',
            system: 'render_visualizations'
        });
    }

    // Setup event synchronization
    static setupEventSync() {
        // Synchronize JavaScript events with Bevy
        window.addEventListener('resize', (e) => {
            this.sendMessageToRust('window_resize', {
                width: window.innerWidth,
                height: window.innerHeight
            });
        });

        // Mouse events
        window.addEventListener('mousemove', (e) => {
            this.sendMessageToRust('mouse_move', {
                x: e.clientX,
                y: e.clientY
            });
        });

        window.addEventListener('mousedown', (e) => {
            this.sendMessageToRust('mouse_down', {
                button: e.button,
                x: e.clientX,
                y: e.clientY
            });
        });

        // Keyboard events
        window.addEventListener('keydown', (e) => {
            this.sendMessageToRust('key_down', {
                key: e.key,
                code: e.code,
                ctrl: e.ctrlKey,
                shift: e.shiftKey,
                alt: e.altKey
            });
        });

        // Audio events
        if (AudioEngine.isInitialized) {
            AudioEngine.on('audioData', (data) => {
                this.sendAudioDataToRust(data);
            });
        }

        // EEG events
        if (EEGControl.isInitialized) {
            EEGControl.on('eegData', (data) => {
                this.sendEEGDataToRust(data);
            });
        }

        // Motion capture events
        if (MotionCapture.isInitialized) {
            MotionCapture.on('poseData', (data) => {
                this.sendPoseDataToRust(data);
            });
        }
    }

    // Send message to Rust/Bevy
    static sendMessageToRust(type, data) {
        const message = {
            type,
            data,
            timestamp: performance.now(),
            id: Math.random().toString(36).substr(2, 9)
        };

        this.messageQueue.push(message);

        // If WebAssembly is available, send immediately
        if (this.wasmModule && this.wasmModule.exports.receive_message) {
            this.sendMessageToWasm(message);
        }
    }

    // Send message to WebAssembly
    static sendMessageToWasm(message) {
        try {
            // Serialize message to JSON
            const messageJson = JSON.stringify(message);
            const encoder = new TextEncoder();
            const messageBytes = encoder.encode(messageJson);

            // Allocate memory in WebAssembly
            const ptr = this.wasmModule.exports.allocate_memory(messageBytes.length);

            // Copy message to WebAssembly memory
            const memoryView = new Uint8Array(this.sharedMemory.buffer, ptr, messageBytes.length);
            memoryView.set(messageBytes);

            // Call WebAssembly function to receive message
            this.wasmModule.exports.receive_message(ptr, messageBytes.length);

            // Free allocated memory
            this.wasmModule.exports.free_memory(ptr, messageBytes.length);

        } catch (error) {
            console.error('Failed to send message to WebAssembly:', error);
        }
    }

    // Receive message from Rust/Bevy
    static receiveMessageFromRust(type, dataPtr, dataLen) {
        try {
            // Read message from WebAssembly memory
            const memoryView = new Uint8Array(this.sharedMemory.buffer, dataPtr, dataLen);
            const decoder = new TextDecoder();
            const messageJson = decoder.decode(memoryView);
            const message = JSON.parse(messageJson);

            // Handle the message
            this.handleMessageFromRust(message);

        } catch (error) {
            console.error('Failed to receive message from Rust:', error);
        }
    }

    // Handle message from Rust
    static handleMessageFromRust(message) {
        const handler = this.messageHandlers.get(message.type);
        if (handler) {
            handler(message.data);
        } else {
            console.warn('Unknown message type from Rust:', message.type);
        }
    }

    // Message handlers
    static handleAudioData(data) {
        // Update audio visualizations
        if (WebGLVisualizations.isInitialized) {
            WebGLVisualizations.updateFrequencyData(data.frequency);
            WebGLVisualizations.updateWaveformData(data.waveform);
        }

        // Update audio meters
        if (RealtimeAudioUI.isInitialized) {
            RealtimeAudioUI.updateAudioMeters();
        }
    }

    static handleEEGData(data) {
        // Update EEG visualizations
        if (EEGControl.isInitialized) {
            EEGControl.updateBrainVisualization(data);
        }
    }

    static handlePoseData(data) {
        // Update motion capture visualizations
        if (MotionCapture.isInitialized) {
            MotionCapture.updatePoseVisualization(data);
        }
    }

    static handleUIUpdate(data) {
        // Update UI state from Bevy
        if (data.view) {
            UIManager.switchView(data.view);
        }
        if (data.transport) {
            // Update transport state
        }
    }

    static handleRenderCommand(data) {
        // Handle rendering commands from Bevy
        if (data.type === 'draw_spectrum') {
            WebGLVisualizations.renderSpectrum(data.width, data.height);
        } else if (data.type === 'draw_waveform') {
            WebGLVisualizations.renderWaveform(data.width, data.height);
        }
    }

    // Send audio data to Rust
    static sendAudioDataToRust(data) {
        this.sendMessageToRust('audio_data', {
            frequency: data.frequency,
            waveform: data.waveform,
            level: data.level
        });
    }

    // Send EEG data to Rust
    static sendEEGDataToRust(data) {
        this.sendMessageToRust('eeg_data', {
            bands: data.bandPowers,
            focus: data.focus,
            relaxation: data.relaxation,
            stress: data.stress
        });
    }

    // Send pose data to Rust
    static sendPoseDataToRust(data) {
        this.sendMessageToRust('pose_data', {
            keypoints: data.currentPose,
            gesture: data.currentGesture
        });
    }

    // Process message queue
    static processMessages() {
        // Process outgoing messages
        while (this.messageQueue.length > 0) {
            const message = this.messageQueue.shift();
            if (this.wasmModule) {
                this.sendMessageToWasm(message);
            }
        }

        // Continue processing
        requestAnimationFrame(() => this.processMessages());
    }

    // Handle Rust logging
    static handleRustLog(ptr, len) {
        try {
            const memoryView = new Uint8Array(this.sharedMemory.buffer, ptr, len);
            const decoder = new TextDecoder();
            const logMessage = decoder.decode(memoryView);
            console.log('🦀 Rust:', logMessage);
        } catch (error) {
            console.error('Failed to handle Rust log:', error);
        }
    }

    // Entity management (called from WebAssembly)
    static createEntity() {
        const entityId = Math.random().toString(36).substr(2, 9);
        console.log('📦 Created Bevy entity:', entityId);
        return entityId;
    }

    static destroyEntity(id) {
        console.log('🗑️ Destroyed Bevy entity:', id);
    }

    static updateTransform(id, x, y, z) {
        // Update entity transform in JavaScript space if needed
        console.log(`🔄 Updated transform for entity ${id}: (${x}, ${y}, ${z})`);
    }

    static renderFrame() {
        // Called when Bevy renders a frame
        // Can be used to synchronize with JavaScript rendering
    }

    // Fallback to JavaScript-only mode
    static fallbackToJS() {
        console.warn('⚠️ Falling back to JavaScript-only mode');

        // Disable Bevy-specific features
        this.isInitialized = false;

        // Show notification to user
        UIManager.showNotification(
            'Bevy integration not available. Running in JavaScript-only mode.',
            'warning'
        );

        // Continue with JavaScript-only functionality
        // All the existing JavaScript modules will work normally
    }

    // Check if Bevy integration is available
    static isAvailable() {
        return this.isInitialized && this.wasmModule !== null;
    }

    // Get Bevy performance stats
    static getPerformanceStats() {
        if (!this.isAvailable()) return null;

        // Get stats from WebAssembly
        if (this.wasmModule.exports.get_performance_stats) {
            const statsPtr = this.wasmModule.exports.get_performance_stats();
            // Read stats from memory (implementation would depend on Rust side)
            return {
                fps: 60, // Placeholder
                frameTime: 16.67, // Placeholder
                memoryUsage: 0 // Placeholder
            };
        }

        return null;
    }

    // Dispose resources
    static dispose() {
        if (this.wasmModule && this.wasmModule.exports.cleanup) {
            this.wasmModule.exports.cleanup();
        }

        this.messageQueue = [];
        this.eventListeners.clear();
        this.sharedMemory = null;
        this.wasmModule = null;
        this.isInitialized = false;

        console.log('🗑️ Bevy Integration disposed');
    }
}

// Event system for Bevy integration
class BevyEventSystem {
    static listeners = new Map();

    static on(event, callback) {
        if (!this.listeners.has(event)) {
            this.listeners.set(event, []);
        }
        this.listeners.get(event).push(callback);
    }

    static off(event, callback) {
        const eventListeners = this.listeners.get(event);
        if (eventListeners) {
            const index = eventListeners.indexOf(callback);
            if (index !== -1) {
                eventListeners.splice(index, 1);
            }
        }
    }

    static emit(event, data) {
        const eventListeners = this.listeners.get(event);
        if (eventListeners) {
            eventListeners.forEach(callback => {
                try {
                    callback(data);
                } catch (error) {
                    console.error('Error in Bevy event listener:', error);
                }
            });
        }
    }
}

// Initialize when DOM is loaded and Bevy is available
document.addEventListener('DOMContentLoaded', async () => {
    // Check if WebAssembly is supported
    if (typeof WebAssembly !== 'undefined') {
        try {
            await BevyIntegration.init();
        } catch (error) {
            console.warn('Bevy integration failed, continuing with JavaScript-only mode');
        }
    } else {
        console.warn('WebAssembly not supported, using JavaScript-only mode');
    }
});

// Export for global access
window.BevyIntegration = BevyIntegration;
window.BevyEventSystem = BevyEventSystem;