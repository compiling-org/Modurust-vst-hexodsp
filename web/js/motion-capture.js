/**
 * Modurust DAW - Motion Capture Integration
 * Gesture-controlled parameters and real-time pose tracking
 */

class MotionCapture {
    static isEnabled = false;
    static isInitialized = false;
    static videoElement = null;
    static canvasElement = null;
    static ctx = null;
    static stream = null;
    static poseDetector = null;
    static gestureRecognizer = null;
    static currentPose = null;
    static currentGesture = null;
    static gestureHistory = [];
    static mappings = new Map();
    static updateInterval = null;

    // Gesture definitions
    static gestures = {
        relaxed: {
            name: 'Relaxed',
            color: '#44ff44',
            description: 'Calm, neutral pose',
            audioParams: { volume: 0.7, reverb: 0.3 },
            visualParams: { brightness: 0.6, saturation: 0.8 }
        },
        focused: {
            name: 'Focused',
            color: '#4444ff',
            description: 'Attentive, concentrated pose',
            audioParams: { volume: 0.9, filter: 2000 },
            visualParams: { brightness: 0.8, saturation: 1.0 }
        },
        meditation: {
            name: 'Meditation',
            color: '#ff44ff',
            description: 'Contemplative, centered pose',
            audioParams: { volume: 0.5, reverb: 0.8 },
            visualParams: { brightness: 0.4, saturation: 0.6 }
        },
        excited: {
            name: 'Excited',
            color: '#ffff44',
            description: 'Energetic, dynamic pose',
            audioParams: { volume: 1.0, distortion: 0.3 },
            visualParams: { brightness: 1.0, saturation: 1.2 }
        },
        stressed: {
            name: 'Stressed',
            color: '#ff4444',
            description: 'Tense, anxious pose',
            audioParams: { volume: 0.8, filter: 500 },
            visualParams: { brightness: 0.7, saturation: 0.9 }
        },
        tired: {
            name: 'Tired',
            color: '#888888',
            description: 'Fatigued, slumped pose',
            audioParams: { volume: 0.4, reverb: 0.5 },
            visualParams: { brightness: 0.3, saturation: 0.5 }
        },
        confused: {
            name: 'Confused',
            color: '#ff8844',
            description: 'Uncertain, questioning pose',
            audioParams: { volume: 0.6, delay: 0.4 },
            visualParams: { brightness: 0.5, saturation: 0.7 }
        }
    };

    // Initialize motion capture
    static init() {
        console.log('🤖 Initializing Motion Capture...');

        // Create motion capture panel
        this.createMotionPanel();

        // Initialize mock pose detection for development
        this.initializeMockPoseDetection();

        console.log('✅ Motion Capture initialized');
    }

    // Create motion capture panel
    static createMotionPanel() {
        const motionPanel = document.createElement('div');
        motionPanel.id = 'motion-panel';
        motionPanel.className = 'motion-panel';
        motionPanel.innerHTML = `
            <div class="panel-header">
                <h3>Motion Capture</h3>
                <div class="motion-status">
                    <span class="status-indicator" id="motion-connection-status"></span>
                    <span class="status-text" id="motion-status-text">Disabled</span>
                </div>
            </div>
            <div class="panel-content">
                <div class="motion-controls">
                    <button class="motion-btn" id="motion-enable-btn">Enable Motion</button>
                    <button class="motion-btn" id="motion-calibrate-btn" disabled>Calibrate</button>
                    <button class="motion-btn" id="motion-reset-btn" disabled>Reset</button>
                </div>

                <div class="motion-visualization">
                    <div class="camera-container">
                        <video id="motion-video" autoplay playsinline muted style="width: 100%; height: 200px; background: #000; border-radius: 4px;"></video>
                        <canvas id="motion-canvas" style="position: absolute; top: 0; left: 0; width: 100%; height: 200px; pointer-events: none;"></canvas>
                    </div>
                    <div class="pose-info">
                        <div class="current-gesture" id="current-gesture-display">
                            <span class="gesture-label">Current Gesture:</span>
                            <span class="gesture-name" id="gesture-name">None</span>
                        </div>
                        <div class="gesture-confidence">
                            <span class="confidence-label">Confidence:</span>
                            <div class="confidence-bar">
                                <div class="confidence-fill" id="confidence-fill"></div>
                            </div>
                            <span class="confidence-value" id="confidence-value">0%</span>
                        </div>
                    </div>
                </div>

                <div class="gesture-library">
                    <h4>Gesture Library</h4>
                    <div class="gesture-grid" id="gesture-grid">
                        ${Object.entries(this.gestures).map(([key, gesture]) => `
                            <div class="gesture-item" data-gesture="${key}">
                                <div class="gesture-color" style="background-color: ${gesture.color}; width: 20px; height: 20px; border-radius: 50%; margin-bottom: 4px;"></div>
                                <div class="gesture-name">${gesture.name}</div>
                                <div class="gesture-description" style="font-size: 10px; color: var(--text-muted);">${gesture.description}</div>
                            </div>
                        `).join('')}
                    </div>
                </div>

                <div class="motion-mappings">
                    <h4>Motion Mappings</h4>
                    <div class="mapping-list" id="motion-mapping-list">
                        <!-- Mappings will be populated dynamically -->
                    </div>
                    <button class="motion-btn" id="add-motion-mapping-btn">Add Mapping</button>
                </div>
            </div>
        `;

        // Add to DOM (you might want to add this to a specific container)
        const arrangementView = document.getElementById('arrangement-view');
        if (arrangementView) {
            arrangementView.appendChild(motionPanel);
        }

        // Setup event listeners
        this.setupEventListeners();
    }

    // Setup event listeners
    static setupEventListeners() {
        const enableBtn = document.getElementById('motion-enable-btn');
        const calibrateBtn = document.getElementById('motion-calibrate-btn');
        const resetBtn = document.getElementById('motion-reset-btn');
        const addMappingBtn = document.getElementById('add-motion-mapping-btn');

        if (enableBtn) {
            enableBtn.addEventListener('click', () => this.toggleMotionCapture());
        }
        if (calibrateBtn) {
            calibrateBtn.addEventListener('click', () => this.calibrateMotion());
        }
        if (resetBtn) {
            resetBtn.addEventListener('click', () => this.resetMotion());
        }
        if (addMappingBtn) {
            addMappingBtn.addEventListener('click', () => this.showAddMappingDialog());
        }
    }

    // Initialize mock pose detection
    static initializeMockPoseDetection() {
        this.videoElement = document.getElementById('motion-video');
        this.canvasElement = document.getElementById('motion-canvas');
        this.ctx = this.canvasElement ? this.canvasElement.getContext('2d') : null;

        // Mock pose data for development
        this.mockPoseData = {
            nose: { x: 0.5, y: 0.2 },
            leftEye: { x: 0.45, y: 0.18 },
            rightEye: { x: 0.55, y: 0.18 },
            leftEar: { x: 0.4, y: 0.22 },
            rightEar: { x: 0.6, y: 0.22 },
            leftShoulder: { x: 0.35, y: 0.35 },
            rightShoulder: { x: 0.65, y: 0.35 },
            leftElbow: { x: 0.25, y: 0.45 },
            rightElbow: { x: 0.75, y: 0.45 },
            leftWrist: { x: 0.2, y: 0.55 },
            rightWrist: { x: 0.8, y: 0.55 },
            leftHip: { x: 0.4, y: 0.6 },
            rightHip: { x: 0.6, y: 0.6 },
            leftKnee: { x: 0.42, y: 0.75 },
            rightKnee: { x: 0.58, y: 0.75 },
            leftAnkle: { x: 0.44, y: 0.9 },
            rightAnkle: { x: 0.56, y: 0.9 }
        };
    }

    // Toggle motion capture
    static async toggleMotionCapture() {
        const enableBtn = document.getElementById('motion-enable-btn');
        const calibrateBtn = document.getElementById('motion-calibrate-btn');
        const resetBtn = document.getElementById('motion-reset-btn');
        const statusIndicator = document.getElementById('motion-connection-status');
        const statusText = document.getElementById('motion-status-text');

        if (this.isEnabled) {
            // Disable motion capture
            this.isEnabled = false;
            enableBtn.textContent = 'Enable Motion';
            calibrateBtn.disabled = true;
            resetBtn.disabled = true;
            statusIndicator.style.backgroundColor = 'var(--text-muted)';
            statusText.textContent = 'Disabled';

            this.stopMotionCapture();
            UIManager.showNotification('Motion capture disabled', 'info');
        } else {
            // Enable motion capture
            try {
                enableBtn.disabled = true;
                enableBtn.textContent = 'Starting...';

                // Start camera
                await this.startCamera();

                this.isEnabled = true;
                enableBtn.disabled = false;
                enableBtn.textContent = 'Disable Motion';
                calibrateBtn.disabled = false;
                resetBtn.disabled = false;
                statusIndicator.style.backgroundColor = 'var(--success-color)';
                statusText.textContent = 'Active';

                this.startMotionCapture();
                UIManager.showNotification('Motion capture enabled', 'success');

            } catch (error) {
                console.error('Failed to enable motion capture:', error);
                enableBtn.disabled = false;
                enableBtn.textContent = 'Enable Motion';
                statusIndicator.style.backgroundColor = 'var(--error-color)';
                statusText.textContent = 'Failed';
                UIManager.showNotification('Failed to start motion capture', 'error');
            }
        }
    }

    // Start camera
    static async startCamera() {
        try {
            this.stream = await navigator.mediaDevices.getUserMedia({
                video: { width: 640, height: 480, facingMode: 'user' }
            });

            if (this.videoElement) {
                this.videoElement.srcObject = this.stream;

                // Wait for video to load
                await new Promise(resolve => {
                    this.videoElement.onloadedmetadata = resolve;
                });

                // Set canvas size to match video
                if (this.canvasElement) {
                    this.canvasElement.width = this.videoElement.videoWidth;
                    this.canvasElement.height = this.videoElement.videoHeight;
                }
            }

            console.log('📹 Camera started');
        } catch (error) {
            console.error('Failed to start camera:', error);
            throw error;
        }
    }

    // Start motion capture
    static startMotionCapture() {
        if (this.updateInterval) {
            clearInterval(this.updateInterval);
        }

        this.updateInterval = setInterval(() => {
            this.updateMotionData();
            this.updateUI();
            this.applyMotionMappings();
        }, 100); // 10 FPS updates

        console.log('🤖 Motion capture started');
    }

    // Stop motion capture
    static stopMotionCapture() {
        if (this.updateInterval) {
            clearInterval(this.updateInterval);
            this.updateInterval = null;
        }

        if (this.stream) {
            this.stream.getTracks().forEach(track => track.stop());
            this.stream = null;
        }

        if (this.videoElement) {
            this.videoElement.srcObject = null;
        }

        this.currentPose = null;
        this.currentGesture = null;

        console.log('🤖 Motion capture stopped');
    }

    // Update motion data
    static updateMotionData() {
        // Generate mock pose data with some variation
        const time = Date.now() / 1000;
        const variation = Math.sin(time * 2) * 0.05;

        this.currentPose = {};
        Object.entries(this.mockPoseData).forEach(([key, point]) => {
            this.currentPose[key] = {
                x: point.x + Math.sin(time * (1 + key.length * 0.1)) * variation,
                y: point.y + Math.cos(time * (1 + key.length * 0.1)) * variation,
                confidence: 0.8 + Math.sin(time * 3) * 0.2
            };
        });

        // Detect gesture based on pose
        this.detectGesture();
    }

    // Detect gesture from pose
    static detectGesture() {
        if (!this.currentPose) return;

        // Simple gesture detection based on pose characteristics
        const pose = this.currentPose;

        // Calculate some pose metrics
        const shoulderWidth = Math.abs(pose.leftShoulder.x - pose.rightShoulder.x);
        const armSpread = Math.abs(pose.leftWrist.x - pose.rightWrist.x);
        const headTilt = Math.abs(pose.nose.y - (pose.leftShoulder.y + pose.rightShoulder.y) / 2);

        let detectedGesture = 'relaxed';
        let confidence = 0.5;

        // Gesture detection logic
        if (armSpread > shoulderWidth * 1.5) {
            detectedGesture = 'excited';
            confidence = 0.8;
        } else if (headTilt > 0.1) {
            detectedGesture = 'confused';
            confidence = 0.7;
        } else if (pose.leftWrist.y > pose.leftShoulder.y && pose.rightWrist.y > pose.rightShoulder.y) {
            detectedGesture = 'meditation';
            confidence = 0.9;
        } else if (shoulderWidth < 0.3) {
            detectedGesture = 'focused';
            confidence = 0.75;
        }

        // Add some temporal stability
        this.gestureHistory.push({ gesture: detectedGesture, confidence, timestamp: Date.now() });

        // Keep only recent history (last 10 detections)
        if (this.gestureHistory.length > 10) {
            this.gestureHistory.shift();
        }

        // Determine most common recent gesture
        const recentGestures = this.gestureHistory.slice(-5);
        const gestureCounts = {};
        recentGestures.forEach(item => {
            gestureCounts[item.gesture] = (gestureCounts[item.gesture] || 0) + 1;
        });

        const mostCommonGesture = Object.entries(gestureCounts)
            .sort(([,a], [,b]) => b - a)[0][0];

        const avgConfidence = recentGestures.reduce((sum, item) => sum + item.confidence, 0) / recentGestures.length;

        this.currentGesture = {
            name: mostCommonGesture,
            confidence: avgConfidence,
            data: this.gestures[mostCommonGesture]
        };
    }

    // Update UI
    static updateUI() {
        // Update gesture display
        const gestureNameElement = document.getElementById('gesture-name');
        const confidenceFill = document.getElementById('confidence-fill');
        const confidenceValue = document.getElementById('confidence-value');

        if (gestureNameElement && this.currentGesture) {
            gestureNameElement.textContent = this.currentGesture.data.name;
            gestureNameElement.style.color = this.currentGesture.data.color;
        }

        if (confidenceFill && confidenceValue && this.currentGesture) {
            const confidencePercent = Math.round(this.currentGesture.confidence * 100);
            confidenceFill.style.width = `${confidencePercent}%`;
            confidenceValue.textContent = `${confidencePercent}%`;
        }

        // Draw pose on canvas
        this.drawPose();
    }

    // Draw pose on canvas
    static drawPose() {
        if (!this.ctx || !this.currentPose) return;

        const ctx = this.ctx;
        const canvas = this.canvasElement;

        // Clear canvas
        ctx.clearRect(0, 0, canvas.width, canvas.height);

        // Draw pose skeleton
        ctx.strokeStyle = this.currentGesture ? this.currentGesture.data.color : '#ffffff';
        ctx.lineWidth = 3;
        ctx.globalAlpha = 0.8;

        // Draw connections between keypoints
        const connections = [
            ['leftShoulder', 'rightShoulder'],
            ['leftShoulder', 'leftElbow'],
            ['leftElbow', 'leftWrist'],
            ['rightShoulder', 'rightElbow'],
            ['rightElbow', 'rightWrist'],
            ['leftShoulder', 'leftHip'],
            ['rightShoulder', 'rightHip'],
            ['leftHip', 'rightHip'],
            ['leftHip', 'leftKnee'],
            ['leftKnee', 'leftAnkle'],
            ['rightHip', 'rightKnee'],
            ['rightKnee', 'rightAnkle'],
            ['nose', 'leftEye'],
            ['nose', 'rightEye'],
            ['leftEye', 'leftEar'],
            ['rightEye', 'rightEar']
        ];

        connections.forEach(([start, end]) => {
            const startPoint = this.currentPose[start];
            const endPoint = this.currentPose[end];

            if (startPoint && endPoint) {
                ctx.beginPath();
                ctx.moveTo(startPoint.x * canvas.width, startPoint.y * canvas.height);
                ctx.lineTo(endPoint.x * canvas.width, endPoint.y * canvas.height);
                ctx.stroke();
            }
        });

        // Draw keypoints
        ctx.fillStyle = this.currentGesture ? this.currentGesture.data.color : '#ffffff';
        ctx.globalAlpha = 1;

        Object.values(this.currentPose).forEach(point => {
            if (point && point.confidence > 0.5) {
                ctx.beginPath();
                ctx.arc(point.x * canvas.width, point.y * canvas.height, 4, 0, 2 * Math.PI);
                ctx.fill();
            }
        });

        ctx.globalAlpha = 1;
    }

    // Apply motion mappings
    static applyMotionMappings() {
        if (!this.isEnabled || !this.currentGesture) return;

        this.mappings.forEach(mapping => {
            if (!mapping.enabled) return;

            const gesture = this.currentGesture;
            let value = 0;

            // Get value from gesture parameters
            if (mapping.gestureParam in gesture.data.audioParams) {
                value = gesture.data.audioParams[mapping.gestureParam];
            } else if (mapping.gestureParam in gesture.data.visualParams) {
                value = gesture.data.visualParams[mapping.gestureParam];
            }

            // Scale value to target range
            const scaledValue = mapping.minValue + (value * (mapping.maxValue - mapping.minValue));

            // Apply to target
            this.applyMappingToTarget(mapping, scaledValue);
        });
    }

    // Apply mapping to target
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
            case 'master_volume':
                AudioEngine.setMasterVolume(value);
                break;
            case 'reverb_mix':
                console.log(`Reverb mix set to ${value.toFixed(2)}`);
                break;
            case 'filter_cutoff':
                console.log(`Filter cutoff set to ${Math.round(value)} Hz`);
                break;
        }
    }

    // Apply visualization mapping
    static applyVisualizationMapping(parameter, value) {
        switch (parameter) {
            case 'brightness':
                console.log(`Brightness set to ${value.toFixed(2)}`);
                break;
            case 'saturation':
                console.log(`Saturation set to ${value.toFixed(2)}`);
                break;
        }
    }

    // Apply node parameter mapping
    static applyNodeMapping(parameter, value) {
        console.log(`Node parameter ${parameter} set to ${value.toFixed(2)}`);
    }

    // Initialize default mappings
    static initializeDefaultMappings() {
        this.addMapping({
            gestureParam: 'volume',
            targetType: 'audio',
            targetParameter: 'master_volume',
            minValue: 0.0,
            maxValue: 1.0
        });

        this.addMapping({
            gestureParam: 'brightness',
            targetType: 'visualization',
            targetParameter: 'brightness',
            minValue: 0.0,
            maxValue: 1.0
        });

        this.updateMappingList();
    }

    // Add mapping
    static addMapping(mapping) {
        const id = `motion_mapping_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
        mapping.id = id;
        mapping.enabled = true;

        this.mappings.set(id, mapping);
        console.log(`🤖 Added motion mapping: ${mapping.gestureParam} → ${mapping.targetParameter}`);
    }

    // Update mapping list UI
    static updateMappingList() {
        const mappingList = document.getElementById('motion-mapping-list');
        if (!mappingList) return;

        mappingList.innerHTML = '';

        this.mappings.forEach(mapping => {
            const mappingItem = document.createElement('div');
            mappingItem.className = 'mapping-item';
            mappingItem.innerHTML = `
                <div class="mapping-info">
                    <span class="mapping-source">${mapping.gestureParam}</span>
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

    // Calibrate motion
    static calibrateMotion() {
        console.log('🎯 Starting motion calibration...');

        const calibrateBtn = document.getElementById('motion-calibrate-btn');
        calibrateBtn.disabled = true;
        calibrateBtn.textContent = 'Calibrating...';

        // Simulate calibration process
        setTimeout(() => {
            calibrateBtn.disabled = false;
            calibrateBtn.textContent = 'Calibrate';

            UIManager.showNotification('Motion calibration completed', 'success');
            console.log('✅ Motion calibration completed');
        }, 3000);
    }

    // Reset motion
    static resetMotion() {
        this.currentPose = null;
        this.currentGesture = null;
        this.gestureHistory = [];

        const gestureNameElement = document.getElementById('gesture-name');
        const confidenceFill = document.getElementById('confidence-fill');
        const confidenceValue = document.getElementById('confidence-value');

        if (gestureNameElement) gestureNameElement.textContent = 'None';
        if (confidenceFill) confidenceFill.style.width = '0%';
        if (confidenceValue) confidenceValue.textContent = '0%';

        if (this.ctx) {
            this.ctx.clearRect(0, 0, this.canvasElement.width, this.canvasElement.height);
        }

        UIManager.showNotification('Motion tracking reset', 'info');
        console.log('🔄 Motion tracking reset');
    }

    // Show add mapping dialog
    static showAddMappingDialog() {
        const dialog = document.createElement('div');
        dialog.className = 'modal active';
        dialog.innerHTML = `
            <div class="modal-content">
                <div class="modal-header">
                    <h2>Add Motion Mapping</h2>
                    <button class="modal-close">&times;</button>
                </div>
                <div class="modal-body">
                    <div class="form-group">
                        <label>Gesture Parameter:</label>
                        <select id="gesture-param-select">
                            <option value="volume">Volume</option>
                            <option value="reverb">Reverb</option>
                            <option value="brightness">Brightness</option>
                            <option value="saturation">Saturation</option>
                            <option value="distortion">Distortion</option>
                            <option value="delay">Delay</option>
                        </select>
                    </div>
                    <div class="form-group">
                        <label>Target Type:</label>
                        <select id="motion-target-type-select">
                            <option value="audio">Audio Parameter</option>
                            <option value="visualization">Visualization</option>
                            <option value="node">Node Parameter</option>
                        </select>
                    </div>
                    <div class="form-group">
                        <label>Target Parameter:</label>
                        <select id="motion-target-param-select">
                            <option value="master_volume">Master Volume</option>
                            <option value="reverb_mix">Reverb Mix</option>
                            <option value="brightness">Brightness</option>
                            <option value="saturation">Saturation</option>
                        </select>
                    </div>
                    <div class="form-group">
                        <label>Min Value:</label>
                        <input type="number" id="motion-min-value-input" value="0" step="0.1">
                    </div>
                    <div class="form-group">
                        <label>Max Value:</label>
                        <input type="number" id="motion-max-value-input" value="1" step="0.1">
                    </div>
                </div>
                <div class="modal-footer">
                    <button class="btn-secondary" id="cancel-motion-mapping">Cancel</button>
                    <button class="btn-primary" id="save-motion-mapping">Add Mapping</button>
                </div>
            </div>
        `;

        document.body.appendChild(dialog);

        // Setup dialog events
        const closeBtn = dialog.querySelector('.modal-close');
        const cancelBtn = dialog.querySelector('#cancel-motion-mapping');
        const saveBtn = dialog.querySelector('#save-motion-mapping');

        const closeDialog = () => dialog.remove();

        closeBtn.addEventListener('click', closeDialog);
        cancelBtn.addEventListener('click', closeDialog);

        saveBtn.addEventListener('click', () => {
            const mapping = {
                gestureParam: dialog.querySelector('#gesture-param-select').value,
                targetType: dialog.querySelector('#motion-target-type-select').value,
                targetParameter: dialog.querySelector('#motion-target-param-select').value,
                minValue: parseFloat(dialog.querySelector('#motion-min-value-input').value),
                maxValue: parseFloat(dialog.querySelector('#motion-max-value-input').value)
            };

            this.addMapping(mapping);
            this.updateMappingList();
            closeDialog();

            UIManager.showNotification('Motion mapping added successfully', 'success');
        });
    }

    // Dispose resources
    static dispose() {
        this.stopMotionCapture();
        this.mappings.clear();

        console.log('🗑️ Motion Capture disposed');
    }
}

// Initialize when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    MotionCapture.init();
});

// Export for global access
window.MotionCapture = MotionCapture;
