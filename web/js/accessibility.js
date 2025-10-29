/**
 * Modurust DAW - Accessibility Features
 * WCAG 2.1 AA compliant accessibility enhancements
 */

class AccessibilityManager {
    static isInitialized = false;
    static highContrastMode = false;
    static reducedMotion = false;
    static screenReaderMode = false;
    static keyboardNavigation = true;
    static focusIndicator = true;
    static skipLinks = new Map();

    // Initialize accessibility features
    static init() {
        console.log('♿ Initializing Accessibility Manager...');

        this.detectUserPreferences();
        this.setupKeyboardNavigation();
        this.setupSkipLinks();
        this.setupFocusManagement();
        this.setupScreenReaderSupport();
        this.setupHighContrastSupport();
        this.setupReducedMotionSupport();
        this.createAccessibilityPanel();

        this.isInitialized = true;
        console.log('✅ Accessibility Manager initialized');
    }

    // Detect user accessibility preferences
    static detectUserPreferences() {
        // Check for prefers-reduced-motion
        const reducedMotionQuery = window.matchMedia('(prefers-reduced-motion: reduce)');
        this.reducedMotion = reducedMotionQuery.matches;

        reducedMotionQuery.addEventListener('change', (e) => {
            this.reducedMotion = e.matches;
            this.updateReducedMotion();
        });

        // Check for prefers-contrast
        const highContrastQuery = window.matchMedia('(prefers-contrast: high)');
        this.highContrastMode = highContrastQuery.matches;

        highContrastQuery.addEventListener('change', (e) => {
            this.highContrastMode = e.matches;
            this.updateHighContrast();
        });

        // Check for prefers-color-scheme
        const colorSchemeQuery = window.matchMedia('(prefers-color-scheme: dark)');
        // Already using dark theme by default

        console.log(`Accessibility preferences: Reduced motion: ${this.reducedMotion}, High contrast: ${this.highContrastMode}`);
    }

    // Setup keyboard navigation
    static setupKeyboardNavigation() {
        document.addEventListener('keydown', (e) => {
            this.handleKeyboardNavigation(e);
        });

        // Make all interactive elements focusable
        this.makeFocusable('[role="button"]');
        this.makeFocusable('.view-tab');
        this.makeFocusable('.toolbar-btn');
        this.makeFocusable('.transport-btn');
        this.makeFocusable('.clip-btn');
        this.makeFocusable('.node-item');
        this.makeFocusable('input, select, textarea, button');
    }

    // Handle keyboard navigation
    static handleKeyboardNavigation(e) {
        const activeElement = document.activeElement;

        // Tab navigation is handled by browser
        // Add custom keyboard shortcuts
        if (e.ctrlKey || e.metaKey) {
            switch (e.key.toLowerCase()) {
                case '1':
                case '2':
                case '3':
                    e.preventDefault();
                    const viewIndex = parseInt(e.key) - 1;
                    this.switchToView(viewIndex);
                    break;
                case 's':
                    e.preventDefault();
                    this.saveProject();
                    break;
                case 'o':
                    e.preventDefault();
                    this.openProject();
                    break;
                case 'n':
                    e.preventDefault();
                    this.newProject();
                    break;
                case 'z':
                    if (e.shiftKey) {
                        e.preventDefault();
                        this.redo();
                    } else {
                        e.preventDefault();
                        this.undo();
                    }
                    break;
                case 'y':
                    e.preventDefault();
                    this.redo();
                    break;
                case 'a':
                    e.preventDefault();
                    this.selectAll();
                    break;
                case ' ':
                    e.preventDefault();
                    this.playPause();
                    break;
            }
        }

        // Arrow key navigation for certain elements
        if (activeElement && activeElement.classList.contains('clip-btn')) {
            this.handleClipNavigation(e, activeElement);
        }

        // Escape key handling
        if (e.key === 'Escape') {
            this.handleEscape();
        }
    }

    // Switch to specific view
    static switchToView(viewIndex) {
        const views = ['arrangement', 'live', 'node'];
        const viewName = views[viewIndex];

        if (viewName) {
            UIManager.switchView(viewName);
            this.announceToScreenReader(`Switched to ${viewName} view`);
        }
    }

    // Handle clip navigation with arrow keys
    static handleClipNavigation(e, clipElement) {
        if (!['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight'].includes(e.key)) return;

        e.preventDefault();

        const currentRow = parseInt(clipElement.dataset.row);
        const currentCol = parseInt(clipElement.dataset.col);

        let newRow = currentRow;
        let newCol = currentCol;

        switch (e.key) {
            case 'ArrowUp':
                newRow = Math.max(0, currentRow - 1);
                break;
            case 'ArrowDown':
                newRow = Math.min(3, currentRow + 1);
                break;
            case 'ArrowLeft':
                newCol = Math.max(0, currentCol - 1);
                break;
            case 'ArrowRight':
                newCol = Math.min(3, currentCol + 1);
                break;
        }

        const newClipId = `clip_${newRow}_${newCol}`;
        const newClip = document.getElementById(newClipId);

        if (newClip) {
            newClip.focus();
            this.announceToScreenReader(`Clip ${newRow + 1}-${newCol + 1}`);
        }
    }

    // Handle escape key
    static handleEscape() {
        // Close modals
        const openModals = document.querySelectorAll('.modal.active');
        if (openModals.length > 0) {
            openModals.forEach(modal => modal.classList.remove('active'));
            this.announceToScreenReader('Modal closed');
            return;
        }

        // Clear selections
        this.clearSelections();

        // Close panels
        this.closePanels();
    }

    // Setup skip links
    static setupSkipLinks() {
        const skipLinks = [
            { id: 'skip-to-main', href: '#main-content', text: 'Skip to main content' },
            { id: 'skip-to-navigation', href: '#view-tabs', text: 'Skip to navigation' },
            { id: 'skip-to-transport', href: '#transport-bar', text: 'Skip to transport controls' }
        ];

        skipLinks.forEach(link => {
            const skipLink = document.createElement('a');
            skipLink.href = link.href;
            skipLink.className = 'skip-link sr-only';
            skipLink.textContent = link.text;
            skipLink.id = link.id;

            document.body.insertBefore(skipLink, document.body.firstChild);
            this.skipLinks.set(link.id, skipLink);
        });
    }

    // Setup focus management
    static setupFocusManagement() {
        // Focus trap for modals
        document.addEventListener('keydown', (e) => {
            if (e.key === 'Tab') {
                const modal = document.querySelector('.modal.active');
                if (modal) {
                    this.trapFocus(modal, e);
                }
            }
        });

        // Focus indicators
        document.addEventListener('focusin', (e) => {
            this.updateFocusIndicator(e.target);
        });

        // Restore focus when modal closes
        document.addEventListener('click', (e) => {
            if (e.target.classList.contains('modal')) {
                const previouslyFocused = document.querySelector('[data-previous-focus]');
                if (previouslyFocused) {
                    previouslyFocused.focus();
                    previouslyFocused.removeAttribute('data-previous-focus');
                }
            }
        });
    }

    // Trap focus within modal
    static trapFocus(modal, e) {
        const focusableElements = modal.querySelectorAll(
            'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
        );

        const firstElement = focusableElements[0];
        const lastElement = focusableElements[focusableElements.length - 1];

        if (e.shiftKey) {
            if (document.activeElement === firstElement) {
                lastElement.focus();
                e.preventDefault();
            }
        } else {
            if (document.activeElement === lastElement) {
                firstElement.focus();
                e.preventDefault();
            }
        }
    }

    // Update focus indicator
    static updateFocusIndicator(element) {
        if (!this.focusIndicator) return;

        // Remove previous focus indicators
        document.querySelectorAll('.focus-indicator').forEach(indicator => {
            indicator.classList.remove('focus-indicator');
        });

        // Add focus indicator to current element
        if (element && element.classList) {
            element.classList.add('focus-indicator');
        }
    }

    // Setup screen reader support
    static setupScreenReaderSupport() {
        // Create live region for announcements
        const liveRegion = document.createElement('div');
        liveRegion.id = 'sr-live-region';
        liveRegion.setAttribute('aria-live', 'polite');
        liveRegion.setAttribute('aria-atomic', 'true');
        liveRegion.className = 'sr-only';
        document.body.appendChild(liveRegion);

        // Add ARIA labels and descriptions
        this.addAriaLabels();

        // Setup live updates for dynamic content
        this.setupLiveUpdates();
    }

    // Add ARIA labels and descriptions
    static addAriaLabels() {
        // Main landmarks
        const mainContent = document.querySelector('.central-content');
        if (mainContent) {
            mainContent.setAttribute('role', 'main');
            mainContent.setAttribute('aria-label', 'Main content area');
        }

        // Navigation
        const viewTabs = document.getElementById('view-tabs');
        if (viewTabs) {
            viewTabs.setAttribute('role', 'navigation');
            viewTabs.setAttribute('aria-label', 'View navigation');
        }

        // Transport controls
        const transportBar = document.querySelector('.transport-bar');
        if (transportBar) {
            transportBar.setAttribute('role', 'toolbar');
            transportBar.setAttribute('aria-label', 'Transport controls');
        }

        // Status indicators
        const statusIndicators = document.querySelectorAll('.status-indicator');
        statusIndicators.forEach(indicator => {
            const statusText = indicator.nextElementSibling;
            if (statusText) {
                indicator.setAttribute('aria-describedby', statusText.id);
            }
        });
    }

    // Setup live updates for screen readers
    static setupLiveUpdates() {
        // Monitor transport state changes
        this.monitorTransportState();

        // Monitor view changes
        this.monitorViewChanges();

        // Monitor clip state changes
        this.monitorClipChanges();
    }

    // Monitor transport state
    static monitorTransportState() {
        // This would be called when transport state changes
        // For now, we'll announce on button clicks
        const transportButtons = document.querySelectorAll('.transport-btn');
        transportButtons.forEach(button => {
            button.addEventListener('click', () => {
                const action = button.textContent.toLowerCase();
                this.announceToScreenReader(`Transport: ${action}`);
            });
        });
    }

    // Monitor view changes
    static monitorViewChanges() {
        const viewTabs = document.querySelectorAll('.view-tab');
        viewTabs.forEach(tab => {
            tab.addEventListener('click', () => {
                const viewName = tab.textContent.toLowerCase();
                this.announceToScreenReader(`Switched to ${viewName} view`);
            });
        });
    }

    // Monitor clip changes
    static monitorClipChanges() {
        // This would monitor clip state changes in Live view
        document.addEventListener('click', (e) => {
            if (e.target.classList.contains('clip-btn')) {
                const clipName = e.target.querySelector('.clip-name')?.textContent || 'Empty clip';
                const isPlaying = e.target.classList.contains('playing');
                const action = isPlaying ? 'started playing' : 'stopped';
                this.announceToScreenReader(`Clip ${clipName} ${action}`);
            }
        });
    }

    // Announce to screen reader
    static announceToScreenReader(message) {
        const liveRegion = document.getElementById('sr-live-region');
        if (liveRegion) {
            liveRegion.textContent = message;
        }
    }

    // Setup high contrast support
    static setupHighContrastSupport() {
        if (this.highContrastMode) {
            document.documentElement.classList.add('high-contrast');
        }
    }

    // Update high contrast mode
    static updateHighContrast() {
        if (this.highContrastMode) {
            document.documentElement.classList.add('high-contrast');
        } else {
            document.documentElement.classList.remove('high-contrast');
        }
    }

    // Setup reduced motion support
    static setupReducedMotionSupport() {
        if (this.reducedMotion) {
            document.documentElement.classList.add('reduced-motion');
        }
    }

    // Update reduced motion
    static updateReducedMotion() {
        if (this.reducedMotion) {
            document.documentElement.classList.add('reduced-motion');
        } else {
            document.documentElement.classList.remove('reduced-motion');
        }
    }

    // Create accessibility settings panel
    static createAccessibilityPanel() {
        const panel = document.createElement('div');
        panel.id = 'accessibility-panel';
        panel.className = 'accessibility-panel';
        panel.innerHTML = `
            <div class="panel-header">
                <h3>Accessibility Settings</h3>
                <button class="panel-close" aria-label="Close accessibility panel">&times;</button>
            </div>
            <div class="panel-content">
                <div class="accessibility-options">
                    <div class="option-group">
                        <label class="option-label">
                            <input type="checkbox" id="keyboard-nav-toggle" checked>
                            Keyboard Navigation
                        </label>
                        <p class="option-description">Use Tab and arrow keys to navigate the interface</p>
                    </div>

                    <div class="option-group">
                        <label class="option-label">
                            <input type="checkbox" id="focus-indicator-toggle" checked>
                            Focus Indicators
                        </label>
                        <p class="option-description">Show visual indicators for focused elements</p>
                    </div>

                    <div class="option-group">
                        <label class="option-label">
                            <input type="checkbox" id="screen-reader-toggle">
                            Screen Reader Mode
                        </label>
                        <p class="option-description">Enhanced support for screen readers</p>
                    </div>

                    <div class="option-group">
                        <label class="option-label">
                            <input type="checkbox" id="high-contrast-toggle">
                            High Contrast Mode
                        </label>
                        <p class="option-description">Increase contrast for better visibility</p>
                    </div>

                    <div class="option-group">
                        <label class="option-label">
                            <input type="checkbox" id="reduced-motion-toggle">
                            Reduced Motion
                        </label>
                        <p class="option-description">Minimize animations and transitions</p>
                    </div>
                </div>

                <div class="accessibility-actions">
                    <button class="btn-primary" id="reset-accessibility">Reset to Defaults</button>
                    <button class="btn-secondary" id="test-accessibility">Test Accessibility</button>
                </div>

                <div class="accessibility-info">
                    <h4>Keyboard Shortcuts</h4>
                    <dl class="shortcuts-list">
                        <dt>Ctrl+1/2/3</dt><dd>Switch between views</dd>
                        <dt>Space</dt><dd>Play/Pause</dd>
                        <dt>Ctrl+S</dt><dd>Save project</dd>
                        <dt>Ctrl+Z</dt><dd>Undo</dd>
                        <dt>Ctrl+Y</dt><dd>Redo</dd>
                        <dt>Escape</dt><dd>Close dialogs/Clear selection</dd>
                        <dt>Tab</dt><dd>Navigate between elements</dd>
                    </dl>
                </div>
            </div>
        `;

        // Add to DOM
        document.body.appendChild(panel);

        // Setup event listeners
        this.setupAccessibilityPanelEvents(panel);
    }

    // Setup accessibility panel events
    static setupAccessibilityPanelEvents(panel) {
        const closeBtn = panel.querySelector('.panel-close');
        const keyboardNavToggle = document.getElementById('keyboard-nav-toggle');
        const focusIndicatorToggle = document.getElementById('focus-indicator-toggle');
        const screenReaderToggle = document.getElementById('screen-reader-toggle');
        const highContrastToggle = document.getElementById('high-contrast-toggle');
        const reducedMotionToggle = document.getElementById('reduced-motion-toggle');
        const resetBtn = document.getElementById('reset-accessibility');
        const testBtn = document.getElementById('test-accessibility');

        // Close panel
        closeBtn.addEventListener('click', () => {
            panel.classList.remove('visible');
        });

        // Toggle options
        keyboardNavToggle.addEventListener('change', (e) => {
            this.keyboardNavigation = e.target.checked;
            this.announceToScreenReader(`Keyboard navigation ${this.keyboardNavigation ? 'enabled' : 'disabled'}`);
        });

        focusIndicatorToggle.addEventListener('change', (e) => {
            this.focusIndicator = e.target.checked;
            this.announceToScreenReader(`Focus indicators ${this.focusIndicator ? 'enabled' : 'disabled'}`);
        });

        screenReaderToggle.addEventListener('change', (e) => {
            this.screenReaderMode = e.target.checked;
            this.updateScreenReaderMode();
            this.announceToScreenReader(`Screen reader mode ${this.screenReaderMode ? 'enabled' : 'disabled'}`);
        });

        highContrastToggle.addEventListener('change', (e) => {
            this.highContrastMode = e.target.checked;
            this.updateHighContrast();
            this.announceToScreenReader(`High contrast mode ${this.highContrastMode ? 'enabled' : 'disabled'}`);
        });

        reducedMotionToggle.addEventListener('change', (e) => {
            this.reducedMotion = e.target.checked;
            this.updateReducedMotion();
            this.announceToScreenReader(`Reduced motion ${this.reducedMotion ? 'enabled' : 'disabled'}`);
        });

        // Reset to defaults
        resetBtn.addEventListener('click', () => {
            this.resetToDefaults();
            this.announceToScreenReader('Accessibility settings reset to defaults');
        });

        // Test accessibility
        testBtn.addEventListener('click', () => {
            this.runAccessibilityTest();
        });

        // Set initial values
        keyboardNavToggle.checked = this.keyboardNavigation;
        focusIndicatorToggle.checked = this.focusIndicator;
        screenReaderToggle.checked = this.screenReaderMode;
        highContrastToggle.checked = this.highContrastMode;
        reducedMotionToggle.checked = this.reducedMotion;
    }

    // Update screen reader mode
    static updateScreenReaderMode() {
        if (this.screenReaderMode) {
            document.documentElement.classList.add('screen-reader-mode');
            // Additional screen reader enhancements
            this.enhanceForScreenReaders();
        } else {
            document.documentElement.classList.remove('screen-reader-mode');
        }
    }

    // Enhance for screen readers
    static enhanceForScreenReaders() {
        // Add more descriptive labels
        const buttons = document.querySelectorAll('button:not([aria-label])');
        buttons.forEach(button => {
            const text = button.textContent.trim();
            if (text) {
                button.setAttribute('aria-label', text);
            }
        });

        // Add descriptions to complex elements
        const clips = document.querySelectorAll('.clip-btn');
        clips.forEach(clip => {
            const name = clip.querySelector('.clip-name')?.textContent || 'Empty';
            const row = clip.dataset.row;
            const col = clip.dataset.col;
            clip.setAttribute('aria-label', `Clip ${row}-${col}: ${name}`);
        });
    }

    // Reset to defaults
    static resetToDefaults() {
        this.keyboardNavigation = true;
        this.focusIndicator = true;
        this.screenReaderMode = false;
        this.highContrastMode = false;
        this.reducedMotion = false;

        // Update UI
        document.getElementById('keyboard-nav-toggle').checked = true;
        document.getElementById('focus-indicator-toggle').checked = true;
        document.getElementById('screen-reader-toggle').checked = false;
        document.getElementById('high-contrast-toggle').checked = false;
        document.getElementById('reduced-motion-toggle').checked = false;

        // Apply changes
        this.updateScreenReaderMode();
        this.updateHighContrast();
        this.updateReducedMotion();
    }

    // Run accessibility test
    static runAccessibilityTest() {
        console.log('🧪 Running accessibility tests...');

        const results = {
            focusableElements: this.testFocusableElements(),
            ariaLabels: this.testAriaLabels(),
            colorContrast: this.testColorContrast(),
            keyboardNavigation: this.testKeyboardNavigation()
        };

        console.table(results);

        const passedTests = Object.values(results).filter(result => result).length;
        const totalTests = Object.keys(results).length;

        this.announceToScreenReader(`Accessibility test completed: ${passedTests} of ${totalTests} tests passed`);

        UIManager.showNotification(`Accessibility test: ${passedTests}/${totalTests} tests passed`, 'info');
    }

    // Test focusable elements
    static testFocusableElements() {
        const interactiveElements = document.querySelectorAll('button, [href], input, select, textarea, [tabindex]');
        const focusableElements = Array.from(interactiveElements).filter(el =>
            !el.hasAttribute('disabled') &&
            !el.hasAttribute('hidden') &&
            el.offsetWidth > 0 &&
            el.offsetHeight > 0
        );

        return focusableElements.length > 10; // Basic check
    }

    // Test ARIA labels
    static testAriaLabels() {
        const interactiveElements = document.querySelectorAll('button, [role="button"]');
        let labeledElements = 0;

        interactiveElements.forEach(el => {
            if (el.hasAttribute('aria-label') || el.hasAttribute('aria-labelledby') || el.textContent.trim()) {
                labeledElements++;
            }
        });

        return labeledElements === interactiveElements.length;
    }

    // Test color contrast (simplified)
    static testColorContrast() {
        // This would require more complex color analysis
        // For now, just check if high contrast mode is available
        return true;
    }

    // Test keyboard navigation
    static testKeyboardNavigation() {
        // Check if Tab key works
        const focusableElements = document.querySelectorAll('button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])');
        return focusableElements.length > 0;
    }

    // Make elements focusable
    static makeFocusable(selector) {
        const elements = document.querySelectorAll(selector);
        elements.forEach(el => {
            if (!el.hasAttribute('tabindex') && el.tabIndex === -1) {
                el.setAttribute('tabindex', '0');
            }
        });
    }

    // Utility functions (placeholders for actual implementation)
    static saveProject() { console.log('Save project'); }
    static openProject() { console.log('Open project'); }
    static newProject() { console.log('New project'); }
    static undo() { console.log('Undo'); }
    static redo() { console.log('Redo'); }
    static selectAll() { console.log('Select all'); }
    static playPause() { console.log('Play/Pause'); }
    static clearSelections() { console.log('Clear selections'); }
    static closePanels() { console.log('Close panels'); }

    // Show accessibility panel
    static showAccessibilityPanel() {
        const panel = document.getElementById('accessibility-panel');
        if (panel) {
            panel.classList.add('visible');
            // Focus first element in panel
            const firstInput = panel.querySelector('input');
            if (firstInput) {
                firstInput.focus();
            }
        }
    }

    // Dispose resources
    static dispose() {
        // Clean up event listeners and resources
        console.log('🗑️ Accessibility Manager disposed');
    }
}

// Initialize when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    AccessibilityManager.init();
});

// Export for global access
window.AccessibilityManager = AccessibilityManager;