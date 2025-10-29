/**
 * Modurust DAW - Node-based Patching View
 * Visual modular synthesis with drag-and-drop patching
 */

class NodeView {
    static canvas = null;
    static ctx = null;
    static nodeCanvas = null;
    static nodeCtx = null;
    static nodes = new Map();
    static connections = [];
    static selectedNode = null;
    static draggedNode = null;
    static draggedConnection = null;
    static isConnecting = false;
    static connectionStart = null;
    static nodePalette = null;
    static isInitialized = false;
    static panX = 0;
    static panY = 0;
    static zoom = 1.0;

    // Node types and their properties
    static nodeTypes = {
        oscillator: {
            name: 'Oscillator',
            category: 'generators',
            inputs: [],
            outputs: ['audio_out'],
            color: '#4444ff',
            parameters: ['frequency', 'waveform', 'amplitude']
        },
        filter: {
            name: 'Filter',
            category: 'effects',
            inputs: ['audio_in'],
            outputs: ['audio_out'],
            color: '#44ff44',
            parameters: ['frequency', 'resonance', 'type']
        },
        envelope: {
            name: 'Envelope',
            category: 'control',
            inputs: ['trigger'],
            outputs: ['envelope_out'],
            color: '#ffff44',
            parameters: ['attack', 'decay', 'sustain', 'release']
        },
        lfo: {
            name: 'LFO',
            category: 'control',
            inputs: [],
            outputs: ['lfo_out'],
            color: '#ff44ff',
            parameters: ['frequency', 'waveform', 'amplitude']
        },
        mixer: {
            name: 'Mixer',
            category: 'effects',
            inputs: ['audio_in_1', 'audio_in_2', 'audio_in_3', 'audio_in_4'],
            outputs: ['audio_out'],
            color: '#ff8844',
            parameters: ['level_1', 'level_2', 'level_3', 'level_4']
        },
        delay: {
            name: 'Delay',
            category: 'effects',
            inputs: ['audio_in'],
            outputs: ['audio_out'],
            color: '#44ffff',
            parameters: ['time', 'feedback', 'mix']
        },
        reverb: {
            name: 'Reverb',
            category: 'effects',
            inputs: ['audio_in'],
            outputs: ['audio_out'],
            color: '#8844ff',
            parameters: ['size', 'damping', 'mix']
        },
        output: {
            name: 'Output',
            category: 'effects',
            inputs: ['audio_in'],
            outputs: [],
            color: '#ff4444',
            parameters: ['volume', 'pan']
        }
    };

    // Initialize node view
    static init() {
        console.log('🔗 Initializing Node View...');

        this.nodeCanvas = document.getElementById('node-canvas');
        this.nodePalette = document.querySelector('.node-palette');

        if (!this.nodeCanvas) {
            console.warn('Node canvas not found');
            return;
        }

        this.nodeCtx = this.nodeCanvas.getContext('2d');
        this.setupCanvas();
        this.setupEventListeners();
        this.createNodePalette();
        this.createDefaultNodes();

        this.isInitialized = true;
        console.log('✅ Node View initialized');
    }

    // Setup canvas
    static setupCanvas() {
        if (!this.nodeCanvas || !this.nodeCtx) return;

        // Set canvas size
        this.resizeCanvas();

        // Set canvas style
        this.nodeCtx.fillStyle = 'var(--primary-bg)';
        this.nodeCtx.fillRect(0, 0, this.nodeCanvas.width, this.nodeCanvas.height);

        // Enable high DPI support
        const dpr = window.devicePixelRatio || 1;
        const rect = this.nodeCanvas.getBoundingClientRect();
        this.nodeCanvas.width = rect.width * dpr;
        this.nodeCanvas.height = rect.height * dpr;
        this.nodeCtx.scale(dpr, dpr);
        this.nodeCanvas.style.width = rect.width + 'px';
        this.nodeCanvas.style.height = rect.height + 'px';
    }

    // Resize canvas
    static resizeCanvas() {
        if (!this.nodeCanvas) return;

        const container = this.nodeCanvas.parentElement;
        if (container) {
            this.nodeCanvas.width = container.clientWidth;
            this.nodeCanvas.height = container.clientHeight;
        }
    }

    // Setup event listeners
    static setupEventListeners() {
        if (!this.nodeCanvas) return;

        // Mouse events
        this.nodeCanvas.addEventListener('mousedown', (e) => this.handleMouseDown(e));
        this.nodeCanvas.addEventListener('mousemove', (e) => this.handleMouseMove(e));
        this.nodeCanvas.addEventListener('mouseup', (e) => this.handleMouseUp(e));
        this.nodeCanvas.addEventListener('wheel', (e) => this.handleWheel(e));

        // Touch events for mobile
        this.nodeCanvas.addEventListener('touchstart', (e) => this.handleTouchStart(e));
        this.nodeCanvas.addEventListener('touchmove', (e) => this.handleTouchMove(e));
        this.nodeCanvas.addEventListener('touchend', (e) => this.handleTouchEnd(e));

        // Toolbar buttons
        const addNodeBtn = document.querySelector('.add-node-btn');
        const clearNodesBtn = document.querySelector('.clear-nodes-btn');
        const savePatchBtn = document.querySelector('.save-patch-btn');
        const categorySelect = document.querySelector('.node-category-select');

        if (addNodeBtn) {
            addNodeBtn.addEventListener('click', () => this.showAddNodeMenu());
        }
        if (clearNodesBtn) {
            clearNodesBtn.addEventListener('click', () => this.clearAllNodes());
        }
        if (savePatchBtn) {
            savePatchBtn.addEventListener('click', () => this.savePatch());
        }
        if (categorySelect) {
            categorySelect.addEventListener('change', (e) => this.filterNodePalette(e.target.value));
        }

        // Window resize
        window.addEventListener('resize', () => this.onResize());
    }

    // Create node palette
    static createNodePalette() {
        if (!this.nodePalette) return;

        this.nodePalette.innerHTML = '<h4>Node Palette</h4>';

        Object.entries(this.nodeTypes).forEach(([type, config]) => {
            const paletteItem = document.createElement('div');
            paletteItem.className = 'palette-item';
            paletteItem.dataset.nodeType = type;
            paletteItem.draggable = true;
            paletteItem.style.backgroundColor = config.color;
            paletteItem.style.color = this.getContrastColor(config.color);
            paletteItem.style.padding = '8px';
            paletteItem.style.margin = '4px 0';
            paletteItem.style.borderRadius = '4px';
            paletteItem.style.cursor = 'grab';
            paletteItem.style.fontSize = '12px';
            paletteItem.style.fontWeight = 'bold';
            paletteItem.style.textAlign = 'center';
            paletteItem.textContent = config.name;

            paletteItem.addEventListener('dragstart', (e) => this.handlePaletteDragStart(e, type));

            this.nodePalette.appendChild(paletteItem);
        });
    }

    // Filter node palette by category
    static filterNodePalette(category) {
        const items = this.nodePalette.querySelectorAll('.palette-item');

        items.forEach(item => {
            const nodeType = item.dataset.nodeType;
            const nodeConfig = this.nodeTypes[nodeType];

            if (category === 'all' || nodeConfig.category === category) {
                item.style.display = 'block';
            } else {
                item.style.display = 'none';
            }
        });
    }

    // Create default nodes
    static createDefaultNodes() {
        // Create a basic patch: Oscillator -> Filter -> Output
        this.addNode('oscillator', 100, 100);
        this.addNode('filter', 300, 100);
        this.addNode('output', 500, 100);

        // Connect them
        setTimeout(() => {
            const oscillator = Array.from(this.nodes.values()).find(n => n.type === 'oscillator');
            const filter = Array.from(this.nodes.values()).find(n => n.type === 'filter');
            const output = Array.from(this.nodes.values()).find(n => n.type === 'output');

            if (oscillator && filter) {
                this.addConnection(oscillator.id, 'audio_out', filter.id, 'audio_in');
            }
            if (filter && output) {
                this.addConnection(filter.id, 'audio_out', output.id, 'audio_in');
            }
        }, 100);
    }

    // Add node to canvas
    static addNode(type, x, y) {
        const config = this.nodeTypes[type];
        if (!config) return null;

        const nodeId = `node_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;

        const node = {
            id: nodeId,
            type,
            x: x / this.zoom - this.panX,
            y: y / this.zoom - this.panY,
            width: 120,
            height: 80,
            config,
            parameters: {},
            selected: false
        };

        // Initialize default parameters
        config.parameters.forEach(param => {
            node.parameters[param] = this.getDefaultParameterValue(param);
        });

        this.nodes.set(nodeId, node);
        this.render();

        console.log(`➕ Added ${config.name} node`);
        return node;
    }

    // Get default parameter value
    static getDefaultParameterValue(param) {
        const defaults = {
            frequency: 440,
            waveform: 'sine',
            amplitude: 0.5,
            resonance: 1,
            type: 'lowpass',
            attack: 0.01,
            decay: 0.1,
            sustain: 0.5,
            release: 0.5,
            time: 0.3,
            feedback: 0.4,
            mix: 0.5,
            size: 0.8,
            damping: 0.2,
            volume: 0.8,
            pan: 0,
            level_1: 0.5,
            level_2: 0.5,
            level_3: 0.5,
            level_4: 0.5
        };
        return defaults[param] || 0.5;
    }

    // Add connection between nodes
    static addConnection(fromNodeId, fromPort, toNodeId, toPort) {
        const fromNode = this.nodes.get(fromNodeId);
        const toNode = this.nodes.get(toNodeId);

        if (!fromNode || !toNode) return;

        // Check if ports exist
        if (!fromNode.config.outputs.includes(fromPort) || !toNode.config.inputs.includes(toPort)) {
            return;
        }

        // Check if connection already exists
        const existingConnection = this.connections.find(conn =>
            conn.fromNodeId === fromNodeId && conn.fromPort === fromPort &&
            conn.toNodeId === toNodeId && conn.toPort === toPort
        );

        if (existingConnection) return;

        const connection = {
            id: `conn_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
            fromNodeId,
            fromPort,
            toNodeId,
            toPort
        };

        this.connections.push(connection);
        this.render();

        console.log(`🔗 Connected ${fromNode.config.name}.${fromPort} → ${toNode.config.name}.${toPort}`);
    }

    // Remove connection
    static removeConnection(connectionId) {
        const index = this.connections.findIndex(conn => conn.id === connectionId);
        if (index !== -1) {
            this.connections.splice(index, 1);
            this.render();
        }
    }

    // Handle palette drag start
    static handlePaletteDragStart(e, nodeType) {
        e.dataTransfer.setData('application/json', JSON.stringify({
            type: 'add-node',
            nodeType
        }));
        e.dataTransfer.effectAllowed = 'copy';
    }

    // Handle mouse down
    static handleMouseDown(e) {
        const rect = this.nodeCanvas.getBoundingClientRect();
        const x = (e.clientX - rect.left) / this.zoom - this.panX;
        const y = (e.clientY - rect.top) / this.zoom - this.panY;

        // Check for node click
        const clickedNode = this.getNodeAtPosition(x, y);
        if (clickedNode) {
            this.selectNode(clickedNode);

            // Check for port click
            const port = this.getPortAtPosition(clickedNode, x, y);
            if (port) {
                this.startConnection(clickedNode, port);
            } else {
                this.startNodeDrag(clickedNode, e);
            }
        } else {
            this.selectedNode = null;
            this.render();
        }
    }

    // Handle mouse move
    static handleMouseMove(e) {
        const rect = this.nodeCanvas.getBoundingClientRect();
        const x = (e.clientX - rect.left) / this.zoom - this.panX;
        const y = (e.clientY - rect.top) / this.zoom - this.panY;

        if (this.draggedNode) {
            // Update node position
            this.draggedNode.x = x - this.draggedNode.dragOffsetX;
            this.draggedNode.y = y - this.draggedNode.dragOffsetY;
            this.render();
        } else if (this.isConnecting) {
            // Update connection preview
            this.render();
            this.drawConnectionPreview(this.connectionStart, { x, y });
        }
    }

    // Handle mouse up
    static handleMouseUp(e) {
        const rect = this.nodeCanvas.getBoundingClientRect();
        const x = (e.clientX - rect.left) / this.zoom - this.panX;
        const y = (e.clientY - rect.top) / this.zoom - this.panY;

        if (this.draggedNode) {
            this.draggedNode = null;
        } else if (this.isConnecting) {
            // Try to complete connection
            const targetNode = this.getNodeAtPosition(x, y);
            if (targetNode && targetNode !== this.connectionStart.node) {
                const targetPort = this.getPortAtPosition(targetNode, x, y);
                if (targetPort && targetPort.type !== this.connectionStart.port.type) {
                    // Valid connection
                    if (this.connectionStart.port.type === 'output') {
                        this.addConnection(
                            this.connectionStart.node.id, this.connectionStart.port.name,
                            targetNode.id, targetPort.name
                        );
                    } else {
                        this.addConnection(
                            targetNode.id, targetPort.name,
                            this.connectionStart.node.id, this.connectionStart.port.name
                        );
                    }
                }
            }
            this.isConnecting = false;
            this.connectionStart = null;
            this.render();
        }
    }

    // Handle wheel zoom
    static handleWheel(e) {
        e.preventDefault();

        const zoomFactor = e.deltaY > 0 ? 0.9 : 1.1;
        this.setZoom(this.zoom * zoomFactor);
    }

    // Handle touch events
    static handleTouchStart(e) {
        e.preventDefault();
        if (e.touches.length === 1) {
            const touch = e.touches[0];
            const mouseEvent = new MouseEvent('mousedown', {
                clientX: touch.clientX,
                clientY: touch.clientY
            });
            this.nodeCanvas.dispatchEvent(mouseEvent);
        }
    }

    static handleTouchMove(e) {
        e.preventDefault();
        if (e.touches.length === 1) {
            const touch = e.touches[0];
            const mouseEvent = new MouseEvent('mousemove', {
                clientX: touch.clientX,
                clientY: touch.clientY
            });
            this.nodeCanvas.dispatchEvent(mouseEvent);
        }
    }

    static handleTouchEnd(e) {
        e.preventDefault();
        const mouseEvent = new MouseEvent('mouseup');
        this.nodeCanvas.dispatchEvent(mouseEvent);
    }

    // Get node at position
    static getNodeAtPosition(x, y) {
        for (const node of this.nodes.values()) {
            if (x >= node.x && x <= node.x + node.width &&
                y >= node.y && y <= node.y + node.height) {
                return node;
            }
        }
        return null;
    }

    // Get port at position
    static getPortAtPosition(node, x, y) {
        const ports = [...node.config.inputs.map(name => ({ name, type: 'input' })),
                       ...node.config.outputs.map(name => ({ name, type: 'output' }))];

        ports.forEach((port, index) => {
            const portX = node.x + (port.type === 'input' ? 0 : node.width);
            const portY = node.y + 20 + index * 15;

            if (x >= portX - 5 && x <= portX + 5 && y >= portY - 5 && y <= portY + 5) {
                port.x = portX;
                port.y = portY;
            }
        });

        return ports.find(port => port.x !== undefined);
    }

    // Select node
    static selectNode(node) {
        // Deselect all nodes
        this.nodes.forEach(n => n.selected = false);

        // Select clicked node
        node.selected = true;
        this.selectedNode = node;
        this.render();
    }

    // Start node drag
    static startNodeDrag(node, e) {
        const rect = this.nodeCanvas.getBoundingClientRect();
        const canvasX = (e.clientX - rect.left) / this.zoom - this.panX;
        const canvasY = (e.clientY - rect.top) / this.zoom - this.panY;

        node.dragOffsetX = canvasX - node.x;
        node.dragOffsetY = canvasY - node.y;
        this.draggedNode = node;
    }

    // Start connection
    static startConnection(node, port) {
        this.isConnecting = true;
        this.connectionStart = { node, port };
    }

    // Set zoom level
    static setZoom(zoom) {
        this.zoom = Math.max(0.1, Math.min(3.0, zoom));
        this.render();
    }

    // Show add node menu
    static showAddNodeMenu() {
        // Toggle node palette visibility
        if (this.nodePalette) {
            this.nodePalette.classList.toggle('visible');
        }
    }

    // Clear all nodes
    static clearAllNodes() {
        if (confirm('Are you sure you want to clear all nodes?')) {
            this.nodes.clear();
            this.connections = [];
            this.selectedNode = null;
            this.render();
            console.log('🗑️ Cleared all nodes');
        }
    }

    // Save patch
    static savePatch() {
        const patch = {
            nodes: Array.from(this.nodes.values()),
            connections: this.connections,
            panX: this.panX,
            panY: this.panY,
            zoom: this.zoom
        };

        const dataStr = JSON.stringify(patch, null, 2);
        const dataBlob = new Blob([dataStr], { type: 'application/json' });

        const link = document.createElement('a');
        link.href = URL.createObjectURL(dataBlob);
        link.download = 'modurust-patch.json';
        link.click();

        console.log('💾 Patch saved');
    }

    // Get contrast color
    static getContrastColor(bgColor) {
        const r = parseInt(bgColor.slice(1, 3), 16);
        const g = parseInt(bgColor.slice(3, 5), 16);
        const b = parseInt(bgColor.slice(5, 7), 16);
        const brightness = (r * 299 + g * 587 + b * 114) / 1000;
        return brightness > 128 ? '#000000' : '#ffffff';
    }

    // Handle window resize
    static onResize() {
        this.resizeCanvas();
        this.render();
    }

    // Activate node view
    static onActivate() {
        console.log('🔗 Node view activated');
        this.render();
    }

    // Render canvas
    static render() {
        if (!this.nodeCtx || !this.isInitialized) return;

        // Clear canvas
        this.nodeCtx.save();
        this.nodeCtx.setTransform(1, 0, 0, 1, 0, 0);
        this.nodeCtx.clearRect(0, 0, this.nodeCanvas.width, this.nodeCanvas.height);
        this.nodeCtx.restore();

        // Apply zoom and pan
        this.nodeCtx.save();
        this.nodeCtx.scale(this.zoom, this.zoom);
        this.nodeCtx.translate(this.panX, this.panY);

        // Draw connections first (behind nodes)
        this.drawConnections();

        // Draw nodes
        this.drawNodes();

        this.nodeCtx.restore();
    }

    // Draw connections
    static drawConnections() {
        this.nodeCtx.strokeStyle = 'var(--accent-color)';
        this.nodeCtx.lineWidth = 2 / this.zoom;

        this.connections.forEach(connection => {
            const fromNode = this.nodes.get(connection.fromNodeId);
            const toNode = this.nodes.get(connection.toNodeId);

            if (!fromNode || !toNode) return;

            const fromPortIndex = fromNode.config.outputs.indexOf(connection.fromPort);
            const toPortIndex = toNode.config.inputs.indexOf(connection.toPort);

            const fromX = fromNode.x + fromNode.width;
            const fromY = fromNode.y + 20 + fromPortIndex * 15;
            const toX = toNode.x;
            const toY = toNode.y + 20 + toPortIndex * 15;

            // Draw curved connection
            this.nodeCtx.beginPath();
            this.nodeCtx.moveTo(fromX, fromY);

            const dx = toX - fromX;
            const dy = toY - fromY;
            const distance = Math.sqrt(dx * dx + dy * dy);
            const cp1x = fromX + distance * 0.4;
            const cp1y = fromY;
            const cp2x = toX - distance * 0.4;
            const cp2y = toY;

            this.nodeCtx.bezierCurveTo(cp1x, cp1y, cp2x, cp2y, toX, toY);
            this.nodeCtx.stroke();
        });
    }

    // Draw connection preview
    static drawConnectionPreview(start, end) {
        if (!start) return;

        const fromX = start.port.x;
        const fromY = start.port.y;

        this.nodeCtx.strokeStyle = 'var(--text-secondary)';
        this.nodeCtx.lineWidth = 2 / this.zoom;
        this.nodeCtx.setLineDash([5, 5]);

        this.nodeCtx.beginPath();
        this.nodeCtx.moveTo(fromX, fromY);
        this.nodeCtx.lineTo(end.x, end.y);
        this.nodeCtx.stroke();

        this.nodeCtx.setLineDash([]);
    }

    // Draw nodes
    static drawNodes() {
        this.nodes.forEach(node => {
            this.drawNode(node);
        });
    }

    // Draw individual node
    static drawNode(node) {
        const x = node.x;
        const y = node.y;
        const width = node.width;
        const height = node.height;

        // Node background
        this.nodeCtx.fillStyle = node.selected ? 'var(--accent-hover)' : node.config.color;
        this.nodeCtx.fillRect(x, y, width, height);

        // Node border
        this.nodeCtx.strokeStyle = node.selected ? 'var(--accent-color)' : 'var(--border-color)';
        this.nodeCtx.lineWidth = node.selected ? 3 : 1;
        this.nodeCtx.strokeRect(x, y, width, height);

        // Node title
        this.nodeCtx.fillStyle = this.getContrastColor(node.config.color);
        this.nodeCtx.font = `${12 / this.zoom}px Arial`;
        this.nodeCtx.fillText(node.config.name, x + 8, y + 16);

        // Input ports
        node.config.inputs.forEach((input, index) => {
            const portX = x;
            const portY = y + 20 + index * 15;

            this.nodeCtx.fillStyle = '#ff4444';
            this.nodeCtx.beginPath();
            this.nodeCtx.arc(portX, portY, 4 / this.zoom, 0, 2 * Math.PI);
            this.nodeCtx.fill();

            this.nodeCtx.fillStyle = this.getContrastColor(node.config.color);
            this.nodeCtx.font = `${10 / this.zoom}px Arial`;
            this.nodeCtx.fillText(input, x + 12, portY + 3);
        });

        // Output ports
        node.config.outputs.forEach((output, index) => {
            const portX = x + width;
            const portY = y + 20 + index * 15;

            this.nodeCtx.fillStyle = '#44ff44';
            this.nodeCtx.beginPath();
            this.nodeCtx.arc(portX, portY, 4 / this.zoom, 0, 2 * Math.PI);
            this.nodeCtx.fill();

            this.nodeCtx.fillStyle = this.getContrastColor(node.config.color);
            this.nodeCtx.font = `${10 / this.zoom}px Arial`;
            this.nodeCtx.fillText(output, x + width - 50, portY + 3);
        });
    }
}

// Initialize when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    NodeView.init();
});

// Export for global access
window.NodeView = NodeView;