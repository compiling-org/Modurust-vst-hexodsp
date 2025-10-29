/**
 * Modurust DAW - UI Manager
 * Manages UI state, interactions, and component coordination
 */

class UIManager {
    static components = new Map();
    static eventListeners = new Map();
    static isInitialized = false;

    // Initialize UI Manager
    static init() {
        console.log('🎨 Initializing UI Manager...');

        this.registerComponents();
        this.setupGlobalEventListeners();
        this.initializeThemes();
        this.setupKeyboardNavigation();

        this.isInitialized = true;
        console.log('✅ UI Manager initialized');
    }

    // Register UI components
    static registerComponents() {
        // Register core components
        this.registerComponent('transport', new TransportComponent());
        this.registerComponent('browser', new BrowserComponent());
        this.registerComponent('mixer', new MixerComponent());
        this.registerComponent('settings', new SettingsComponent());

        console.log('✅ UI components registered');
    }

    // Register a component
    static registerComponent(name, component) {
        this.components.set(name, component);
        component.init();
    }

    // Get a component
    static getComponent(name) {
        return this.components.get(name);
    }

    // Setup global event listeners
    static setupGlobalEventListeners() {
        // Modal management
        document.addEventListener('click', (e) => {
            if (e.target.classList.contains('modal')) {
                this.closeModal(e.target.id);
            }
        });

        // Close buttons
        document.addEventListener('click', (e) => {
            if (e.target.classList.contains('modal-close') || e.target.closest('.modal-close')) {
                const modal = e.target.closest('.modal');
                if (modal) {
                    this.closeModal(modal.id);
                }
            }
        });

        // Panel toggles
        document.addEventListener('click', (e) => {
            if (e.target.classList.contains('panel-toggle')) {
                this.togglePanel(e.target);
            }
        });

        console.log('✅ Global event listeners set up');
    }

    // Initialize themes
    static initializeThemes() {
        const savedTheme = localStorage.getItem('modurust-theme') || 'dark';
        this.setTheme(savedTheme);
    }

    // Set theme
    static setTheme(theme) {
        document.documentElement.setAttribute('data-theme', theme);
        localStorage.setItem('modurust-theme', theme);

        // Update theme selector if it exists
        const themeSelect = document.getElementById('theme-select');
        if (themeSelect) {
            themeSelect.value = theme;
        }
    }

    // Setup keyboard navigation
    static setupKeyboardNavigation() {
        document.addEventListener('keydown', (e) => {
            // Tab navigation
            if (e.key === 'Tab') {
                this.handleTabNavigation(e);
            }

            // Escape key
            if (e.key === 'Escape') {
                this.handleEscapeKey(e);
            }
        });
    }

    // Handle tab navigation
    static handleTabNavigation(e) {
        const focusableElements = document.querySelectorAll(
            'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
        );

        const firstElement = focusableElements[0];
        const lastElement = focusableElements[focusableElements.length - 1];

        if (e.shiftKey) {
            // Shift + Tab
            if (document.activeElement === firstElement) {
                lastElement.focus();
                e.preventDefault();
            }
        } else {
            // Tab
            if (document.activeElement === lastElement) {
                firstElement.focus();
                e.preventDefault();
            }
        }
    }

    // Handle escape key
    static handleEscapeKey(e) {
        // Close modals
        const activeModal = document.querySelector('.modal.active');
        if (activeModal) {
            this.closeModal(activeModal.id);
            e.preventDefault();
            return;
        }

        // Close panels or menus
        // Implementation for closing panels/menus
    }

    // Show modal
    static showModal(modalId) {
        const modal = document.getElementById(modalId);
        if (modal) {
            modal.classList.add('active');
            modal.setAttribute('aria-hidden', 'false');

            // Focus management
            const focusableElement = modal.querySelector('button, input, select, textarea');
            if (focusableElement) {
                focusableElement.focus();
            }

            // Announce to screen readers
            this.announceToScreenReader(`Opened ${modal.querySelector('h2')?.textContent || modalId} dialog`);
        }
    }

    // Close modal
    static closeModal(modalId) {
        const modal = document.getElementById(modalId);
        if (modal) {
            modal.classList.remove('active');
            modal.setAttribute('aria-hidden', 'true');

            // Return focus to trigger element
            const triggerElement = document.querySelector(`[data-modal="${modalId}"]`);
            if (triggerElement) {
                triggerElement.focus();
            }
        }
    }

    // Toggle panel
    static togglePanel(toggleButton) {
        const panel = toggleButton.closest('.sidebar-left, .sidebar-right');
        if (panel) {
            const isCollapsed = panel.classList.contains('collapsed');

            if (isCollapsed) {
                panel.classList.remove('collapsed');
                toggleButton.textContent = '−';
                toggleButton.setAttribute('aria-expanded', 'true');
            } else {
                panel.classList.add('collapsed');
                toggleButton.textContent = '+';
                toggleButton.setAttribute('aria-expanded', 'false');
            }

            // Save panel state
            const panelId = panel.id;
            localStorage.setItem(`panel-${panelId}-collapsed`, !isCollapsed);

            // Trigger resize event for views
            window.dispatchEvent(new Event('resize'));
        }
    }

    // Restore panel states
    static restorePanelStates() {
        const panels = document.querySelectorAll('.sidebar-left, .sidebar-right');
        panels.forEach(panel => {
            const panelId = panel.id;
            const isCollapsed = localStorage.getItem(`panel-${panelId}-collapsed`) === 'true';

            if (isCollapsed) {
                panel.classList.add('collapsed');
                const toggle = panel.querySelector('.panel-toggle');
                if (toggle) {
                    toggle.textContent = '+';
                    toggle.setAttribute('aria-expanded', 'false');
                }
            }
        });
    }

    // Show notification
    static showNotification(message, type = 'info', duration = 3000) {
        // Remove existing notifications
        this.clearNotifications();

        // Create notification element
        const notification = document.createElement('div');
        notification.className = `notification notification-${type}`;
        notification.innerHTML = `
            <span class="notification-icon">${this.getNotificationIcon(type)}</span>
            <span class="notification-message">${message}</span>
            <button class="notification-close" aria-label="Close notification">&times;</button>
        `;

        // Add to DOM
        document.body.appendChild(notification);

        // Setup close button
        const closeBtn = notification.querySelector('.notification-close');
        closeBtn.addEventListener('click', () => this.clearNotifications());

        // Auto-hide
        if (duration > 0) {
            setTimeout(() => {
                if (notification.parentNode) {
                    notification.remove();
                }
            }, duration);
        }

        // Announce to screen readers
        this.announceToScreenReader(message);
    }

    // Clear notifications
    static clearNotifications() {
        const notifications = document.querySelectorAll('.notification');
        notifications.forEach(notification => notification.remove());
    }

    // Get notification icon
    static getNotificationIcon(type) {
        switch (type) {
            case 'success': return '✅';
            case 'error': return '❌';
            case 'warning': return '⚠️';
            case 'info':
            default: return 'ℹ️';
        }
    }

    // Announce to screen readers
    static announceToScreenReader(message) {
        const announcement = document.createElement('div');
        announcement.setAttribute('aria-live', 'polite');
        announcement.setAttribute('aria-atomic', 'true');
        announcement.className = 'sr-only';
        announcement.textContent = message;

        document.body.appendChild(announcement);

        // Remove after announcement
        setTimeout(() => {
            if (announcement.parentNode) {
                announcement.remove();
            }
        }, 1000);
    }

    // Update component states
    static updateComponentStates() {
        this.components.forEach(component => {
            if (component.update) {
                component.update();
            }
        });
    }

    // Handle window resize
    static handleResize() {
        // Update component layouts
        this.components.forEach(component => {
            if (component.onResize) {
                component.onResize();
            }
        });
    }

    // Save UI state
    static saveUIState() {
        const uiState = {
            theme: document.documentElement.getAttribute('data-theme') || 'dark',
            panels: {}
        };

        // Save panel states
        const panels = document.querySelectorAll('.sidebar-left, .sidebar-right');
        panels.forEach(panel => {
            const panelId = panel.id;
            uiState.panels[panelId] = {
                collapsed: panel.classList.contains('collapsed')
            };
        });

        localStorage.setItem('modurust-ui-state', JSON.stringify(uiState));
    }

    // Load UI state
    static loadUIState() {
        try {
            const uiState = JSON.parse(localStorage.getItem('modurust-ui-state'));
            if (uiState) {
                // Restore theme
                if (uiState.theme) {
                    this.setTheme(uiState.theme);
                }

                // Restore panel states
                if (uiState.panels) {
                    Object.entries(uiState.panels).forEach(([panelId, state]) => {
                        const panel = document.getElementById(panelId);
                        if (panel && state.collapsed) {
                            panel.classList.add('collapsed');
                            const toggle = panel.querySelector('.panel-toggle');
                            if (toggle) {
                                toggle.textContent = '+';
                                toggle.setAttribute('aria-expanded', 'false');
                            }
                        }
                    });
                }
            }
        } catch (error) {
            console.warn('Failed to load UI state:', error);
        }
    }

    // Cleanup
    static dispose() {
        // Save state before cleanup
        this.saveUIState();

        // Dispose components
        this.components.forEach(component => {
            if (component.dispose) {
                component.dispose();
            }
        });

        this.components.clear();
        this.eventListeners.clear();
        this.isInitialized = false;

        console.log('🗑️ UI Manager disposed');
    }
}

// Base Component class
class UIComponent {
    constructor() {
        this.element = null;
        this.eventListeners = [];
    }

    init() {
        // Override in subclasses
    }

    update() {
        // Override in subclasses
    }

    onResize() {
        // Override in subclasses
    }

    dispose() {
        // Remove event listeners
        this.eventListeners.forEach(({ element, type, handler }) => {
            element.removeEventListener(type, handler);
        });
        this.eventListeners = [];
    }

    // Add event listener with tracking
    addEventListener(element, type, handler) {
        element.addEventListener(type, handler);
        this.eventListeners.push({ element, type, handler });
    }
}

// Transport Component
class TransportComponent extends UIComponent {
    init() {
        // Transport controls are handled in main.js
        console.log('🎵 Transport component initialized');
    }
}

// Browser Component
class BrowserComponent extends UIComponent {
    init() {
        this.setupBrowserTabs();
        console.log('📁 Browser component initialized');
    }

    setupBrowserTabs() {
        const tabs = document.querySelectorAll('.browser-tab');
        tabs.forEach(tab => {
            this.addEventListener(tab, 'click', () => this.switchTab(tab.dataset.tab));
        });
    }

    switchTab(tabName) {
        // Hide all sections
        document.querySelectorAll('.browser-section').forEach(section => {
            section.classList.remove('active');
        });

        // Remove active class from tabs
        document.querySelectorAll('.browser-tab').forEach(tab => {
            tab.classList.remove('active');
        });

        // Show selected section
        const selectedSection = document.getElementById(`${tabName}-tab`);
        const selectedTab = document.querySelector(`[data-tab="${tabName}"]`);

        if (selectedSection) {
            selectedSection.classList.add('active');
        }
        if (selectedTab) {
            selectedTab.classList.add('active');
        }
    }
}

// Mixer Component
class MixerComponent extends UIComponent {
    init() {
        this.createMixerChannels();
        console.log('🎛️ Mixer component initialized');
    }

    createMixerChannels() {
        const mixerChannels = document.querySelector('.mixer-channels');
        if (!mixerChannels) return;

        // Clear existing channels
        mixerChannels.innerHTML = '';

        // Create channels for each track
        AppState.tracks.forEach(track => {
            const channelElement = this.createChannelElement(track);
            mixerChannels.appendChild(channelElement);
        });
    }

    createChannelElement(track) {
        const channel = document.createElement('div');
        channel.className = 'mixer-channel';
        channel.innerHTML = `
            <div class="channel-name">${track.name}</div>
            <div class="channel-controls">
                <div class="fader-container">
                    <input type="range" class="volume-fader" min="0" max="100" value="${track.volume * 100}"
                           orient="vertical" data-track-id="${track.id}">
                    <div class="fader-label">Vol</div>
                </div>
                <div class="pan-container">
                    <input type="range" class="pan-knob" min="-100" max="100" value="${track.panValue * 100}"
                           data-track-id="${track.id}">
                    <div class="pan-label">Pan</div>
                </div>
            </div>
            <div class="channel-buttons">
                <button class="mute-btn" data-track-id="${track.id}">M</button>
                <button class="solo-btn" data-track-id="${track.id}">S</button>
            </div>
        `;

        // Add event listeners
        const volumeFader = channel.querySelector('.volume-fader');
        const panKnob = channel.querySelector('.pan-knob');
        const muteBtn = channel.querySelector('.mute-btn');
        const soloBtn = channel.querySelector('.solo-btn');

        this.addEventListener(volumeFader, 'input', (e) => {
            AudioEngine.setTrackVolume(track.id, e.target.value / 100);
        });

        this.addEventListener(panKnob, 'input', (e) => {
            AudioEngine.setTrackPan(track.id, e.target.value / 100);
        });

        this.addEventListener(muteBtn, 'click', () => {
            const isMuted = muteBtn.classList.contains('active');
            AudioEngine.setTrackMute(track.id, !isMuted);
            muteBtn.classList.toggle('active');
        });

        this.addEventListener(soloBtn, 'click', () => {
            const isSolo = soloBtn.classList.contains('active');
            AudioEngine.setTrackSolo(track.id, !isSolo);
            soloBtn.classList.toggle('active');
        });

        return channel;
    }
}

// Settings Component
class SettingsComponent extends UIComponent {
    init() {
        this.setupSettingsModal();
        console.log('⚙️ Settings component initialized');
    }

    setupSettingsModal() {
        const saveBtn = document.getElementById('save-settings');
        const cancelBtn = document.getElementById('cancel-settings');
        const themeSelect = document.getElementById('theme-select');

        if (saveBtn) {
            this.addEventListener(saveBtn, 'click', () => this.saveSettings());
        }

        if (cancelBtn) {
            this.addEventListener(cancelBtn, 'click', () => UIManager.closeModal('settings-modal'));
        }

        if (themeSelect) {
            this.addEventListener(themeSelect, 'change', (e) => {
                UIManager.setTheme(e.target.value);
            });
        }
    }

    saveSettings() {
        // Save settings logic
        const sampleRate = document.getElementById('sample-rate-select')?.value;
        const bufferSize = document.getElementById('buffer-size-select')?.value;

        // Apply settings to audio engine
        if (sampleRate) {
            console.log(`Setting sample rate to ${sampleRate}`);
        }
        if (bufferSize) {
            console.log(`Setting buffer size to ${bufferSize}`);
        }

        UIManager.closeModal('settings-modal');
        UIManager.showNotification('Settings saved successfully', 'success');
    }
}

// Initialize UI Manager when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    UIManager.init();
});

// Export for global access
window.UIManager = UIManager;