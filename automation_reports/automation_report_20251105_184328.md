# Automation Report
Generated: 2025-11-05T18:42:52.4697938+05:30

## Environment
- rustc: rustc 1.90.0 (1159e78c4 2025-09-14)
- cargo: cargo 1.90.0 (840b83a10 2025-07-30)
- PowerShell: 5.1.26100.6899

## Build
Status: Success

### Build Warnings (truncated)
```
warning: unnecessary parentheses around closure body
warning: unused variable: `current_len`
warning: variable does not need to be mutable
warning: `hexodsp-daw` (lib) generated 3 warnings (run `cargo fix --lib -p hexodsp-daw` to apply 2 suggestions)
warning: unused variable: `ui_state`
warning: `hexodsp-daw` (bin "hexodsp-daw") generated 1 warning
```

## Tests
Status: Failed or not present
Output (truncated):
```
warning: unnecessary parentheses around closure body
   --> src\audio_engine\node_graph.rs:585:22
    |
585 |             .map(|v| (v * 20.0 - 60.0))
    |                      ^               ^
    |
    = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
    |
585 -             .map(|v| (v * 20.0 - 60.0))
585 +             .map(|v| v * 20.0 - 60.0)
    |
System.Management.Automation.RemoteException
warning: unused variable: `current_len`
   --> src\audio_engine\node_graph.rs:518:17
    |
518 |             let current_len = self.track_volumes.len();
    |                 ^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_current_len`
    |
    = note: `#[warn(unused_variables)]` on by default
System.Management.Automation.RemoteException
warning: variable does not need to be mutable
   --> src\audio_engine\bridge.rs:394:9
    |
394 |     let mut engine = HexoDSPEngine::new(event_queue.clone())?;    
    |         ----^^^^^^
    |         |
    |         help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` on by default
System.Management.Automation.RemoteException
warning: unused variable: `ui_state`
  --> src\main.rs:13:9
   |
13 |     let ui_state: UiState = fs::read_to_string(UI_STATE_PATH)
   |         ^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_ui_state`
   |
   = note: `#[warn(unused_variables)]` on by default
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
