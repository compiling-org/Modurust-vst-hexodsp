# Automation Report
Generated: 2025-11-06T00:30:04.7989202+05:30

## Environment
- rustc: rustc 1.90.0 (1159e78c4 2025-09-14)
- cargo: cargo 1.90.0 (840b83a10 2025-07-30)
- PowerShell: 5.1.26100.6899

## Build
Status: Success

## Tests
Status: Failed or not present
Output (truncated):
```
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
  |                  no `NeuroNodeGraph` in `audio_engine::node_graph`
7 |     transport_sync::{GlobalTransport, TransportState},
  |     ^^^^^^^^^^^^^^ could not find `transport_sync` in `hexodsp_daw`
8 |     audio_nodes::SineOscillator,
  |     ^^^^^^^^^^^ could not find `audio_nodes` in `hexodsp_daw`
System.Management.Automation.RemoteException
For more information about this error, try `rustc --explain E0432`.
error[E0599]: no method named `add_node` found for struct `NodeGraph` in the current scope
  --> examples\node_graph_demo.rs:15:24
   |
15 |     let osc_id = graph.add_node("oscillator");
   |                        ^^^^^^^^ method not found in `NodeGraph`
System.Management.Automation.RemoteException
error: could not compile `hexodsp-daw` (example "basic_daw_usage") due to 1 previous error
error: could not compile `hexodsp-daw` (test "integration_tests") due to 1 previous error
error[E0599]: no method named `add_node` found for struct `NodeGraph` in the current scope
  --> examples\node_graph_demo.rs:16:27
   |
16 |     let filter_id = graph.add_node("filter");
   |                           ^^^^^^^^ method not found in `NodeGraph`
System.Management.Automation.RemoteException
error[E0599]: no method named `add_node` found for struct `NodeGraph` in the current scope
  --> examples\node_graph_demo.rs:17:27
   |
17 |     let output_id = graph.add_node("output");
   |                           ^^^^^^^^ method not found in `NodeGraph`
System.Management.Automation.RemoteException
For more information about this error, try `rustc --explain E0599`.
error: could not compile `hexodsp-daw` (example "node_graph_demo") due to 3 previous errors
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
