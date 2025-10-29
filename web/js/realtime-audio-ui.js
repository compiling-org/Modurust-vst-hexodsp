/**
 * Modurust DAW - Real-time Audio Processing UI
 * Advanced audio monitoring, effects, and processing controls
 */

class RealtimeAudioUI {
    static isInitialized = false;
    static effectsChain = new Map();
    static audioMeters = new Map();
    static spectrumAnalyzer = null;
    static waveformDisplay = null;

    // Initialize real-time audio UI
    static init() {
        console.log('🎛️ Initializing Real-time Audio UI...');

        // Create audio processing panel
        this.createAudioProcessingPanel();

        // Initialize spectrum analyzer
        this.initializeSpectrumAnalyzer();

        // Initialize waveform display
        this.initializeWaveformDisplay();

        // Setup audio meters
        this.setupAudioMeters();

        // Initialize effects controls
        this.initializeEffectsControls();

        this.isInitialized = true;
        console.log('✅ Real-time Audio UI initialized');
    }

    // Create audio processing panel
    static createAudioProcessingPanel() {
        const audioPanel = document.createElement('div');
        audioPanel.id = 'audio-processing-panel';
        audioPanel.className = 'audio-processing-panel';
        audioPanel.innerHTML = `
            <div class="panel-header">
                <h3>Audio Processing</h3>
                <div class="audio-status">
                    <span class="status-indicator" id="audio-engine-status"></span>
                    <span class="status-text" id="audio-engine-text">Ready</span>
                </div>
            </div>
            <div class="panel-content">
                <div class="audio-monitors">
                    <div class="spectrum-container">
                        <h4>Spectrum Analyzer</h4>
                        <canvas id="spectrum-canvas" width="400" height="150"></canvas>
                        <div class="spectrum-controls">
                            <button class="spectrum-btn" id="spectrum-freeze">Freeze</button>
                            <button class="spectrum-btn" id="spectrum-reset">Reset</button>
                            <select id="spectrum-scale">
                                <option value="linear">Linear</option>
                                <option value="log">Logarithmic</option>
                            </select>
                        </div>
                    </div>

                    <div class="waveform-container">
                        <h4>Waveform</h4>
                        <canvas id="waveform-canvas" width="400" height="100"></canvas>
                        <div class="waveform-controls">
                            <button class="waveform-btn" id="waveform-hold">Hold</button>
                            <button class="waveform-btn" id="waveform-trigger">Trigger</button>
                        </div>
                    </div>

                    <div class="level-meters">
                        <h4>Level Meters</h4>
                        <div class="meter-container">
                            <div class="audio-meter" id="input-meter">
                                <div class="meter-label">Input</div>
                                <div class="meter-bar">
                                    <div class="meter-fill" id="input-fill"></div>
                                </div>
                                <div class="meter-value" id="input-value">-∞ dB</div>
                            </div>
                            <div class="audio-meter" id="output-meter">
                                <div class="meter-label">Output</div>
                                <div class="meter-bar">
                                    <div class="meter-fill" id="output-fill"></div>
                                </div>
                                <div class="meter-value" id="output-value">-∞ dB</div>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="effects-chain">
                    <h4>Effects Chain</h4>
                    <div class="effects-list" id="effects-list">
                        <!-- Effects will be populated dynamically -->
                    </div>
                    <div class="effects-controls">
                        <button class="effect-btn" id="add-effect-btn">Add Effect</button>
                        <button class="effect-btn" id="clear-effects-btn">Clear All</button>
                    </div>
                </div>

                <div class="master-controls">
                    <h4>Master Section</h4>
                    <div class="master-params">
                        <div class="param-group">
                            <label>Volume</label>
                            <input type="range" id="master-volume" min="0" max="100" value="80" step="1">
                            <span class="param-value" id="master-volume-value">80%</span>
                        </div>
                        <div class="param-group">
                            <label>Pan</label>
                            <input type="range" id="master-pan" min="-50" max="50" value="0" step="1">
                            <span class="param-value" id="master-pan-value">0</span>
                        </div>
                        <div class="param-group">
                            <label>Input Gain</label>
                            <input type="range" id="input-gain" min="-24" max="24" value="0" step="0.1">
                            <span class="param-value" id="input-gain-value">0 dB</span>
                        </div>
                    </div>
                </div>

                <div class="audio-routing">
                    <h4>Audio Routing</h4>
                    <div class="routing-matrix" id="routing-matrix">
                        <!-- Routing matrix will be populated dynamically -->
                    </div>
                </div>
            </div>
        `;

        // Add to DOM (you might want to add this to a specific container)
        const liveView = document.getElementById('live-view');
        if (liveView) {
            liveView.appendChild(audioPanel);
        }

        // Setup event listeners
        this.setupEventListeners();
    }

    // Setup event listeners
    static setupEventListeners() {
        // Spectrum controls
        const spectrumFreeze = document.getElementById('spectrum-freeze');
        const spectrumReset = document.getElementById('spectrum-reset');
        const spectrumScale = document.getElementById('spectrum-scale');

        if (spectrumFreeze) {
            spectrumFreeze.addEventListener('click', () => this.toggleSpectrumFreeze());
        }
        if (spectrumReset) {
            spectrumReset.addEventListener('click', () => this.resetSpectrum());
        }
        if (spectrumScale) {
            spectrumScale.addEventListener('change', (e) => this.setSpectrumScale(e.target.value));
        }

        // Waveform controls
        const waveformHold = document.getElementById('waveform-hold');
        const waveformTrigger = document.getElementById('waveform-trigger');

        if (waveformHold) {
            waveformHold.addEventListener('click', () => this.toggleWaveformHold());
        }
        if (waveformTrigger) {
            waveformTrigger.addEventListener('click', () => this.triggerWaveform());
        }

        // Master controls
        const masterVolume = document.getElementById('master-volume');
        const masterPan = document.getElementById('master-pan');
        const inputGain = document.getElementById('input-gain');

        if (masterVolume) {
            masterVolume.addEventListener('input', (e) => this.setMasterVolume(e.target.value));
        }
        if (masterPan) {
            masterPan.addEventListener('input', (e) => this.setMasterPan(e.target.value));
        }
        if (inputGain) {
            inputGain.addEventListener('input', (e) => this.setInputGain(e.target.value));
        }

        // Effects controls
        const addEffectBtn = document.getElementById('add-effect-btn');
        const clearEffectsBtn = document.getElementById('clear-effects-btn');

        if (addEffectBtn) {
            addEffectBtn.addEventListener('click', () => this.showAddEffectDialog());
        }
        if (clearEffectsBtn) {
            clearEffectsBtn.addEventListener('click', () => this.clearAllEffects());
        }
    }

    // Initialize spectrum analyzer
    static initializeSpectrumAnalyzer() {
        const canvas = document.getElementById('spectrum-canvas');
        if (!canvas) return;

        const ctx = canvas.getContext('2d');
        this.spectrumCtx = ctx;
        this.spectrumCanvas = canvas;
        this.spectrumFrozen = false;
        this.spectrumScale = 'linear';
        this.frozenData = null;

        // Start spectrum rendering
        this.renderSpectrum();
    }

    // Initialize waveform display
    static initializeWaveformDisplay() {
        const canvas = document.getElementById('waveform-canvas');
        if (!canvas) return;

        const ctx = canvas.getContext('2d');
        this.waveformCtx = ctx;
        this.waveformCanvas = canvas;
        this.waveformHold = false;
        this.waveformTrigger = false;
        this.heldData = null;

        // Start waveform rendering
        this.renderWaveform();
    }

    // Setup audio meters
    static setupAudioMeters() {
        this.audioMeters.set('input', {
            element: document.getElementById('input-fill'),
            valueElement: document.getElementById('input-value'),
            level: -Infinity,
            peak: -Infinity,
            peakHoldTime: 0
        });

        this.audioMeters.set('output', {
            element: document.getElementById('output-fill'),
            valueElement: document.getElementById('output-value'),
            level: -Infinity,
            peak: -Infinity,
            peakHoldTime: 0
        });
    }

    // Initialize effects controls
    static initializeEffectsControls() {
        // Create default effects chain
        this.addEffect('reverb', 'Reverb');
        this.addEffect('delay', 'Delay');
        this.addEffect('filter', 'Filter');

        this.updateEffectsList();
        this.createRoutingMatrix();
    }

    // Render spectrum analyzer
    static renderSpectrum() {
        if (!this.spectrumCtx || !this.isInitialized) return;

        const ctx = this.spectrumCtx;
        const canvas = this.spectrumCanvas;

        // Clear canvas
        ctx.fillStyle = 'var(--primary-bg)';
        ctx.fillRect(0, 0, canvas.width, canvas.height);

        // Get frequency data
        let frequencyData;
        if (this.spectrumFrozen && this.frozenData) {
            frequencyData = this.frozenData;
        } else {
            frequencyData = AudioEngine.getFrequencyData();
            if (this.spectrumFrozen) {
                this.frozenData = new Uint8Array(frequencyData);
            }
        }

        // Draw spectrum
        ctx.strokeStyle = 'var(--accent-color)';
        ctx.lineWidth = 2;
        ctx.beginPath();

        const sliceWidth = canvas.width / frequencyData.length;
        let x = 0;

        for (let i = 0; i < frequencyData.length; i++) {
            let value = frequencyData[i];
            if (this.spectrumScale === 'log') {
                // Apply logarithmic scaling
                const logIndex = Math.log(i + 1) / Math.log(frequencyData.length);
                const logValueIndex = Math.floor(logIndex * frequencyData.length);
                value = frequencyData[Math.min(logValueIndex, frequencyData.length - 1)];
            }

            const y = (value / 255) * canvas.height;

            if (i === 0) {
                ctx.moveTo(x, canvas.height - y);
            } else {
                ctx.lineTo(x, canvas.height - y);
            }

            x += sliceWidth;
        }

        ctx.stroke();

        // Draw frequency labels
        ctx.fillStyle = 'var(--text-secondary)';
        ctx.font = '10px Arial';
        ctx.fillText('20Hz', 10, canvas.height - 5);
        ctx.fillText('20kHz', canvas.width - 35, canvas.height - 5);

        // Continue rendering
        requestAnimationFrame(() => this.renderSpectrum());
    }

    // Render waveform display
    static renderWaveform() {
        if (!this.waveformCtx || !this.isInitialized) return;

        const ctx = this.waveformCtx;
        const canvas = this.waveformCanvas;

        // Clear canvas
        ctx.fillStyle = 'var(--primary-bg)';
        ctx.fillRect(0, 0, canvas.width, canvas.height);

        // Get waveform data
        let waveformData;
        if (this.waveformHold && this.heldData) {
            waveformData = this.heldData;
        } else {
            waveformData = AudioEngine.getTimeDomainData();
            if (this.waveformHold) {
                this.heldData = new Uint8Array(waveformData);
            }
        }

        // Draw waveform
        ctx.strokeStyle = 'var(--accent-color)';
        ctx.lineWidth = 1;
        ctx.beginPath();

        const sliceWidth = canvas.width / waveformData.length;
        let x = 0;

        for (let i = 0; i < waveformData.length; i++) {
            const value = waveformData[i];
            const y = ((value - 128) / 128) * (canvas.height / 2) + (canvas.height / 2);

            if (i === 0) {
                ctx.moveTo(x, y);
            } else {
                ctx.lineTo(x, y);
            }

            x += sliceWidth;
        }

        ctx.stroke();

        // Draw center line
        ctx.strokeStyle = 'var(--border-color)';
        ctx.lineWidth = 1;
        ctx.beginPath();
        ctx.moveTo(0, canvas.height / 2);
        ctx.lineTo(canvas.width, canvas.height / 2);
        ctx.stroke();

        // Continue rendering
        requestAnimationFrame(() => this.renderWaveform());
    }

    // Update audio meters
    static updateAudioMeters() {
        if (!this.isInitialized) return;

        // Simulate meter levels (in a real implementation, these would come from audio analysis)
        const time = Date.now() / 1000;

        this.audioMeters.forEach((meter, name) => {
            // Generate realistic meter levels
            let level = -60 + Math.sin(time * 2) * 20 + Math.random() * 10;
            level = Math.max(-60, Math.min(0, level));

            // Update peak
            if (level > meter.peak) {
                meter.peak = level;
                meter.peakHoldTime = Date.now();
            }

            // Reset peak after 3 seconds
            if (Date.now() - meter.peakHoldTime > 3000) {
                meter.peak = level;
            }

            meter.level = level;

            // Update UI
            const percentage = ((level + 60) / 60) * 100;
            meter.element.style.height = `${Math.max(0, percentage)}%`;

            // Color based on level
            if (level > -6) {
                meter.element.style.backgroundColor = 'var(--error-color)';
            } else if (level > -18) {
                meter.element.style.backgroundColor = 'var(--warning-color)';
            } else {
                meter.element.style.backgroundColor = 'var(--success-color)';
            }

            // Update text
            const dbValue = level.toFixed(1);
            meter.valueElement.textContent = `${dbValue} dB`;
        });
    }

    // Toggle spectrum freeze
    static toggleSpectrumFreeze() {
        this.spectrumFrozen = !this.spectrumFrozen;

        const freezeBtn = document.getElementById('spectrum-freeze');
        if (freezeBtn) {
            freezeBtn.textContent = this.spectrumFrozen ? 'Unfreeze' : 'Freeze';
            freezeBtn.style.backgroundColor = this.spectrumFrozen ? 'var(--warning-color)' : '';
        }

        if (!this.spectrumFrozen) {
            this.frozenData = null;
        }
    }

    // Reset spectrum
    static resetSpectrum() {
        this.spectrumFrozen = false;
        this.frozenData = null;

        const freezeBtn = document.getElementById('spectrum-freeze');
        if (freezeBtn) {
            freezeBtn.textContent = 'Freeze';
            freezeBtn.style.backgroundColor = '';
        }
    }

    // Set spectrum scale
    static setSpectrumScale(scale) {
        this.spectrumScale = scale;
        console.log(`Spectrum scale set to: ${scale}`);
    }

    // Toggle waveform hold
    static toggleWaveformHold() {
        this.waveformHold = !this.waveformHold;

        const holdBtn = document.getElementById('waveform-hold');
        if (holdBtn) {
            holdBtn.textContent = this.waveformHold ? 'Release' : 'Hold';
            holdBtn.style.backgroundColor = this.waveformHold ? 'var(--warning-color)' : '';
        }

        if (!this.waveformHold) {
            this.heldData = null;
        }
    }

    // Trigger waveform
    static triggerWaveform() {
        this.waveformTrigger = true;
        console.log('Waveform triggered');

        // Reset trigger after a short delay
        setTimeout(() => {
            this.waveformTrigger = false;
        }, 100);
    }

    // Set master volume
    static setMasterVolume(value) {
        const percentage = value;
        AudioEngine.setMasterVolume(percentage / 100);

        const valueElement = document.getElementById('master-volume-value');
        if (valueElement) {
            valueElement.textContent = `${percentage}%`;
        }
    }

    // Set master pan
    static setMasterPan(value) {
        // Convert -50..50 to -1..1
        const panValue = value / 50;

        const valueElement = document.getElementById('master-pan-value');
        if (valueElement) {
            valueElement.textContent = value > 0 ? `R${value}` : value < 0 ? `L${Math.abs(value)}` : 'C';
        }

        console.log(`Master pan set to: ${panValue.toFixed(2)}`);
    }

    // Set input gain
    static setInputGain(value) {
        const dbValue = parseFloat(value);

        const valueElement = document.getElementById('input-gain-value');
        if (valueElement) {
            valueElement.textContent = `${dbValue.toFixed(1)} dB`;
        }

        console.log(`Input gain set to: ${dbValue.toFixed(1)} dB`);
    }

    // Add effect to chain
    static addEffect(type, name) {
        const effectId = `effect_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;

        const effect = {
            id: effectId,
            type,
            name,
            enabled: true,
            parameters: this.getDefaultEffectParameters(type),
            order: this.effectsChain.size
        };

        this.effectsChain.set(effectId, effect);

        // Add to audio engine
        AudioEngine.createEffect(type, name);

        console.log(`➕ Added effect: ${name}`);
    }

    // Get default effect parameters
    static getDefaultEffectParameters(type) {
        const defaults = {
            reverb: { size: 0.5, damping: 0.5, mix: 0.3 },
            delay: { time: 0.3, feedback: 0.4, mix: 0.2 },
            filter: { frequency: 1000, resonance: 1, type: 'lowpass' },
            compressor: { threshold: -24, ratio: 4, attack: 0.003, release: 0.25 },
            equalizer: { low: 0, mid: 0, high: 0, presence: 0 }
        };

        return defaults[type] || {};
    }

    // Update effects list UI
    static updateEffectsList() {
        const effectsList = document.getElementById('effects-list');
        if (!effectsList) return;

        effectsList.innerHTML = '';

        // Sort effects by order
        const sortedEffects = Array.from(this.effectsChain.values()).sort((a, b) => a.order - b.order);

        sortedEffects.forEach(effect => {
            const effectItem = document.createElement('div');
            effectItem.className = 'effect-item';
            effectItem.innerHTML = `
                <div class="effect-header">
                    <span class="effect-name">${effect.name}</span>
                    <div class="effect-controls">
                        <button class="effect-toggle ${effect.enabled ? 'active' : ''}" data-effect-id="${effect.id}">
                            ${effect.enabled ? 'ON' : 'OFF'}
                        </button>
                        <button class="effect-edit" data-effect-id="${effect.id}">⚙️</button>
                        <button class="effect-remove" data-effect-id="${effect.id}">×</button>
                    </div>
                </div>
                <div class="effect-params" id="params-${effect.id}">
                    ${this.createEffectParametersUI(effect)}
                </div>
            `;

            effectsList.appendChild(effectItem);
        });

        // Add event listeners
        effectsList.addEventListener('click', (e) => {
            const effectId = e.target.dataset.effectId;
            if (!effectId) return;

            if (e.target.classList.contains('effect-toggle')) {
                this.toggleEffect(effectId);
            } else if (e.target.classList.contains('effect-edit')) {
                this.editEffect(effectId);
            } else if (e.target.classList.contains('effect-remove')) {
                this.removeEffect(effectId);
            }
        });

        // Add parameter change listeners
        sortedEffects.forEach(effect => {
            const paramsContainer = document.getElementById(`params-${effect.id}`);
            if (paramsContainer) {
                const inputs = paramsContainer.querySelectorAll('input');
                inputs.forEach(input => {
                    input.addEventListener('input', (e) => {
                        this.updateEffectParameter(effect.id, e.target.name, e.target.value);
                    });
                });
            }
        });
    }

    // Create effect parameters UI
    static createEffectParametersUI(effect) {
        let html = '';

        Object.entries(effect.parameters).forEach(([param, value]) => {
            const paramName = param.charAt(0).toUpperCase() + param.slice(1);
            const inputType = typeof value === 'number' && value <= 1 && value >= 0 ? 'range' : 'number';
            const min = inputType === 'range' ? '0' : '';
            const max = inputType === 'range' ? '1' : '';
            const step = inputType === 'range' ? '0.01' : '0.1';

            html += `
                <div class="effect-param">
                    <label>${paramName}</label>
                    <input type="${inputType}" name="${param}" value="${value}"
                           min="${min}" max="${max}" step="${step}">
                    <span class="param-display">${value}</span>
                </div>
            `;
        });

        return html;
    }

    // Toggle effect
    static toggleEffect(effectId) {
        const effect = this.effectsChain.get(effectId);
        if (effect) {
            effect.enabled = !effect.enabled;
            AudioEngine.setEffectEnabled(effect.type, effect.enabled);
            this.updateEffectsList();
            console.log(`Effect ${effect.name} ${effect.enabled ? 'enabled' : 'disabled'}`);
        }
    }

    // Edit effect
    static editEffect(effectId) {
        const effect = this.effectsChain.get(effectId);
        if (!effect) return;

        // Toggle parameter visibility
        const paramsContainer = document.getElementById(`params-${effectId}`);
        if (paramsContainer) {
            paramsContainer.classList.toggle('visible');
        }
    }

    // Remove effect
    static removeEffect(effectId) {
        const effect = this.effectsChain.get(effectId);
        if (effect) {
            this.effectsChain.delete(effectId);
            this.updateEffectsList();
            console.log(`🗑️ Removed effect: ${effect.name}`);
        }
    }

    // Update effect parameter
    static updateEffectParameter(effectId, param, value) {
        const effect = this.effectsChain.get(effectId);
        if (effect) {
            effect.parameters[param] = parseFloat(value);

            // Update display
            const paramsContainer = document.getElementById(`params-${effectId}`);
            if (paramsContainer) {
                const display = paramsContainer.querySelector(`[name="${param}"] + .param-display`);
                if (display) {
                    display.textContent = parseFloat(value).toFixed(2);
                }
            }

            console.log(`Updated ${effect.name} ${param}: ${value}`);
        }
    }

    // Show add effect dialog
    static showAddEffectDialog() {
        const dialog = document.createElement('div');
        dialog.className = 'modal active';
        dialog.innerHTML = `
            <div class="modal-content">
                <div class="modal-header">
                    <h2>Add Effect</h2>
                    <button class="modal-close">&times;</button>
                </div>
                <div class="modal-body">
                    <div class="form-group">
                        <label>Effect Type:</label>
                        <select id="effect-type-select">
                            <option value="reverb">Reverb</option>
                            <option value="delay">Delay</option>
                            <option value="filter">Filter</option>
                            <option value="compressor">Compressor</option>
                            <option value="equalizer">Equalizer</option>
                            <option value="distortion">Distortion</option>
                            <option value="chorus">Chorus</option>
                            <option value="phaser">Phaser</option>
                        </select>
                    </div>
                    <div class="form-group">
                        <label>Effect Name:</label>
                        <input type="text" id="effect-name-input" placeholder="Enter effect name">
                    </div>
                </div>
                <div class="modal-footer">
                    <button class="btn-secondary" id="cancel-effect">Cancel</button>
                    <button class="btn-primary" id="add-effect">Add Effect</button>
                </div>
            </div>
        `;

        document.body.appendChild(dialog);

        // Setup dialog events
        const closeBtn = dialog.querySelector('.modal-close');
        const cancelBtn = dialog.querySelector('#cancel-effect');
        const addBtn = dialog.querySelector('#add-effect');
        const nameInput = dialog.querySelector('#effect-name-input');

        const closeDialog = () => dialog.remove();

        closeBtn.addEventListener('click', closeDialog);
        cancelBtn.addEventListener('click', closeDialog);

        addBtn.addEventListener('click', () => {
            const type = dialog.querySelector('#effect-type-select').value;
            const name = nameInput.value.trim() || type.charAt(0).toUpperCase() + type.slice(1);

            this.addEffect(type, name);
            this.updateEffectsList();
            closeDialog();

            UIManager.showNotification('Effect added successfully', 'success');
        });
    }

    // Clear all effects
    static clearAllEffects() {
        if (confirm('Are you sure you want to clear all effects?')) {
            this.effectsChain.clear();
            this.updateEffectsList();
            console.log('🗑️ Cleared all effects');
        }
    }

    // Create routing matrix
    static createRoutingMatrix() {
        const routingMatrix = document.getElementById('routing-matrix');
        if (!routingMatrix) return;

        routingMatrix.innerHTML = '';

        // Create a simple routing matrix for 8 tracks
        const tracks = ['Track 1', 'Track 2', 'Track 3', 'Track 4', 'Track 5', 'Track 6', 'Track 7', 'Track 8'];

        tracks.forEach((track, index) => {
            const trackRow = document.createElement('div');
            trackRow.className = 'routing-row';
            trackRow.innerHTML = `
                <div class="routing-track">${track}</div>
                <div class="routing-outputs">
                    <label class="routing-checkbox">
                        <input type="checkbox" checked data-track="${index}" data-output="main">
                        Main
                    </label>
                    <label class="routing-checkbox">
                        <input type="checkbox" data-track="${index}" data-output="send1">
                        Send 1
                    </label>
                    <label class="routing-checkbox">
                        <input type="checkbox" data-track="${index}" data-output="send2">
                        Send 2
                    </label>
                </div>
            `;

            routingMatrix.appendChild(trackRow);
        });

        // Add event listeners
        routingMatrix.addEventListener('change', (e) => {
            if (e.target.type === 'checkbox') {
                const track = e.target.dataset.track;
                const output = e.target.dataset.output;
                const enabled = e.target.checked;

                console.log(`Routing: Track ${track} ${enabled ? '→' : '✗'} ${output}`);
            }
        });
    }

    // Update UI (called by main update loop)
    static update() {
        if (!this.isInitialized) return;

        this.updateAudioMeters();
    }

    // Dispose resources
    static dispose() {
        this.effectsChain.clear();
        this.audioMeters.clear();

        console.log('🗑️ Real-time Audio UI disposed');
    }
}

// Initialize when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    RealtimeAudioUI.init();
});

// Export for global access
window.RealtimeAudioUI = RealtimeAudioUI;