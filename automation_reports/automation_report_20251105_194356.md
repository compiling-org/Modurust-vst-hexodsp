# Automation Report
Generated: 2025-11-05T19:43:29.2388885+05:30

## Environment
- rustc: rustc 1.90.0 (1159e78c4 2025-09-14)
- cargo: cargo 1.90.0 (840b83a10 2025-07-30)
- PowerShell: 5.1.26100.6899

## Build
Status: Success

### Build Warnings (truncated)
```
warning: unused import: `std::convert::Infallible`
warning: use of deprecated type alias `bevy::prelude::EventReader`: Renamed to `MessageReader`.
warning: variable does not need to be mutable
warning: `hexodsp-daw` (lib) generated 3 warnings (run `cargo fix --lib -p hexodsp-daw` to apply 2 suggestions)
warning: unused import: `std::sync::Arc`
warning: `hexodsp-daw` (bin "hexodsp-daw") generated 1 warning (run `cargo fix --bin "hexodsp-daw"` to apply 1 suggestion)
```

## Tests
Status: Failed or not present
Output (truncated):
```
warning: unused import: `std::convert::Infallible`
 --> src\web_interface.rs:8:5
  |
8 | use std::convert::Infallible;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default
System.Management.Automation.RemoteException
warning: use of deprecated type alias `bevy::prelude::EventReader`: Renamed to `MessageReader`.
   --> src\ui\bevy_egui_ui.rs:105:22
    |
105 |     mut exit_events: EventReader<bevy::app::AppExit>,
    |                      ^^^^^^^^^^^
    |
    = note: `#[warn(deprecated)]` on by default
System.Management.Automation.RemoteException
warning: variable does not need to be mutable
    --> src\ui\egui_ui_full.rs:2198:35
     |
2198 |                         if let Ok(mut bridge) = br.lock() {
     |                                   ----^^^^^^
     |                                   |
     |                                   help: remove this `mut`
     |
     = note: `#[warn(unused_mut)]` on by default
System.Management.Automation.RemoteException
warning: unused import: `std::sync::Arc`
 --> src\main.rs:4:5
  |
4 | use std::sync::Arc;
  |     ^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default
System.Management.Automation.RemoteException
error[E0432]: unresolved imports `hexodsp_daw::daw_core`, `hexodsp_daw::transport_sync`, `hexodsp_daw::node_graph::NeuroNodeGraph`
 --> examples\basic_daw_usage.rs:7:5
  |
7 |     daw_core::{TrackDatabaseEntry, AudioContainerFormat},
  |     ^^^^^^^^ could not find `daw_core` in `hexodsp_daw`
8 |     transport_sync::GlobalTransport,
  |     ^^^^^^^^^^^^^^ could not find `transport_sync` in `hexodsp_daw`
9 |     node_graph::NeuroNodeGraph,
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ no `NeuroNodeGraph` in `audio_engine::node_graph`
System.Management.Automation.RemoteException
error[E0432]: unresolved imports `hexodsp_daw::node_graph::NeuroNodeGraph`, `hexodsp_daw::node_graph::NeuroNodeId`, `hexodsp_daw::transport_sync`, `hexodsp_daw::audio_nodes`
 --> tests\integration_tests.rs:6:18
  |
6 |     node_graph::{NeuroNodeGraph, NeuroNodeId},
  |                  ^^^^^^^^^^^^^^  ^^^^^^^^^^^ no `NeuroNodeId` in `audio_engine::node_graph`
  |                  |
```

## Unwired UI Click Handlers
None found (heuristic).

## AudioParamMessage Variants
- ConnectNodes
- CreateNode
- DeleteNode
- DisconnectNodes
- GroupVolume
- MasterCompRatio
- MasterCompThreshold
- MasterEQHigh
- MasterEqHighFreq
- MasterEqHighGain
- MasterEqHmidFreq
- MasterEqHmidGain
- MasterEqLmidFreq
- MasterEqLmidGain
- MasterEQLow
- MasterEqLowFreq
- MasterEqLowGain
- MasterLimCeiling
- MasterMute
- MasterPan
- MasterVolume
- Pause
- Play
- Record
- ReloadConfig
- ResetEngine
- ReturnPan
- ReturnVolume
- SendLevel
- SetInputMonitoring
- SetLoop
- SetNodeParameter
- SetParameter
- SetTempo
- SetTimeSignature
- Stop
- TrackArm
- TrackEffect
- TrackEQ
- TrackEqGain
- TrackMute
- TrackPan
- TrackSolo
- TrackVolume

### Variant Coverage Diff
- Present in engine, missing in UI:
  - ConnectNodes
  - CreateNode
  - DeleteNode
  - DisconnectNodes
  - MasterEQHigh
  - MasterEQLow
  - Pause
  - ReloadConfig
  - ResetEngine
  - SendLevel
  - SetInputMonitoring
  - SetNodeParameter
  - SetParameter
  - SetTimeSignature
  - TrackArm
  - TrackEffect
  - TrackEQ
- Present in UI, missing in engine:
  - MasterCompRatio
  - MasterCompThreshold
  - MasterEqHighFreq
  - MasterEqHighGain
  - MasterEqHmidFreq
  - MasterEqHmidGain
  - MasterEqLmidFreq
  - MasterEqLmidGain
  - MasterEqLowFreq
  - MasterEqLowGain
  - MasterLimCeiling
  - TrackEqGain

## Next Actions
- Wire listed unwired UI handlers to appropriate AudioParamMessage variants.
- Prefer using existing variants before proposing new ones.
- Re-run this script after changes to update the report.

Done.
