/**
 * Modurust DAW - Main Application JavaScript
 * Entry point for the web-based DAW interface with unified Bevy UI integration
 */

// Global application state
const AppState = {
    currentView: 'arrangement',
    isPlaying: false,
    isRecording: false,
    tempo: 120,
    timeSignature: [4, 4],
    loopEnabled: false,
    metronomeEnabled: true,
    zoom: 1.0,
    tracks: [],
    selectedTrackId: null,
    scenes: [],
    nodes: [],
    connections: [],
    audioContext: null,
    isInitialized: false,
    useBevyUI: true, // Try Bevy UI first, fallback to JavaScript
    bevyUI: null
};

// DOM element references
const DOM = {
    app: document.getElementById('app'),
    viewTabs: document.querySelectorAll('.view-tab'),
    transportBtns: document.querySelectorAll('.transport-btn'),
    currentTime: document.getElementById('current-time'),
    playheadPosition: document.getElementById('playhead-position'),
    tempoDisplay: document.getElementById('tempo-display'),
    settingsBtn: document.querySelector('.settings-btn'),
    loadingOverlay: document.getElementById('loading-overlay'),
    modals: document.getElementById('modals-container')
};

// Initialize the application
async function initApp() {
    try {
        console.log('🚀 Initializing Modurust DAW...');

        // Show loading overlay
        showLoading(true);

        // Try to initialize Bevy UI first (unified UI system)
        await initBevyUI();

        // Initialize audio system
        await initAudioSystem();

        // Initialize UI components (fallback if Bevy UI fails)
        if (!AppState.useBevyUI) {
            initUI();
            initViews();
        }

        // Initialize audio engine
        await AudioEngine.init();

        // Set up event listeners
        setupEventListeners();

        // Initialize default state
        initializeDefaultState();

        // Hide loading overlay
        showLoading(false);

        AppState.isInitialized = true;

        const uiType = AppState.useBevyUI ? 'Bevy UI' : 'JavaScript UI';
        console.log(`✅ Modurust DAW initialized successfully with ${uiType}`);

        // Show welcome message
        showWelcomeMessage();

    } catch (error) {
        console.error('❌ Failed to initialize Modurust DAW:', error);
        showError('Failed to initialize application: ' + error.message);
        showLoading(false);
    }
}

// Initialize Bevy UI (primary UI system)
async function initBevyUI() {
    console.log('🎮 Initializing Bevy UI...');

    try {
        // Check if WebAssembly is supported
        if (typeof WebAssembly === 'undefined') {
            throw new Error('WebAssembly not supported');
        }

        // Load Bevy WebAssembly module
        const response = await fetch('/wasm/bevy-daw.wasm');
        if (!response.ok) {
            throw new Error('Bevy WebAssembly module not found');
        }

        const wasmBuffer = await response.arrayBuffer();

        // Instantiate WebAssembly with Bevy UI imports
        const wasmModule = await WebAssembly.instantiate(wasmBuffer, {
            env: {
                // JavaScript functions callable from Bevy/Rust
                js_log: (ptr, len) => handleRustLog(ptr, len),
                js_request_animation_frame: (callback) => requestAnimationFrame(callback),
                js_performance_now: () => performance.now(),
                js_send_message: (type, dataPtr, dataLen) => receiveMessageFromBevy(type, dataPtr, dataLen),

                // Bevy UI specific imports
                create_entity: () => createBevyEntity(),
                destroy_entity: (id) => destroyBevyEntity(id),
                update_transform: (id, x, y, z) => updateBevyTransform(id, x, y, z),
                render_frame: () => renderBevyFrame(),

                memory: new WebAssembly.Memory({ initial: 256, maximum: 512 }),
            }
        });

        AppState.bevyUI = wasmModule.instance;
        AppState.sharedMemory = wasmModule.instance.exports.memory;

        // Initialize Bevy UI
        if (AppState.bevyUI.exports.init_bevy_ui) {
            const result = AppState.bevyUI.exports.init_bevy_ui();
            if (result !== 0) {
                throw new Error('Bevy UI initialization failed');
            }
        }

        // Setup canvas for Bevy rendering
        setupBevyCanvas();

        console.log('✅ Bevy UI initialized');

    } catch (error) {
        console.warn('⚠️ Bevy UI not available, falling back to JavaScript UI:', error.message);
        AppState.useBevyUI = false;
        AppState.bevyUI = null;
    }
}

// Setup canvas for Bevy rendering
function setupBevyCanvas() {
    // Create or get canvas element
    let canvas = document.getElementById('bevy-canvas');
    if (!canvas) {
        canvas = document.createElement('canvas');
        canvas.id = 'bevy-canvas';
        canvas.style.position = 'absolute';
        canvas.style.top = '0';
        canvas.style.left = '0';
        canvas.style.width = '100%';
        canvas.style.height = '100%';
        canvas.style.zIndex = '1';
        document.body.appendChild(canvas);
    }

    // Set canvas size
    const rect = canvas.getBoundingClientRect();
    canvas.width = rect.width * window.devicePixelRatio;
    canvas.height = rect.height * window.devicePixelRatio;

    // Initialize Bevy with canvas
    if (AppState.bevyUI && AppState.bevyUI.exports.set_canvas_size) {
        AppState.bevyUI.exports.set_canvas_size(canvas.width, canvas.height);
    }
}

// Handle logging from Bevy/Rust
function handleRustLog(ptr, len) {
    try {
        const memoryView = new Uint8Array(AppState.sharedMemory.buffer, ptr, len);
        const decoder = new TextDecoder();
        const logMessage = decoder.decode(memoryView);
        console.log('🦀 Bevy:', logMessage);
    } catch (error) {
        console.error('Failed to handle Bevy log:', error);
    }
}

// Receive message from Bevy
function receiveMessageFromBevy(type, dataPtr, dataLen) {
    try {
        const memoryView = new Uint8Array(AppState.sharedMemory.buffer, dataPtr, dataLen);
        const decoder = new TextDecoder();
        const messageJson = decoder.decode(memoryView);
        const message = JSON.parse(messageJson);

        // Handle message based on type
        handleBevyMessage(message);

    } catch (error) {
        console.error('Failed to receive message from Bevy:', error);
    }
}

// Handle messages from Bevy UI
function handleBevyMessage(message) {
    switch (message.type) {
        case 'button_clicked':
            handleBevyButtonClick(message.component_id);
            break;
        case 'slider_changed':
            handleBevySliderChange(message.component_id, message.value);
            break;
        case 'ui_update':
            handleBevyUIUpdate(message);
            break;
        case 'request_audio_data':
            sendAudioDataToBevy();
            break;
        case 'request_eeg_data':
            sendEEGDataToBevy();
            break;
        default:
            console.log('Unknown Bevy message:', message);
    }
}

// Handle button clicks from Bevy UI
function handleBevyButtonClick(componentId) {
    switch (componentId) {
        case 'play_button':
            togglePlayback();
            break;
        case 'stop_button':
            stopPlayback();
            break;
        case 'record_button':
            toggleRecording();
            break;
        default:
            console.log('Bevy button clicked:', componentId);
    }
}

// Handle slider changes from Bevy UI
function handleBevySliderChange(componentId, value) {
    switch (componentId) {
        case 'master_volume':
            if (AudioEngine && AudioEngine.setMasterVolume) {
                AudioEngine.setMasterVolume(value);
            }
            break;
        case 'master_pan':
            // Handle pan (convert -1..1 to degrees or whatever)
            break;
        default:
            console.log('Bevy slider changed:', componentId, value);
    }
}

// Handle UI updates from Bevy
function handleBevyUIUpdate(update) {
    // Update JavaScript UI components based on Bevy state
    if (update.transport) {
        updateTransportUI();
    }
    if (update.view) {
        switchView(update.view);
    }
}

// Send audio data to Bevy
function sendAudioDataToBevy() {
    if (!AppState.bevyUI) return;

    const audioData = AudioEngine?.getAudioData ? AudioEngine.getAudioData() : {
        frequency: new Float32Array(256),
        waveform: new Float32Array(256),
        level: 0
    };

    sendMessageToBevy('audio_data', audioData);
}

// Send EEG data to Bevy
function sendEEGDataToBevy() {
    if (!AppState.bevyUI) return;

    const eegData = window.EEGControl?.getCurrentData ? window.EEGControl.getCurrentData() : {
        bands: { delta: 0, theta: 0, alpha: 0, beta: 0, gamma: 0 },
        focus: 0,
        relaxation: 0,
        stress: 0
    };

    sendMessageToBevy('eeg_data', eegData);
}

// Send message to Bevy
function sendMessageToBevy(type, data) {
    if (!AppState.bevyUI || !AppState.bevyUI.exports.receive_message) return;

    try {
        const message = { type, data, timestamp: performance.now() };
        const messageJson = JSON.stringify(message);
        const encoder = new TextEncoder();
        const messageBytes = encoder.encode(messageJson);

        // Allocate memory in WebAssembly
        const ptr = AppState.bevyUI.exports.allocate_memory(messageBytes.length);

        // Copy message to WebAssembly memory
        const memoryView = new Uint8Array(AppState.sharedMemory.buffer, ptr, messageBytes.length);
        memoryView.set(messageBytes);

        // Send message
        AppState.bevyUI.exports.receive_message(ptr, messageBytes.length);

        // Free memory
        AppState.bevyUI.exports.free_memory(ptr, messageBytes.length);

    } catch (error) {
        console.error('Failed to send message to Bevy:', error);
    }
}

// Bevy entity management (called from WebAssembly)
function createBevyEntity() {
    const entityId = 'bevy_entity_' + Math.random().toString(36).substr(2, 9);
    console.log('📦 Created Bevy entity:', entityId);
    return entityId;
}

function destroyBevyEntity(id) {
    console.log('🗑️ Destroyed Bevy entity:', id);
}

function updateBevyTransform(id, x, y, z) {
    console.log(`🔄 Updated Bevy transform for ${id}: (${x}, ${y}, ${z})`);
}

function renderBevyFrame() {
    // Called when Bevy renders a frame
    // Can synchronize with JavaScript rendering
    if (AppState.useBevyUI) {
        requestAnimationFrame(() => renderBevyFrame());
    }
}

// Initialize audio system
async function initAudioSystem() {
    try {
        // Request user interaction for audio context
        if (!AppState.audioContext) {
            AppState.audioContext = new (window.AudioContext || window.webkitAudioContext)();

            // Resume audio context on user interaction
            document.addEventListener('click', async () => {
                if (AppState.audioContext.state === 'suspended') {
                    await AppState.audioContext.resume();
                    console.log('🎵 Audio context resumed');
                }
            }, { once: true });
        }

        console.log('✅ Audio system initialized');
    } catch (error) {
        console.error('❌ Audio system initialization failed:', error);
        throw error;
    }
}

// Initialize UI components
function initUI() {
    // Initialize view tabs
    DOM.viewTabs.forEach(tab => {
        tab.addEventListener('click', () => switchView(tab.dataset.view));
    });

    // Initialize transport controls
    DOM.transportBtns.forEach(btn => {
        btn.addEventListener('click', () => handleTransportAction(btn.id));
    });

    // Initialize settings button
    DOM.settingsBtn.addEventListener('click', () => showSettingsModal());

    // Inspector toggle (header)
    const inspectorToggleBtn = document.querySelector('.toggle-inspector-btn');
    if (inspectorToggleBtn) {
        inspectorToggleBtn.addEventListener('click', () => toggleInspectorPanel());
    }

    // Audio monitor toggles (header and footer)
    const audioMonHeaderToggle = document.querySelector('.toggle-audio-monitor-btn');
    const audioMonFooterToggle = document.querySelector('.audio-monitor-toggle');
    if (audioMonHeaderToggle) {
        audioMonHeaderToggle.addEventListener('click', () => toggleAudioMonitor());
    }
    if (audioMonFooterToggle) {
        audioMonFooterToggle.addEventListener('click', () => toggleAudioMonitor());
    }

    // Wire inspector controls
    setupInspectorControls();

    console.log('✅ UI components initialized');
}

// Initialize views (only if not using Bevy UI)
function initViews() {
    if (AppState.useBevyUI) {
        console.log('ℹ️ Skipping JavaScript views - using Bevy UI');
        return;
    }

    // Initialize arrangement view
    ArrangementView.init();

    // Initialize live view
    LiveView.init();

    // Initialize node view
    NodeView.init();

    console.log('✅ JavaScript views initialized');
}

// Set up event listeners
function setupEventListeners() {
    // Keyboard shortcuts
    document.addEventListener('keydown', handleKeyboardShortcut);

    // Window resize
    window.addEventListener('resize', handleWindowResize);

    // Before unload
    window.addEventListener('beforeunload', handleBeforeUnload);

    // Start audio monitor meter updates
    startAudioMonitorLoop();

    console.log('✅ Event listeners set up');
}

// Initialize default state
function initializeDefaultState() {
    // Create default tracks
    createDefaultTracks();

    // Create default scenes
    createDefaultScenes();

    // Set initial view
    switchView('arrangement');

    // Update UI
    updateUI();

    // Initialize inspector with first track
    if (AppState.tracks && AppState.tracks.length > 0) {
        AppState.selectedTrackId = AppState.tracks[0].id;
    }
    updateInspectorPanel();

    console.log('✅ Default state initialized');
}

// Select a track and update inspector
function selectTrack(trackId) {
    AppState.selectedTrackId = trackId;
    updateInspectorPanel();
    // Ensure inspector is visible when selecting a track
    const inspectorPanel = document.getElementById('inspector-panel');
    if (inspectorPanel && inspectorPanel.classList.contains('collapsed')) {
        inspectorPanel.classList.remove('collapsed');
        inspectorPanel.setAttribute('aria-hidden', 'false');
        const toggle = inspectorPanel.querySelector('.panel-toggle');
        if (toggle) {
            toggle.textContent = '−';
            toggle.setAttribute('aria-expanded', 'true');
        }
    }
}

// Update inspector panel contents based on selected track
function updateInspectorPanel() {
    const inspectorPanel = document.getElementById('inspector-panel');
    if (!inspectorPanel) return;

    const empty = inspectorPanel.querySelector('.inspector-empty');
    const fields = inspectorPanel.querySelector('.inspector-fields');
    const nameEl = document.getElementById('inspector-name');
    const typeEl = document.getElementById('inspector-type');
    const inputEl = document.getElementById('inspector-input');
    const volEl = document.getElementById('inspector-volume');
    const panEl = document.getElementById('inspector-pan');
    const muteBtn = document.getElementById('inspector-mute');
    const soloBtn = document.getElementById('inspector-solo');
    const armBtn = document.getElementById('inspector-arm');

    const track = AppState.tracks.find(t => t.id === AppState.selectedTrackId);

    if (!track) {
        if (empty) empty.style.display = 'block';
        if (fields) fields.style.display = 'none';
        return;
    }

    if (empty) empty.style.display = 'none';
    if (fields) fields.style.display = 'block';

    if (nameEl) nameEl.textContent = track.name || '—';
    if (typeEl) typeEl.textContent = track.type || '—';
    if (inputEl) inputEl.textContent = track.input || 'None';
    if (volEl) volEl.value = Math.round((track.volume ?? 0.8) * 100);
    if (panEl) panEl.value = Math.round((track.panValue ?? 0) * 50);

    if (muteBtn) muteBtn.classList.toggle('active', !!track.mute);
    if (soloBtn) soloBtn.classList.toggle('active', !!track.solo);
    if (armBtn) armBtn.classList.toggle('active', !!track.armed);
}

// Wire inspector control events
function setupInspectorControls() {
    const volEl = document.getElementById('inspector-volume');
    const panEl = document.getElementById('inspector-pan');
    const muteBtn = document.getElementById('inspector-mute');
    const soloBtn = document.getElementById('inspector-solo');
    const armBtn = document.getElementById('inspector-arm');

    if (volEl) {
        volEl.addEventListener('input', (e) => {
            const trackId = AppState.selectedTrackId;
            if (!trackId) return;
            const vol = clamp(e.target.value / 100, 0, 1);
            if (window.AudioEngine && AudioEngine.setTrackVolume) {
                AudioEngine.setTrackVolume(trackId, vol);
            }
            const track = AppState.tracks.find(t => t.id === trackId);
            if (track) track.volume = vol;
            if (window.ArrangementView && ArrangementView.updateTrackHeader) {
                ArrangementView.updateTrackHeader(trackId);
            }
            const mixer = window.UIManager?.getComponent ? UIManager.getComponent('mixer') : null;
            if (mixer && mixer.createMixerChannels) mixer.createMixerChannels();
        });
    }
    if (panEl) {
        panEl.addEventListener('input', (e) => {
            const trackId = AppState.selectedTrackId;
            if (!trackId) return;
            const pan = clamp(e.target.value / 50, -1, 1);
            if (window.AudioEngine && AudioEngine.setTrackPan) {
                AudioEngine.setTrackPan(trackId, pan);
            }
            const track = AppState.tracks.find(t => t.id === trackId);
            if (track) track.panValue = pan;
            if (window.ArrangementView && ArrangementView.updateTrackHeader) {
                ArrangementView.updateTrackHeader(trackId);
            }
            const mixer = window.UIManager?.getComponent ? UIManager.getComponent('mixer') : null;
            if (mixer && mixer.createMixerChannels) mixer.createMixerChannels();
        });
    }
    if (muteBtn) {
        muteBtn.addEventListener('click', () => {
            const trackId = AppState.selectedTrackId; if (!trackId) return;
            const track = AppState.tracks.find(t => t.id === trackId); if (!track) return;
            const newMute = !track.mute;
            if (window.AudioEngine && AudioEngine.setTrackMute) {
                AudioEngine.setTrackMute(trackId, newMute);
            }
            track.mute = newMute;
            updateInspectorPanel();
            if (window.ArrangementView && ArrangementView.updateTrackHeader) {
                ArrangementView.updateTrackHeader(trackId);
            }
            const mixer = window.UIManager?.getComponent ? UIManager.getComponent('mixer') : null;
            if (mixer && mixer.createMixerChannels) mixer.createMixerChannels();
        });
    }
    if (soloBtn) {
        soloBtn.addEventListener('click', () => {
            const trackId = AppState.selectedTrackId; if (!trackId) return;
            const track = AppState.tracks.find(t => t.id === trackId); if (!track) return;
            const newSolo = !track.solo;
            if (window.AudioEngine && AudioEngine.setTrackSolo) {
                AudioEngine.setTrackSolo(trackId, newSolo);
            }
            track.solo = newSolo;
            updateInspectorPanel();
            if (window.ArrangementView && ArrangementView.updateTrackHeader) {
                ArrangementView.updateTrackHeader(trackId);
            }
            const mixer = window.UIManager?.getComponent ? UIManager.getComponent('mixer') : null;
            if (mixer && mixer.createMixerChannels) mixer.createMixerChannels();
        });
    }
    if (armBtn) {
        armBtn.addEventListener('click', () => {
            const trackId = AppState.selectedTrackId; if (!trackId) return;
            const track = AppState.tracks.find(t => t.id === trackId); if (!track) return;
            track.armed = !track.armed;
            updateInspectorPanel();
            if (window.ArrangementView && ArrangementView.updateTrackHeader) {
                ArrangementView.updateTrackHeader(trackId);
            }
        });
    }
}

// Toggle inspector sidebar collapsed state
function toggleInspectorPanel() {
    const inspectorPanel = document.getElementById('inspector-panel');
    if (!inspectorPanel) return;
    const isCollapsed = inspectorPanel.classList.contains('collapsed');
    if (isCollapsed) {
        inspectorPanel.classList.remove('collapsed');
        inspectorPanel.setAttribute('aria-hidden', 'false');
    } else {
        inspectorPanel.classList.add('collapsed');
        inspectorPanel.setAttribute('aria-hidden', 'true');
    }
    const toggle = inspectorPanel.querySelector('.panel-toggle');
    if (toggle) {
        toggle.textContent = inspectorPanel.classList.contains('collapsed') ? '+' : '−';
        toggle.setAttribute('aria-expanded', inspectorPanel.classList.contains('collapsed') ? 'false' : 'true');
    }
}

// Toggle audio monitor visibility
function toggleAudioMonitor() {
    const audioMon = document.getElementById('audio-monitor');
    if (!audioMon) return;
    audioMon.classList.toggle('collapsed');
}

// Drive audio monitor meters using AudioEngine.getStereoLevels()
function startAudioMonitorLoop() {
    const leftFill = document.getElementById('meter-left');
    const rightFill = document.getElementById('meter-right');
    const leftVal = document.getElementById('meter-left-value');
    const rightVal = document.getElementById('meter-right-value');

    function updateMeters() {
        if (!(window.AudioEngine && AudioEngine.getStereoLevels)) {
            requestAnimationFrame(updateMeters);
            return;
        }

        const { left, right } = AudioEngine.getStereoLevels();
        // Map dB (-Infinity..0) to 0..100%
        const mapDbToPct = (db) => {
            const minDb = -60; // floor
            if (!isFinite(db)) return 0;
            const clamped = Math.max(minDb, Math.min(0, db));
            return Math.round(((clamped - minDb) / (0 - minDb)) * 100);
        };

        const lp = mapDbToPct(left);
        const rp = mapDbToPct(right);

        if (leftFill) leftFill.style.height = `${lp}%`;
        if (rightFill) rightFill.style.height = `${rp}%`;
        if (leftVal) leftVal.textContent = isFinite(left) ? `${left.toFixed(1)} dB` : '-∞ dB';
        if (rightVal) rightVal.textContent = isFinite(right) ? `${right.toFixed(1)} dB` : '-∞ dB';

        requestAnimationFrame(updateMeters);
    }

    requestAnimationFrame(updateMeters);
}

// Create default tracks
function createDefaultTracks() {
    const defaultTracks = [
        { id: 1, name: 'Kick', type: 'audio', color: '#ff4444', volume: 0.8, panValue: 0, mute: false, solo: false, armed: false, input: 'Mic 1' },
        { id: 2, name: 'Snare', type: 'audio', color: '#44ff44', volume: 0.8, panValue: 0, mute: false, solo: false, armed: false, input: 'Mic 2' },
        { id: 3, name: 'Bass', type: 'audio', color: '#4444ff', volume: 0.8, panValue: 0, mute: false, solo: false, armed: false, input: 'DI 1' },
        { id: 4, name: 'Lead', type: 'audio', color: '#ffff44', volume: 0.8, panValue: 0, mute: false, solo: false, armed: false, input: 'Mic 3' },
        { id: 5, name: 'Drums', type: 'audio', color: '#ff44ff', volume: 0.8, panValue: 0, mute: false, solo: false, armed: false, input: 'Mic 4' },
        { id: 6, name: 'Piano', type: 'midi', color: '#44ffff', volume: 0.8, panValue: 0, mute: false, solo: false, armed: false, input: 'MIDI 1' },
        { id: 7, name: 'Strings', type: 'audio', color: '#ffffff', volume: 0.8, panValue: 0, mute: false, solo: false, armed: false, input: 'Mic 5' },
        { id: 8, name: 'FX', type: 'audio', color: '#888888', volume: 0.8, panValue: 0, mute: false, solo: false, armed: false, input: 'Return 1' }
    ];

    AppState.tracks = defaultTracks;
}

// Create default scenes
function createDefaultScenes() {
    const defaultScenes = [
        { id: 1, name: 'Intro', color: '#ff4444' },
        { id: 2, name: 'Verse', color: '#44ff44' },
        { id: 3, name: 'Chorus', color: '#4444ff' },
        { id: 4, name: 'Bridge', color: '#ffff44' }
    ];

    AppState.scenes = defaultScenes;
}

// Switch between views
function switchView(viewName) {
    if (AppState.currentView === viewName) return;

    // Update state
    AppState.currentView = viewName;

    // Update UI
    DOM.viewTabs.forEach(tab => {
        tab.classList.toggle('active', tab.dataset.view === viewName);
    });

    // Hide all views
    document.querySelectorAll('.view-container').forEach(container => {
        container.classList.remove('active');
    });

    // Show selected view
    const selectedView = document.getElementById(`${viewName}-view`);
    if (selectedView) {
        selectedView.classList.add('active');
    }

    // Notify view-specific handlers
    switch (viewName) {
        case 'arrangement':
            ArrangementView.onActivate();
            break;
        case 'live':
            LiveView.onActivate();
            break;
        case 'node':
            NodeView.onActivate();
            break;
    }

    console.log(`🔄 Switched to ${viewName} view`);
}

// Handle transport actions
function handleTransportAction(action) {
    switch (action) {
        case 'play-btn':
            togglePlayback();
            break;
        case 'pause-btn':
            pausePlayback();
            break;
        case 'stop-btn':
            stopPlayback();
            break;
        case 'record-btn':
            toggleRecording();
            break;
        case 'rewind-btn':
            rewindToStart();
            break;
    }
}

// Toggle playback
function togglePlayback() {
    AppState.isPlaying = !AppState.isPlaying;

    const playBtn = document.getElementById('play-btn');
    if (AppState.isPlaying) {
        playBtn.textContent = '⏸';
        playBtn.title = 'Pause';
        AudioEngine.startPlayback();
        startTimeUpdates();
    } else {
        playBtn.textContent = '▶';
        playBtn.title = 'Play';
        AudioEngine.pausePlayback();
        stopTimeUpdates();
    }

    updateTransportUI();
}

// Pause playback
function pausePlayback() {
    if (AppState.isPlaying) {
        togglePlayback();
    }
}

// Stop playback
function stopPlayback() {
    AppState.isPlaying = false;
    AppState.currentTime = 0;

    const playBtn = document.getElementById('play-btn');
    playBtn.textContent = '▶';
    playBtn.title = 'Play';

    AudioEngine.stopPlayback();
    stopTimeUpdates();
    updateUI();
}

// Toggle recording
function toggleRecording() {
    AppState.isRecording = !AppState.isRecording;

    const recordBtn = document.getElementById('record-btn');
    if (AppState.isRecording) {
        recordBtn.style.color = '#ff4444';
        AudioEngine.startRecording();
    } else {
        recordBtn.style.color = '';
        AudioEngine.stopRecording();
    }

    updateTransportUI();
}

// Rewind to start
function rewindToStart() {
    AppState.currentTime = 0;
    AudioEngine.seekTo(0);
    updateUI();
}

// Handle keyboard shortcuts
function handleKeyboardShortcut(event) {
    // Don't handle shortcuts when typing in inputs
    if (event.target.tagName === 'INPUT' || event.target.tagName === 'TEXTAREA') {
        return;
    }

    switch (event.code) {
        case 'Space':
            event.preventDefault();
            togglePlayback();
            break;
        case 'KeyR':
            if (event.ctrlKey || event.metaKey) {
                event.preventDefault();
                toggleRecording();
            }
            break;
        case 'KeyS':
            if (event.ctrlKey || event.metaKey) {
                event.preventDefault();
                stopPlayback();
            }
            break;
        case 'Digit1':
            event.preventDefault();
            switchView('arrangement');
            break;
        case 'Digit2':
            event.preventDefault();
            switchView('live');
            break;
        case 'Digit3':
            event.preventDefault();
            switchView('node');
            break;
    }
}

// Handle window resize
function handleWindowResize() {
    // Update canvas sizes and layouts
    ArrangementView.onResize();
    LiveView.onResize();
    NodeView.onResize();
}

// Handle before unload
function handleBeforeUnload(event) {
    // Save current state
    saveAppState();

    // Show confirmation dialog for unsaved changes
    if (hasUnsavedChanges()) {
        event.preventDefault();
        event.returnValue = 'You have unsaved changes. Are you sure you want to leave?';
    }
}

// Save application state
function saveAppState() {
    const state = {
        currentView: AppState.currentView,
        tempo: AppState.tempo,
        timeSignature: AppState.timeSignature,
        tracks: AppState.tracks,
        scenes: AppState.scenes,
        nodes: AppState.nodes,
        connections: AppState.connections,
        zoom: AppState.zoom
    };

    localStorage.setItem('modurust-daw-state', JSON.stringify(state));
}

// Load application state
function loadAppState() {
    try {
        const savedState = localStorage.getItem('modurust-daw-state');
        if (savedState) {
            const state = JSON.parse(savedState);
            Object.assign(AppState, state);
            console.log('✅ Application state loaded');
        }
    } catch (error) {
        console.warn('⚠️ Failed to load application state:', error);
    }
}

// Check for unsaved changes
function hasUnsavedChanges() {
    // Implement logic to check for unsaved changes
    return false; // Placeholder
}

// Update UI elements
function updateUI() {
    updateTransportUI();
    updateTimeDisplay();
    updateStatusIndicators();
}

// Update transport UI
function updateTransportUI() {
    const playBtn = document.getElementById('play-btn');
    const recordBtn = document.getElementById('record-btn');

    if (AppState.isPlaying) {
        playBtn.textContent = '⏸';
        playBtn.title = 'Pause';
    } else {
        playBtn.textContent = '▶';
        playBtn.title = 'Play';
    }

    if (AppState.isRecording) {
        recordBtn.style.color = '#ff4444';
    } else {
        recordBtn.style.color = '';
    }
}

// Update time display
function updateTimeDisplay() {
    const timeString = formatTime(AppState.currentTime || 0);
    DOM.currentTime.textContent = timeString;
    DOM.playheadPosition.textContent = timeString;
}

// Format time as MM:SS.mmm
function formatTime(seconds) {
    const mins = Math.floor(seconds / 60);
    const secs = Math.floor(seconds % 60);
    const ms = Math.floor((seconds % 1) * 1000);
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}.${ms.toString().padStart(3, '0')}`;
}

// Update status indicators
function updateStatusIndicators() {
    // Update audio status
    const audioStatus = document.querySelector('.status-item:nth-child(1) .status-text');
    if (audioStatus) {
        audioStatus.textContent = AppState.audioContext?.state === 'running' ? 'Audio: Ready' : 'Audio: Suspended';
    }

    // Update EEG status (placeholder)
    const eegStatus = document.querySelector('.status-item:nth-child(2) .status-text');
    if (eegStatus) {
        eegStatus.textContent = 'EEG: Disconnected'; // Will be updated when EEG integration is added
    }
}

// Start time updates
function startTimeUpdates() {
    if (AppState.timeUpdateInterval) {
        clearInterval(AppState.timeUpdateInterval);
    }

    AppState.timeUpdateInterval = setInterval(() => {
        if (AppState.isPlaying) {
            AppState.currentTime += 1 / 60; // 60 FPS updates
            updateTimeDisplay();
        }
    }, 1000 / 60);
}

// Stop time updates
function stopTimeUpdates() {
    if (AppState.timeUpdateInterval) {
        clearInterval(AppState.timeUpdateInterval);
        AppState.timeUpdateInterval = null;
    }
}

// Show settings modal
function showSettingsModal() {
    const modal = document.getElementById('settings-modal');
    modal.classList.add('active');
}

// Show loading overlay
function showLoading(show) {
    DOM.loadingOverlay.style.display = show ? 'flex' : 'none';
}

// Show error message
function showError(message) {
    console.error('❌ Error:', message);
    // TODO: Implement proper error notification system
    alert('Error: ' + message);
}

// Utility functions
function clamp(value, min, max) {
    return Math.min(Math.max(value, min), max);
}

function lerp(a, b, t) {
    return a + (b - a) * t;
}

function debounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
        const later = () => {
            clearTimeout(timeout);
            func(...args);
        };
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
    };
}

// Initialize application when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    initApp();
});

// Show welcome message
function showWelcomeMessage() {
    const uiType = AppState.useBevyUI ? 'Bevy UI' : 'JavaScript UI';
    const message = `
🎵 Welcome to Modurust DAW!

Using: ${uiType}
A revolutionary digital audio workstation featuring:
• Three-view interface (Arrangement, Live, Node)
• Real-time audio processing with Web Audio API
• WebGL-powered visualizations
• EEG and motion capture integration
• Professional accessibility features

Use Ctrl+1/2/3 to switch views, Space to play/pause.
`;

    console.log(message);

    // Show in UI if available
    if (!AppState.useBevyUI) {
        setTimeout(() => {
            if (window.UIManager && window.UIManager.showNotification) {
                window.UIManager.showNotification(`Welcome to Modurust DAW with ${uiType}! Press F1 for help.`, 'info');
            }
        }, 1000);
    }
}

// Export global functions for debugging
window.ModurustDAW = {
    getState: () => AppState,
    switchView,
    togglePlayback,
    stopPlayback,
    toggleRecording,
    saveState: saveAppState,
    loadState: loadAppState,
    isUsingBevyUI: () => AppState.useBevyUI,
    getBevyUI: () => AppState.bevyUI,
    selectTrack
};