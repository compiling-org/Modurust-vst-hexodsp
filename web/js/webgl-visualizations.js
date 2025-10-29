/**
 * Modurust DAW - WebGL Audio Visualizations
 * Real-time audio visualization using WebGL for performance
 */

class WebGLVisualizations {
    static gl = null;
    static canvas = null;
    static programs = new Map();
    static buffers = new Map();
    static textures = new Map();
    static isInitialized = false;

    // Shader sources
    static vertexShaderSource = `
        attribute vec2 a_position;
        attribute vec2 a_texCoord;
        varying vec2 v_texCoord;

        void main() {
            gl_Position = vec4(a_position, 0.0, 1.0);
            v_texCoord = a_texCoord;
        }
    `;

    static spectrumFragmentShader = `
        precision mediump float;
        uniform sampler2D u_frequencyData;
        uniform float u_time;
        uniform vec2 u_resolution;
        varying vec2 v_texCoord;

        void main() {
            vec2 uv = v_texCoord;
            float frequency = texture2D(u_frequencyData, vec2(uv.x, 0.0)).r;

            // Create gradient based on frequency intensity
            vec3 color = mix(vec3(0.0, 0.0, 0.2), vec3(0.0, 1.0, 1.0), frequency);

            // Add some animation
            color += sin(u_time * 2.0 + uv.x * 10.0) * 0.1;

            gl_FragColor = vec4(color, 1.0);
        }
    `;

    static waveformFragmentShader = `
        precision mediump float;
        uniform sampler2D u_waveformData;
        uniform float u_time;
        uniform vec2 u_resolution;
        varying vec2 v_texCoord;

        void main() {
            vec2 uv = v_texCoord;
            float waveform = texture2D(u_waveformData, vec2(uv.x, 0.0)).r;

            // Center the waveform
            float distance = abs(uv.y - 0.5 - waveform * 0.5);

            // Create sharp waveform line
            float intensity = 1.0 - smoothstep(0.0, 0.01, distance);

            vec3 color = vec3(0.0, 1.0, 0.0) * intensity;

            gl_FragColor = vec4(color, intensity);
        }
    `;

    static fractalFragmentShader = `
        precision mediump float;
        uniform float u_time;
        uniform vec2 u_resolution;
        uniform vec2 u_mouse;
        varying vec2 v_texCoord;

        // Complex number operations
        vec2 complexMultiply(vec2 a, vec2 b) {
            return vec2(a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x);
        }

        vec2 complexSquare(vec2 z) {
            return vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y);
        }

        float mandelbrot(vec2 c) {
            vec2 z = vec2(0.0, 0.0);
            int iterations = 0;
            const int maxIterations = 100;

            for (int i = 0; i < maxIterations; i++) {
                if (dot(z, z) > 4.0) break;
                z = complexSquare(z) + c;
                iterations++;
            }

            return float(iterations) / float(maxIterations);
        }

        void main() {
            vec2 uv = (v_texCoord - 0.5) * 4.0;
            uv.x *= u_resolution.x / u_resolution.y;

            // Animate the fractal
            uv += sin(u_time * 0.5) * 0.5;

            float m = mandelbrot(uv);
            vec3 color = vec3(m, m * 0.5, m * 0.8);

            gl_FragColor = vec4(color, 1.0);
        }
    `;

    // Initialize WebGL
    static init(canvasId = 'visualization-canvas') {
        console.log('🎨 Initializing WebGL Visualizations...');

        this.canvas = document.getElementById(canvasId);
        if (!this.canvas) {
            console.warn('Visualization canvas not found');
            return;
        }

        // Get WebGL context
        this.gl = this.canvas.getContext('webgl') || this.canvas.getContext('experimental-webgl');
        if (!this.gl) {
            console.error('WebGL not supported');
            return;
        }

        // Initialize WebGL settings
        this.gl.clearColor(0.0, 0.0, 0.0, 1.0);
        this.gl.enable(this.gl.BLEND);
        this.gl.blendFunc(this.gl.SRC_ALPHA, this.gl.ONE_MINUS_SRC_ALPHA);

        // Create shaders and programs
        this.createProgram('spectrum', this.vertexShaderSource, this.spectrumFragmentShader);
        this.createProgram('waveform', this.vertexShaderSource, this.waveformFragmentShader);
        this.createProgram('fractal', this.vertexShaderSource, this.fractalFragmentShader);

        // Create geometry buffers
        this.createQuadBuffer();

        // Create data textures
        this.createDataTextures();

        this.isInitialized = true;
        console.log('✅ WebGL Visualizations initialized');
    }

    // Create shader program
    static createProgram(name, vertexSource, fragmentSource) {
        const vertexShader = this.createShader(this.gl.VERTEX_SHADER, vertexSource);
        const fragmentShader = this.createShader(this.gl.FRAGMENT_SHADER, fragmentSource);

        if (!vertexShader || !fragmentShader) return;

        const program = this.gl.createProgram();
        this.gl.attachShader(program, vertexShader);
        this.gl.attachShader(program, fragmentShader);
        this.gl.linkProgram(program);

        if (!this.gl.getProgramParameter(program, this.gl.LINK_STATUS)) {
            console.error('Shader program linking failed:', this.gl.getProgramInfoLog(program));
            return;
        }

        this.programs.set(name, program);
    }

    // Create shader
    static createShader(type, source) {
        const shader = this.gl.createShader(type);
        this.gl.shaderSource(shader, source);
        this.gl.compileShader(shader);

        if (!this.gl.getShaderParameter(shader, this.gl.COMPILE_STATUS)) {
            console.error('Shader compilation failed:', this.gl.getShaderInfoLog(shader));
            this.gl.deleteShader(shader);
            return null;
        }

        return shader;
    }

    // Create quad buffer for full-screen rendering
    static createQuadBuffer() {
        const positions = [
            -1.0, -1.0,
             1.0, -1.0,
            -1.0,  1.0,
             1.0,  1.0
        ];

        const texCoords = [
            0.0, 1.0,
            1.0, 1.0,
            0.0, 0.0,
            1.0, 0.0
        ];

        const positionBuffer = this.gl.createBuffer();
        this.gl.bindBuffer(this.gl.ARRAY_BUFFER, positionBuffer);
        this.gl.bufferData(this.gl.ARRAY_BUFFER, new Float32Array(positions), this.gl.STATIC_DRAW);
        this.buffers.set('position', positionBuffer);

        const texCoordBuffer = this.gl.createBuffer();
        this.gl.bindBuffer(this.gl.ARRAY_BUFFER, texCoordBuffer);
        this.gl.bufferData(this.gl.ARRAY_BUFFER, new Float32Array(texCoords), this.gl.STATIC_DRAW);
        this.buffers.set('texCoord', texCoordBuffer);
    }

    // Create data textures for audio data
    static createDataTextures() {
        // Frequency data texture (for spectrum)
        const freqTexture = this.gl.createTexture();
        this.gl.bindTexture(this.gl.TEXTURE_2D, freqTexture);
        this.gl.texParameteri(this.gl.TEXTURE_2D, this.gl.TEXTURE_WRAP_S, this.gl.CLAMP_TO_EDGE);
        this.gl.texParameteri(this.gl.TEXTURE_2D, this.gl.TEXTURE_WRAP_T, this.gl.CLAMP_TO_EDGE);
        this.gl.texParameteri(this.gl.TEXTURE_2D, this.gl.TEXTURE_MIN_FILTER, this.gl.LINEAR);
        this.gl.texParameteri(this.gl.TEXTURE_2D, this.gl.TEXTURE_MAG_FILTER, this.gl.LINEAR);
        this.textures.set('frequency', freqTexture);

        // Waveform data texture
        const waveTexture = this.gl.createTexture();
        this.gl.bindTexture(this.gl.TEXTURE_2D, waveTexture);
        this.gl.texParameteri(this.gl.TEXTURE_2D, this.gl.TEXTURE_WRAP_S, this.gl.CLAMP_TO_EDGE);
        this.gl.texParameteri(this.gl.TEXTURE_2D, this.gl.TEXTURE_WRAP_T, this.gl.CLAMP_TO_EDGE);
        this.gl.texParameteri(this.gl.TEXTURE_2D, this.gl.TEXTURE_MIN_FILTER, this.gl.LINEAR);
        this.gl.texParameteri(this.gl.TEXTURE_2D, this.gl.TEXTURE_MAG_FILTER, this.gl.LINEAR);
        this.textures.set('waveform', waveTexture);
    }

    // Update frequency data texture
    static updateFrequencyData(frequencyData) {
        if (!this.isInitialized || !frequencyData) return;

        const texture = this.textures.get('frequency');
        this.gl.bindTexture(this.gl.TEXTURE_2D, texture);

        // Convert frequency data to RGBA texture data
        const textureData = new Uint8Array(frequencyData.length * 4);
        for (let i = 0; i < frequencyData.length; i++) {
            const value = Math.floor(frequencyData[i] * 255);
            textureData[i * 4] = value;     // R
            textureData[i * 4 + 1] = value; // G
            textureData[i * 4 + 2] = value; // B
            textureData[i * 4 + 3] = 255;   // A
        }

        this.gl.texImage2D(
            this.gl.TEXTURE_2D, 0, this.gl.RGBA, frequencyData.length, 1, 0,
            this.gl.RGBA, this.gl.UNSIGNED_BYTE, textureData
        );
    }

    // Update waveform data texture
    static updateWaveformData(waveformData) {
        if (!this.isInitialized || !waveformData) return;

        const texture = this.textures.get('waveform');
        this.gl.bindTexture(this.gl.TEXTURE_2D, texture);

        // Convert waveform data to RGBA texture data
        const textureData = new Uint8Array(waveformData.length * 4);
        for (let i = 0; i < waveformData.length; i++) {
            const value = Math.floor(((waveformData[i] + 1) / 2) * 255); // Convert from -1..1 to 0..255
            textureData[i * 4] = value;     // R
            textureData[i * 4 + 1] = value; // G
            textureData[i * 4 + 2] = value; // B
            textureData[i * 4 + 3] = 255;   // A
        }

        this.gl.texImage2D(
            this.gl.TEXTURE_2D, 0, this.gl.RGBA, waveformData.length, 1, 0,
            this.gl.RGBA, this.gl.UNSIGNED_BYTE, textureData
        );
    }

    // Render spectrum visualization
    static renderSpectrum(width, height) {
        if (!this.isInitialized) return;

        const program = this.programs.get('spectrum');
        if (!program) return;

        this.gl.useProgram(program);

        // Set uniforms
        const timeLocation = this.gl.getUniformLocation(program, 'u_time');
        const resolutionLocation = this.gl.getUniformLocation(program, 'u_resolution');
        const textureLocation = this.gl.getUniformLocation(program, 'u_frequencyData');

        this.gl.uniform1f(timeLocation, performance.now() / 1000);
        this.gl.uniform2f(resolutionLocation, width, height);
        this.gl.uniform1i(textureLocation, 0);

        // Bind texture
        this.gl.activeTexture(this.gl.TEXTURE0);
        this.gl.bindTexture(this.gl.TEXTURE_2D, this.textures.get('frequency'));

        // Bind buffers and draw
        this.bindQuadBuffers(program);
        this.gl.viewport(0, 0, width, height);
        this.gl.drawArrays(this.gl.TRIANGLE_STRIP, 0, 4);
    }

    // Render waveform visualization
    static renderWaveform(width, height) {
        if (!this.isInitialized) return;

        const program = this.programs.get('waveform');
        if (!program) return;

        this.gl.useProgram(program);

        // Set uniforms
        const timeLocation = this.gl.getUniformLocation(program, 'u_time');
        const resolutionLocation = this.gl.getUniformLocation(program, 'u_resolution');
        const textureLocation = this.gl.getUniformLocation(program, 'u_waveformData');

        this.gl.uniform1f(timeLocation, performance.now() / 1000);
        this.gl.uniform2f(resolutionLocation, width, height);
        this.gl.uniform1i(textureLocation, 0);

        // Bind texture
        this.gl.activeTexture(this.gl.TEXTURE0);
        this.gl.bindTexture(this.gl.TEXTURE_2D, this.textures.get('waveform'));

        // Bind buffers and draw
        this.bindQuadBuffers(program);
        this.gl.viewport(0, 0, width, height);
        this.gl.drawArrays(this.gl.TRIANGLE_STRIP, 0, 4);
    }

    // Render fractal visualization
    static renderFractal(width, height, mouseX = 0, mouseY = 0) {
        if (!this.isInitialized) return;

        const program = this.programs.get('fractal');
        if (!program) return;

        this.gl.useProgram(program);

        // Set uniforms
        const timeLocation = this.gl.getUniformLocation(program, 'u_time');
        const resolutionLocation = this.gl.getUniformLocation(program, 'u_resolution');
        const mouseLocation = this.gl.getUniformLocation(program, 'u_mouse');

        this.gl.uniform1f(timeLocation, performance.now() / 1000);
        this.gl.uniform2f(resolutionLocation, width, height);
        this.gl.uniform2f(mouseLocation, mouseX / width, mouseY / height);

        // Bind buffers and draw
        this.bindQuadBuffers(program);
        this.gl.viewport(0, 0, width, height);
        this.gl.drawArrays(this.gl.TRIANGLE_STRIP, 0, 4);
    }

    // Bind quad buffers for rendering
    static bindQuadBuffers(program) {
        // Position attribute
        const positionLocation = this.gl.getAttribLocation(program, 'a_position');
        this.gl.enableVertexAttribArray(positionLocation);
        this.gl.bindBuffer(this.gl.ARRAY_BUFFER, this.buffers.get('position'));
        this.gl.vertexAttribPointer(positionLocation, 2, this.gl.FLOAT, false, 0, 0);

        // Texture coordinate attribute
        const texCoordLocation = this.gl.getAttribLocation(program, 'a_texCoord');
        this.gl.enableVertexAttribArray(texCoordLocation);
        this.gl.bindBuffer(this.gl.ARRAY_BUFFER, this.buffers.get('texCoord'));
        this.gl.vertexAttribPointer(texCoordLocation, 2, this.gl.FLOAT, false, 0, 0);
    }

    // Clear canvas
    static clear() {
        if (!this.isInitialized) return;
        this.gl.clear(this.gl.COLOR_BUFFER_BIT);
    }

    // Resize canvas
    static resize(width, height) {
        if (!this.canvas) return;

        this.canvas.width = width;
        this.canvas.height = height;
        this.gl.viewport(0, 0, width, height);
    }

    // Get canvas
    static getCanvas() {
        return this.canvas;
    }

    // Check if WebGL is supported
    static isSupported() {
        try {
            const canvas = document.createElement('canvas');
            return !!(canvas.getContext('webgl') || canvas.getContext('experimental-webgl'));
        } catch (e) {
            return false;
        }
    }

    // Dispose resources
    static dispose() {
        if (!this.isInitialized) return;

        // Delete programs
        this.programs.forEach(program => this.gl.deleteProgram(program));
        this.programs.clear();

        // Delete buffers
        this.buffers.forEach(buffer => this.gl.deleteBuffer(buffer));
        this.buffers.clear();

        // Delete textures
        this.textures.forEach(texture => this.gl.deleteTexture(texture));
        this.textures.clear();

        this.isInitialized = false;
        console.log('🗑️ WebGL Visualizations disposed');
    }
}

// Visualization Manager - coordinates different visualizations
class VisualizationManager {
    static visualizations = new Map();
    static currentVisualization = 'spectrum';
    static animationId = null;
    static isRunning = false;

    // Initialize visualization manager
    static init() {
        console.log('📊 Initializing Visualization Manager...');

        if (!WebGLVisualizations.isSupported()) {
            console.warn('WebGL not supported, falling back to Canvas 2D');
            return;
        }

        // Create visualization canvas
        const vizCanvas = document.createElement('canvas');
        vizCanvas.id = 'visualization-canvas';
        vizCanvas.style.width = '100%';
        vizCanvas.style.height = '200px';
        vizCanvas.style.backgroundColor = '#000';

        // Add to DOM (you might want to add this to a specific container)
        const arrangementView = document.getElementById('arrangement-view');
        if (arrangementView) {
            const vizContainer = document.createElement('div');
            vizContainer.id = 'visualization-container';
            vizContainer.style.padding = '16px';
            vizContainer.style.backgroundColor = 'var(--secondary-bg)';
            vizContainer.style.borderTop = '1px solid var(--border-color)';
            vizContainer.appendChild(vizCanvas);
            arrangementView.appendChild(vizContainer);
        }

        // Initialize WebGL
        WebGLVisualizations.init('visualization-canvas');

        // Create visualization controls
        this.createControls();

        console.log('✅ Visualization Manager initialized');
    }

    // Create visualization controls
    static createControls() {
        const container = document.getElementById('visualization-container');
        if (!container) return;

        const controls = document.createElement('div');
        controls.id = 'visualization-controls';
        controls.style.display = 'flex';
        controls.style.gap = '12px';
        controls.style.marginBottom = '12px';
        controls.style.alignItems = 'center';

        controls.innerHTML = `
            <select id="viz-type-select" style="padding: 6px 12px; background: var(--tertiary-bg); border: 1px solid var(--border-color); border-radius: 4px; color: var(--text-primary);">
                <option value="spectrum">Spectrum Analyzer</option>
                <option value="waveform">Waveform</option>
                <option value="fractal">Fractal</option>
                <option value="off">Off</option>
            </select>
            <button id="viz-play-pause" style="padding: 6px 12px; background: var(--accent-color); border: none; border-radius: 4px; color: white; cursor: pointer;">Start</button>
            <div style="font-size: 12px; color: var(--text-secondary);">Real-time audio visualization</div>
        `;

        container.insertBefore(controls, container.firstChild);

        // Add event listeners
        const typeSelect = document.getElementById('viz-type-select');
        const playPauseBtn = document.getElementById('viz-play-pause');

        typeSelect.addEventListener('change', (e) => {
            this.setVisualization(e.target.value);
        });

        playPauseBtn.addEventListener('click', () => {
            this.toggleVisualization();
        });
    }

    // Set current visualization type
    static setVisualization(type) {
        this.currentVisualization = type;

        if (type === 'off') {
            this.stop();
        } else if (this.isRunning) {
            // Restart with new type
            this.stop();
            this.start();
        }

        console.log(`🔄 Visualization set to: ${type}`);
    }

    // Toggle visualization on/off
    static toggleVisualization() {
        if (this.isRunning) {
            this.stop();
        } else {
            this.start();
        }
    }

    // Start visualization
    static start() {
        if (this.isRunning || !WebGLVisualizations.isInitialized) return;

        this.isRunning = true;
        this.animate();

        const playPauseBtn = document.getElementById('viz-play-pause');
        if (playPauseBtn) {
            playPauseBtn.textContent = 'Stop';
            playPauseBtn.style.backgroundColor = 'var(--error-color)';
        }

        console.log('▶ Visualization started');
    }

    // Stop visualization
    static stop() {
        if (!this.isRunning) return;

        this.isRunning = false;
        if (this.animationId) {
            cancelAnimationFrame(this.animationId);
            this.animationId = null;
        }

        // Clear canvas
        WebGLVisualizations.clear();

        const playPauseBtn = document.getElementById('viz-play-pause');
        if (playPauseBtn) {
            playPauseBtn.textContent = 'Start';
            playPauseBtn.style.backgroundColor = 'var(--accent-color)';
        }

        console.log('⏹️ Visualization stopped');
    }

    // Animation loop
    static animate() {
        if (!this.isRunning) return;

        // Update audio data
        this.updateAudioData();

        // Render current visualization
        const canvas = WebGLVisualizations.getCanvas();
        if (canvas) {
            const rect = canvas.getBoundingClientRect();
            WebGLVisualizations.resize(rect.width, rect.height);

            switch (this.currentVisualization) {
                case 'spectrum':
                    WebGLVisualizations.renderSpectrum(rect.width, rect.height);
                    break;
                case 'waveform':
                    WebGLVisualizations.renderWaveform(rect.width, rect.height);
                    break;
                case 'fractal':
                    const mouseX = 0; // You could track mouse position
                    const mouseY = 0;
                    WebGLVisualizations.renderFractal(rect.width, rect.height, mouseX, mouseY);
                    break;
            }
        }

        this.animationId = requestAnimationFrame(() => this.animate());
    }

    // Update audio data from AudioEngine
    static updateAudioData() {
        if (!AudioEngine.isInitialized) return;

        // Get frequency data
        const frequencyData = AudioEngine.getFrequencyData();
        WebGLVisualizations.updateFrequencyData(frequencyData);

        // Get waveform data
        const waveformData = AudioEngine.getTimeDomainData();
        WebGLVisualizations.updateWaveformData(waveformData);
    }

    // Dispose resources
    static dispose() {
        this.stop();
        WebGLVisualizations.dispose();
        console.log('🗑️ Visualization Manager disposed');
    }
}

// Initialize when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    VisualizationManager.init();
});

// Export for global access
window.WebGLVisualizations = WebGLVisualizations;
window.VisualizationManager = VisualizationManager;