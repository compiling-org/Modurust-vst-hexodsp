# HexoDSP DAW Roadmap

## Overview
- Focus: Stabilize Bevy+egui UI, wire audio engine, deliver usable DAW workflows.
- Reference: MeadowlarkDAW/Meadowlark for audio engine patterns (UI not adopted).

## Phase A — UI Reintegration (Weeks 1–2)
- Restore full Arrangement/Live/Node views via `eframe_ui_full::ui_system`.
- Persist `UiState` and theme selections; load/save to `ui_state.json`.
- Implement preset browser with real content and drag/drop actions.
- Acceptance: Complex UI renders and responds; state persists across runs.

## Phase B — Audio Wiring (Weeks 2–4)
- Define bridge messages for transport, parameters, clip ops.
- Bind transport controls to `transport_sync`; add meters and latency display.
- Connect node/device creation to `NodeInstanceManager` and `AudioEngineBridge`.
- Acceptance: Play/stop works, meters update, no audio thread violations.

## Phase C — Plugins & Devices (Weeks 4–6)
- VST3 scan and host surface; load/unload and parameter listing.
- Device chain UI with automation stubs; routing overview.
- Acceptance: Plugins load, parameters visible, basic automation recorded.

## Phase D — Performance & Polish (Weeks 6–8)
- Optimize UI redraws and egui usage; reduce allocations in audio path.
- Accessibility pass; theming consistency; input handling cleanup.
- Acceptance: Stable operation under load, target latency <10ms.

## Tracking & Ownership
- Issues: Create tasks per acceptance criteria.
- Owners: TBD; use labels `ui`, `audio`, `plugins`, `infra`.

## Notes
- Keep Bevy `LogPlugin` disabled due to `env_logger` init.
- Use thin Bevy `Resource` wrappers for large UI state structs.