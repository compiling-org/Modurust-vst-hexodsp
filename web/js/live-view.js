/**
 * Modurust DAW - Live Performance View
 * Session-based workflow with scenes and clip matrix
 */

class LiveView {
    static clipMatrix = null;
    static sceneButtons = [];
    static clips = new Map();
    static currentScene = 0;
    static isInitialized = false;

    // Initialize live view
    static init() {
        console.log('🎵 Initializing Live View...');

        this.clipMatrix = document.querySelector('.clip-matrix');
        const sceneButtonsContainer = document.querySelector('.scene-buttons');

        if (!this.clipMatrix || !sceneButtonsContainer) {
            console.warn('Live view elements not found');
            return;
        }

        this.createSceneButtons(sceneButtonsContainer);
        this.createClipMatrix();
        this.setupEventListeners();

        this.isInitialized = true;
        console.log('✅ Live View initialized');
    }

    // Setup event listeners
    static setupEventListeners() {
        // Stop all button
        const stopAllBtn = document.querySelector('.stop-all-btn');
        if (stopAllBtn) {
            stopAllBtn.addEventListener('click', () => this.stopAllClips());
        }

        // Solo button
        const soloBtn = document.querySelector('.solo-btn');
        if (soloBtn) {
            soloBtn.addEventListener('click', () => this.toggleSoloMode());
        }
    }

    // Create scene buttons
    static createSceneButtons(container) {
        container.innerHTML = '';

        for (let i = 0; i < 4; i++) {
            const sceneBtn = document.createElement('button');
            sceneBtn.className = 'scene-btn';
            sceneBtn.textContent = `Scene ${i + 1}`;
            sceneBtn.dataset.sceneId = i;
            sceneBtn.style.padding = '12px 24px';
            sceneBtn.style.backgroundColor = i === this.currentScene ? 'var(--accent-color)' : 'var(--secondary-bg)';
            sceneBtn.style.border = '1px solid var(--border-color)';
            sceneBtn.style.borderRadius = 'var(--border-radius)';
            sceneBtn.style.color = i === this.currentScene ? 'var(--primary-bg)' : 'var(--text-primary)';
            sceneBtn.style.cursor = 'pointer';
            sceneBtn.style.fontSize = '14px';
            sceneBtn.style.fontWeight = 'bold';
            sceneBtn.style.transition = 'var(--transition)';

            sceneBtn.addEventListener('click', () => this.selectScene(i));
            sceneBtn.addEventListener('mouseenter', () => {
                if (i !== this.currentScene) {
                    sceneBtn.style.backgroundColor = 'var(--tertiary-bg)';
                }
            });
            sceneBtn.addEventListener('mouseleave', () => {
                if (i !== this.currentScene) {
                    sceneBtn.style.backgroundColor = 'var(--secondary-bg)';
                }
            });

            container.appendChild(sceneBtn);
            this.sceneButtons.push(sceneBtn);
        }
    }

    // Create clip matrix
    static createClipMatrix() {
        if (!this.clipMatrix) return;

        this.clipMatrix.innerHTML = '';

        // Create 4x4 grid of clips
        for (let row = 0; row < 4; row++) {
            for (let col = 0; col < 4; col++) {
                const clipElement = this.createClipElement(row, col);
                this.clipMatrix.appendChild(clipElement);
            }
        }
    }

    // Create individual clip element
    static createClipElement(row, col) {
        const clipId = `clip_${row}_${col}`;
        const clip = document.createElement('button');
        clip.className = 'clip-btn';
        clip.dataset.clipId = clipId;
        clip.dataset.row = row;
        clip.dataset.col = col;
        clip.style.width = '100%';
        clip.style.height = '80px';
        clip.style.backgroundColor = 'var(--secondary-bg)';
        clip.style.border = '1px solid var(--border-color)';
        clip.style.borderRadius = 'var(--border-radius)';
        clip.style.cursor = 'pointer';
        clip.style.display = 'flex';
        clip.style.flexDirection = 'column';
        clip.style.alignItems = 'center';
        clip.style.justifyContent = 'center';
        clip.style.padding = '8px';
        clip.style.transition = 'var(--transition)';
        clip.style.position = 'relative';

        // Clip content
        clip.innerHTML = `
            <div class="clip-number" style="font-size: 18px; font-weight: bold; color: var(--text-secondary);">
                ${row * 4 + col + 1}
            </div>
            <div class="clip-name" style="font-size: 11px; color: var(--text-muted); text-align: center; margin-top: 4px;">
                Empty
            </div>
            <div class="clip-status" style="position: absolute; top: 4px; right: 4px; width: 8px; height: 8px; border-radius: 50%; background-color: var(--text-muted);"></div>
        `;

        // Add event listeners
        clip.addEventListener('click', () => this.triggerClip(row, col));
        clip.addEventListener('contextmenu', (e) => {
            e.preventDefault();
            this.showClipContextMenu(e, row, col);
        });
        clip.addEventListener('mouseenter', () => {
            if (!clip.classList.contains('playing')) {
                clip.style.backgroundColor = 'var(--tertiary-bg)';
                clip.style.borderColor = 'var(--border-hover)';
            }
        });
        clip.addEventListener('mouseleave', () => {
            if (!clip.classList.contains('playing')) {
                clip.style.backgroundColor = 'var(--secondary-bg)';
                clip.style.borderColor = 'var(--border-color)';
            }
        });

        // Store clip data
        this.clips.set(clipId, {
            element: clip,
            row,
            col,
            name: 'Empty',
            color: this.getClipColor(row, col),
            isPlaying: false,
            isRecording: false,
            hasContent: false
        });

        return clip;
    }

    // Get clip color based on position
    static getClipColor(row, col) {
        const colors = [
            '#ff4444', '#44ff44', '#4444ff', '#ffff44',
            '#ff44ff', '#44ffff', '#ffffff', '#888888',
            '#ff8844', '#44ff88', '#8844ff', '#88ff44',
            '#ff4488', '#4488ff', '#88ff88', '#ff8888'
        ];
        return colors[row * 4 + col] || '#666666';
    }

    // Select scene
    static selectScene(sceneId) {
        // Update current scene
        this.currentScene = sceneId;

        // Update scene button visuals
        this.sceneButtons.forEach((btn, index) => {
            if (index === sceneId) {
                btn.style.backgroundColor = 'var(--accent-color)';
                btn.style.color = 'var(--primary-bg)';
            } else {
                btn.style.backgroundColor = 'var(--secondary-bg)';
                btn.style.color = 'var(--text-primary)';
            }
        });

        // Load scene data (placeholder)
        this.loadSceneData(sceneId);

        console.log(`🎬 Selected scene ${sceneId + 1}`);
    }

    // Load scene data
    static loadSceneData(sceneId) {
        // Placeholder for loading scene-specific clip data
        // In a real implementation, this would load saved scene configurations

        // Update clip names and content based on scene
        const sceneNames = [
            ['Kick 1', 'Snare 1', 'HiHat', 'Crash', 'Bass 1', 'Lead 1', 'Pad 1', 'FX 1', 'Kick 2', 'Snare 2', 'Ride', 'Tom', 'Bass 2', 'Lead 2', 'Pad 2', 'FX 2'],
            ['Drum Loop 1', 'Percussion', 'Shaker', 'Tambourine', 'Synth Bass', 'Arp 1', 'Chord 1', 'Reverb Tail', 'Drum Loop 2', 'Conga', 'Cowbell', 'Woodblock', 'FM Bass', 'Arp 2', 'Chord 2', 'Delay Tail'],
            ['Build 1', 'Build 2', 'Drop 1', 'Drop 2', 'Melody 1', 'Melody 2', 'Harmony 1', 'Harmony 2', 'Build 3', 'Build 4', 'Drop 3', 'Drop 4', 'Melody 3', 'Melody 4', 'Harmony 3', 'Harmony 4'],
            ['Ambient 1', 'Ambient 2', 'Texture 1', 'Texture 2', 'Drone 1', 'Drone 2', 'Noise 1', 'Noise 2', 'Ambient 3', 'Ambient 4', 'Texture 3', 'Texture 4', 'Drone 3', 'Drone 4', 'Noise 3', 'Noise 4']
        ];

        const sceneData = sceneNames[sceneId] || [];

        this.clips.forEach((clip, clipId) => {
            const index = clip.row * 4 + clip.col;
            const name = sceneData[index] || 'Empty';
            const hasContent = name !== 'Empty';

            clip.name = name;
            clip.hasContent = hasContent;

            // Update UI
            const nameElement = clip.element.querySelector('.clip-name');
            if (nameElement) {
                nameElement.textContent = name;
                nameElement.style.color = hasContent ? 'var(--text-primary)' : 'var(--text-muted)';
            }

            // Update background color
            if (hasContent) {
                clip.element.style.backgroundColor = clip.color;
                clip.element.style.color = this.getContrastColor(clip.color);
            } else {
                clip.element.style.backgroundColor = 'var(--secondary-bg)';
                clip.element.style.color = 'var(--text-primary)';
            }
        });
    }

    // Get contrast color for text
    static getContrastColor(bgColor) {
        // Simple contrast calculation - in a real implementation,
        // you'd use a proper contrast ratio calculation
        const r = parseInt(bgColor.slice(1, 3), 16);
        const g = parseInt(bgColor.slice(3, 5), 16);
        const b = parseInt(bgColor.slice(5, 7), 16);
        const brightness = (r * 299 + g * 587 + b * 114) / 1000;
        return brightness > 128 ? '#000000' : '#ffffff';
    }

    // Trigger clip
    static triggerClip(row, col) {
        const clipId = `clip_${row}_${col}`;
        const clip = this.clips.get(clipId);

        if (!clip || !clip.hasContent) return;

        // Toggle play state
        if (clip.isPlaying) {
            this.stopClip(row, col);
        } else {
            this.playClip(row, col);
        }
    }

    // Play clip
    static playClip(row, col) {
        const clipId = `clip_${row}_${col}`;
        const clip = this.clips.get(clipId);

        if (!clip || !clip.hasContent) return;

        // Stop any currently playing clips in the same row (follow action simulation)
        for (let c = 0; c < 4; c++) {
            if (c !== col) {
                this.stopClip(row, c);
            }
        }

        // Start playing
        clip.isPlaying = true;
        clip.element.classList.add('playing');
        clip.element.style.backgroundColor = '#00ff88';
        clip.element.style.boxShadow = '0 0 20px var(--accent-color)';

        // Update status indicator
        const statusElement = clip.element.querySelector('.clip-status');
        if (statusElement) {
            statusElement.style.backgroundColor = 'var(--success-color)';
        }

        console.log(`▶ Playing clip ${clip.name} (${row + 1}-${col + 1})`);

        // Simulate clip playback duration
        setTimeout(() => {
            if (clip.isPlaying) {
                this.stopClip(row, col);
            }
        }, 4000); // 4 second clips
    }

    // Stop clip
    static stopClip(row, col) {
        const clipId = `clip_${row}_${col}`;
        const clip = this.clips.get(clipId);

        if (!clip) return;

        clip.isPlaying = false;
        clip.element.classList.remove('playing');
        clip.element.style.backgroundColor = clip.hasContent ? clip.color : 'var(--secondary-bg)';
        clip.element.style.boxShadow = 'none';

        // Update status indicator
        const statusElement = clip.element.querySelector('.clip-status');
        if (statusElement) {
            statusElement.style.backgroundColor = 'var(--text-muted)';
        }

        console.log(`⏹ Stopped clip ${clip.name} (${row + 1}-${col + 1})`);
    }

    // Stop all clips
    static stopAllClips() {
        this.clips.forEach((clip, clipId) => {
            if (clip.isPlaying) {
                const [row, col] = clipId.replace('clip_', '').split('_').map(Number);
                this.stopClip(row, col);
            }
        });

        console.log('⏹ Stopped all clips');
    }

    // Toggle solo mode
    static toggleSoloMode() {
        // Placeholder for solo mode implementation
        console.log('🎵 Solo mode toggled');
    }

    // Show clip context menu
    static showClipContextMenu(e, row, col) {
        // Create context menu
        const menu = document.createElement('div');
        menu.className = 'context-menu';
        menu.style.position = 'fixed';
        menu.style.left = `${e.clientX}px`;
        menu.style.top = `${e.clientY}px`;
        menu.style.backgroundColor = 'var(--secondary-bg)';
        menu.style.border = '1px solid var(--border-color)';
        menu.style.borderRadius = 'var(--border-radius)';
        menu.style.boxShadow = 'var(--shadow)';
        menu.style.zIndex = '1000';
        menu.style.minWidth = '150px';

        menu.innerHTML = `
            <div class="context-menu-item" data-action="record">Record Clip</div>
            <div class="context-menu-item" data-action="duplicate">Duplicate</div>
            <div class="context-menu-item" data-action="clear">Clear Clip</div>
            <div class="context-menu-item" data-action="rename">Rename Clip</div>
        `;

        // Add event listeners
        menu.addEventListener('click', (e) => {
            const action = e.target.dataset.action;
            if (action) {
                this.handleContextMenuAction(action, row, col);
                menu.remove();
            }
        });

        // Remove menu when clicking elsewhere
        document.addEventListener('click', () => menu.remove(), { once: true });

        document.body.appendChild(menu);
    }

    // Handle context menu actions
    static handleContextMenuAction(action, row, col) {
        const clipId = `clip_${row}_${col}`;
        const clip = this.clips.get(clipId);

        switch (action) {
            case 'record':
                this.startRecordingClip(row, col);
                break;
            case 'duplicate':
                this.duplicateClip(row, col);
                break;
            case 'clear':
                this.clearClip(row, col);
                break;
            case 'rename':
                this.renameClip(row, col);
                break;
        }
    }

    // Start recording clip
    static startRecordingClip(row, col) {
        const clipId = `clip_${row}_${col}`;
        const clip = this.clips.get(clipId);

        if (!clip) return;

        clip.isRecording = true;
        clip.element.classList.add('recording');
        clip.element.style.backgroundColor = 'var(--error-color)';

        // Update status indicator
        const statusElement = clip.element.querySelector('.clip-status');
        if (statusElement) {
            statusElement.style.backgroundColor = 'var(--error-color)';
        }

        console.log(`● Started recording clip (${row + 1}-${col + 1})`);

        // Simulate recording duration
        setTimeout(() => {
            this.stopRecordingClip(row, col);
        }, 4000);
    }

    // Stop recording clip
    static stopRecordingClip(row, col) {
        const clipId = `clip_${row}_${col}`;
        const clip = this.clips.get(clipId);

        if (!clip || !clip.isRecording) return;

        clip.isRecording = false;
        clip.element.classList.remove('recording');
        clip.hasContent = true;
        clip.name = `Recording ${row + 1}-${col + 1}`;

        // Update UI
        const nameElement = clip.element.querySelector('.clip-name');
        if (nameElement) {
            nameElement.textContent = clip.name;
            nameElement.style.color = 'var(--text-primary)';
        }

        clip.element.style.backgroundColor = clip.color;
        clip.element.style.color = this.getContrastColor(clip.color);

        // Update status indicator
        const statusElement = clip.element.querySelector('.clip-status');
        if (statusElement) {
            statusElement.style.backgroundColor = 'var(--text-muted)';
        }

        console.log(`⬜ Finished recording clip (${row + 1}-${col + 1})`);
    }

    // Duplicate clip
    static duplicateClip(row, col) {
        // Find next available slot
        for (let r = 0; r < 4; r++) {
            for (let c = 0; c < 4; c++) {
                const targetClip = this.clips.get(`clip_${r}_${c}`);
                if (targetClip && !targetClip.hasContent) {
                    // Copy clip data
                    const sourceClip = this.clips.get(`clip_${row}_${col}`);
                    if (sourceClip) {
                        targetClip.name = sourceClip.name + ' (Copy)';
                        targetClip.hasContent = true;
                        targetClip.color = sourceClip.color;

                        // Update UI
                        const nameElement = targetClip.element.querySelector('.clip-name');
                        if (nameElement) {
                            nameElement.textContent = targetClip.name;
                            nameElement.style.color = 'var(--text-primary)';
                        }

                        targetClip.element.style.backgroundColor = targetClip.color;
                        targetClip.element.style.color = this.getContrastColor(targetClip.color);
                    }
                    return;
                }
            }
        }

        UIManager.showNotification('No available slots for duplication', 'warning');
    }

    // Clear clip
    static clearClip(row, col) {
        const clipId = `clip_${row}_${col}`;
        const clip = this.clips.get(clipId);

        if (!clip) return;

        clip.name = 'Empty';
        clip.hasContent = false;
        clip.isPlaying = false;
        clip.isRecording = false;

        // Update UI
        const nameElement = clip.element.querySelector('.clip-name');
        if (nameElement) {
            nameElement.textContent = clip.name;
            nameElement.style.color = 'var(--text-muted)';
        }

        clip.element.style.backgroundColor = 'var(--secondary-bg)';
        clip.element.style.color = 'var(--text-primary)';
        clip.element.classList.remove('playing', 'recording');

        // Update status indicator
        const statusElement = clip.element.querySelector('.clip-status');
        if (statusElement) {
            statusElement.style.backgroundColor = 'var(--text-muted)';
        }

        console.log(`🗑️ Cleared clip (${row + 1}-${col + 1})`);
    }

    // Rename clip
    static renameClip(row, col) {
        const clipId = `clip_${row}_${col}`;
        const clip = this.clips.get(clipId);

        if (!clip) return;

        const newName = prompt('Enter new clip name:', clip.name);
        if (newName && newName.trim()) {
            clip.name = newName.trim();

            // Update UI
            const nameElement = clip.element.querySelector('.clip-name');
            if (nameElement) {
                nameElement.textContent = clip.name;
            }

            console.log(`✏️ Renamed clip to "${clip.name}"`);
        }
    }

    // Handle window resize
    static onResize() {
        // Update clip matrix layout if needed
    }

    // Activate live view
    static onActivate() {
        console.log('🎵 Live view activated');
        this.selectScene(this.currentScene); // Refresh current scene
    }

    // Update clips (called by animation loop)
    static update() {
        if (!this.isInitialized) return;

        // Update playing clip animations
        this.clips.forEach(clip => {
            if (clip.isPlaying) {
                // Add subtle animation to playing clips
                const hue = (Date.now() / 50) % 360;
                const statusElement = clip.element.querySelector('.clip-status');
                if (statusElement) {
                    statusElement.style.backgroundColor = `hsl(${hue}, 70%, 50%)`;
                }
            }
        });
    }
}

// Initialize when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    LiveView.init();
});

// Export for global access
window.LiveView = LiveView;