# Automation Report
Generated: 2025-11-05T21:44:24.4266612+05:30

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
warning: variable does not need to be mutable
warning: value assigned to `ry` is never read
warning: `hexodsp-daw` (lib) generated 5 warnings (run `cargo fix --lib -p hexodsp-daw` to apply 3 suggestions)
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
   --> src\ui\bevy_egui_ui.rs:109:22
    |
109 |     mut exit_events: EventReader<bevy::app::AppExit>,
    |                      ^^^^^^^^^^^
    |
    = note: `#[warn(deprecated)]` on by default
System.Management.Automation.RemoteException
warning: variable does not need to be mutable
    --> src\ui\egui_ui_full.rs:2270:35
     |
2270 |                         if let Ok(mut bridge) = br.lock() {
     |                                   ----^^^^^^
     |                                   |
     |                                   help: remove this `mut`
     |
     = note: `#[warn(unused_mut)]` on by default
System.Management.Automation.RemoteException
warning: variable does not need to be mutable
    --> src\ui\egui_ui_full.rs:2319:35
     |
2319 |                         if let Ok(mut bridge) = br.lock() {
     |                                   ----^^^^^^
     |                                   |
     |                                   help: remove this `mut`
System.Management.Automation.RemoteException
warning: value assigned to `ry` is never read
   --> src\ui\hexagonal_node_view.rs:507:9
    |
507 |         ry = -rx - rz;
    |         ^^
    |
    = help: maybe it is overwritten before being read?
    = note: `#[warn(unused_assignments)]` on by default
System.Management.Automation.RemoteException
warning: unused import: `std::sync::Arc`
 --> src\main.rs:5:5
  |
5 | use std::sync::Arc;
  |     ^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default
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
