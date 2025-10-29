/**
 * Modurust DAW - EEG Control Integration
 * Brainwave-controlled parameters and real-time neurofeedback
 */

class EEGControl {
    static isConnected = false;
    static isEnabled = false;
    static currentData = null;
    static mappings = new Map();
    static updateInterval = null;
    static websocket = null;
    static mockData = true; // Use mock data for development

    // EEG frequency bands
    static bands = {
        delta: { min: 0.5, max: 4, color: '#4477ff' },
        theta: { min: 4, max: 8, color: '#44ff77' },
        alpha: { min: 8, max: 12, color: '#ffff44' },
        beta: { min: 12, max: 30, color: '#ff7744' },
        gamma: { min: 30, max: 100, color: '#ff44ff' }
    };

    // Initialize EEG control
    static init() {
        console.log('🧠 Initializing EEG Control...');

        // Create EEG control panel
        this.createEEGPanel();

        // Initialize default mappings
        this.initializeDefaultMappings();

        // Setup WebSocket connection (placeholder)
        this.setupWebSocket();

        console.log('✅ EEG Control initialized');
    }

    // Create EEG control panel
    static createEEGPanel() {
        const eegPanel = document.createElement('div');
        eegPanel.id = 'eeg-panel';
        eegPanel.className = 'eeg-panel';
        eegPanel.innerHTML = `
            <div class="panel-header">
                <h3>EEG Control</h3>
                <div class="eeg-status">
                    <span class="status-indicator" id="eeg-connection-status"></span>
                    <span class="status-text" id="eeg-status-text">Disconnected</span>
                </div>
            </div>
            <div class="panel-content">
                <div class="eeg-controls">
                    <button class="eeg-btn" id="eeg-connect-btn">Connect EEG</button>
                    <button class="eeg-btn" id="eeg-enable-btn" disabled>Enable Control</button>
                    <button class="eeg-btn" id="eeg-calibrate-btn" disabled>Calibrate</button>
                </div>

                <div class="eeg-visualization">
                    <canvas id="eeg-brain-canvas" width="300" height="200"></canvas>
                    <div class="eeg-metrics">
                        <div class="metric" id="focus-metric">
                            <span class="metric-label">Focus</span>
                            <div class="metric-bar">
                                <div class="metric-fill" id="focus-fill"></div>
                            </div>
                            <span class="metric-value" id="focus-value">0%</span>
                        </div>
                        <div class="metric" id="relaxation-metric">
                            <span class="metric-label">Relaxation</span>
                            <div class="metric-bar">
                                <div class="metric-fill" id="relaxation-fill"></div>
                            </div>
                            <span class="metric-value" id="relaxation-value">0%</span>
                        </div>
                        <div class="metric" id="stress-metric">
                            <span class="metric-label">Stress</span>
                            <div class="metric-bar">
                                <div class="metric-fill" id="stress-fill"></div>
                            </div>
                            <span class="metric-value" id="stress-value">0%</span>
                        </div>
                    </div>
                </div>

                <div class="eeg-mappings">
                    <h4>Parameter Mappings</h4>
                    <div class="mapping-list" id="mapping-list">
                        <!-- Mappings will be populated dynamically -->
                    </div>
                    <button class="eeg-btn" id="add-mapping-btn">Add Mapping</button>
                </div>

                <div class="eeg-frequency-bands">
                    <h4>Frequency Bands</h4>
                    <div class="band-display">
                        ${Object.entries(this.bands).map(([band, config]) => `
                            <div class="band-item">
                                <span class="band-name" style="color: ${config.color}">${band.toUpperCase()}</span>
                                <div class="band-bar">
                                    <div class="band-fill" id="${band}-fill" style="background-color: ${config.color}"></div>
                                </div>
                                <span class="band-value" id="${band}-value">0.00</span>
                            </div>
                        `).join('')}
                    </div>
                </div>
            </div>
        `;

        // Add to DOM (you might want to add this to a specific container)
        const mixerPanel = document.querySelector('.sidebar-right');
        if (mixerPanel) {
            mixerPanel.appendChild(eegPanel);
        }

        // Setup event listeners
        this.setupEventListeners();

        // Initialize brain visualization
        this.initializeBrainVisualization();
    }

    // Setup event listeners
    static setupEventListeners() {
        const connectBtn = document.getElementById('eeg-connect-btn');
        const enableBtn = document.getElementById('eeg-enable-btn');
        const calibrateBtn = document.getElementById('eeg-calibrate-btn');
        const addMappingBtn = document.getElementById('add-mapping-btn');

        connectBtn.addEventListener('click', () => this.connectEEG());
        enableBtn.addEventListener('click', () => this.toggleEEGControl());
        calibrateBtn.addEventListener('click', () => this.calibrateEEG());
        addMappingBtn.addEventListener('click', () => this.showAddMappingDialog());
    }

    // Initialize brain visualization
    static initializeBrainVisualization() {
        const canvas = document.getElementById('eeg-brain-canvas');
        if (!canvas) return;

        const ctx = canvas.getContext('2d');
        this.brainCtx = ctx;

        // Draw basic brain outline
        this.drawBrainOutline();
    }

    // Draw brain outline
    static drawBrainOutline() {
        if (!this.brainCtx) return;

        const ctx = this.brainCtx;
        const canvas = ctx.canvas;

        ctx.clearRect(0, 0, canvas.width, canvas.height);

        // Draw brain silhouette
        ctx.strokeStyle = 'var(--accent-color)';
        ctx.lineWidth = 2;
        ctx.beginPath();

        // Left hemisphere
        ctx.ellipse(75, 100, 60, 80, 0, 0, Math.PI * 2);
        // Right hemisphere
        ctx.ellipse(225, 100, 60, 80, 0, 0, Math.PI * 2);

        // Connecting area
        ctx.ellipse(150, 120, 40, 50, 0, 0, Math.PI * 2);

        ctx.stroke();

        // Add frequency band indicators
        this.drawFrequencyIndicators();
    }

    // Draw frequency band indicators
    static drawFrequencyIndicators() {
        if (!this.brainCtx || !this.currentData) return;

        const ctx = this.brainCtx;

        Object.entries(this.bands).forEach(([band, config], index) => {
            const power = this.currentData.bandPowers?.[band] || 0;
            const intensity = Math.min(power * 2, 1); // Scale for visualization

            // Position indicators around the brain
            const angle = (index / Object.keys(this.bands).length) * Math.PI * 2;
            const x = 150 + Math.cos(angle) * 100;
            const y = 100 + Math.sin(angle) * 80;

            // Draw indicator
            ctx.fillStyle = config.color;
            ctx.globalAlpha = intensity;
            ctx.beginPath();
            ctx.arc(x, y, 8 + intensity * 12, 0, Math.PI * 2);
            ctx.fill();
            ctx.globalAlpha = 1;

            // Label
            ctx.fillStyle = 'var(--text-primary)';
            ctx.font = '10px Arial';
            ctx.textAlign = 'center';
            ctx.fillText(band.toUpperCase(), x, y + 25);
        });
    }

    // Connect to EEG device
    static async connectEEG() {
        console.log('🔌 Connecting to EEG device...');

        const connectBtn = document.getElementById('eeg-connect-btn');
        const statusIndicator = document.getElementById('eeg-connection-status');
        const statusText = document.getElementById('eeg-status-text');
        const enableBtn = document.getElementById('eeg-enable-btn');

        connectBtn.disabled = true;
        connectBtn.textContent = 'Connecting...';

        try {
            if (this.mockData) {
                // Simulate connection delay
                await new Promise(resolve => setTimeout(resolve, 2000));

                this.isConnected = true;
                statusIndicator.style.backgroundColor = 'var(--success-color)';
                statusText.textContent = 'Connected (Mock)';
                enableBtn.disabled = false;

                // Start mock data updates
                this.startMockDataUpdates();

                UIManager.showNotification('EEG device connected successfully', 'success');
            } else {
                // Real WebSocket connection would go here
                // this.websocket = new WebSocket('ws://localhost:8080/eeg');
                throw new Error('Real EEG connection not implemented');
            }

        } catch (error) {
            console.error('EEG connection failed:', error);
            statusIndicator.style.backgroundColor = 'var(--error-color)';
            statusText.textContent = 'Connection Failed';
            UIManager.showNotification('Failed to connect to EEG device', 'error');
        } finally {
            connectBtn.disabled = false;
            connectBtn.textContent = 'Connect EEG';
        }
    }

    // Toggle EEG control
    static toggleEEGControl() {
        this.isEnabled = !this.isEnabled;

        const enableBtn = document.getElementById('eeg-enable-btn');
        const calibrateBtn = document.getElementById('eeg-calibrate-btn');

        if (this.isEnabled) {
            enableBtn.textContent = 'Disable Control';
            enableBtn.style.backgroundColor = 'var(--warning-color)';
            calibrateBtn.disabled = false;

            // Start parameter control
            this.startParameterControl();

            UIManager.showNotification('EEG control enabled', 'success');
        } else {
            enableBtn.textContent = 'Enable Control';
            enableBtn.style.backgroundColor = '';
            calibrateBtn.disabled = true;

            // Stop parameter control
            this.stopParameterControl();

            UIManager.showNotification('EEG control disabled', 'info');
        }

        console.log(`EEG control ${this.isEnabled ? 'enabled' : 'disabled'}`);
    }

    // Calibrate EEG
    static calibrateEEG() {
        console.log('🎯 Starting EEG calibration...');

        const calibrateBtn = document.getElementById('eeg-calibrate-btn');
        calibrateBtn.disabled = true;
        calibrateBtn.textContent = 'Calibrating...';

        // Simulate calibration process
        setTimeout(() => {
            calibrateBtn.disabled = false;
            calibrateBtn.textContent = 'Calibrate';

            UIManager.showNotification('EEG calibration completed', 'success');
            console.log('✅ EEG calibration completed');
        }, 5000);
    }

    // Start mock data updates
    static startMockDataUpdates() {
        this.updateInterval = setInterval(() => {
            this.updateMockData();
            this.updateUI();
            this.applyParameterMappings();
        }, 100); // 10 FPS updates
    }

    // Update mock EEG data
    static updateMockData() {
        const time = Date.now() / 1000;

        // Generate realistic-looking EEG data
        this.currentData = {
            timestamp: time,
            bandPowers: {
                delta: 0.3 + Math.sin(time * 0.5) * 0.2 + Math.random() * 0.1,
                theta: 0.4 + Math.sin(time * 0.7) * 0.15 + Math.random() * 0.1,
                alpha: 0.5 + Math.sin(time * 0.3) * 0.25 + Math.random() * 0.1,
                beta: 0.6 + Math.sin(time * 1.0) * 0.2 + Math.random() * 0.1,
                gamma: 0.2 + Math.sin(time * 1.5) * 0.1 + Math.random() * 0.05
            },
            focus: Math.max(0, Math.min(1, 0.5 + Math.sin(time * 0.8) * 0.3)),
            relaxation: Math.max(0, Math.min(1, 0.6 + Math.sin(time * 0.4) * 0.2)),
            stress: Math.max(0, Math.min(1, 0.3 + Math.sin(time * 0.6) * 0.15))
        };
    }

    // Update UI with current data
    static updateUI() {
        if (!this.currentData) return;

        // Update frequency band displays
        Object.entries(this.currentData.bandPowers).forEach(([band, power]) => {
            const fillElement = document.getElementById(`${band}-fill`);
            const valueElement = document.getElementById(`${band}-value`);

            if (fillElement) {
                fillElement.style.width = `${power * 100}%`;
            }
            if (valueElement) {
                valueElement.textContent = power.toFixed(2);
            }
        });

        // Update metrics
        const focusFill = document.getElementById('focus-fill');
        const focusValue = document.getElementById('focus-value');
        const relaxationFill = document.getElementById('relaxation-fill');
        const relaxationValue = document.getElementById('relaxation-value');
        const stressFill = document.getElementById('stress-fill');
        const stressValue = document.getElementById('stress-value');

        if (focusFill) focusFill.style.width = `${this.currentData.focus * 100}%`;
        if (focusValue) focusValue.textContent = `${Math.round(this.currentData.focus * 100)}%`;

        if (relaxationFill) relaxationFill.style.width = `${this.currentData.relaxation * 100}%`;
        if (relaxationValue) relaxationValue.textContent = `${Math.round(this.currentData.relaxation * 100)}%`;

        if (stressFill) stressFill.style.width = `${this.currentData.stress * 100}%`;
        if (stressValue) stressValue.textContent = `${Math.round(this.currentData.stress * 100)}%`;

        // Update brain visualization
        this.drawBrainOutline();
    }

    // Initialize default mappings
    static initializeDefaultMappings() {
        // Default EEG to audio parameter mappings
        this.addMapping({
            eegBand: 'alpha',
            targetType: 'audio',
            targetParameter: 'reverb_mix',
            minValue: 0.0,
            maxValue: 1.0,
            smoothing: 0.8
        });

        this.addMapping({
            eegBand: 'beta',
            targetType: 'audio',
            targetParameter: 'filter_cutoff',
            minValue: 200,
            maxValue: 8000,
            smoothing: 0.6
        });

        this.addMapping({
            eegBand: 'focus',
            targetType: 'visualization',
            targetParameter: 'fractal_zoom',
            minValue: 0.1,
            maxValue: 3.0,
            smoothing: 0.9
        });

        this.updateMappingList();
    }

    // Add parameter mapping
    static addMapping(mapping) {
        const id = `mapping_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
        mapping.id = id;
        mapping.enabled = true;
        mapping.lastValue = 0;

        this.mappings.set(id, mapping);
        console.log(`🔗 Added EEG mapping: ${mapping.eegBand} → ${mapping.targetParameter}`);
    }

    // Apply parameter mappings
    static applyParameterMappings() {
        if (!this.isEnabled || !this.currentData) return;

        this.mappings.forEach(mapping => {
            if (!mapping.enabled) return;

            let eegValue = 0;

            // Get EEG value
            if (mapping.eegBand in this.currentData.bandPowers) {
                eegValue = this.currentData.bandPowers[mapping.eegBand];
            } else if (mapping.eegBand in this.currentData) {
                eegValue = this.currentData[mapping.eegBand];
            }

            // Apply smoothing
            const smoothedValue = mapping.lastValue * mapping.smoothing + eegValue * (1 - mapping.smoothing);
            mapping.lastValue = smoothedValue;

            // Scale to target range
            const targetValue = mapping.minValue + (smoothedValue * (mapping.maxValue - mapping.minValue));

            // Apply to target
            this.applyMappingToTarget(mapping, targetValue);
        });
    }

    // Apply mapping to target parameter
    static applyMappingToTarget(mapping, value) {
        switch (mapping.targetType) {
            case 'audio':
                this.applyAudioMapping(mapping.targetParameter, value);
                break;
            case 'visualization':
                this.applyVisualizationMapping(mapping.targetParameter, value);
                break;
            case 'node':
                this.applyNodeMapping(mapping.targetParameter, value);
                break;
        }
    }

    // Apply audio parameter mapping
    static applyAudioMapping(parameter, value) {
        switch (parameter) {
            case 'reverb_mix':
                // Update reverb mix in audio engine
                console.log(`Reverb mix set to ${value.toFixed(2)}`);
                break;
            case 'filter_cutoff':
                // Update filter cutoff
                console.log(`Filter cutoff set to ${Math.round(value)} Hz`);
                break;
            case 'master_volume':
                AudioEngine.setMasterVolume(value);
                break;
        }
    }

    // Apply visualization mapping
    static applyVisualizationMapping(parameter, value) {
        switch (parameter) {
            case 'fractal_zoom':
                // Update fractal zoom in visualization
                console.log(`Fractal zoom set to ${value.toFixed(2)}`);
                break;
        }
    }

    // Apply node parameter mapping
    static applyNodeMapping(parameter, value) {
        // Apply to node parameters in NodeView
        console.log(`Node parameter ${parameter} set to ${value.toFixed(2)}`);
    }

    // Start parameter control
    static startParameterControl() {
        if (!this.updateInterval) {
            this.startMockDataUpdates();
        }
    }

    // Stop parameter control
    static stopParameterControl() {
        // Keep data updates running, just disable parameter application
        console.log('Parameter control stopped');
    }

    // Update mapping list UI
    static updateMappingList() {
        const mappingList = document.getElementById('mapping-list');
        if (!mappingList) return;

        mappingList.innerHTML = '';

        this.mappings.forEach(mapping => {
            const mappingItem = document.createElement('div');
            mappingItem.className = 'mapping-item';
            mappingItem.innerHTML = `
                <div class="mapping-info">
                    <span class="mapping-source">${mapping.eegBand.toUpperCase()}</span>
                    <span class="mapping-arrow">→</span>
                    <span class="mapping-target">${mapping.targetParameter}</span>
                </div>
                <div class="mapping-controls">
                    <button class="mapping-toggle ${mapping.enabled ? 'active' : ''}" data-mapping-id="${mapping.id}">
                        ${mapping.enabled ? 'ON' : 'OFF'}
                    </button>
                    <button class="mapping-delete" data-mapping-id="${mapping.id}">×</button>
                </div>
            `;

            mappingList.appendChild(mappingItem);
        });

        // Add event listeners
        mappingList.addEventListener('click', (e) => {
            const mappingId = e.target.dataset.mappingId;
            if (!mappingId) return;

            if (e.target.classList.contains('mapping-toggle')) {
                this.toggleMapping(mappingId);
            } else if (e.target.classList.contains('mapping-delete')) {
                this.deleteMapping(mappingId);
            }
        });
    }

    // Toggle mapping
    static toggleMapping(mappingId) {
        const mapping = this.mappings.get(mappingId);
        if (mapping) {
            mapping.enabled = !mapping.enabled;
            this.updateMappingList();
            console.log(`Mapping ${mappingId} ${mapping.enabled ? 'enabled' : 'disabled'}`);
        }
    }

    // Delete mapping
    static deleteMapping(mappingId) {
        this.mappings.delete(mappingId);
        this.updateMappingList();
        console.log(`Mapping ${mappingId} deleted`);
    }

    // Show add mapping dialog
    static showAddMappingDialog() {
        const dialog = document.createElement('div');
        dialog.className = 'modal active';
        dialog.innerHTML = `
            <div class="modal-content">
                <div class="modal-header">
                    <h2>Add EEG Mapping</h2>
                    <button class="modal-close">&times;</button>
                </div>
                <div class="modal-body">
                    <div class="form-group">
                        <label>EEG Source:</label>
                        <select id="eeg-source-select">
                            <option value="alpha">Alpha Waves</option>
                            <option value="beta">Beta Waves</option>
                            <option value="theta">Theta Waves</option>
                            <option value="delta">Delta Waves</option>
                            <option value="gamma">Gamma Waves</option>
                            <option value="focus">Focus Level</option>
                            <option value="relaxation">Relaxation Level</option>
                            <option value="stress">Stress Level</option>
                        </select>
                    </div>
                    <div class="form-group">
                        <label>Target Type:</label>
                        <select id="target-type-select">
                            <option value="audio">Audio Parameter</option>
                            <option value="visualization">Visualization</option>
                            <option value="node">Node Parameter</option>
                        </select>
                    </div>
                    <div class="form-group">
                        <label>Target Parameter:</label>
                        <select id="target-param-select">
                            <option value="master_volume">Master Volume</option>
                            <option value="reverb_mix">Reverb Mix</option>
                            <option value="filter_cutoff">Filter Cutoff</option>
                            <option value="fractal_zoom">Fractal Zoom</option>
                        </select>
                    </div>
                    <div class="form-group">
                        <label>Min Value:</label>
                        <input type="number" id="min-value-input" value="0">
                    </div>
                    <div class="form-group">
                        <label>Max Value:</label>
                        <input type="number" id="max-value-input" value="1">
                    </div>
                    <div class="form-group">
                        <label>Smoothing:</label>
                        <input type="range" id="smoothing-input" min="0" max="1" step="0.1" value="0.8">
                        <span id="smoothing-value">0.8</span>
                    </div>
                </div>
                <div class="modal-footer">
                    <button class="btn-secondary" id="cancel-mapping">Cancel</button>
                    <button class="btn-primary" id="save-mapping">Add Mapping</button>
                </div>
            </div>
        `;

        document.body.appendChild(dialog);

        // Setup dialog events
        const closeBtn = dialog.querySelector('.modal-close');
        const cancelBtn = dialog.querySelector('#cancel-mapping');
        const saveBtn = dialog.querySelector('#save-mapping');
        const smoothingInput = dialog.querySelector('#smoothing-input');
        const smoothingValue = dialog.querySelector('#smoothing-value');

        const closeDialog = () => dialog.remove();

        closeBtn.addEventListener('click', closeDialog);
        cancelBtn.addEventListener('click', closeDialog);

        smoothingInput.addEventListener('input', () => {
            smoothingValue.textContent = smoothingInput.value;
        });

        saveBtn.addEventListener('click', () => {
            const mapping = {
                eegBand: dialog.querySelector('#eeg-source-select').value,
                targetType: dialog.querySelector('#target-type-select').value,
                targetParameter: dialog.querySelector('#target-param-select').value,
                minValue: parseFloat(dialog.querySelector('#min-value-input').value),
                maxValue: parseFloat(dialog.querySelector('#max-value-input').value),
                smoothing: parseFloat(dialog.querySelector('#smoothing-input').value)
            };

            this.addMapping(mapping);
            this.updateMappingList();
            closeDialog();

            UIManager.showNotification('EEG mapping added successfully', 'success');
        });
    }

    // Setup WebSocket connection (placeholder)
    static setupWebSocket() {
        // Placeholder for real WebSocket connection
        // this.websocket = new WebSocket('ws://localhost:8080/eeg');
    }

    // Dispose resources
    static dispose() {
        if (this.updateInterval) {
            clearInterval(this.updateInterval);
            this.updateInterval = null;
        }

        if (this.websocket) {
            this.websocket.close();
            this.websocket = null;
        }

        this.isConnected = false;
        this.isEnabled = false;
        this.mappings.clear();

        console.log('🗑️ EEG Control disposed');
    }
}

// Initialize when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    EEGControl.init();
});

// Export for global access
window.EEGControl = EEGControl;