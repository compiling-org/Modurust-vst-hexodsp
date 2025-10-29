/**
 * Modurust DAW - Arrangement View
 * Traditional DAW timeline with tracks, clips, and automation
 */

class ArrangementView {
    static canvas = null;
    static ctx = null;
    static trackList = null;
    static timelineArea = null;
    static timelineHeader = null;
    static trackLanes = null;
    static zoom = 1.0;
    static scrollX = 0;
    static scrollY = 0;
    static isInitialized = false;
    static clips = new Map();
    static selectedClip = null;
    static isDragging = false;
    static dragOffset = { x: 0, y: 0 };
    static draggedAutomationPoint = null;
    static isDraggingAutomation = false;

    // Initialize arrangement view
    static init() {
        console.log('🎼 Initializing Arrangement View...');

        this.trackList = document.querySelector('.track-list');
        this.timelineArea = document.querySelector('.timeline-area');
        this.timelineHeader = document.querySelector('.timeline-header');
        this.trackLanes = document.querySelector('.track-lanes');

        if (!this.trackList || !this.timelineArea) {
            console.warn('Arrangement view elements not found');
            return;
        }

        this.setupCanvas();
        this.setupEventListeners();
        this.createTrackHeaders();
        this.createTimelineRuler();
        this.createTrackLanes();

        this.isInitialized = true;
        console.log('✅ Arrangement View initialized');
    }

    // Setup canvas for timeline
    static setupCanvas() {
        // Create canvas for timeline visualization
        this.canvas = document.createElement('canvas');
        this.canvas.width = 2000;
        this.canvas.height = 400;
        this.canvas.style.width = '100%';
        this.canvas.style.height = '100%';
        this.ctx = this.canvas.getContext('2d');

        // Add canvas to timeline area
        if (this.timelineArea) {
            this.timelineArea.appendChild(this.canvas);
        }
    }

    // Setup event listeners
    static setupEventListeners() {
        // Zoom controls
        const zoomInBtn = document.querySelector('.zoom-in-btn');
        const zoomOutBtn = document.querySelector('.zoom-out-btn');
        const zoomSlider = document.querySelector('.zoom-slider');

        if (zoomInBtn) {
            zoomInBtn.addEventListener('click', () => this.zoomIn());
        }
        if (zoomOutBtn) {
            zoomOutBtn.addEventListener('click', () => this.zoomOut());
        }
        if (zoomSlider) {
            zoomSlider.addEventListener('input', (e) => this.setZoom(e.target.value / 10));
        }

        // Add track button
        const addTrackBtn = document.querySelector('.add-track-btn');
        if (addTrackBtn) {
            addTrackBtn.addEventListener('click', () => this.addTrack());
        }

        // Track type selector
        const trackTypeSelect = document.querySelector('.track-type-select');
        if (trackTypeSelect) {
            trackTypeSelect.addEventListener('change', (e) => this.setNewTrackType(e.target.value));
        }

        // Canvas interactions
        if (this.canvas) {
            this.canvas.addEventListener('mousedown', (e) => this.handleMouseDown(e));
            this.canvas.addEventListener('mousemove', (e) => this.handleMouseMove(e));
            this.canvas.addEventListener('mouseup', () => this.handleMouseUp());
            this.canvas.addEventListener('wheel', (e) => this.handleWheel(e));
        }

        // Window resize
        window.addEventListener('resize', () => this.onResize());
    }

    // Create track headers
    static createTrackHeaders() {
        if (!this.trackList) return;

        this.trackList.innerHTML = '';

        AppState.tracks.forEach(track => {
            const trackHeader = this.createTrackHeader(track);
            this.trackList.appendChild(trackHeader);
        });
    }

    // Create individual track header
    static createTrackHeader(track) {
        const header = document.createElement('div');
        header.className = 'track-header';
        header.style.height = '60px';
        header.style.borderBottom = '1px solid var(--border-color)';
        header.style.display = 'flex';
        header.style.alignItems = 'center';
        header.style.padding = '8px';
        header.style.backgroundColor = 'var(--secondary-bg)';
        header.dataset.trackId = track.id;

        header.innerHTML = `
            <div class="track-info" style="flex: 1;">
                <div class="track-name" style="font-weight: bold; color: ${track.color};">${track.name}</div>
                <div class="track-type" style="font-size: 11px; color: var(--text-secondary);">${track.type}</div>
                <div class="track-input" style="font-size: 10px; color: var(--text-muted);">Input: ${track.input || 'None'}</div>
            </div>
            <div class="track-controls" style="display: flex; gap: 4px;">
                <button class="track-btn record-btn" title="Record" style="width: 24px; height: 24px; background: var(--error-color); color: white; border: none; border-radius: 4px;">●</button>
                <button class="track-btn solo-btn" title="Solo" style="width: 24px; height: 24px; background: var(--secondary-bg); border: 1px solid var(--border-color); border-radius: 4px;">S</button>
                <button class="track-btn mute-btn" title="Mute" style="width: 24px; height: 24px; background: var(--secondary-bg); border: 1px solid var(--border-color); border-radius: 4px;">M</button>
                <button class="track-btn arm-btn" title="Arm for Recording" style="width: 24px; height: 24px; background: var(--secondary-bg); border: 1px solid var(--border-color); border-radius: 4px;">R</button>
            </div>
        `;

        // Add event listeners
        const recordBtn = header.querySelector('.record-btn');
        const soloBtn = header.querySelector('.solo-btn');
        const muteBtn = header.querySelector('.mute-btn');
        const armBtn = header.querySelector('.arm-btn');

        recordBtn.addEventListener('click', () => this.toggleTrackRecording(track.id));
        soloBtn.addEventListener('click', () => this.toggleTrackSolo(track.id));
        muteBtn.addEventListener('click', () => this.toggleTrackMute(track.id));
        armBtn.addEventListener('click', () => this.toggleTrackArm(track.id));

        // Update button states
        this.updateTrackHeaderButtons(header, track);

        return header;
    }

    // Create timeline ruler
    static createTimelineRuler() {
        if (!this.timelineHeader) return;

        const ruler = this.timelineHeader.querySelector('.time-ruler');
        if (!ruler) return;

        ruler.innerHTML = '';

        // Create time markers
        const pixelsPerBeat = 100 * this.zoom;
        const beatsPerBar = 4;
        const pixelsPerBar = pixelsPerBeat * beatsPerBar;

        for (let bar = 1; bar <= 32; bar++) {
            const marker = document.createElement('div');
            marker.className = 'time-marker';
            marker.style.position = 'absolute';
            marker.style.left = `${(bar - 1) * pixelsPerBar}px`;
            marker.style.width = `${pixelsPerBar}px`;
            marker.style.height = '100%';
            marker.style.borderLeft = bar % 4 === 1 ? '2px solid var(--accent-color)' : '1px solid var(--border-color)';
            marker.style.display = 'flex';
            marker.style.alignItems = 'flex-end';
            marker.style.padding = '4px';
            marker.style.fontSize = '11px';
            marker.style.color = 'var(--text-secondary)';
            marker.textContent = bar.toString();

            ruler.appendChild(marker);
        }
    }

    // Create track lanes
    static createTrackLanes() {
        if (!this.trackLanes) return;

        this.trackLanes.innerHTML = '';

        AppState.tracks.forEach(track => {
            const trackLane = this.createTrackLane(track);
            this.trackLanes.appendChild(trackLane);
        });
    }

    // Create individual track lane
    static createTrackLane(track) {
        const lane = document.createElement('div');
        lane.className = 'track-lane';
        lane.style.height = '120px'; // Increased height for automation lane
        lane.style.borderBottom = '1px solid var(--border-color)';
        lane.style.position = 'relative';
        lane.style.backgroundColor = 'var(--primary-bg)';
        lane.dataset.trackId = track.id;

        // Create automation lane toggle
        const automationToggle = document.createElement('button');
        automationToggle.className = 'automation-toggle';
        automationToggle.textContent = '▼ Automation';
        automationToggle.style.position = 'absolute';
        automationToggle.style.top = '0';
        automationToggle.style.right = '0';
        automationToggle.style.backgroundColor = 'var(--secondary-bg)';
        automationToggle.style.border = '1px solid var(--border-color)';
        automationToggle.style.borderRadius = '4px';
        automationToggle.style.padding = '2px 8px';
        automationToggle.style.fontSize = '10px';
        automationToggle.style.cursor = 'pointer';
        automationToggle.style.zIndex = '10';

        automationToggle.addEventListener('click', () => {
            const automationLane = lane.querySelector('.automation-lane');
            const isVisible = automationLane.style.display !== 'none';
            automationLane.style.display = isVisible ? 'none' : 'block';
            automationToggle.textContent = isVisible ? '▼ Automation' : '▲ Automation';
            lane.style.height = isVisible ? '60px' : '120px';
        });

        lane.appendChild(automationToggle);

        // Create clip lane (top half)
        const clipLane = document.createElement('div');
        clipLane.className = 'clip-lane';
        clipLane.style.height = '60px';
        clipLane.style.position = 'relative';
        clipLane.style.borderBottom = '1px solid var(--border-color)';
        lane.appendChild(clipLane);

        // Create automation lane (bottom half, initially hidden)
        const automationLane = document.createElement('div');
        automationLane.className = 'automation-lane';
        automationLane.style.height = '60px';
        automationLane.style.position = 'relative';
        automationLane.style.backgroundColor = 'var(--tertiary-bg)';
        automationLane.style.display = 'none';
        lane.appendChild(automationLane);

        // Add some sample clips
        if (track.id === 1) {
            this.addSampleClip(clipLane, track.id, 0, 4, 'Kick Pattern');
        }
        if (track.id === 2) {
            this.addSampleClip(clipLane, track.id, 2, 6, 'Snare Fill');
        }
        if (track.id === 6) {
            this.addSampleClip(clipLane, track.id, 1, 8, 'Chord Progression');
        }

        // Add sample automation
        this.addAutomationPoint(automationLane, track.id, 'volume', 0, 0.8);
        this.addAutomationPoint(automationLane, track.id, 'volume', 4, 0.6);
        this.addAutomationPoint(automationLane, track.id, 'volume', 8, 0.9);

        return lane;
    }

    // Add sample clip to track lane
    static addSampleClip(lane, trackId, startBeat, duration, name) {
        const clip = document.createElement('div');
        clip.className = 'audio-clip';
        clip.style.position = 'absolute';
        clip.style.left = `${startBeat * 100 * this.zoom}px`;
        clip.style.width = `${duration * 100 * this.zoom}px`;
        clip.style.height = '40px';
        clip.style.top = '10px';
        clip.style.backgroundColor = 'var(--accent-color)';
        clip.style.border = '1px solid var(--accent-hover)';
        clip.style.borderRadius = '4px';
        clip.style.cursor = 'pointer';
        clip.style.display = 'flex';
        clip.style.alignItems = 'center';
        clip.style.padding = '0 8px';
        clip.style.fontSize = '12px';
        clip.style.fontWeight = 'bold';
        clip.style.color = 'var(--primary-bg)';
        clip.textContent = name;
        clip.dataset.trackId = trackId;
        clip.dataset.startBeat = startBeat;
        clip.dataset.duration = duration;

        // Add clip controls
        const controls = document.createElement('div');
        controls.className = 'clip-controls';
        controls.style.position = 'absolute';
        controls.style.top = '-20px';
        controls.style.right = '0';
        controls.style.display = 'none';
        controls.style.backgroundColor = 'var(--secondary-bg)';
        controls.style.border = '1px solid var(--border-color)';
        controls.style.borderRadius = '4px';
        controls.style.padding = '2px';
        controls.innerHTML = `
            <button class="clip-btn duplicate-btn" title="Duplicate">📋</button>
            <button class="clip-btn split-btn" title="Split">✂️</button>
            <button class="clip-btn delete-btn" title="Delete">🗑️</button>
        `;

        clip.appendChild(controls);

        // Show/hide controls on hover
        clip.addEventListener('mouseenter', () => {
            controls.style.display = 'flex';
        });
        clip.addEventListener('mouseleave', () => {
            controls.style.display = 'none';
        });

        // Add event listeners for controls
        controls.querySelector('.duplicate-btn').addEventListener('click', (e) => {
            e.stopPropagation();
            this.duplicateClip(trackId, startBeat);
        });
        controls.querySelector('.split-btn').addEventListener('click', (e) => {
            e.stopPropagation();
            this.splitClip(trackId, startBeat);
        });
        controls.querySelector('.delete-btn').addEventListener('click', (e) => {
            e.stopPropagation();
            this.deleteClip(trackId, startBeat);
        });

        // Add drag functionality
        clip.addEventListener('mousedown', (e) => this.startClipDrag(e, clip));

        // Add double-click to rename
        clip.addEventListener('dblclick', (e) => {
            e.stopPropagation();
            this.renameClip(trackId, startBeat);
        });

        lane.appendChild(clip);

        // Store clip reference
        const clipId = `clip_${trackId}_${startBeat}`;
        this.clips.set(clipId, {
            element: clip,
            trackId,
            startBeat,
            duration,
            name
        });
    }

    // Handle mouse down on canvas
    static handleMouseDown(e) {
        const rect = this.canvas.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;

        // Check if clicking on a clip
        const clickedClip = this.getClipAtPosition(x, y);
        if (clickedClip) {
            this.selectClip(clickedClip);
            this.startClipDrag(e, clickedClip.element);
        } else {
            this.selectedClip = null;
            this.updateClipSelection();
        }
    }

    // Get clip at position
    static getClipAtPosition(x, y) {
        for (const [id, clip] of this.clips) {
            const rect = clip.element.getBoundingClientRect();
            const canvasRect = this.canvas.getBoundingClientRect();

            if (x >= rect.left - canvasRect.left &&
                x <= rect.right - canvasRect.left &&
                y >= rect.top - canvasRect.top &&
                y <= rect.bottom - canvasRect.top) {
                return clip;
            }
        }
        return null;
    }

    // Select clip
    static selectClip(clip) {
        this.selectedClip = clip;
        this.updateClipSelection();
    }

    // Update clip selection visual
    static updateClipSelection() {
        // Remove selection from all clips
        this.clips.forEach(clip => {
            clip.element.style.boxShadow = 'none';
            clip.element.style.borderColor = 'var(--accent-hover)';
        });

        // Add selection to current clip
        if (this.selectedClip) {
            this.selectedClip.element.style.boxShadow = '0 0 0 2px var(--accent-color)';
            this.selectedClip.element.style.borderColor = 'var(--accent-color)';
        }
    }

    // Start clip drag
    static startClipDrag(e, clipElement) {
        this.isDragging = true;
        this.dragOffset.x = e.clientX - clipElement.getBoundingClientRect().left;
        this.dragOffset.y = e.clientY - clipElement.getBoundingClientRect().top;

        e.preventDefault();
    }

    // Handle mouse move
    static handleMouseMove(e) {
        if (this.isDragging && this.selectedClip) {
            const rect = this.canvas.getBoundingClientRect();
            const x = e.clientX - rect.left - this.dragOffset.x;
            const y = e.clientY - rect.top - this.dragOffset.y;

            // Constrain to track lane
            const trackLane = this.selectedClip.element.parentElement;
            const laneRect = trackLane.getBoundingClientRect();
            const canvasRect = this.canvas.getBoundingClientRect();

            const constrainedY = Math.max(10, Math.min(y, 10)); // Keep in lane
            const newX = Math.max(0, x);

            this.selectedClip.element.style.left = `${newX}px`;
            this.selectedClip.element.style.top = `${constrainedY}px`;

            // Update clip position data
            const pixelsPerBeat = 100 * this.zoom;
            this.selectedClip.startBeat = newX / pixelsPerBeat;
        }
    }

    // Handle mouse up
    static handleMouseUp() {
        this.isDragging = false;
        this.isDraggingAutomation = false;
        this.draggedAutomationPoint = null;
    }

    // Handle wheel zoom
    static handleWheel(e) {
        e.preventDefault();

        if (e.ctrlKey) {
            // Zoom
            const zoomDelta = e.deltaY > 0 ? 0.9 : 1.1;
            this.setZoom(this.zoom * zoomDelta);
        } else {
            // Scroll
            this.scrollX += e.deltaX;
            this.scrollY += e.deltaY;
            this.updateScroll();
        }
    }

    // Zoom in
    static zoomIn() {
        this.setZoom(this.zoom * 1.2);
    }

    // Zoom out
    static zoomOut() {
        this.setZoom(this.zoom / 1.2);
    }

    // Set zoom level
    static setZoom(zoom) {
        this.zoom = Math.max(0.1, Math.min(5.0, zoom));

        // Update zoom slider
        const zoomSlider = document.querySelector('.zoom-slider');
        if (zoomSlider) {
            zoomSlider.value = this.zoom * 10;
        }

        // Update timeline ruler
        this.createTimelineRuler();

        // Update clip positions
        this.updateClipPositions();

        console.log(`🔍 Zoom set to ${this.zoom.toFixed(2)}x`);
    }

    // Update clip positions based on zoom
    static updateClipPositions() {
        this.clips.forEach(clip => {
            const pixelsPerBeat = 100 * this.zoom;
            clip.element.style.left = `${clip.startBeat * pixelsPerBeat}px`;
            clip.element.style.width = `${clip.duration * pixelsPerBeat}px`;
        });
    }

    // Update scroll
    static updateScroll() {
        if (this.timelineArea) {
            this.timelineArea.scrollLeft = this.scrollX;
        }
        if (this.trackLanes) {
            this.trackLanes.scrollTop = this.scrollY;
        }
    }

    // Add new track
    static addTrack() {
        const trackTypeSelect = document.querySelector('.track-type-select');
        const trackType = trackTypeSelect ? trackTypeSelect.value : 'audio';

        const trackId = AppState.tracks.length + 1;
        const trackName = `${trackType.charAt(0).toUpperCase() + trackType.slice(1)} ${trackId}`;

        const newTrack = {
            id: trackId,
            name: trackName,
            type: trackType,
            color: this.getRandomColor(),
            volume: 0.8,
            panValue: 0,
            mute: false,
            solo: false
        };

        AppState.tracks.push(newTrack);

        // Add to audio engine
        AudioEngine.createTrack(trackId, trackName);

        // Update UI
        this.createTrackHeaders();
        this.createTrackLanes();

        // Update mixer
        UIManager.getComponent('mixer').createMixerChannels();

        console.log(`➕ Added new ${trackType} track: ${trackName}`);
    }

    // Set new track type
    static setNewTrackType(type) {
        // This will be used when adding new tracks
        console.log(`Track type set to: ${type}`);
    }

    // Toggle track recording
    static toggleTrackRecording(trackId) {
        console.log(`Recording toggled for track ${trackId}`);
        // Implementation for track recording
    }

    // Toggle track solo
    static toggleTrackSolo(trackId) {
        const track = AppState.tracks.find(t => t.id === trackId);
        if (track) {
            track.solo = !track.solo;
            AudioEngine.setTrackSolo(trackId, track.solo);
            this.updateTrackHeader(trackId);
        }
    }

    // Toggle track mute
    static toggleTrackMute(trackId) {
        const track = AppState.tracks.find(t => t.id === trackId);
        if (track) {
            track.mute = !track.mute;
            AudioEngine.setTrackMute(trackId, track.mute);
            this.updateTrackHeader(trackId);
        }
    }

    // Update track header visual state
    static updateTrackHeader(trackId) {
        const track = AppState.tracks.find(t => t.id === trackId);
        if (!track) return;

        const header = this.trackList.querySelector(`[data-track-id="${trackId}"]`);
        if (!header) return;

        this.updateTrackHeaderButtons(header, track);
    }

    // Update track header button states
    static updateTrackHeaderButtons(header, track) {
        const soloBtn = header.querySelector('.solo-btn');
        const muteBtn = header.querySelector('.mute-btn');
        const armBtn = header.querySelector('.arm-btn');

        // Update solo button
        if (track.solo) {
            soloBtn.style.backgroundColor = 'var(--warning-color)';
            soloBtn.style.color = 'white';
        } else {
            soloBtn.style.backgroundColor = 'var(--secondary-bg)';
            soloBtn.style.color = 'var(--text-primary)';
        }

        // Update mute button
        if (track.mute) {
            muteBtn.style.backgroundColor = 'var(--error-color)';
            muteBtn.style.color = 'white';
        } else {
            muteBtn.style.backgroundColor = 'var(--secondary-bg)';
            muteBtn.style.color = 'var(--text-primary)';
        }

        // Update arm button
        if (track.armed) {
            armBtn.style.backgroundColor = 'var(--error-color)';
            armBtn.style.color = 'white';
        } else {
            armBtn.style.backgroundColor = 'var(--secondary-bg)';
            armBtn.style.color = 'var(--text-primary)';
        }
    }

    // Toggle track arm for recording
    static toggleTrackArm(trackId) {
        const track = AppState.tracks.find(t => t.id === trackId);
        if (track) {
            track.armed = !track.armed;
            this.updateTrackHeader(trackId);
            console.log(`Arm toggled for track ${trackId}: ${track.armed}`);
        }
    }

    // Add automation point
    static addAutomationPoint(lane, trackId, parameter, beat, value) {
        const point = document.createElement('div');
        point.className = 'automation-point';
        point.style.position = 'absolute';
        point.style.left = `${beat * 100 * this.zoom}px`;
        point.style.top = `${(1.0 - value) * 40}px`; // 40px height, inverted Y
        point.style.width = '8px';
        point.style.height = '8px';
        point.style.backgroundColor = '#00ff88';
        point.style.border = '1px solid #00aa55';
        point.style.borderRadius = '50%';
        point.style.cursor = 'pointer';
        point.dataset.trackId = trackId;
        point.dataset.parameter = parameter;
        point.dataset.beat = beat;
        point.dataset.value = value;

        // Add drag functionality for automation points
        point.addEventListener('mousedown', (e) => this.startAutomationDrag(e, point));

        lane.appendChild(point);
    }

    // Start automation point drag
    static startAutomationDrag(e, point) {
        this.draggedAutomationPoint = point;
        this.isDraggingAutomation = true;
        e.preventDefault();
    }

    // Duplicate clip
    static duplicateClip(trackId, startBeat) {
        const clipId = `clip_${trackId}_${startBeat}`;
        const originalClip = this.clips.get(clipId);
        if (!originalClip) return;

        // Create duplicate at next available position
        const newStartBeat = startBeat + originalClip.duration + 1;
        const newName = `${originalClip.name} (Copy)`;

        const lane = document.querySelector(`[data-track-id="${trackId}"]`);
        if (lane) {
            this.addSampleClip(lane, trackId, newStartBeat, originalClip.duration, newName);
            console.log(`Duplicated clip "${originalClip.name}" to beat ${newStartBeat}`);
        }
    }

    // Split clip at current position
    static splitClip(trackId, startBeat) {
        const clipId = `clip_${trackId}_${startBeat}`;
        const clip = this.clips.get(clipId);
        if (!clip) return;

        // For now, just delete and create two smaller clips
        // In a real implementation, this would split at the playhead position
        const midPoint = startBeat + clip.duration / 2;
        const firstHalf = clip.duration / 2;
        const secondHalf = clip.duration / 2;

        // Remove original
        this.deleteClip(trackId, startBeat);

        // Add two new clips
        const lane = document.querySelector(`[data-track-id="${trackId}"]`);
        if (lane) {
            this.addSampleClip(lane, trackId, startBeat, firstHalf, `${clip.name} 1`);
            this.addSampleClip(lane, trackId, midPoint, secondHalf, `${clip.name} 2`);
            console.log(`Split clip "${clip.name}" into two parts`);
        }
    }

    // Delete clip
    static deleteClip(trackId, startBeat) {
        const clipId = `clip_${trackId}_${startBeat}`;
        const clip = this.clips.get(clipId);
        if (!clip) return;

        clip.element.remove();
        this.clips.delete(clipId);
        console.log(`Deleted clip "${clip.name}"`);
    }

    // Rename clip
    static renameClip(trackId, startBeat) {
        const clipId = `clip_${trackId}_${startBeat}`;
        const clip = this.clips.get(clipId);
        if (!clip) return;

        const newName = prompt('Enter new clip name:', clip.name);
        if (newName && newName.trim()) {
            clip.name = newName.trim();
            clip.element.textContent = clip.name;
            console.log(`Renamed clip to "${clip.name}"`);
        }
    }

    // Get random color for tracks
    static getRandomColor() {
        const colors = [
            '#ff4444', '#44ff44', '#4444ff', '#ffff44',
            '#ff44ff', '#44ffff', '#ffffff', '#888888'
        ];
        return colors[Math.floor(Math.random() * colors.length)];
    }

    // Handle window resize
    static onResize() {
        if (this.canvas) {
            const rect = this.timelineArea.getBoundingClientRect();
            this.canvas.width = rect.width;
            this.canvas.height = rect.height;
        }
    }

    // Activate arrangement view
    static onActivate() {
        console.log('🎼 Arrangement view activated');
        this.updateUI();
    }

    // Update UI
    static updateUI() {
        this.createTrackHeaders();
        this.createTimelineRuler();
        this.createTrackLanes();
    }

    // Render timeline (called by animation loop)
    static render() {
        if (!this.ctx || !this.isInitialized) return;

        // Clear canvas
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

        // Draw grid lines
        this.drawGrid();

        // Draw playhead
        this.drawPlayhead();

        // Draw automation curves (placeholder)
        // this.drawAutomation();
    }

    // Draw grid
    static drawGrid() {
        const pixelsPerBeat = 100 * this.zoom;
        const beatsPerBar = 4;

        this.ctx.strokeStyle = 'var(--border-color)';
        this.ctx.lineWidth = 1;

        // Vertical lines (beats)
        for (let x = 0; x < this.canvas.width; x += pixelsPerBeat) {
            this.ctx.beginPath();
            this.ctx.moveTo(x, 0);
            this.ctx.lineTo(x, this.canvas.height);
            this.ctx.stroke();
        }

        // Vertical lines (bars) - thicker
        this.ctx.strokeStyle = 'var(--accent-color)';
        this.ctx.lineWidth = 2;

        for (let x = 0; x < this.canvas.width; x += pixelsPerBeat * beatsPerBar) {
            this.ctx.beginPath();
            this.ctx.moveTo(x, 0);
            this.ctx.lineTo(x, this.canvas.height);
            this.ctx.stroke();
        }
    }

    // Draw playhead
    static drawPlayhead() {
        const playheadX = AppState.currentTime * 100 * this.zoom; // Assuming 100 pixels per second

        if (playheadX >= 0 && playheadX <= this.canvas.width) {
            this.ctx.strokeStyle = 'var(--error-color)';
            this.ctx.lineWidth = 2;

            this.ctx.beginPath();
            this.ctx.moveTo(playheadX, 0);
            this.ctx.lineTo(playheadX, this.canvas.height);
            this.ctx.stroke();
        }
    }
}

// Initialize when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    ArrangementView.init();
});

// Export for global access
window.ArrangementView = ArrangementView;