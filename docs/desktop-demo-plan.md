# Desktop Demo Plan (Desktop-Only) - ⚠️ REALITY CHECK

**⚠️ CRITICAL: This plan documents INTENDED functionality, not current implementation. Most features are NOT YET BUILT.**

This plan provides an explicit, click-by-click script to run and verify the desktop demo. It applies to Windows and uses Bevy + `bevy_egui` UI with CPAL audio output.

## Prerequisites - ACTUAL REQUIREMENTS
- Rust toolchain installed (`rustup`, stable).
- A working audio output device on Windows.
- Project checked out at `c:\Users\kapil\compiling\hexodsp-vst3`.
- **⚠️ Warning: Limited functionality available**

## How to Run (Windows) - CURRENT EXPERIENCE
- `cargo run` (no environment variables needed).
- The desktop window opens using Bevy + `bevy_egui`.
- **Result: Basic UI with visual nodes only - no audio processing**

## Demo Script (Operator Steps)
1. Open the Node View from the desktop UI.
2. Use the toolbar to add nodes in order:
   - Generator: `Sine Oscillator`
   - Filter: `Low-Pass Filter (LPF)`
   - Effect: `Delay`
   - Output: `Output`
3. Create connections:
   - `Sine Osc → LPF`
   - `LPF → Delay`
   - `Delay → Output`
4. Transport control:
   - Click `Play` to enable audio output. Audio is gated by transport; no sound until `Play`.
   - Use `Stop` to stop audio.
5. Parameter panel (selected node):
   - Select `Sine Oscillator` and adjust `Frequency` (e.g., 220–880 Hz).
   - Select `LPF` and adjust `Cutoff` (e.g., 200–8000 Hz).
   - Select `Delay` and adjust `Feedback` (e.g., 0.0–0.8).
   - Confirm each slider change produces audible differences.
6. Canvas controls for visuals:
   - Toggle `Lock Nodes` and attempt to drag nodes (should not move when locked).
   - Toggle `Grid Snap` and drag nodes (positions snap to grid).
   - Adjust `Sides` slider (3–12). Default is 6 (hex). Nodes re-render accordingly.
   - Toggle `Ports on vertices` to move ports from edges to corners; wiring remains valid.

## Expected Behaviors - REALITY CHECK

### ❌ CLAIMED (NOT WORKING):
- Nodes render as regular polygons; accurate hit-testing, selection, and dragging.
- Ports placed on edges by default; shift to vertices when toggled.
- Connections draw routed curves with clear directionality.
- Audio outputs via CPAL backend; transport gates audio (`Play`/`Stop`).
- Parameter sliders send updates to the audio engine; sound changes immediately.

### ✅ ACTUAL (WORKING):
- **Visual node rendering**: Basic polygon shapes
- **Mouse interaction**: Can select and drag nodes
- **Visual connections**: Lines drawn between nodes
- **Basic CPAL initialization**: Test tone generation only
- **UI framework**: Basic egui components

## Engine Bridge Notes - CURRENT LIMITATIONS

### ❌ CLAIMED BRIDGE FUNCTIONALITY:
- UI sends `AudioParamMessage` to the audio engine via the bridge:
  - `CreateNode(ui_node_id, node_kind)` when adding a node.
  - `ConnectNodes(src_ui_node_id, src_port, dst_ui_node_id, dst_port)` when wiring ports.
  - `SetNodeParameter(ui_node_id, param_name, value)` when adjusting sliders.
- Transport gating uses engine calls that enable/disable playback in CPAL output.

### ✅ ACTUAL BRIDGE STATUS:
- **Basic message framework exists** but **no audio processing**
- **Visual node creation** works but **no audio connection**
- **Parameter sliders move** but **don't affect audio**
- **No transport system** - no play/stop functionality

## 🎯 CURRENT DEMO REALITY SUMMARY

### What You Can Actually Do:
```bash
cargo run
# Opens basic UI window
# Can see visual nodes
# Can drag nodes around
# Can draw connections
# Can move sliders (no audio effect)
# Basic test tone audio only
```

### What You CANNOT Do (Despite Claims):
- ❌ **No real audio processing** - Only test tones
- ❌ **No transport controls** - Cannot play/stop
- ❌ **No parameter audio changes** - Sliders move but don't affect sound
- ❌ **No file I/O** - Cannot save/load anything
- ❌ **No professional DAW features** - This is a visual prototype only

### Honest Assessment:
**This is a basic visual demonstration of the UI framework, not a functional DAW demo. All audio processing, transport, and professional features are aspirational and not implemented.**

## Engine Bridge Notes (for developers)
- UI sends `AudioParamMessage` to the audio engine via the bridge:
  - `CreateNode(ui_node_id, node_kind)` when adding a node.
  - `ConnectNodes(src_ui_node_id, src_port, dst_ui_node_id, dst_port)` when wiring ports.
  - `SetNodeParameter(ui_node_id, param_name, value)` when adjusting sliders.
- Transport gating uses engine calls that enable/disable playback in CPAL output.

## Validation Checklist
- No panics or fatal logs while running `cargo run`.
- Adding nodes and wiring works; no ghost or duplicate connections.
- Grid snapping and locking behave consistently during drags.
- Parameter widgets update the engine without desync; audible changes occur.
- Transport `Play` starts audio; `Stop` ends audio cleanly.

## Known Limits (This Sprint)
- Preset management and advanced modulation are deferred.
- Cross-platform packaging beyond Windows is out of scope for this demo.

## Comprehensive Demo Scenarios (End-to-End)

### Scenario 1: Transport and Mixer
- Locate transport controls (`Play`, `Stop`, `Record`, `Tempo`) and master mixer.
- Click `Play` to enable audio; click `Stop` to disable.
- Adjust master `Volume` and `Pan`; confirm level and stereo changes.
- On Track 1, toggle `Mute` and `Solo`; confirm routing behavior matches expectations.

### Scenario 2: Tracks and Returns
- Ensure Track 1 is present; test `Mute`, `Solo`, and `Volume` interactions.
- Create/enable a Return bus (e.g., `Reverb` or `Delay`) if available.
- Use track `Send` to route audio to the Return; adjust return level; confirm audible effect.

### Scenario 3: Arrangement Basics
- Open Arrangement view.
- Create a simple clip (or use a test tone) to validate timeline playback.
- Adjust project `Tempo`; confirm it reflects in transport and any timing-dependent UI.

### Scenario 4: Node Graph Patch
- Open Node View.
- Add nodes: `Sine Oscillator` → `Low-Pass Filter (LPF)` → `Delay` → `Output`.
- Connect: `Osc → LPF → Delay → Output`.
- Adjust parameters on selected nodes and confirm immediate audible changes.

### Scenario 5: Canvas Controls and Interaction
- Toggle `Lock Nodes`; attempt to drag (no movement when locked).
- Toggle `Grid Snap`; drag to see snapping behavior.
- Change `Sides` (3–12); default 6 (hex); confirm re-render.
- Toggle `Ports on vertices`; ports move to corners; wiring remains valid.

### Scenario 6: Presets and Parameter Recall
- Load a preset for a simple chain; verify nodes/parameters restore.
- Save a preset; reload to confirm persistence.

### Scenario 7: Recording/Playback Validation
- Arm Track 1 and press `Record` while playing to capture input/tone.
- Stop and play back the recording; confirm normal playback without errors.

### Expected Outcomes Across Scenarios
- Transport gates audio; mixer and track controls affect output.
- Node parameter edits send `SetNodeParameter` and reflect in sound.
- Returns receive sends; arrangement plays clips or test tones.
- Recording produces a buffer that plays back cleanly.
# HexoDSP Desktop Demo Plan (v1.0)

This document describes the scope, user journeys, feature set, acceptance criteria, and technical outline for the HexoDSP Desktop Demo. It is intended to be at least 10 pages worth of content in its current form and serves as the foundation for a full DAW product documentation (which will be 100+ pages when complete). The demo must present a cohesive, functional subset of the final DAW, showcasing at least 10–20 core features users expect, with real workflows that can be executed end-to-end.

## Executive Summary

- Goal: Deliver a desktop demo that is “full enough” to demonstrate professional DAW fundamentals: transport, recording, monitoring, mixing, node-based sound design, and performance controls (Live View, Gesture Control), while establishing a clean path to the complete DAW.
- Audience: Producers, sound designers, live performers, and internal stakeholders evaluating viability and UX.
- Scope: 20 demo features grouped into transport/recording, live performance, mixing/master, node graph, file management, and workflow utilities.
- Outcome: A demo that runs locally, plays, records to WAV, exports audio, shows meters, loads presets, and enables a basic Live View and Gesture Control interaction path.

## Demo Goals and Non-Goals

### Goals
- Sample-accurate transport: play, stop, pause, record.
- Real-time audio IO: output stream with gating, input monitoring.
- Recording/export: record stereo mix to WAV, export current session.
- Mixer basics: master volume, pan, mute; simple EQ and dynamics toggles.
- Node graph: instantiate generators/effects, connect nodes, adjust parameters.
- Live View: clip launch and deck controls, crossfader, basic scene management.
- Gesture Control: map gestures to transport, crossfader, and clip triggers.
- File browser: navigate samples/projects, load presets.
- UI modes: Arrangement, Live, Node views.
- Persistence: save/load UI state.

### Non-Goals (for demo v1.0)
- Full multitrack audio recording with comp lanes.
- Deep MIDI editing with advanced quantize and groove templates.
- Third-party plugin host completeness (VST3/AU) beyond placeholders.
- Full automation editing across all parameters.
- Comprehensive project management, collaboration, or cloud sync.

## User Stories

- As a producer, I want to play/stop/record and export a take to share.
- As a live performer, I want a Live View to trigger clips and crossfade decks.
- As a sound designer, I want to wire nodes and tweak parameters quickly.
- As a reviewer, I want a single demo script to validate core functionality.

## Demo Feature Set (19 Items)

1. Transport controls: Play/Stop/Pause.
2. Record to WAV (stereo mix) with start/stop.
3. Export current mix to WAV file.
4. Master volume and pan with smooth ramping.
5. Master mute/mono/phase toggles (basic UI wiring).
6. Input monitoring with gain control.
7. Tempo setting (BPM) and basic metronome plan.
8. Loop enable/length (simple loop block in transport).
9. Arrangement View: timeline position and zoom.
10. Live View: scene buttons, crossfader, deck levels.
11. Gesture Control: mapping transport and crossfader.
12. Node Graph: instantiate nodes and adjust parameters.
13. Preset loading: generators/effects (demo library).
14. File browser: navigate sample library and projects.
15. UI persist/load: save UI state (theme, layout) to disk.
16. Mixer: per-channel volume/pan placeholders; master meters.
17. EQ/Dynamics toggles on Master (basic stub hooks).
18. Track arm flags for future recording routes.
19. Piano roll placeholder and grid resolution control.

Each feature includes status, usage, acceptance criteria, and technical notes below.

## Extended Feature Matrix (Demo Coverage)

- Visuals & Shaders
  - NUWE shaders and ISF plugins in Node View (placeholder UI present; parameters adjustable; audio-reactive visuals planned).
  - Desktop visualizations and performance meters; shader pipelines integrated.
- MIDI 2.0 & MPE
  - `midi2_mpe.rs` foundation; per-note expression pathways planned for demo routing.
  - Basic input device binding and timing alignment with transport.
- Microtuning
  - Support plan for Scala `.scl` and `.kbm` import; tuning tables applied to generators.
  - UI hooks in Node View preset parameters; acceptance tests include pitch verification.
- Modular Patch System
  - `modular_patch_system.rs` present; demo presets for generators/effects available.
  - Node graph canvas supports add/connect/edit with visual feedback.
- Gesture Control
  - `gesture_integration.rs` and `hid_osc_bindings.rs` to map gestures to transport/crossfader.
  - Mouse gesture fallback layer for desktop; live visual hints.
- AI Audio (Optional Demo Toggle)
  - `sai_audio.rs` and `stream_diffusion_audio.rs` stubs; non-blocking UI hooks.
  - Acceptance: actions do not crash engine; clear “experimental” flag.
 

Each item includes a demo-appropriate placeholder where full functionality is not yet implemented; the goal is visible discoverability and a working UX path.

## One-Week Development Guideline (Sprint Plan)

This demo plan is a practical guideline to build “full enough” placeholders for all core areas within one week. Each day targets a cohesive set of features with minimal viable implementations and strict acceptance criteria.

### Day 1: Transport & Audio IO Baseline
- Tasks:
  - Wire `AudioParamMessage::Play/Stop/Pause` to `Transport` and `AudioIO::set_playback_enabled`.
  - Confirm `start_output_only()` runs stable; add BPM control via `SetTempo`.
  - Persist UI toggles and view mode via `ui_state.json`.
- Acceptance:
  - `▶` and `◼` work without glitches.
  - BPM reflects in transport state; UI persists across restarts.

### Day 2: WAV Recording (Stereo Mix)
- Tasks:
  - Implement `AudioIO::start_recording(path)`, `stop_recording()`, `is_recording()` using `hound`.
  - Record frames inside output callback (buffered to avoid blocking).
  - Create `./recordings/` output path and timestamped filenames.
- Acceptance:
  - `⏺` toggles recording; WAV file plays back externally at correct rate.

### Day 3: Export Mix
- Tasks:
  - Reuse recording pipeline for quick export via File menu.
  - Create `./exports/` path and default filename pattern.
  - Add simple duration selection (fixed-length export stub).
- Acceptance:
  - Export produces a valid WAV; no runtime errors.

### Day 4: Live View Basics
- Tasks:
  - Crossfader control maps to deck A/B mix coefficient.
  - Deck volume and simple EQ dials update internal params.
  - Scene buttons toggle visual state (clip trigger stubs).
- Acceptance:
  - Crossfader visibly changes deck levels; UI is responsive.

### Day 5: Gesture Control (Desktop Fallback)
- Tasks:
  - Map mouse gestures to Play/Stop/Record and crossfader sweep.
  - Add HID/OSC binding stubs in `gesture_integration.rs` / `hid_osc_bindings.rs`.
  - Visual gesture hints in Live View.
- Acceptance:
  - Gestures execute mapped actions with ≥90% reliability in tests.

### Day 6: Node Graph, Shaders/Visuals, Microtuning
- Tasks:
  - Confirm node instantiation and parameter editing workflow; wire demo presets.
  - Add visual nodes (NUWE/ISF) placeholders; parameters editable; basic render hooks.
  - Microtuning: stub loaders for `.scl`/`.kbm`; apply tuning table to generators.
- Acceptance:
  - Presets load; visual nodes present; tuning tables apply without transport drift.

### Day 7: QA, Performance, Doc Updates
- Tasks:
  - Run the demo script end-to-end; fix stability issues.
  - Ensure audio callback avoids blocking; confirm CPU budget.
  - Update documentation sections and screenshots as needed.
- Acceptance:
  - Demo script passes; no crashes; recordings/exports verified.

## Feature Maturity Levels and Demo Definition of Done

### Maturity Levels
- Placeholder: UI element exists; parameters editable; no full audio/visual effect.
- Basic: Feature changes sound/visuals in a minimal way; stable; limited options.
- Partial: Multiple options; integrated with transport/mixer; measurable improvement.
- Full: Production-grade feature; comprehensive controls and routing.

For this demo, target “Basic” across transport/record/export/mixer and “Placeholder→Basic” for Live View, Gesture, Shaders/Visuals, MIDI 2.0, and Microtuning.

### Demo Definition of Done (Checklist)
- App launches and runs `bevy_egui` UI with `HexoDSPUiPlugin`.
- Transport: `Play/Stop` work; BPM adjusts clock; Pause stub visible.
- Recording: `⏺` writes a playable WAV under `./recordings/`.
- Export: File menu produces WAV under `./exports/`.
- Mixer: Master volume/pan affect output; meters show activity.
- Live View: Crossfader and deck levels respond; scene buttons toggle state.
- Gesture: Mouse gestures mapped to transport and crossfader with visual hints.
- Node Graph: Add node, adjust parameter, load preset; audible change.
- Shaders/Visuals: Visual node placeholder renders; parameters persist.
- Microtuning: Load tuning table; apply to generator; pitches verify.
- UI State: Persists across restarts.
## Architecture Overview (Desktop Demo)

- `AudioIO` (CPAL) provides real-time audio input/output. Key methods:
  - `start_output_only()` initializes the output stream and runs the render callback.
  - `set_playback_enabled(bool)` gates audio in the callback.
  - Future: `start_recording(path)`, `stop_recording()`, `is_recording()`.
- `Transport` manages sample-accurate timing and states: `Playing`, `Stopped`, `Paused`, `Recording`.
- `AudioEngineBridge` mediates UI ↔ audio engine, carrying `AudioParamMessage` commands like `Play`, `Stop`, `Record`, `SetTempo`, `MasterVolume`.
- `HexoDSPEngine` orchestrates `AudioIO`, `Transport`, `NodeGraph`, and processing state feedback for UI.
- UI layers (`bevy_egui` / `egui_ui_full`) render views, menus, transport bar, mixer, and node canvas.

## Feature Details and Acceptance Criteria

### 1) Transport: Play/Stop/Pause
- Status: Implemented (Play/Stop). Pause stub planned.
- Usage: Click `▶` to play, `◼` to stop.
- Acceptance: Playback gate toggles; UI shows active state via feedback.
- Technical: `AudioParamMessage::Play`/`Stop` mapped to `Transport` and `AudioIO::set_playback_enabled`.

### 2) Record to WAV (Stereo Mix)
- Status: Planned in demo; implemented using `hound` crate.
- Usage: Click `⏺` to start/stop recording; file written under `./recordings/`.
- Acceptance:
  - Recording starts immediately when transport is enabled.
  - WAV file contains stereo mix at device sample rate and 16/24-bit.
  - Filename includes timestamp and BPM.
- Technical: Add `AudioIO::start_recording(path)`, `stop_recording()`, `is_recording()`; write frames in output callback with backpressure-safe buffer.

### 3) Export Current Mix to WAV
- Status: Planned (demo v1.0); offline bounce deferred.
- Usage: File menu → Export Mix; writes `./exports/session-<timestamp>.wav`.
- Acceptance: File exists with correct duration, sample rate, and bit depth.
- Technical: Reuse recording pipeline; optionally run for a fixed duration.

### 4) Master Volume and Pan
- Status: Implemented smoothing in `AudioIO`.
- Usage: Adjust master volume/pan sliders; audible changes without clicks.
- Acceptance: No zipper noise; stereo balance reflects pan.
- Technical: Smoothing via small ramp per buffer; atomic params in audio thread.

### 5) Master Toggles (Mute/Mono/Phase)
- Status: UI stubs; audio path wiring planned.
- Usage: Toggle buttons in master section.
- Acceptance: Mute silences output; Mono sums channels; Phase invert flips polarity.
- Technical: Implement lightweight DSP in output callback.

### 6) Input Monitoring with Gain
- Status: Implemented gating; gain control planned.
- Usage: Enable monitoring; adjust input gain; hear input over output.
- Acceptance: Monitoring respects mute and master level; no feedback loops.
- Technical: Mix input capture with output path safely.

### 7) Tempo (BPM)
- Status: Implemented `SetTempo(tempo)` wiring.
- Usage: Drag BPM control in transport bar.
- Acceptance: Transport state reflects BPM; future metronome aligns.
- Technical: `Transport::set_bpm` updates clock.

### 8) Loop Enable/Length
- Status: Planned simple loop region.
- Usage: Enable loop and set length; transport wraps.
- Acceptance: Playhead wraps predictably; no clicks.
- Technical: Transport loop flags with beat counters.

### 9) Arrangement View
- Status: Implemented basic timeline and zoom.
- Usage: Navigate timeline; position playhead.
- Acceptance: Position updates; scrubbing reflects transport.
- Technical: View state persists.

### 10) Live View (Scene/Deck/Crossfader)
- Status: UI present; demo controls planned to be active.
- Usage: Trigger scene buttons; use crossfader; control deck A/B.
- Acceptance: Visible feedback; crossfader affects deck levels.
- Technical: Map UI to `AudioParamMessage` routes and deck mixing.

### 11) Gesture Control
- Status: Integration planned via `gesture_integration.rs` and `hid_osc_bindings.rs`.
- Usage: Gestures for Play/Stop/Record; crossfader sweep.
- Acceptance: Recognized gestures trigger mapped actions reliably.
- Technical: Device bindings via HID/OSC; fallback mouse gesture layer.

### 12) Node Graph Basics
- Status: Implemented canvas and nodes; parameter editing present.
- Usage: Add generator/effect nodes; connect; tweak parameters.
- Acceptance: Audible changes through the master bus.
- Technical: `NodeInstanceManager` routes `AudioParamMessage` to nodes.

### 13) Preset Loading
- Status: Demo preset library available.
- Usage: Load presets from library for quick sound setup.
- Acceptance: Preset parameters apply to nodes; sound changes accordingly.
- Technical: Preset manager binds to node graph types.

### 14) File Browser
- Status: Implemented navigation and selection.
- Usage: Browse sample library; select files; set paths.
- Acceptance: Navigating directories reflects on-screen; selected file tracked.
- Technical: `UiState` manages current path and file list.

### 15) UI Persist/Load
- Status: Implemented `ui_state.json` persist.
- Usage: Exit and restart; state restored.
- Acceptance: Transport visibility, view mode, and theme persist.
- Technical: Serde save/load on Bevy app init/shutdown.

### 16) Mixer (Channels + Master)
- Status: UI scaffolding; master meters planned.
- Usage: Adjust channel volume/pan; see peak meters.
- Acceptance: Meters show activity; sliders affect output.
- Technical: Processing state feeds UI levels.

### 17) EQ/Dynamics Toggles
- Status: UI toggles; audio wiring planned.
- Usage: Enable/disable master EQ/compressor/limiter.
- Acceptance: Audible change; toggles reflect state.
- Technical: DSP nodes or inline processors.

### 18) Track Arm Flags
- Status: UI flags; audio routing planned.
- Usage: Arm tracks for future multitrack record.
- Acceptance: UI reflects armed state; no unintended record.
- Technical: Routing reserved for vNext.

### 19) Piano Roll Placeholder
- Status: Present with grid resolution options.
- Usage: Open piano roll; change grid.
- Acceptance: UI responds; data structure persists.
- Technical: Editor scaffolding ready for note data.



## Transport, Audio IO, and Recording Design

### Transport → AudioIO Gating
- `Play` sets transport to `Playing` and calls `AudioIO::set_playback_enabled(true)`.
- `Stop` sets transport to `Stopped` and gates output (`false`).
- Playback gating is implemented in CPAL output callback for minimal latency.

### WAV Recording (Demo v1.0)
- Implement `AudioIO::start_recording(file_path)` using `hound` WAV writer.
- Write stereo frames in output callback after gating and processing.
- Guard with `is_recording` atomic flag; buffer writes to avoid blocking.
- Stop via `AudioIO::stop_recording()`; flush and close file.
- File naming: `recordings/hexodsp_<YYYYMMDD_HHMMSS>_<bpm>.wav`.
- Bit depth: 24-bit PCM preferred; fallback to 16-bit for compatibility.

### Export Mix
- Reuse the recording pipeline with a temporary session flag.
- Trigger from File menu; open path picker (future); default to `exports/`.

## Live View Plan (Demo)

- Controls: Scene buttons, deck A/B volume, crossfader, EQ knobs.
- Actions mapped to engine:
  - Scene buttons → play/pause clip routes (stub in demo).
  - Crossfader → mix coefficient between deck buses.
  - Deck volume → per-deck gain.
- Acceptance Tests:
  - Crossfader visibly affects deck levels.
  - Scene trigger changes visual state; optional audio cue.
  - No audio dropouts during rapid gestures.

## Gesture Control Plan (Demo)

- Integrations: `gesture_integration.rs`, `hid_osc_bindings.rs`.
- Gesture Map:
  - Swipe right: `Play`.
  - Swipe left: `Stop`.
  - Long press: `Record` toggle.
  - Vertical sweep: crossfader A↔B.
- Acceptance Tests:
  - 90%+ recognition accuracy for primary gestures.
  - No accidental triggers during normal mouse use.
- Fallback: Mouse gesture layer with visual hints in Live View.

### 12.1) Shader Visuals (NUWE/ISF)
- Status: UI present in Node View with `nuwe_shaders` and `isf_plugins` scaffolding.
- Usage: Add a visual node/preset; adjust parameters; link to audio reactivity.
- Acceptance: Visual nodes render stubs; parameters persist; no engine instability.
- Technical: Bind shader params to `AudioParamMessage` and feedback.

### 12.2) Microtuning
- Status: Planned; demo includes UI hooks and import dialog placeholder.
- Usage: Load `.scl`/`.kbm`; apply to generator nodes; test reference pitches.
- Acceptance: Pitch tables applied consistently; transport timing unaffected.
- Technical: Tuning table applied per voice; MPE compatible.

## Demo Script (Reviewer Walkthrough)

1. Launch app; confirm window title “HexoDSP DAW”.
2. Open transport bar; click `▶` to start playback; adjust master volume/pan.
3. Enable input monitoring; speak/clap to see level activity.
4. Click `⏺` to start recording; perform simple tweaks; click `◼` to stop.
5. Verify `recordings/*.wav` exists and plays back in an external player.
6. Switch to Live View; move crossfader; adjust deck volumes; observe feedback.
7. Load a preset; tweak node parameters; hear changes.
7a. Add a shader visual node; adjust parameters; observe visual feedback.
7b. Load a microtuning table; audition generator pitches.
8. Use File menu to export mix; verify `exports/*.wav` exists.
9. Quit and relaunch; confirm UI state persisted.

## Performance and Stability Baseline

- Audio buffer sizes: prefer 128–512 frames per callback.
- CPU budget: transport and basic mixing under 10% on mid-tier hardware.
- Avoid blocking in callback; use lock-free or preallocated buffers.
- Logging: minimal in audio thread; batch to UI/bridge.

## Roadmap to Full DAW Documentation (100+ Pages)

- Detailed chapters for editing, routing, plugins, automation, collaboration.
- Deep reference for every node type and DSP module.
- Full troubleshooting and hardware compatibility matrix.
- Developer guide for extending nodes, presets, and UI modules.

## Risks and Mitigations

- Risk: Recording in callback could block. Mitigation: ring buffer and writer thread.
- Risk: Gesture false positives. Mitigation: thresholds and confirmation patterns.
- Risk: Device sample format mismatch. Mitigation: format conversion layer.
- Risk: Feature creep. Mitigation: hard demo scope with acceptance criteria.

## Appendix: Paths and Commands

- Run Desktop: `cargo run --release`
- Recording Output: `./recordings/`
- Export Output: `./exports/`
- Config: UI state file `ui_state.json` in project root.
- Key Files:
  - `src/audio_engine/cpal_io.rs`: AudioIO and stream callbacks.
  - `src/audio_engine/mod.rs`: Engine orchestrator and state feedback.
  - `src/audio_engine/transport.rs`: Transport state and timing.
  - `src/ui/egui_ui_full.rs`: Full UI, transport bar, views.
  - `src/ui/bevy_egui_ui.rs`: Bevy plugin initialization.

---

This demo plan defines the minimally “full enough” experience to represent a professional DAW in a compact format while building toward the complete product. It includes the required 10–20 feature set and the recording/export, Live View, and Gesture Control paths that reviewers can exercise today and that engineering can expand rapidly.