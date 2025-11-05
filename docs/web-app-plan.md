# HexoDSP Web App Plan (Post-Demo, Elaborate)

This document defines a comprehensive plan to build and evolve the HexoDSP Web App in parallel with the desktop application, focusing on browser-based UI, WebGL visualizations, Web Audio integration, Web MIDI/MPE, and remote preview integrations. It complements the desktop-only plan and is intended to guide development over the next 4–8 weeks.

## Executive Summary
- Objective: Deliver a “full enough” web app that mirrors core DAW workflows: transport, basic recording/export pathways (remote), mixer, node graph editing, Live View, gesture mapping, visualizations, and basic MIDI/MPE and microtuning hooks.
- Scope: Browser-based client with HTML/CSS/JavaScript and WebGL; optional Web Audio and Web MIDI; flexible backend integration for audio engine via remote bridge.
- Outcomes: A web UI that allows users to preview and control DAW workflows, trigger visuals, and interact with transport/mixer/node graph with acceptable latency, while deferring heavy DSP to desktop/server where needed.

## Guiding Principles
- Web-first UX clarity: fast loading, responsive panels, strong visual feedback.
- Progressive enhancement: fallbacks where APIs are unavailable.
- Latency-aware: keep time-critical audio on desktop/server when needed; use remote control for UI and visualization.
- Accessibility: keyboard navigation, ARIA roles, color contrast.

## Non-Goals (Initial Release)
- Full browser-native multitrack audio recording and offline bounce.
- Complete plugin hosting in-browser.
- Cloud collaboration and project sharing.

## Architecture Overview (Web)
- UI Layers:
  - `web/index.html`: main entry
  - `web/js/main.js`: app bootstrap and router
  - `web/js/ui-manager.js`: view toggles and layout management
  - `web/js/arrangement-view.js`: timeline and playhead
  - `web/js/live-view.js`: scenes, decks, crossfader
  - `web/js/node-view.js`: node graph canvas and parameter panels
  - `web/js/webgl-visualizations.js`: shader effects and meters
  - `web/js/audio-engine.js`: browser-side audio hooks and remote bridge
  - `web/js/bevy-integration.js`: optional interop
  - `web/js/realtime-audio-ui.js`: transport/mixer UI bindings
  - `web/js/motion-capture.js`: gesture/motion inputs
  - `web/js/arrangement-view.js`: timeline helpers
  - `web/js/accessibility.js`: A11y utilities
- Visuals: WebGL 2.0 shaders, ISF/NUWE-compatible parameter model.
- Audio Integration: Web Audio API for simple synths/meters; remote control for full DSP via desktop engine.
- MIDI/MPE: Web MIDI API pathways with per-note expression mapping.
- Remote Bridge Options:
  - WebSocket → desktop engine for transport/mixer control and feedback.
  - REST/WebRTC for asset management and streaming (future).

## Feature Matrix (Web)
1. Transport (Play/Stop/Pause, BPM, Loop)
2. Mixer (Master volume/pan; meters)
3. Node Graph (Add/connect/edit; presets)
4. Arrangement View (Timeline, zoom, playhead)
5. Live View (Scenes, decks, crossfader, EQ knobs)
6. Gesture Control (mouse/touch; motion-capture integration)
7. Visualizations (WebGL shaders, audio-reactive meters)
8. Web Audio (simple oscillators, filters; meter sources)
9. Web MIDI/MPE (device mapping, per-note expressions)
10. Microtuning (Scala `.scl`/`.kbm` loaders; UI hooks)
11. File Browser (assets/presets; drag/drop into views)
12. Persistence (localStorage/sessionStorage; settings)
13. Accessibility (keyboard navigation, ARIA, contrast)

## Timeline (4 Weeks)

### Week 1: Transport, Layout, and Visual Baseline
- Deliverables:
  - Implement transport bar with Play/Stop/Pause and BPM control.
  - Establish view layout: Arrangement, Live, Node; responsive panels.
  - Wire meters and basic visualization canvas (WebGL init).
- Acceptance:
  - Controls respond instantly; BPM updates UI and timers.
  - Views render correctly across common desktop browsers.
  - Visual canvas displays meters and a simple shader effect.

### Week 2: Node Graph, Presets, and Mixer
- Deliverables:
  - Node graph canvas: add/connect/remove; parameter panels.
  - Preset loader for generators/effects; UI interactions.
  - Mixer UI: master volume/pan; level meters.
- Acceptance:
  - Node graph operations are stable; parameter changes visible.
  - Presets load and apply; no console errors.
  - Mixer updates reflect in UI; meters animate.

### Week 3: Live View, Gesture Control, and Web Audio
- Deliverables:
  - Live View: scenes, decks, crossfader, EQ knob UI.
  - Gesture Control: touch/mouse gestures; motion-capture hooks.
  - Web Audio: simple oscillator/filter nodes for local preview.
- Acceptance:
  - Crossfader and deck controls provide smooth UX.
  - Gestures trigger mapped actions reliably (≥90%).
  - Web Audio nodes run with acceptable CPU and stable timing.

### Week 4: MIDI/MPE, Microtuning, Remote Bridge + QA
- Deliverables:
  - Web MIDI/MPE: device detection; per-note expression mapping.
  - Microtuning: `.scl`/`.kbm` loaders; apply tuning tables to preview nodes.
  - Remote bridge: WebSocket control path to desktop engine for transport/mixer; processing state feedback.
  - QA: performance, accessibility, and documentation.
- Acceptance:
  - MPE mapping demonstrable; timing aligned.
  - Tuning tables apply correctly; preview pitches validate.
  - Remote control works end-to-end; UI reflects engine state.

## Detailed Feature Plans

### Transport (Web)
- Status: Implement via `realtime-audio-ui.js` and `audio-engine.js`.
- Usage: Play/Stop/Pause buttons; BPM drag; loop toggle.
- Acceptance: Immediate UI response; timers and playhead sync.
- Technical: `requestAnimationFrame` timers; optional remote sync.

### Mixer (Web)
- Status: UI sliders; meter animation via audio/visual sources.
- Usage: Master volume/pan; meter visualization.
- Acceptance: Smooth UI updates; predictable levels.
- Technical: Gain/panner nodes for preview; remote engine mapping.

### Node Graph (Web)
- Status: Canvas rendering and interactions with drag/select/connect.
- Usage: Add generators/effects; connect; adjust parameters.
- Acceptance: Stable operations; parameter UI updates.
- Technical: Internal graph model; sync with desktop presets via JSON.

### Arrangement View (Web)
- Status: Timeline and playhead UI; zoom/pan.
- Usage: Position playhead; view clips (placeholder).
- Acceptance: Smooth scrolling; playhead reflects transport time.
- Technical: Canvas/SVG renderer; state persistence.

### Live View (Web)
- Status: Scene/deck buttons; crossfader; EQ knobs.
- Usage: Performance panel actions; visual feedback.
- Acceptance: Responsive controls; no UI stutter.
- Technical: Map controls to preview audio and remote engine params.

### Gesture Control (Web)
- Status: Mouse/touch gestures; motion-capture hooks.
- Usage: Swipe/press gestures for transport; sweep for crossfader.
- Acceptance: ≥90% recognition accuracy; visual hints.
- Technical: Pointer events; threshold/debounce; optional OSC.

### Visualizations (WebGL)
- Status: Shader pipeline; meters; ISF/NUWE-like parameter model.
- Usage: Load shader presets; bind parameters; audio-reactive visuals.
- Acceptance: Stable rendering; adjustable parameters persist.
- Technical: WebGL 2.0; audio analysis via AnalyserNode; parameter binding.

### Web Audio (Preview)
- Status: Oscillator/filter chains; analyser for meters.
- Usage: Local preview of synth/effect nodes.
- Acceptance: Accurate pitch/tone; stable timing.
- Technical: Web Audio Graph; safe sample-rate handling.

### Web MIDI/MPE
- Status: Device enumeration; note-on/off with MPE mapping.
- Usage: Bind device; play notes; adjust per-note expressions.
- Acceptance: Minimal latency; consistent mapping.
- Technical: Web MIDI API; scheduling aligned with UI timers.

### Microtuning (Web)
- Status: `.scl`/`.kbm` parsing; tuning table application.
- Usage: Load tuning files; audition pitches with preview nodes.
- Acceptance: Pitches validate; UI state persists.
- Technical: Tuning tables per voice; preview oscillators honor tuning.

### File Browser (Web)
- Status: Asset/preset browsing; selection and drag/drop.
- Usage: Navigate collections; apply presets to views.
- Acceptance: Smooth navigation; no broken links.
- Technical: JSON-backed collections; caching strategies.

### Persistence & Settings (Web)
- Status: localStorage/sessionStorage-based persistence.
- Usage: Save theme/layout; restore on reload.
- Acceptance: Reliable restore; schema versioning.
- Technical: Namespaced keys; migration policy.

### Accessibility (Web)
- Status: Keyboard navigation; ARIA roles; contrast modes.
- Usage: Navigate without mouse; high-contrast mode.
- Acceptance: WCAG AA baseline; keyboard-only usability.
- Technical: Tab order; roles; focus management.

## Integration with Desktop
- Remote control targets: transport, mixer master, node parameters, processing state feedback.
- Discovery: desktop app exposes WebSocket endpoint; web client connects on launch.
- Security: local-only; optional token; CORS policies configured.

## Performance & QA
- Targets: <16ms frame budget; responsive controls; minimal layout thrash.
- Testing: unit tests for parsers; integration tests for UI flows; manual acceptance.
- Profiling: use browser devtools; track frame times and GC pauses.

## Risks & Mitigations
- Web Audio limitations → use remote engine for heavy DSP.
- Device/API variance → progressive enhancement and fallbacks.
- Latency sensitivity → align scheduling and debouncing; offer desktop handoff.

## Paths & Commands (Web)
- Entry: `web/index.html`
- Assets: `web/css/`, `web/js/`
- Local preview: serve `web/` via a static server (e.g., `python -m http.server 8080`), then open `http://localhost:8080/`.
- Build: optional bundler step for production (Vite/ESBuild/Rollup).

## Roadmap (Beyond 4 Weeks)
- Collaboration and cloud sync options.
- Advanced automation editor and clip management.
- Plugin host integrations; unified preset ecosystems.
- WebRTC streaming for remote audio/visual sync.

---

This web plan provides a clear, step-by-step pathway to bring the browser-based experience to parity with the desktop demo where appropriate, while respecting the technical boundaries of web technologies and latency constraints. It is designed to be developed alongside the desktop app so teams can choose the most effective platform for each milestone.

## Architecture Deep Dive (Web)
- Rendering pipeline: HTML/CSS layout + WebGL canvas; modular panels for Arrangement, Live, Node.
- State management: central store in `ui-manager.js`; view-specific controllers; event bus for cross-view sync.
- Timing model: UI timers with `requestAnimationFrame`; transport sync via WebSocket to desktop engine when available.
- Audio preview: Web Audio nodes for local oscillator/filter and meter analysis; keep heavy DSP remote.
- Data layer: presets and settings via JSON; collections cached locally with version tags.

## WASM Integration Strategy
- Rust → Wasm via `wasm-bindgen` (optional), compiled audio utilities and parsers.
- Audio constraints: AudioWorklet preferred for low-latency processing; SharedArrayBuffer gated by COOP/COEP.
- Fallbacks: run preview-only nodes without Worklet when SAB is unavailable; defer heavy processing.
- Bridge: structured messages between JS and Wasm for parameter updates and analysis results.

## Browser Compatibility Matrix
- Target: Chrome (latest), Edge (latest), Firefox (latest), Safari (latest).
- WebGL: prefer WebGL 2.0; detect and gracefully degrade if only WebGL 1.0 is available.
- Web MIDI: enable via flags where supported; fallback UI for unsupported browsers.
- Worklet/SAB: feature detection; disable advanced preview when unsupported.

## Security Model & Policies
- Permissions: explicit user consent for MIDI devices; clear UI prompts.
- CSP: restrict script sources; use strict MIME types; avoid inline scripts in production.
- CORS: local-only endpoints by default; configurable for remote engine in trusted environments.
- Storage: use namespaced localStorage/sessionStorage keys; sanitize JSON payloads.

## Persistence & Project Compatibility
- Settings: theme, layout, view preferences saved to localStorage.
- Presets: JSON modules compatible with desktop presets; simple import/export flows.
- Projects: read-only preview for desktop project files; web exports organized under `web/exports/` during dev.

## UI Component Inventory (Web)
- Transport bar: play/stop/pause, BPM control, loop toggle, indicators.
- Meters: master/track meters in canvas; peak/RMS visualizations.
- Mixer: volume/pan; expand/collapse tracks; simple routing selector.
- Node graph: canvas with drag/select/connect, parameter inspector, preset loader.
- Arrangement: timeline with zoom/pan; clip placeholders.
- Live View: scenes, decks, crossfader, EQ knobs, visual feedback.
- File browser: pane with asset/preset lists; drag/drop to views.
- Settings: modal with theme, accessibility, device flags.

## Detailed Week-by-Week (Daily Tasks)

### Week 1 (Transport, Layout, Visual Baseline)
- Day 1: Bootstrap `main.js` and router; establish panel layout.
- Day 2: Implement transport controls; BPM input and validation.
- Day 3: Playhead rendering in Arrangement; timer integration.
- Day 4: WebGL init; draw meters and basic shader effect.
- Day 5: Accessibility pass: keyboard navigation and ARIA roles.
- Day 6: Performance audit and micro-optimizations; reduce layout thrash.
- Day 7: QA and documentation; screenshots and usage notes.

### Week 2 (Node Graph, Presets, Mixer)
- Day 8: Node graph canvas interactions; selection and connections.
- Day 9: Parameter inspector; bind sliders to graph parameters.
- Day 10: Preset loader; JSON schema and import/export.
- Day 11: Mixer UI with meters; gain/pan smoothing in preview.
- Day 12: Asset browser; drag/drop integration with views.
- Day 13: Error handling and recovery flows; console log hygiene.
- Day 14: QA; acceptance checks for stability and UX.

### Week 3 (Live View, Gesture, Web Audio)
- Day 15: Live View scenes and deck controls; crossfader coefficient.
- Day 16: Gesture mapping and visual hints; touch and mouse support.
- Day 17: Web Audio oscillators/filters; analyser for meters.
- Day 18: Device flags and feature detection; progressive enhancement flows.
- Day 19: Motion-capture stubs; event pipeline and safety.
- Day 20: Accessibility improvements; contrast modes and focus management.
- Day 21: QA; record demo run; performance profiling.

### Week 4 (MIDI/MPE, Microtuning, Remote Bridge, QA)
- Day 22: Web MIDI device discovery and mapping UI.
- Day 23: Per-note expression pathways; timing alignment strategies.
- Day 24: `.scl`/`.kbm` parsing; tuning table application to preview nodes.
- Day 25: WebSocket remote control to desktop engine; state feedback in UI.
- Day 26: Error recovery for remote disconnects; reconnection strategies.
- Day 27: Accessibility and performance polish; bug triage.
- Day 28: Documentation and review; screenshots and usage.

## Acceptance Criteria (Consolidated)
- Transport: responsive controls and accurate playhead with loop flags.
- Node graph: stable interactions; parameters persist; presets load/apply.
- Mixer: smooth volume/pan; meters animate consistently.
- Live View: crossfader responsive; scenes and deck controls work.
- Gesture: ≥90% accuracy; minimal false positives; clear visual hints.
- Web Audio: preview nodes stable; CPU usage acceptable; meters correct.
- Web MIDI/MPE: device mapping and per-note expression demonstrable.
- Microtuning: tuning tables apply and audit; persisted settings restore.
- Remote bridge: end-to-end control path to desktop engine; robust disconnect handling.
- Accessibility: WCAG AA baseline; keyboard-only navigation viable.

## Testing & QA (Expanded)
- Unit tests: parsers for tuning and presets; utilities in `audio-engine.js`.
- Integration tests: UI flows with Playwright/Cypress; transport/mixer/node interactions.
- Performance tests: frame budget and GC pauses; animation stutter checks.
- Manual acceptance: demo script; screenshots; reviewer notes.

## Deployment & Packaging
- Dev server: static hosting for `web/` directory; optional bundler.
- Production: bundle JS/CSS; hash assets; CSP-compliant builds.
- PWA: installable app (optional); offline caches for UI assets (no audio processing offline).

## Risk Register (Expanded)
- Browser API variance: feature detection and fallback layer.
- Security/CSP constraints: strict policies; avoid inline scripts.
- Latency variability: keep heavy DSP off-browser; remote engine.
- Accessibility gaps: regular audits and fixes; keyboard testing.
- Scope growth: maintain non-goals; progressive enhancement only.

## Glossary (Web)
- WebGL: GPU-accelerated graphics API for browsers.
- AudioWorklet: browser audio processing with real-time constraints.
- SharedArrayBuffer (SAB): shared memory for low-latency audio; gated by COOP/COEP.
- Web MIDI: browser API for MIDI device access.

## Appendices
- Paths: `web/index.html`, `web/js/*`, `web/css/*`.
- Build: bundler options and commands; environment flags for feature toggles.
- Runbooks: troubleshooting device access, CSP issues, and remote bridge failures.

---

# UI State Machines & Router Specs
- Router: hash-based navigation across `Arrangement`, `Live`, and `Node` views.
- States: `Idle`, `Playing`, `Paused`, `Disconnected`, `Connected` (remote), `Loading`.
- Transitions: play/pause/stop; remote connect/disconnect; view switching with persistence.
- Guards: prevent conflicting actions during `Loading`; debounce rapid toggles.

# Component Contracts
- TransportBar: inputs (BPM, loop, commands) → events (play, pause, stop, setTempo).
- MixerPanel: inputs (gain/pan) → events (changeGain, changePan).
- NodeGraphCanvas: inputs (nodes, edges) → events (addNode, connectNodes, setParam).
- ArrangementTimeline: inputs (clips, playhead) → events (setPlayhead, zoom, scroll).
- LiveViewPanel: inputs (scenes, decks) → events (triggerScene, setCrossfader).
- FileBrowser: inputs (assets, presets) → events (applyPreset, importAsset).

# Event Bus & Subscriptions
- Bus channels: `transport`, `mixer`, `nodeGraph`, `live`, `assets`, `remote`.
- Subscriptions: components register handlers; lifecycle attach/detach on view enter/exit.
- Backpressure: batch UI updates; coalesce repetitive parameter changes.

# Error Handling & Recovery
- UI errors: toast notifications with actionable tips; log hygiene.
- Remote disconnects: exponential backoff; manual reconnect button.
- Parser failures: validate JSON schema; show diff and invalid fields.

# Accessibility Deep Dive
- Keyboard-only flows for transport/mixer/node graph; focus management and ARIA roles.
- Screen-reader labels for controls; skip links and headings structure.
- Contrast modes and theme tokens; ensure WCAG AA.

# Internationalization (I18N) Plan
- String catalogs: JSON with keys; language switcher UI.
- Date/time/number formatting with locale-aware utilities.
- Future: RTL layout support; plugin for direction flipping.

# Visual Design Tokens
- Colors: primary, secondary, background, accent; semantic roles.
- Typography: font scales and weights; monospace for code-like values.
- Spacing: unit grid; responsive breakpoints.
- Components: button/slider/toggle defaults and interactive states.

# Testing Framework & Coverage
- Unit: Jest/Vitest for utilities and parsers.
- Integration: Cypress/Playwright for end-to-end flows.
- Coverage targets: 70%+ on core modules; prioritize parsers and router.

# DevOps & CI/CD Pipeline
- Linting and tests on PRs; build artifacts for production bundles.
- Static hosting deployment; environment flags for feature toggles.
- Monitoring: optional error reporting with user opt-in; anonymized metrics (if enabled).

# Detailed Backlog (Web)
- Node graph multi-select and group operations.
- Automation lanes preview for select parameters.
- Drag/drop clip editing in Arrangement.
- Preset marketplace integration (future).
- PWA install polish and offline caches for UI assets.

# Design Rationale Notes (Web)
- Web-first progressive enhancement ensures graceful degradation across browsers.
- Remote bridge offloads heavy DSP to the desktop engine, minimizing latency constraints.
- Local preview nodes retain UX feedback without claiming full DAW parity in-browser.

# Open Questions & Decisions (Web)
- Router strategy: hash-based vs history API for production; current choice is hash for simplicity.
- Worklet/SAB gating: how aggressively to enable advanced preview features.
- Remote bridge authentication: token-based and local-only defaults.

# Troubleshooting Guide (Web)
- No MIDI access: confirm browser support; enable flags; reconnect devices.
- Visual stutter: reduce shader complexity; profile frame times; disable excessive DOM updates.
- Remote connection failures: check desktop endpoint; CORS and CSP policies.

## Router Lifecycle & View Contracts (Expanded)
- Routes: `#/arrangement`, `#/live`, `#/node`, `#/settings`.
- Lifecycle hooks: `onEnter`, `onExit`, `onResume` per view.
- Persistence: store last route in `localStorage` under `hexodsp.route`.
- Guarded transitions: prevent `onExit` during modal dialogs; debounce rapid toggles.

Contracts:
- Arrangement: owns `playhead`, `zoom`, `clipsPreview[]`.
- Live: owns `scenes[]`, `decks{A,B}`, `crossfader`, `eqKnobs`.
- Node: owns `nodes[]`, `edges[]`, `inspectorParams`, `selected[]`.
- Settings: owns `theme`, `accessibility`, `deviceFlags`.

## Component Props/Events (Detailed)

TransportBar
- Props: `bpm:number`, `loop:{start:number,end:number}`, `isPlaying:boolean`, `isPaused:boolean`.
- Events: `play`, `pause`, `stop`, `setTempo(number)`, `setLoop({start,end})`.

MixerPanel
- Props: `gain:number`, `pan:number`, `meters:{peak:number,rms:number}`.
- Events: `changeGain(number)`, `changePan(number)`.

NodeGraphCanvas
- Props: `nodes:Node[]`, `edges:Edge[]`, `selected:Id[]`.
- Events: `addNode(NodeDef)`, `connectNodes({src,dst})`, `setParam({id,key,value})`, `deleteSelection()`.

ArrangementTimeline
- Props: `clips:Clip[]`, `playhead:number`, `zoom:number`.
- Events: `setPlayhead(number)`, `zoomTo(number)`, `scrollTo(number)`.

LiveViewPanel
- Props: `scenes:Scene[]`, `decks:{A:Deck,B:Deck}`, `crossfader:number`.
- Events: `triggerScene(id)`, `setCrossfader(number)`, `setDeckParam({deck,key,value})`.

FileBrowser
- Props: `assets:Asset[]`, `presets:Preset[]`.
- Events: `applyPreset(id)`, `importAsset(File)`, `deleteAsset(id)`.

## WebSocket Protocol (Desktop Bridge)
- URL: `ws://localhost:7777/hexodsp`
- Outbound messages (web → desktop):
  - `{"type":"transport","cmd":"play"}`
  - `{"type":"transport","cmd":"pause"}`
  - `{"type":"transport","cmd":"setTempo","bpm":128}`
  - `{"type":"mixer","cmd":"setGain","value":0.8}`
  - `{"type":"node","cmd":"setParam","id":"node-osc","key":"freq","value":440}`
- Inbound messages (desktop → web):
  - `{"type":"state","transport":{"playing":true,"bpm":128}}`
  - `{"type":"meters","master":{"peak":-6.2,"rms":-12.5}}`
  - `{"type":"graph","snapshot":{...}}`

Error handling:
- Retry with exponential backoff; show status banner.
- Queue user commands when disconnected; flush on reconnect.

## I18N Catalogs (Examples)
Strings JSON:
```
{
  "transport.play": "Play",
  "transport.pause": "Pause",
  "transport.stop": "Stop",
  "transport.bpm": "BPM",
  "mixer.gain": "Gain",
  "mixer.pan": "Pan",
  "live.crossfader": "Crossfader",
  "node.add": "Add Node",
  "file.import": "Import",
  "settings.title": "Settings"
}
```

Layout considerations:
- Support LTR/RTL via `dir` attribute; mirror icons where needed.
- Date/number formatting via Intl API; avoid custom parsers.

## CI/CD Pipeline (Detailed)
- Pre-commit: lint (ESLint), format (Prettier), type checks (TypeScript optional).
- CI steps: install deps, run tests, build production bundle, upload artifacts.
- Security: `npm audit` gating; CSP validation tests.
- Deployment: static hosting (GitHub Pages/Netlify); cache-busting with hashed assets.

## Security & CSP Configuration
- CSP example:
```
Content-Security-Policy: default-src 'self';
  script-src 'self'; style-src 'self' 'unsafe-inline';
  img-src 'self' data:; connect-src 'self' ws://localhost:7777;
  worker-src 'self'; frame-ancestors 'none';
```
- Avoid inline scripts in production; use strict MIME types for modules.
- CORS: accept local origins only; configurable for trusted networks.

## PWA & Offline Behavior
- Optional PWA manifest with icons and name.
- Service Worker:
  - Cache UI assets (`index.html`, CSS, JS bundles).
  - Do not cache remote audio or state; network-only for bridge.
- Offline mode: limited UI preview with disabled remote features.

## Performance Budgets & Profiles
- Frame budget: 16ms target; animations under 8ms work.
- Shader complexity: cap uniforms/loops; test on mid-tier GPUs.
- GC hygiene: avoid large transient arrays; reuse buffers.
- DevTools profiles stored under `automation_reports/` for trending.

## Device/API Variance Strategies
- Feature detection gates: WebGL2, Web MIDI, Worklet, SAB.
- Progressive enhancement: degrade gracefully; show capability banners.
- Settings toggles: enable/disable advanced features with clear warnings.

## Expanded Troubleshooting Matrix
- WebGL init fails: check extensions; fallback to 2D canvas meters.
- MIDI device not listed: verify permissions; replug; browser restart.
- High CPU usage: reduce shader params; disable background tabs; limit analyser FFT.
- Stale UI after reconnect: force state resync; clear caches.

## Accessibility Audit Checklist (Web)
- Keyboard-only navigation verified for transport/mixer/node graph.
- ARIA roles present for interactive controls and landmarks.
- Focus order logical; visible focus rings.
- Contrast ratios meet WCAG AA; high-contrast theme available.
- Skip links to main content and key panels.
- Tooltips and labels assistive-friendly; avoid title-only.

## UI Performance Cookbook (Web)
- Minimize layout thrash: batch DOM writes; use `requestAnimationFrame`.
- Canvas/WebGL: reuse buffers; avoid per-frame allocations.
- Event coalescing: debounce sliders/gestures at 60–120 Hz.
- Avoid `querySelectorAll` in hot paths; cache handles.
- Use `transform` for animations; avoid layout-triggering properties.
- Profile: record frame times; GC pauses; shader time; network costs.

## Browser Feature Matrix by Module
- Transport/Mixer: all modern browsers; no special APIs.
- Node Graph Canvas: WebGL2 preferred; fallback to Canvas2D.
- Web Audio Preview: Chrome/Edge/Firefox stable; Safari quirks.
- Web MIDI: Chrome-based; Edge supports; Firefox/Safari partial or behind flags.
- AudioWorklet: Chrome/Edge stable; Firefox partial; Safari varies.
- SharedArrayBuffer: gated by COOP/COEP; Chrome/Edge; others limited.

## E2E Test Suite Case List (Web)
- Transport: play/pause/stop; BPM set; loop toggle; playhead sync.
- Mixer: gain/pan changes; meters animate; values persist.
- Node Graph: add/connect/delete; parameter edits; preset apply.
- Arrangement: zoom/pan; scroll performance; playhead render.
- Live View: scene trigger; crossfader behavior; deck params.
- Gesture: swipe/touch mapping accuracy; debounce effectiveness.
- Web Audio: oscillator pitch correctness; analyser meters.
- MIDI: device detection; note-on/off; per-note expression mapping.
- Microtuning: `.scl`/`.kbm` parsing; tuning table application.
- Remote Bridge: connect/disconnect; state feedback; command queue.
- Accessibility: keyboard-only navigation; focus management; ARIA validation.

## Runbooks & SOPs (Web)
- Remote disconnect: show banner; retry with backoff; manual reconnect.
- CSP violation: check headers; disable inline scripts; module MIME types.
- MIDI access denied: prompt user; open settings; re-request permission.
- Shader instability: reduce uniforms; disable heavy passes; lower resolution.
- Performance spike: enable perf overlay; limit analyser FFT; disable background tabs.

## Detailed Backlog Additions (Web)
- Node Graph multi-select: box selection, group operations, alignment guides.
- Automation lanes preview: draw lanes for selected params; simple keyframes.
- Arrangement clip editing: drag/create clip placeholders; snap-to-grid.
- Preset import/export UI: drag JSON; validate schema; show diff on error.
- Feature flags panel: toggle WebGL2/Worklet/MIDI; persist settings.
- Performance overlay: frame time, GC, shader timing; enable/disable.
- Device capability banner: show detected features; guide users on limitations.
- Error boundary components: catch UI exceptions; show recovery dialogs.

## Security Hardening Details
- Hash-based assets: `link rel="preload"` for critical bundles.
- Subresource Integrity (SRI): add `integrity` attributes; verify during build.
- Strict MIME types: serve `.js` as `text/javascript` or `application/javascript`.
- Sandbox iframes (if used): `sandbox="allow-scripts"`; avoid untrusted content.
- HTTPS-only for production; HSTS header.

## Deployment Profiles
- Dev: verbose logs; source maps; relaxed CSP for local testing.
- Staging: minified bundles; CSP close to production; feature flags enabled.
- Production: hashed bundles; strict CSP; error reporting opt-in.


## Extended I18N Examples
Strings (French sample):
```
{
  "transport.play": "Lecture",
  "transport.pause": "Pause",
  "transport.stop": "Arrêter",
  "transport.bpm": "BPM",
  "mixer.gain": "Gain",
  "mixer.pan": "Panoramique",
  "live.crossfader": "Crossfader",
  "node.add": "Ajouter un nœud",
  "file.import": "Importer",
  "settings.title": "Paramètres"
}
```

Layout RTL note:
- Ensure icons and sliders mirror; validate with Arabic/Hebrew locales.