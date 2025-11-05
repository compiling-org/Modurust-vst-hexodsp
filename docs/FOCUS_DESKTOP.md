# Desktop-Only Focus: One-Month Plan (Expanded)

This document defines a comprehensive, desktop-only plan for HexoDSP over the next month. It consolidates and expands the demo plan into a full-featured roadmap for transport, recording/export, mixer, node graph, Live View, Gesture Control, MIDI 2.0/MPE, microtuning, and desktop shader/visual pipelines. All content is strictly desktop; web and WebGL are out of scope.

## Executive Summary
- Objective: Deliver a “full enough” desktop DAW experience in one month, with reliable transport, recording/export, mixer, node graph editing, Live View performance controls, gesture mapping, MIDI 2.0/MPE pathways, microtuning scaffolds, and desktop-only visual/shader hooks.
- Scope: Desktop-only using Bevy + `bevy_egui` for UI and CPAL for audio IO. No browser, web server, or WebGL paths.
- Outcomes: A runnable desktop app that records to WAV, exports mixes, shows responsive meters and UI, supports basic Live View performance, accepts gesture mappings, and demonstrates future-ready MIDI2/MPE and microtuning scaffolds.

## Guiding Principles
- Desktop-first: keep runtime simple (`cargo run`), no web features.
- Sample-accurate transport and stable audio IO take precedence.
- Visible placeholders are acceptable where deep features aren’t ready, but must be discoverable and reliable.
- Performance budgets and acceptance criteria drive integration readiness.

## Non-Goals (Month)
- Web UI, browser preview, WebGL visualizations, or web deployment.
- Full multitrack audio recording with advanced comping.
- Complete third-party plugin hosting beyond minimal VST3 interop stubs.
- Deep automation editing across all parameters.

## Architecture Overview (Desktop)
- `AudioIO` (CPAL): output stream, input monitoring, playback gate, planned recording/export.
- `Transport`: `Playing`, `Stopped`, `Paused`, `Recording` with clock-based BPM and looping.
- `AudioEngineBridge`: carries `AudioParamMessage` (Play, Stop, Record, SetTempo, MasterVolume, etc.).
- `HexoDSPEngine`: orchestrates transport, audio IO, node graph, and processing state feedback.
- `UI` (Bevy + `bevy_egui`): renders Arrangement, Live, and Node views, transport bar, mixer, and parameter panels.
- Desktop visuals/shaders: integrate visual nodes via desktop GPU pipelines; no WebGL.

## Core Feature Set (Desktop-Only)
1. Transport controls: Play/Stop/Pause, BPM, loop.
2. Recording to WAV (stereo mix): start/stop; timestamped files.
3. Export current mix to WAV: file menu; duration selection stub.
4. Master mixer: volume/pan smoothing; mute/mono/phase toggles.
5. Input monitoring: gating, gain control, feedback-safe routing.
6. Node graph: add/connect/edit generators/effects; preset loading.
7. Arrangement view: basic timeline and playhead controls.
8. Live View: scene buttons, deck A/B, crossfader, basic EQ knobs.
9. Gesture Control: map gestures to transport and crossfader.
10. MIDI 2.0 & MPE scaffolds: per-note expression pathways.
11. Microtuning: `.scl`/`.kbm` stub loaders; apply tuning tables.
12. Desktop shader/visual nodes: UI placeholders; parameter wiring.
13. UI persist/load: `ui_state.json` for layouts and themes.
14. Processing state feedback: meters and peaks for master/tracks.

## One-Month Timeline (Weeks 1–4)

### Week 1: Transport & Audio IO Baseline
- Deliverables:
  - Wire `Play/Stop/Pause/SetTempo` to `Transport` and `AudioIO::set_playback_enabled`.
  - Confirm `start_output_only()` stability; set defaults for sample rate/buffer.
  - Persist UI toggles and view mode (`ui_state.json`).
- Acceptance:
  - Play/Stop stable with no glitches; BPM reflects in transport state.
  - Node graph basic path (`sine → lpf → delay → output`) audible.
  - UI persistence restores view/layout/theme.
- Technical notes:
  - Implement ramped volume/pan (small per-buffer steps) to avoid zipper noise.
  - Ensure atomic parameters for audio thread safety.

### Week 2: Recording & Export + Mixer/EQ/Dynamics
- Deliverables:
  - `AudioIO::start_recording(path)`, `stop_recording()`, `is_recording()` using `hound`.
  - Buffered write from output callback via ring buffer; writer thread flushes.
  - Export mix via File menu to `./exports/` using the same pipeline.
  - Master toggles: mute, mono, phase; basic EQ/dynamics stubs.
- Acceptance:
  - `⏺` toggles record; WAV plays externally at correct rate/bit depth.
  - Export produces valid WAV; no runtime errors.
  - Master toggles produce expected audible changes; no clipping/surges.
- Technical notes:
  - Timestamped filenames: `recordings/hexodsp_<YYYYMMDD_HHMMSS>_<bpm>.wav`.
  - Prefer 24-bit PCM with fallback to 16-bit.
  - Pan law and mono sum implemented safely.

### Week 3: Live View, Gesture Control, Node Graph Advanced
- Deliverables:
  - Live View: crossfader coefficient, deck A/B volume, basic EQ knobs.
  - Scene buttons toggle visual state; clip trigger stubs.
  - Gesture Control: map mouse gestures to transport and crossfader; HID/OSC stubs.
  - Node Graph: expanded presets, improved connection feedback, selection tools.
- Acceptance:
  - Crossfader visibly affects deck levels; gestures recognized ≥90% reliably.
  - Node parameter edits reflect in sound; presets load consistently.
  - No dropouts during rapid gestures and view changes.
- Technical notes:
  - Gesture fallback layer with thresholds and debounce.
  - `NodeInstanceManager` ensures parameter routing correctness.

### Week 4: MIDI 2.0/MPE, Microtuning, Desktop Visuals + QA/Perf
- Deliverables:
  - MIDI 2.0/MPE scaffolds: per-note expression routing; timing alignment with transport.
  - Microtuning: stub loaders for `.scl`/`.kbm`; apply tuning tables to generators.
  - Desktop visual nodes: shader parameter UI; audio-reactive hooks.
  - Full QA pass: performance, stability, documentation updates, screenshots.
- Acceptance:
  - MPE pathways do not break timing; per-note expression demonstrable.
  - Tuning tables apply consistently; no transport drift.
  - Visual nodes render placeholders; parameters persist; stable runtime.
  - All demo scenarios pass; recordings/exports verified.
- Technical notes:
  - Tune application per voice; ensure compatibility with MPE pathways.
  - Visual pipeline strictly desktop GPU; avoid any WebGL references.

## Feature Details and Acceptance Criteria (Expanded)

### Transport (Play/Stop/Pause, BPM, Loop)
- Status: Play/Stop implemented; Pause/Loop planned.
- Usage: Transport bar controls; BPM drag with feedback.
- Acceptance: Gate works in CPAL callback; BPM reflects in UI and transport state.
- Technical: `AudioParamMessage::Play/Stop/SetTempo`; transport loop flags and beat counters.

### Recording to WAV (Stereo Mix)
- Status: Planned; implemented with `hound`.
- Usage: `⏺` button toggles recording; files under `./recordings/`.
- Acceptance: Correct sample rate and bit depth; immediate start while playing.
- Technical: Atomic `is_recording`; ring buffer → writer thread; flush on stop.

### Export Mix to WAV
- Status: Planned; File menu action.
- Usage: Export to `./exports/session-<timestamp>.wav`.
- Acceptance: Duration and format correct; runs without errors.
- Technical: Reuse recording pipeline; fixed-duration stub supported.

### Master Mixer (Volume, Pan, Mute/Mono/Phase)
- Status: Smoothing implemented; toggles planned.
- Usage: Sliders/toggles in master section.
- Acceptance: Smooth changes; mono sums; phase invert works.
- Technical: Ramp per buffer; pan law; safe mono sums.

### Input Monitoring (Gain)
- Status: Gating implemented; gain planned.
- Usage: Enable monitoring; adjust gain.
- Acceptance: No feedback loops; respects master mute/level.
- Technical: Mix input capture with output path safely.

### Arrangement View
- Status: Timeline and zoom present.
- Usage: Navigate and position playhead; scrubbing.
- Acceptance: Position updates and reflects transport.
- Technical: Persist view state.

### Live View (Scene/Deck/Crossfader)
- Status: UI present; demo controls active per plan.
- Usage: Trigger scene, move crossfader, adjust deck controls.
- Acceptance: Crossfader affects deck levels; scene toggles state.
- Technical: Map to engine parameters via bridge.

### Gesture Control
- Status: Planned via `gesture_integration.rs`, `hid_osc_bindings.rs`.
- Usage: Gestures mapped to transport and crossfader.
- Acceptance: ≥90% recognition; minimal false positives.
- Technical: Thresholds and debouncing; visual hints.

### Node Graph (Add/Connect/Edit, Presets)
- Status: Canvas and nodes implemented; presets present.
- Usage: Add generators/effects; connect; tweak parameters.
- Acceptance: Audible changes via master bus; connections visually correct.
- Technical: `NodeInstanceManager` routes param messages.

### Preset Loading
- Status: Demo library available.
- Usage: Load and apply presets to nodes.
- Acceptance: Parameters apply and persist.
- Technical: Bind preset manager to node types.

### MIDI 2.0 & MPE (Scaffolds)
- Status: Foundation present in `midi2_mpe.rs`.
- Usage: Device bindings and per-note expression pathways.
- Acceptance: Timing aligned; no transport jitter.
- Technical: Integrate transport time with MIDI processing.

### Microtuning
- Status: UI hooks and import dialog placeholders.
- Usage: Load `.scl`/`.kbm`; apply tuning tables.
- Acceptance: Pitch tables apply consistently; stable timing.
- Technical: Per-voice tuning; MPE compatible.

### Desktop Visuals/Shader Nodes
- Status: UI placeholders; parameter wiring.
- Usage: Add visual node; adjust parameters; observe feedback.
- Acceptance: Stable runtime; parameters persist.
- Technical: Bind shader params to `AudioParamMessage` and feedback; desktop GPU-only.

### UI Persist/Load
- Status: Implemented `ui_state.json`.
- Usage: Restart restores layout/theme/view.
- Acceptance: State persists without errors.
- Technical: Serde save/load on init/shutdown.

### Processing State Feedback (Meters)
- Status: Feedback wired for peaks and spectrum.
- Usage: Observe master/track/return activity.
- Acceptance: Levels reflect audio; no desync.
- Technical: Engine updates processing state for UI.

## Demo Scenarios (Reviewer Walkthrough)
1. Launch app; window title shows HexoDSP DAW.
2. Transport: Play/Stop; adjust master volume/pan.
3. Input monitoring: enable, observe levels.
4. Recording: start/stop; verify file.
5. Live View: crossfader and deck controls; scene toggle.
6. Node View: add nodes, tweak parameters; load preset.
7. Microtuning: load tuning table; audition pitches.
8. Export mix; verify file in `./exports/`.
9. Restart app; ensure UI state restored.

## Performance & Stability Baseline
- Buffer sizes: 128–512 frames per callback.
- CPU budget: core transport/mixing under ~10% on mid-tier hardware.
- Use lock-free structures and preallocation; avoid blocking in callbacks.
- Minimize logging in audio thread; batch to UI/bridge.

## Testing Strategy
- Unit tests for transport, mixing, and parameter smoothing.
- Integration tests for node graph parameter routing.
- Manual acceptance tests per scenario; checklist maintained.
- Performance tests: buffer underflow/overflow resilience.

## Risks & Mitigations
- Recording blocking risk → ring buffer + writer thread.
- Gesture false positives → thresholds, debounce, confirmations.
- Sample format mismatches → conversion layer.
- Feature creep → hard scope and clear acceptance criteria.

## Packaging & Delivery (Desktop)
- Windows: `cargo run --release`; minimal packaging pass in Week 4.
- No installers or cross-platform targets in month scope beyond sanity checks.

## Paths & Commands (Desktop)
-
### Desktop Scripts (preferred)
- Build: `./scripts/desktop_build.ps1` or `./scripts/desktop_build.ps1 -Release`
- Run: `./scripts/desktop_run.ps1` or `./scripts/desktop_run.ps1 -Release`
- These scripts enforce `--no-default-features` so no web code is compiled or run.

- Run: `cargo run --release`
- Recordings: `./recordings/`
- Exports: `./exports/`
- Config: `ui_state.json`
- Key files:
  - `src/audio_engine/cpal_io.rs`
  - `src/audio_engine/mod.rs`
  - `src/audio_engine/transport.rs`
  - `src/ui/egui_ui_full.rs`
  - `src/ui/bevy_egui_ui.rs`

## Decision Record
- Desktop-only for the entire month; web features are out of scope.
- Any PR touching web paths must remain disabled by default and must not block desktop delivery.
- UI-to-engine integration uses `AudioParamMessage` for node graph control and parameter edits.

---

This one-month desktop plan expands the demo scope into a practical, reliable roadmap for delivering a “full enough” DAW experience without web dependencies. It maintains strict acceptance criteria, performance budgets, and a clear architecture to keep development focused and testable.

## Architecture Deep Dive
- Engine threading model: audio callback thread (real-time), UI thread (non-real-time), and bridge thread for parameter messages.
- Message bus: `AudioParamMessage` and `ProcessingState` updates; lock-free ring buffers for audio → writer and state → UI.
- Scheduling: transport clock (beats/ticks) drives tempo-based events; nodes pull audio in block-sized chunks.
- Latency accounting: end-to-end path budget, buffer size selection, pan/volume smoothing ramp per buffer.
- Memory: preallocate buffers per node; avoid dynamic allocation in callback.
- Error handling: fail-fast on device init; recoverable state for UI when audio device resets.

## Audio Engine Specifications
- Sample rate handling: detect device rate; resample generators if needed; prefer 48kHz, allow 44.1/96k.
- Block size management: adaptive 128–512 frames; guardrails for underflow/overflow.
- DSP node API: process(buffer, params) with timebase; parameter smoothing contract; bypass and mute behavior.
- Master path: summing, gain, pan law, mono sum, phase invert; metering taps with peak and RMS.
- Recording/export pipeline: ring buffer → writer thread; format negotiation (24-bit PCM, fallback 16-bit).

## UI Design System
- Views: Arrangement, Live, Node; unified transport bar; mixer panel; parameter inspector; meters.
- Component inventory: buttons, sliders, toggles, vector meters, node canvas, preset browser.
- Input devices: keyboard, mouse, basic HID/OSC stubs; gesture layer with thresholds/debounce.
- Accessibility: focus order, high-contrast theme options, keyboard shortcuts for transport and views.
- Persistence: `ui_state.json` schemas for layout, theme, view mode, last project.

## Project Persistence & Format
- Project file: `hexodsp_project.json` (future): tracks, nodes, connections, parameters, tempo, tuning tables.
- Versioning: schema version tag; migration stubs for future backward compatibility.
- Asset management: presets in `src/presets/`; exports in `./exports/`; recordings in `./recordings/`.

## Plugin Host Strategy (Desktop)
- VST3 interop stubs: scanning, basic parameter mapping, and sandbox launch (future).
- Safety: plugin isolation, watchdog for crashes, clear bypass behavior.
- Roadmap: introduce AU/CLAP later; keep host optional and disabled by default in month scope.

## MIDI 2.0 & MPE Implementation Notes
- Message formats: per-note pitch bend, pressure, timbre; device mapping.
- Scheduling: align MIDI timestamps with transport beats; jitter buffering and correction.
- Pathways: per-voice routing into generators; mod matrix hooks for expression.

## Automation & Modulation
- Automation lanes: transport tempo, master volume/pan, crossfader, select node params.
- Curves: linear/exponential; sample-accurate stepping; smoothing.
- Modulation routing: LFO/envelope stubs; simple per-parameter depth.

## Testing Strategy (Expanded)
- Unit tests: transport state transitions; pan law; phase invert.
- Integration tests: node graph parameter routing; recording pipeline writer flush.
- Audio golden tests: load fixtures; compare peaks/RMS within tolerance.
- UI smoke tests: view toggles, transport controls, node operations.
- Performance tests: underflow/overflow simulation; gesture stress with transport running.

## Performance Targets (Expanded)
- CPU: transport + summing + meters under ~10% on mid-tier hardware.
- Memory: avoid allocations in callback; preallocate per node; static buffers.
- I/O: writer thread throughput sustains 24-bit PCM without underruns.
- Frame pacing: UI remains responsive during heavy audio load.

## Security & Privacy
- Data storage: local-only config and project files.
- Plugin isolation: sandbox processes when enabled; explicit opt-in.
- Telemetry: none by default; add diagnostic logs only on demand.

## Release Engineering
- Packaging: Windows portable zip with README and `cargo run --release` instructions.
- Artifacts: screenshots for documentation; example projects.
- Versioning: semantic version for monthly milestones.

## Week-by-Week Detailed Plan (Daily Tasks)

### Week 1 (Transport & Audio IO)
- Day 1: Audit audio device init paths; add robust error messages.
- Day 2: Implement `Pause` in transport and UI wiring.
- Day 3: BPM setter smoothing and UI feedback confirmation.
- Day 4: Loop region stub UI and transport flags.
- Day 5: Node graph baseline preset loading; verify audible chain.
- Day 6: Master meter taps; peak/RMS calculations.
- Day 7: QA pass; performance capture; docs screenshots.

### Week 2 (Recording/Export & Mixer)
- Day 8: Ring buffer integration; writer thread scaffolding.
- Day 9: `start_recording/stop_recording/is_recording`; timestamp pathing.
- Day 10: Export to WAV; duration stub and menu wiring.
- Day 11: Master mute/mono/phase; pan law verification.
- Day 12: Input monitoring gain; feedback-safe routing.
- Day 13: File verification script; validate sample rate/bit depth.
- Day 14: QA and docs updates; acceptance checklist.

### Week 3 (Live View, Gesture, Node Graph Enhancements)
- Day 15: Crossfader coefficient wiring and visual feedback.
- Day 16: Deck A/B gain and EQ stubs; scene toggle hooks.
- Day 17: Gesture thresholds/debounce; mapping UI.
- Day 18: HID/OSC stubs for external controllers.
- Day 19: Node selection/connection UX improvements.
- Day 20: Preset browser enhancements; persist last used.
- Day 21: QA gesture stress; record end-to-end demo.

### Week 4 (MIDI/MPE, Microtuning, Visuals, QA)
- Day 22: MIDI device enumeration and mapping stubs.
- Day 23: Per-note expression routing; timing alignment.
- Day 24: `.scl`/`.kbm` loaders; tuning table application.
- Day 25: Visual node parameter UI; audio-reactive hooks.
- Day 26: Stability hardening; panic recovery paths.
- Day 27: Performance profiling and optimizations.
- Day 28: Documentation finalization; screenshots; reviewer script.

## Acceptance Criteria (Consolidated)
- Transport: Play/Stop/Pause/SetTempo/Loop flags are reliable under load.
- Recording/Export: WAV files validate externally; no underruns.
- Mixer: smooth pan/volume; mute/mono/phase toggles behave correctly.
- Node Graph: preset load/apply; connections persist; audio path intact.
- Live View: crossfader and scene controls responsive with visible feedback.
- Gesture: ≥90% recognition accuracy; no unintended triggers.
- MIDI/MPE: timing aligned; per-note expressions demonstrable.
- Microtuning: tuning tables apply consistently to generators.
- Visuals: desktop-only shader nodes render placeholders; parameters persist.
- Persistence: `ui_state.json` restores layouts/themes/views reliably.

## Risk Register (Expanded)
- Audio device variability: add device selection UI; retry strategy.
- Recording contention: writer thread backpressure handling.
- Gesture mapping complexity: simplified defaults; clear reset options.
- MIDI timing drift: jitter buffers and correction.
- Visual pipeline instability: isolate visuals from audio thread.
- Scope creep: maintain non-goals; defer advanced features.

## Glossary
- BPM: Beats Per Minute; transport tempo indicator.
- MPE: MIDI Polyphonic Expression; per-note expressive control.
- `.scl`/`.kbm`: Scala tuning file formats.
- Pan law: method to balance perceived loudness across stereo field.

## Appendices
- CLI: `cargo run --release`; environment setup per `docs/developer-setup.md`.
- File paths: `./recordings/`, `./exports/`, `ui_state.json`.
- Runbooks: crash recovery steps; troubleshooting audio device.

---

# Detailed Module Specifications (Desktop)

## Source Tree Overview
- `src/main.rs`: entrypoint; initializes UI and engine.
- `src/lib.rs`: library exports and core types.
- `src/daw_core.rs`: DAW coordinator; manages transport and engine state.
- `src/audio_engine/mod.rs`: module root for audio engine.
- `src/audio_engine/transport.rs`: transport implementation (states, clock, tempo, loop flags).
- `src/audio_engine/cpal_io.rs`: CPAL device init, output/input streams, callback wiring.
- `src/audio_engine/dsp_core.rs`: DSP processing primitives and node interfaces.
- `src/audio_engine/node_graph.rs`: runtime graph structures for processing chain.
- `src/audio_engine/node_instance_manager.rs`: lifecycle and parameter routing for nodes.
- `src/node_graph.rs`: UI/engine coordination for graph editing.
- `src/event_queue.rs`: message/event bus between UI and engine.
- `src/ui/bevy_egui_ui.rs`: desktop UI with Bevy + `bevy_egui`.
- `src/ui/egui_ui_full.rs`: legacy UI path; used for reference/testing.
- `src/ui/professional_daw_ui.rs`: advanced UI scaffolding.
- `src/transport_sync.rs`: synchronization helpers between UI and audio engine.
- `src/presets/*`: preset definitions for nodes (generators/effects/utilities).
- `src/midi2_mpe.rs`: MIDI 2.0 and MPE pathways.
- `src/gesture_integration.rs`: gesture mapping and thresholds.
- `src/hid_osc_bindings.rs`: hardware/OSC stubs for external control.
- `src/theming_system.rs`: themes and UI styling.

## Transport Specifications
- States: `Stopped`, `Playing`, `Paused`, `Recording`.
- Clock: beat/tick counters; BPM (float) with smoothing window to avoid jitter.
- Commands: `Play`, `Stop`, `Pause`, `SetTempo(f32)`, `SetLoop(start,end)`.
- Looping: inclusive time region with wrap detection; UI indicators.
- Timebase: sample counter and beat/tick mapping; sample-accurate scheduling.

## Audio IO Specifications
- Device selection: default device auto-select; device dropdown planned.
- Stream configuration: sample rate, channel count, buffer size guardrails.
- Output callback: pull from engine; apply master gain/pan; meter taps.
- Input monitoring: capture and mix into output path safely; gain and gate.
- Error handling: retry on device lost; safe fallback to paused transport.

## Recording & Export Specifications
- Recording start/stop: atomic flags; writer thread with ring buffer.
- File formats: WAV 24-bit PCM preferred; fallback 16-bit.
- Paths: `./recordings/hexodsp_<ts>_<bpm>.wav`; `./exports/session_<ts>.wav`.
- Buffering: multi-segment ring; backpressure handling; flush on stop.
- Validation: external playback check; header correctness; bit depth verification.

## Mixer Specifications
- Master gain: ramp per buffer; pan law (constant power).
- Mono sum: L+R averaging with headroom; phase invert toggle.
- Meters: peak and RMS; configurable hold and decay.
- Future: track strips; sends/returns; simple EQ/dynamics.

## Node Graph Data Model
- Nodes: typed generators/effects; unique IDs; parameter sets.
- Connections: directed edges; channel mapping; bypass flags.
- Parameters: typed (float/int/bool); smoothing meta; min/max.
- Presets: JSON-backed; parameter snapshots and routing hints.
- Persistence: graph serialized into project file (future plan).

## Preset Ecosystem
- Structure: per-node type presets; global presets for chains.
- Location: `src/presets/` grouped by category.
- Loading: UI browser; apply to selected node/chain; persist last-used.
- Compatibility: versioned schemas; migration stubs.

## UI Wireframes (Textual)
- Transport bar: left-aligned play/stop/pause; center BPM; right loop indicator.
- Mixer panel: master fader, pan knob, mute/mono/phase toggles; meters.
- Arrangement view: timeline ruler, playhead, zoom controls; clip placeholders.
- Live View: scenes grid, deck A/B panels, crossfader, EQ knobs.
- Node View: canvas with nodes and cables; parameter inspector on right.
- Status area: CPU %, buffer size, device name; recording indicator.

## Accessibility Plan
- Keyboard shortcuts: space (play/stop), `p` (pause), `b` (BPM focus), `l` (loop toggle).
- Focus order: transport → mixer → views → inspector; visible focus rings.
- High contrast themes: minimum contrast ratios; color-blind safe palettes.
- Screen reader hints: labels and roles for controls; descriptive tooltips.

## Performance Engineering Playbook
- Profiling: measure callback durations, buffer underflow/overflow, CPU budgets.
- Logging: disable in audio thread; aggregate in bridge; sampling debug logs.
- Memory: preallocate buffers; avoid heap in callback; fixed-size containers.
- Visuals isolation: ensure shader/UI work never blocks audio thread.
- Stress tests: rapid transport toggles, gesture floods, preset loads with audio active.

## QA Matrices
- Transport: state transitions with inputs; loop boundaries; BPM changes under load.
- Recording: start while playing; stop while paused; file validation matrix.
- Mixer: gain/pan ramps; mute/mono/phase toggles; meter accuracy checks.
- Node graph: add/connect/remove operations; parameter edits; preset apply.
- Live View: scene toggles; crossfader behavior; deck controls.
- Persistence: `ui_state.json` save/load across sessions and themes.

## Release Notes Template
- Features: list of added/changed modules and UI components.
- Fixes: stability and performance improvements.
- Known Issues: device quirks and workaround notes.
- Upgrade Notes: config schema changes and migrations.

## Contribution Workflow
- Code style: keep changes minimal and scoped; avoid unrelated fixes.
- Testing: write unit/integration tests near changed modules when feasible.
- Documentation: update feature acceptance criteria and usage notes.
- Review: focus on thread safety, audio callback behavior, and UX clarity.

## Advanced Roadmap (Post-Month)
- Full track/mixer architecture with sends/returns and buses.
- Automation editor across tracks and parameters; curve tools.
- Plugin hosting expansions (VST3/AU/CLAP) with sandboxing.
- Project file format and save/load workflows.
- Advanced visuals pipeline with audio-reactive shaders and presets.
- Collaboration hooks (future): local session notes and export bundles.

## Design Rationale Notes
- Desktop-only scope reduces complexity and maximizes reliability.
- CPAL chosen for cross-platform device access with minimal dependencies.
- Bevy + `bevy_egui` offers performant UI with Rust ergonomics.
- Atomic flags and lock-free structures improve real-time safety.

## Open Questions & Decisions
- Device selection UI timing: Week 1 vs Week 2?
- 24-bit vs 32-bit float export: compatibility vs fidelity.
- Preset schema extensibility for automation metadata.
- Gesture mapping defaults and reset behavior.

## Dependencies & Environment
- Rust toolchain: 1.75+; stable channel.
- Libraries: CPAL, hound, serde, bevy, bevy_egui.
- OS targets: Windows primary; macOS/Linux sanity checks.
- Hardware: mid-tier CPU targets; optional external controllers.

## Troubleshooting Guide
- No audio: check device availability; sample rate mismatch; fall back to default.
- Recording silent: verify monitoring and master path; check ring buffer writer.
- UI stutter: reduce logging; profile visuals; confirm audio thread isolation.
- Gesture misfires: adjust thresholds; review device DPI and pointer settings.

## Deep API Reference (Desktop)

### Audio Engine Core APIs
- `AudioEngine::init(config: EngineConfig) -> Result<()>`
- `AudioEngine::start() -> Result<()>`
- `AudioEngine::stop() -> Result<()>`
- `AudioEngine::process(block: &mut AudioBlock, now: TransportTick)`
- `AudioEngine::set_param(node_id: NodeId, param_id: ParamId, value: f32, ramp: Option<RampSpec>)`
- `AudioEngine::connect(src: OutputPort, dst: InputPort) -> Result<()>`
- `AudioEngine::disconnect(src: OutputPort, dst: InputPort) -> Result<()>`
- `AudioEngine::graph_snapshot() -> GraphSnapshot`

Messages:
- `AudioParamMessage { node_id, param_id, value, ramp_ms, curve }`
- `ProcessingState { xruns: u32, callback_time_us: u32, cpu_pct: f32, buffer_size: u32 }`
- `NodeLifecycleEvent { node_id, event: Created | Initialized | Active | Bypassed | Destroyed }`

### Node Instance Manager APIs
- `NodeInstanceManager::create(def: NodeDef) -> NodeId`
- `NodeInstanceManager::delete(node_id: NodeId) -> Result<()>`
- `NodeInstanceManager::instantiate(node_id: NodeId) -> Result<()>`
- `NodeInstanceManager::list() -> Vec<NodeInfo>`
- `NodeInstanceManager::set_bypass(node_id: NodeId, bypass: bool)`
- `NodeInstanceManager::load_preset(node_id: NodeId, preset: Preset)`
- Contracts: instantiation deterministic; param ranges enforced per `ParamSpec`.

### Transport and Timeline APIs
- `Transport::set_tempo(bpm: f32)`
- `Transport::set_meter(numerator: u8, denominator: u8)`
- `Transport::seek(tick: u64)`
- `Transport::play()` / `Transport::pause()` / `Transport::stop()`
- `Transport::loop(range: Option<LoopRange>)`
- `Transport::subscribe(cb: impl Fn(TransportEvent))`

Events:
- `TransportEvent::Tick(now: u64)`
- `TransportEvent::StateChanged { is_playing: bool }`
- `TransportEvent::LoopChanged { start: u64, end: u64 }`

### Persistence APIs
- `Project::new(name: &str) -> Project`
- `Project::add_track(track: Track) -> TrackId`
- `Project::remove_track(track_id: TrackId)`
- `Project::save(path: &Path) -> Result<()>`
- `Project::load(path: &Path) -> Result<Project>`
- `Project::export_audio(format: ExportFormat, path: &Path) -> Result<()>`

Schema guarantees:
- UUIDv4 IDs; references validated on load.
- Backward compatibility via `schema_version` and `migrations[]`.

## Project File Schema Proposal

Top-level keys:
- `schema_version`, `project` (name, author, timestamps, tags[]).
- `audio_settings` (sample_rate, buffer_size, latency_mode).
- `tracks[]` (clips, nodes, automation).
- `buses[]` (aux/master), `graph[]` connections.
- `presets` (inline/references), `midi` device profiles.

Example (truncated):
```
{
  "schema_version": 1,
  "project": {"name": "Example", "author": "User"},
  "audio_settings": {"sample_rate": 48000, "buffer_size": 256},
  "tracks": [{"id": "track-01", "name": "Synth"}],
  "graph": [],
  "midi": {"devices": []}
}
```

Migration notes:
- `migrations[]` entries `{from, to, transform_id}` with deterministic transforms.
- Validate after transform; write atomically.

## DSP Node Specifications

Taxonomy: generators, modulators, processors (filters/dynamics), utilities.

Common contracts:
- Parameters: `id`, `unit`, `min`, `max`, `default`, `smoothing_ms`.
- Audio-rate modulation enabled via `audio_mod=true` flags.
- Bypass pop-free with `smoothing_ms >= 2`.

Examples:
- `generator.osc`: wave, freq(0.1–20000 Hz), detune(±50 cents).
- `effect.filter`: type(lpf/hpf/bpf), cutoff(10–20000 Hz), res(0–1).
- `effect.delay`: time(1–2000 ms), feedback(0–0.95), mix(0–1).
- `utility.gain`: gain(-60–+12 dB), pan(-1–+1).

## QA Test Case Catalog

Functional:
- Project round-trip with 100+ nodes and 10 tracks.
- 1000 edge connect/disconnect without deadlocks.
- Parameter automation with 10k points under playback.
- Record 8 inputs for 10 minutes without xruns.
- Export WAV at 44.1/48/96 kHz with dither on/off.

Performance:
- CPU < 70% at 256 buffer with 50 nodes.
- Callback p95 < 1.5 ms at 48 kHz.
- GC pauses never in realtime path; non-RT p95 < 2 ms.

Stress/soak:
- 24-hour soak; memory growth < 5%.
- Fan-out 1→32 graphs produce pop-free output.

Compatibility:
- ASIO/WASAPI/CoreAudio enumeration; device change resiliency.
- VST3 scan handles 500 plugins with cached metadata.

Accessibility:
- Keyboard maps for transport, selection, editing.
- Screen-reader labels for all controls.

## Scenario Walkthroughs

Electronic track (30 min):
- Create project; set tempo 128 BPM.
- Drum sampler; program 8-bar pattern.
- Bass synth; sidechain compressor keyed by drum.
- Automate filter; record MIDI; quantize groove.
- Reverb/delay buses; tune sends; export preview.

Live session (guitar + vocal):
- Configure inputs; buffer 128; low-latency.
- Arm tracks; pre-roll; tuner and compressor.
- Record takes; comp clips; EQ/de-esser.
- Bounce stems; export master at -14 LUFS.

## Design Diary (Rationale)
- `egui+eframe` for predictable desktop rendering and latency.
- WebGL visualizations deferred to maintain realtime audio stability.
- UUID-based IDs for robust project merging.
- Parameter smoothing to prevent zipper noise.
- Standardized preset format for desktop/web portability.

## Performance Engineering Cookbook

Profiling:
- Log `callback_time_us` histograms from CPAL.
- Use `cargo flamegraph` under non-RT builds.
- Measure buffer underruns vs graph fan-out.

Tuning:
- Adjust buffer size 128/256/512 by workload.
- Prefer offline render for heavy chains.
- Reduce UI parameter update rate under load.

Logging hygiene:
- Disable verbose logging in realtime path.
- Sample performance logs at 1 Hz.

## Release Notes Template
Version x.y.z
- New: highlights
- Improvements: performance/UX
- Fixes: bug summaries
- Known issues: workarounds
- Migration: schema changes
- Checksums: installer/binaries

## Contributor Workflow
- Branches: `feat/`, `fix/`, `docs/`, `perf/`.
- PR checklist: tests, docs, changelog, perf notes.
- Gates: rustfmt + clippy; no allocs in realtime path.

## Advanced Roadmap (Desktop)
- Advanced modulation matrix; macro controls; clip FX.
- Elastic audio; transient detection; time-stretch.
- Multi-core scheduler; spatial audio; surround.

---

# Comprehensive Catalogs & Playbooks (Desktop)

## Full Keyboard Shortcuts Map
- Space: Play/Stop toggle
- P: Pause
- B: Focus BPM input
- L: Toggle loop
- M: Master mute
- Shift+M: Master mono
- I: Phase invert
- G: Open gesture mapping
- 1/2/3: Switch views (Arrangement/Live/Node)
- Ctrl+S: Save UI state
- Ctrl+O: Open project (future)
- Ctrl+E: Export mix
- R: Start/Stop recording
- Arrow Left/Right: Nudge playhead
- Ctrl+Arrow Left/Right: Jump to marker (future)
- +/-: Zoom timeline
- Ctrl+Shift+P: Performance overlay

## Parameter Catalog (Core Nodes)

Generator.Oscillator
- wave: enum(sine|saw|square|triangle)
- freq: Hz(0.1–20000), smoothing_ms: 5
- detune: cents(-50..+50)
- phase: radians(0..2π)
- sync: bool

Generator.Noise
- type: enum(white|pink|brown)
- color: 0..1
- seed: u32

Effect.Filter
- type: enum(lpf|hpf|bpf|notch)
- cutoff: Hz(10–20000), smoothing_ms: 10
- resonance: 0..1
- drive: dB(0..24)
- keytrack: 0..1

Effect.Delay
- time_ms: 1–2000
- feedback: 0..0.95
- mix: 0..1
- sync_beats: 0..16
- mode: enum(tape|digital)

Effect.Reverb
- size: 0..1
- decay_s: 0.1–20
- pre_delay_ms: 0–100
- damp: 0..1
- mix: 0..1

Utility.Gain
- gain_db: -60..+12
- pan: -1..+1
- mute: bool
- mono: bool
- phase_invert: bool

Meter.Analyser
- window: enum(hann|hamming|rect)
- fft_size: enum(256|512|1024|2048)
- smoothing: 0..1
- hold_ms: 0–2000

## Preset Schema Examples

Preset (node-level)
```
{
  "schema": 1,
  "node_type": "generator.osc",
  "params": {
    "wave": "saw",
    "freq": 220.0,
    "detune": 2.5
  },
  "tags": ["bass", "analog"],
  "author": "HexoDSP"
}
```

Chain Preset (multi-node)
```
{
  "schema": 1,
  "nodes": [
    {"id": "osc", "type": "generator.osc", "params": {"wave": "square", "freq": 110}},
    {"id": "filt", "type": "effect.filter", "params": {"type": "lpf", "cutoff": 800, "resonance": 0.4}},
    {"id": "verb", "type": "effect.reverb", "params": {"size": 0.6, "mix": 0.25}}
  ],
  "graph": [
    {"from": {"node": "osc", "port": "out"}, "to": {"node": "filt", "port": "in"}},
    {"from": {"node": "filt", "port": "out"}, "to": {"node": "verb", "port": "in"}},
    {"from": {"node": "verb", "port": "out"}, "to": {"node": "master", "port": "in"}}
  ],
  "tags": ["lead", "space"]
}
```

## Project Save/Load Migration Playbook
- Version gate: `schema_version` at file root; refuse to load future versions.
- Migration chain: sequential transforms from `n` to `n+1` with idempotent rules.
- Validation: structural (IDs, references) then semantic (ranges, enums).
- Atomic writes: temp file + rename; fallback to previous backup.
- Backups: keep last 5 versions with timestamp.
- Logs: record transform IDs and summaries; no realtime thread logging.

Common transforms
- Split `mixer.master` into `gain_db`, `pan`, `flags`.
- Introduce `automation[]` envelopes with explicit interp.
- Normalize preset refs to absolute canonical IDs.

## Acceptance Matrices by Subsystem

Transport
- Play/Stop/Pause: stable under rapid toggles (500 toggles, no crash)
- BPM changes: 60→160→90 with audio active, no jitter perceivable
- Loop: enter/exit loop boundaries cleanly; wrap detection on edges

Recording/Export
- Start recording mid-play; stop at arbitrary points; file integrity OK
- Export during pause; validate duration and headers
- 24-bit PCM preference respected; fallback to 16-bit on unsupported

Mixer
- Gain ramps: no zipper noise; pan law constant power; mono sum headroom
- Phase invert: audible polarity flip; meters adjust correctly

Node Graph
- Add/remove/connect nodes under playback; no audio dropouts
- Parameter edits reflect immediately; smoothing prevents artifacts
- Preset apply persists; last-used preset restored on reload

Live View & Gesture
- Crossfader behavior smooth; deck EQ knobs bounded
- Gesture mapping accuracy ≥90%; debounced; visual hints visible

MIDI/MPE & Microtuning
- Per-note expression routes stable; timing aligned to transport
- Tuning tables apply per voice; compatible with MPE pathways

Persistence
- `ui_state.json` round-trip across sessions; schema version respected

## Implementation Guidelines & Anti-Patterns

Do
- Prefer immutable messages; copy-on-write for params off audio thread
- Preallocate buffers; avoid heap in callback
- Use atomic flags for record/export state; serialize writer commands
- Keep visuals/UI work off realtime path; isolate via bridge

Avoid
- Blocking operations in audio callback
- Allocations or locks in callback path
- Mixed responsibilities in single module; maintain separation of concerns
- Unbounded logs; any logging in callback

## Extended Daily Plans (Augmented)

Week 1 adds
- Device dropdown prototype; error banners for init failures
- Pause/loop edge cases; wrap tests and indicators
- BPM smoothing curve experiments; select default curve

Week 2 adds
- Export format chooser; validate headers via script
- Mixer polish: meter decay/hold; visual calibrations
- Input monitoring gate threshold tuning; feedback detection

Week 3 adds
- Gesture mapping presets; reset-to-defaults flow
- Deck EQ ranges and tooltips; scene label editing
- Preset browser search and categories; persistence of last-used

Week 4 adds
- MIDI device profiles cache; reconnection workflows
- Tuning import validations; error dialogs with recovery
- Visual nodes parameter maps standardized; QA soak reports

## Architecture Narratives (Text Diagrams)

Threads
```
UI ──► Bridge ──► Audio Callback
            ▲            │
            │            ▼
          Writer ◄──── Ring Buffer
```

Message flow
```
UI Events → AudioParamMessage → Engine Params
Engine State → ProcessingState → UI Meters/Indicators
```

Graph processing
```
Node(in) → DSP → Node(out) → … → Master → Output
```

## Expanded Risk Analysis
- Device API quirks: provide manual override paths; whitelist stable configs
- File I/O stalls: writer backpressure handling with drop warnings
- Gesture misinterpretation: provide sensitivity controls; disable per view
- Preset compatibility drift: pin versions; expose migration tool
- Visual instability: frame-rate caps; watchdog for runaway shaders

## SOPs & Runbooks (Desktop)
- Audio device lost: pause transport; retry init; show banner; log event
- Crash recovery: write crash dump; restore last autosave; safe mode launch
- Recording interrupted: preserve partial file; annotate log; prompt continue
- Performance spike: auto-enable performance overlay; suggest buffer increase

## Extended Glossary
- RMS: root-mean-square level; used for loudness approximation
- LUFS: loudness units relative to full scale; mastering target
- Jitter: timing variation; minimized via smoothing and stable clocks
- Xrun: buffer under/overrun in audio callback

## Keyboard Shortcuts Map (Desktop)
- Global: `Space` play/pause, `S` stop, `L` toggle loop, `Ctrl+B` toggle buffer size panel.
- Transport: `↑/↓` BPM ±0.5, `Shift+↑/↓` BPM ±5, `R` record arm toggle.
- Mixer: `G` focus gain slider, `P` focus pan, `M` mute, `Solo` via `Alt+M`.
- Node Graph: `N` add node, `Del` delete, `Ctrl+D` duplicate, `Alt+Drag` connect.
- Arrangement: `Z` zoom in, `Shift+Z` zoom out, `A` add marker, `Ctrl+G` toggle grid.
- Live View: `1/2` deck select, `C` crossfader focus, `Q/W/E` eq bands select.
- File Browser: `Enter` open/import, `Backspace` up directory, `Ctrl+F` search.
- Preset Browser: `Enter` apply, `Ctrl+S` save, `Ctrl+Shift+S` save-as.
- Accessibility: `Tab/Shift+Tab` structured nav, `F6` cycle major panels.

## Core Node Parameter Catalog
- Oscillator: waveform(enum), freq(Hz), detune(cents), phase(rad), sync(bool).
- Filter: type(enum), cutoff(Hz), resonance(Q), drive(dB), slope(12/24 dB).
- Amp: gain(dB), pan(%), mute(bool), solo(bool).
- Delay: time(ms), feedback(%), mix(%), filter-cutoff(Hz).
- Reverb: size(%), decay(s), pre-delay(ms), damping(%), mix(%).
- Compressor: threshold(dB), ratio(x:1), attack(ms), release(ms), knee(dB).
- EQ3: low(dB), mid(dB), high(dB), low-freq(Hz), mid-freq(Hz), high-freq(Hz).
- Analyser: fft-size, smoothing, window-type, peak-hold(ms), range(dB).
- Sampler: start(ms), end(ms), rate(Hz), loop(bool), loop-start/end(ms).
- GainStage: gain(dB), trim(dB), meter-avg(ms), meter-peak(ms).

## Preset Schema Examples
Minimal preset
```
{
  "version": 1,
  "name": "Basic Lead",
  "graph": {
    "nodes": [
      {"id": "osc1", "type": "Oscillator", "params": {"waveform": "saw", "freq": 440}},
      {"id": "filt", "type": "Filter", "params": {"type": "lowpass", "cutoff": 1200, "resonance": 0.7}},
      {"id": "amp", "type": "Amp", "params": {"gain": -6}}
    ],
    "edges": [
      {"from": "osc1", "to": "filt"},
      {"from": "filt", "to": "amp"},
      {"from": "amp", "to": "output"}
    ]
  },
  "tags": ["lead", "synth"],
  "author": "system"
}
```

Preset with modulation
```
{
  "version": 1,
  "name": "LFO Pulse",
  "graph": {
    "nodes": [
      {"id": "osc1", "type": "Oscillator", "params": {"waveform": "square", "freq": 220}},
      {"id": "lfo1", "type": "Oscillator", "params": {"waveform": "sine", "freq": 2}},
      {"id": "amp", "type": "Amp", "params": {"gain": -3}}
    ],
    "edges": [
      {"from": "osc1", "to": "amp"},
      {"from": "amp", "to": "output"}
    ],
    "modulations": [
      {"source": "lfo1", "target": "osc1.freq", "amount": 20}
    ]
  }
}
```

## Project Save/Load Migration Playbook
- Policy: never break old saves; add `migrations[]` array with discrete steps.
- Step types: `add_param`, `rename_param`, `move_node`, `replace_node`, `reconnect_edge`.
- Example migration
```
{
  "from": 1,
  "to": 2,
  "steps": [
    {"type": "rename_param", "node": "Filter", "old": "resonance", "new": "q"},
    {"type": "add_param", "node": "Compressor", "param": "knee", "default": 3}
  ]
}
```
- Loader algorithm: apply steps sequentially with validation; log warnings; skip unsafe.
- Exporter: always write latest version; include `compat.notes` section for known changes.

## Subsystem Acceptance Matrices
- Transport: start/stop jitter < 2ms; loop boundary glitch-free; BPM set accuracy ±0.1.
- Recording: header correct; sample count matches duration; peak not clipped with headroom.
- Mixer: gain setpoint ±0.1 dB; pan law consistent; meter peak/hold behavior predictable.
- Node Graph: operations atomic; undo/redo consistent; connections validated; no cycles.
- Preset: apply idempotent; load under 50ms median; tags searchable; schema validated.
- Accessibility: keyboard coverage > 95%; screen reader labels on primary controls.

## Implementation Guidelines
- Keep audio callback minimal; move logic to precomputed tables and atomic flags.
- Use typed structs for parameters; avoid dynamic lookups in tight loops.
- Prefer lock-free queues with fixed capacity; backpressure with drop+warn policy.
- GPU visuals: bound uniforms; reuse FBOs; limit post-process passes under 2 per frame.
- Feature flags: gate risky features; collect opt-in diagnostics for perf tuning.

## Extended Architecture Narratives
State snapshot
```
Engine Params → Frame Cache → UI Render → Visual Output
           ▲                     │
           └─────── Metrics ◄────┘
```

Device init
```
Enumerate → Select → Configure → Prime Buffers → Begin Callback
```

## Error Code Reference (Desktop)
- EDEV01: Device not found
- EDEV02: Device access denied
- EIO01: File write error
- EIO02: Disk full
- ENET01: Bridge disconnected
- EPARAM01: Parameter out of range
- EGRAPH01: Invalid connection
- EREC01: Recording overflow
- EREC02: Recording interrupted

## Telemetry & Privacy Policy Details
- Telemetry disabled by default; opt-in via settings with granular toggles.
- Metrics: frame time histogram, callback overrun counts, device change events.
- Crash dumps: local-only unless user chooses to upload.
- Anonymization: no personal data; hash device identifiers; rotate IDs.
- Data retention: user-configurable; auto-purge after 30 days.

## Release Governance
- Versioning: semantic (major.minor.patch); breaking changes only on major.
- Release cadence: weekly beta; monthly stable; hotfix as needed.
- Sign-off: QA matrix pass; perf targets met; accessibility checks validated.
- Change log: human-readable summary; link to detailed notes; migration notes.