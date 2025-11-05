# Automation Report
Generated: 2025-11-05T17:00:02.1744117+05:30

## Environment
- rustc: rustc 1.90.0 (1159e78c4 2025-09-14)
- cargo: cargo 1.90.0 (840b83a10 2025-07-30)
- PowerShell: 5.1.26100.6899

## Build
Status: Success

### Build Warnings (truncated)
```
warning: unused import: `egui`
warning: unnecessary parentheses around closure body
warning: variable does not need to be mutable
warning: variable does not need to be mutable
warning: `hexodsp-daw` (lib) generated 4 warnings (run `cargo fix --lib -p hexodsp-daw` to apply 4 suggestions)
warning: unused variable: `ui_state`
warning: `hexodsp-daw` (bin "hexodsp-daw") generated 1 warning
```

## Tests
Status: Failed or not present
Output (truncated):
```
warning: unused import: `egui`
 --> src\ui\bevy_egui_ui.rs:2:17
  |
2 | use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
  |                 ^^^^
  |
  = note: `#[warn(unused_imports)]` on by default
System.Management.Automation.RemoteException
warning: unnecessary parentheses around closure body
   --> src\audio_engine\node_graph.rs:495:22
    |
495 |             .map(|v| (v * 20.0 - 60.0))
    |                      ^               ^
    |
    = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
    |
495 -             .map(|v| (v * 20.0 - 60.0))
495 +             .map(|v| v * 20.0 - 60.0)
    |
System.Management.Automation.RemoteException
warning: variable does not need to be mutable
  --> src\ui\bevy_egui_ui.rs:46:9
   |
46 |     let mut engine = HexoDSPEngine::new(event_queue.clone()).expect("Failed to initialize au...
   |         ----^^^^^^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default
System.Management.Automation.RemoteException
warning: variable does not need to be mutable
   --> src\audio_engine\bridge.rs:394:9
    |
394 |     let mut engine = HexoDSPEngine::new(event_queue.clone())?;    
    |         ----^^^^^^
    |         |
    |         help: remove this `mut`
System.Management.Automation.RemoteException
warning: variable does not need to be mutable
  --> src\ui\bevy_egui_ui.rs:46:9
   |
46 |     let mut engine = HexoDSPEngine::new(event_queue.clone()).expect("Failed to initialize audio engine");
   |         ----^^^^^^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default
System.Management.Automation.RemoteException
warning: unused variable: `ui_state`
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
