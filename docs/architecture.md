# Modurust DAW Architecture

## Overview

Modurust is built on a modular architecture that separates concerns into distinct layers, enabling high performance, maintainability, and extensibility.

## Core Architecture

### Three-View System
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  Arrangement    │    │      Live       │    │      Node       │
│    View         │    │     View        │    │     View        │
│                 │    │                 │    │                 │
│ • Timeline      │    │ • Clip Matrix   │    │ • Visual Patch  │
│ • Automation    │    │ • Crossfader    │    │ • Node Graph    │
│ • Mixing        │    │ • Scenes        │    │ • Connections   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Module Hierarchy

```
Modurust DAW
├── Core Systems
│   ├── daw_core.rs          - Main DAW engine
│   ├── transport_sync.rs    - Timing & sync
│   ├── node_graph.rs        - Visual patching
│   ├── audio_backend.rs     - Audio processing
│   ├── midi2_mpe.rs         - MIDI handling
│   ├── audio_nodes.rs       - Audio nodes
│   ├── daw_nodes.rs         - DAW nodes
│   └── player_backend.rs    - File playback
├── User Interface
│   ├── ui.rs                - Three-view UI
│   ├── web_interface.rs     - Browser interface
│   └── hid_osc_bindings.rs  - Controllers
├── AI Tools
│   ├── sai_audio.rs         - Sonic AI
│   ├── stream_diffusion_audio.rs - Audio diffusion
│   ├── ai_audio.rs          - AI effects
│   ├── mcp_server.rs       - MCP server
│   └── ai_stem_separation.rs - Stem separation
└── Infrastructure
    ├── modular_architecture.rs - Plugin system
    ├── error_handling.rs    - Error recovery
    ├── performance_profiling.rs - Monitoring
    └── logging_monitoring.rs - Logging
```

## Data Flow

### Audio Processing Pipeline
```
Input Device → Audio Backend → Node Graph → Output Device
                    ↓
              MIDI 2.0/MPE → Controllers → UI Updates
                    ↓
              AI Tools → Processing → Effects
```

### Real-time Threading Model
```
Main Thread (UI)
├── UI Updates
├── User Input
└── Project Management

Audio Thread (Real-time)
├── Audio Processing (<1ms latency)
├── MIDI Processing (<0.1ms latency)
└── Node Graph Execution

Worker Threads (Background)
├── AI Processing
├── File I/O
├── Network Communication
└── Plugin Loading
```

## Key Design Patterns

### 1. Entity-Component-System (ECS)
- **Components**: Audio nodes, UI elements, project data
- **Systems**: Audio processing, UI rendering, file management
- **Entities**: Projects, tracks, clips, nodes

### 2. Observer Pattern
- **Subjects**: Transport state, audio levels, MIDI events
- **Observers**: UI updates, automation recording, effects

### 3. Strategy Pattern
- **Strategies**: Different audio backends (WASAPI, CoreAudio, ALSA)
- **Context**: Audio system abstraction
- **Clients**: Node graph, effects processing

### 4. Command Pattern
- **Commands**: Undo/redo operations, user actions
- **Invoker**: UI event handlers
- **Receiver**: DAW core systems

## Performance Optimizations

### Memory Management
- **Arena Allocation**: Node graph uses arena allocation for fast node creation/destruction
- **Object Pooling**: Audio buffers and MIDI messages use object pools
- **Zero-Copy**: Audio data flows through the system without copying where possible

### Concurrency
- **Lock-Free Queues**: Audio/MIDI communication uses lock-free data structures
- **Message Passing**: Inter-thread communication via channels
- **Atomic Operations**: State synchronization without locks

### SIMD Acceleration
- **DSP Operations**: Audio processing uses SIMD instructions
- **Vectorization**: FFT, filtering, and mixing operations are vectorized
- **Platform Optimization**: Code adapts to available SIMD instruction sets

## Plugin Architecture

### Plugin Types
- **Audio Nodes**: Signal processing units
- **MIDI Effects**: MIDI processing plugins
- **AI Tools**: Machine learning integrations
- **UI Extensions**: Custom interface components

### Plugin Interface
```rust
pub trait Plugin: Send + Sync {
    fn initialize(&mut self, context: &PluginContext) -> Result<(), PluginError>;
    fn process(&mut self, input: &PluginInput) -> Result<PluginOutput, PluginError>;
    fn shutdown(&mut self) -> Result<(), PluginError>;
}
```

### Hot Reloading
- **Dynamic Loading**: Plugins can be loaded/unloaded at runtime
- **Version Compatibility**: Automatic version checking and compatibility
- **State Preservation**: Plugin state maintained across reloads

## Error Handling

### Error Types
- **Recoverable Errors**: Audio glitches, MIDI timeouts
- **Fatal Errors**: System crashes, memory corruption
- **User Errors**: Invalid configurations, missing files

### Recovery Strategies
- **Graceful Degradation**: System continues with reduced functionality
- **Automatic Restart**: Failed components restart automatically
- **User Notification**: Clear error messages and recovery options

## Testing Strategy

### Unit Tests
- **Module Tests**: Individual component testing
- **Integration Tests**: Component interaction testing
- **Performance Tests**: Latency and throughput benchmarks

### Fuzz Testing
- **Audio Data**: Random audio data testing
- **MIDI Data**: Random MIDI message testing
- **UI Events**: Random user interaction testing

### Continuous Integration
- **Automated Builds**: Every commit tested
- **Cross-Platform**: Windows, macOS, Linux testing
- **Performance Regression**: Automatic performance monitoring

## Security Considerations

### Audio Security
- **Buffer Overflow Protection**: All audio buffers bounds-checked
- **Memory Safety**: Rust's ownership system prevents memory corruption
- **Input Validation**: All audio/MIDI input validated

### Plugin Security
- **Sandboxing**: Plugins run in isolated environments
- **Permission System**: Fine-grained access controls
- **Code Signing**: Plugin authenticity verification

## Future Extensibility

### Planned Extensions
- **VST3 Plugin Hosting**: Native VST3 support
- **LV2 Plugin Support**: Linux audio plugin standard
- **WebAssembly Plugins**: Browser-based plugin support
- **Distributed Processing**: Multi-machine audio processing

### API Stability
- **Semantic Versioning**: Clear versioning scheme
- **Deprecation Warnings**: Gradual API changes
- **Migration Guides**: Easy upgrade paths

This architecture provides a solid foundation for a professional-grade digital audio workstation while maintaining the flexibility needed for future enhancements and user requirements.

## Real-Time State Management (Rust Advantage)

- UI → Audio bridge uses lock-free channels to pass control messages (`Play`, `Stop`, `SetTempo`, volume, node graph ops).
- Real-time feedback bus returns transport state, peak meters, spectrum, and current parameter snapshots to the UI.
- Parameter changes are represented as small, copy-free messages; the audio thread polls and applies them in its callback.

Implementation details:
- Inter-thread messaging: Crossbeam channels in `src/audio_engine/bridge.rs` provide non-blocking UI → audio communication.
- Feedback cadence: The bridge throttles UI feedback (default 30 Hz) to keep UI smooth without burdening the audio thread.
- Parameter snapshotting: `AudioEngineState.current_params` collects live parameter values for UI synchronization.

Limitations to address:
- Sample-accurate automation: `src/event_queue.rs` defines a lock-free, time-stamped `EventQueue`, but it is not wired into the audio callback yet.
- Atomic parameters: `NodeInstanceManager` stores parameters via `Arc<Mutex<f32>>`; replace with `AtomicF32` or split-atomic design for RT safety.

## Bridge Protocol and Node Mapping

- Protocol: `AudioParamMessage` includes transport, mixer, EQ/effects, generic parameter setting, and node graph control (`AddNode`, `RemoveNode`, `ConnectNodes`).
- Manager: `src/node_instance_manager.rs` provides a UI-side `NodeInstanceManager` to create/delete/connect nodes and push parameter changes through the bridge.

Current state:
- Engine handling: `HexoDSPEngine::handle_param_message` processes transport and simple volume messages, but does not yet handle `AddNode`, `RemoveNode`, `ConnectNodes`, or `SetParameter` for graph nodes.
- UI mapping gap: Hexagonal Node View (`src/ui/hexagonal_node_view.rs`) is visual-only and not yet synchronized with `NodeGraph` in the audio engine.

Planned refinement:
- NodeInstanceManager integration: Connect Node View actions to real audio node creation on the RT thread.
- Protocol coverage: Implement handling of node operations in `HexoDSPEngine`, and route parameter updates down to DSP modules.
- Bi-directional sync: Ensure engine → UI parameter snapshots and meters reflect actual nodes shown in the Node View per selected track.

## Deep Time Manipulation and Polyspectral Processing (Status)

- Phase vocoder / time-warp: Documented in UI files as roadmap, not implemented in DSP modules yet.
- Time anchors: Not implemented in transport/arrangement editing.
- Polyspectral routing: FFT-based band routing mentioned in docs; no `rustfft`-backed modules in the engine today.

Next steps:
- Add FFT analysis modules, phase vocoder node, and band-split routers to `dsp_core`.
- Extend transport and clip model for “time anchors” and real-time warp.

## Hexagonal Patching View (UI/UX)

- The Node View renders a hardware-accelerated hex grid with draggable nodes and ports.
- Color-coded nodes and patch cables are present; embedded UI widgets per node (EQ spectrum, synth controls) are planned.

Status:
- Visual canvas present (`src/ui/hexagonal_node_view.rs`, integrated in `src/ui/bevy_egui_ui.rs`).
- Functional audio mapping pending; visual patches do not yet generate sound.

## “Modu-Commit” Workflow (Status)

- UI buttons for Commit/Branch exist in `bevy_egui_ui.rs` under “Patch History”.
- Track state snapshots and branching are not implemented in engine/core.

Planned:
- Snapshot: Serialize all node parameters, arrangements, and automation per track.
- Branching: Maintain multiple named states (A/B/C) with instant switching.

## Implementation Status Summary

- Atomic Parameter Storage: Partial. Bridge and state are lightweight; parameters use `Mutex<f32>` in manager. Replace with atomics for RT.
- Time-Stamped Event Queue: Present (`src/event_queue.rs`), not integrated in audio callback; sample-accurate automation pending.
- Module Hot-Swapping: Node add/remove/connect exist; safe in-place swap with `Arc` and state preservation not implemented.
- Deep Time Manipulation: Not implemented (phase vocoder, anchors, clip warp).
- Polyspectral Processing: Not implemented (no FFT nodes/band routers yet).
- Hexagonal Patching View: Implemented visually; audio mapping pending.
- Modu-Commit Workflow: UI placeholders only; snapshotting/branching engine support pending.

## Actionable Next Steps (Phase 5B)

- Implement `NodeInstanceManager` hookup in UI, and engine-side handling of node graph messages.
- Wire `EventQueue` into the audio thread, applying param changes at buffer sample offsets.
- Replace `Arc<Mutex<f32>>` with atomic primitives for RT-safe parameter access.
- Add FFT modules and a phase vocoder to `dsp_core`, with routing nodes for band processing.
- Build snapshot/branch APIs and a History Panel that controls track-level states.