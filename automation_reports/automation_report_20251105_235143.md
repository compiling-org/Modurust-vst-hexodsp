# Automation Report
Generated: 2025-11-05T23:46:51.4229729+05:30

## Environment
- rustc: rustc 1.90.0 (1159e78c4 2025-09-14)
- cargo: cargo 1.90.0 (840b83a10 2025-07-30)
- PowerShell: 5.1.26100.6899

## Build
Status: Failed
Output:
```
   Compiling proc-macro2 v1.0.103
   Compiling unicode-ident v1.0.22
   Compiling quote v1.0.41
   Compiling serde_core v1.0.228
   Compiling cfg-if v1.0.4
   Compiling windows-link v0.2.1
   Compiling serde v1.0.228
   Compiling libm v0.2.15
   Compiling equivalent v1.0.2
   Compiling parking_lot_core v0.9.12
   Compiling scopeguard v1.2.0
   Compiling log v0.4.28
   Compiling smallvec v1.15.1
   Compiling crossbeam-utils v0.8.21
   Compiling pin-project-lite v0.2.16
   Compiling windows-sys v0.61.2
   Compiling winnow v0.7.13
   Compiling hashbrown v0.16.0
   Compiling foldhash v0.1.5
   Compiling lock_api v0.4.14
   Compiling toml_datetime v0.7.3
   Compiling autocfg v1.5.0
   Compiling thiserror v2.0.17
   Compiling hashbrown v0.15.5
   Compiling version_check v0.9.5
   Compiling parking v2.2.1
   Compiling getrandom v0.3.4
   Compiling rand_core v0.9.3
   Compiling arrayvec v0.7.6
   Compiling parking_lot v0.12.5
   Compiling once_cell v1.21.3
   Compiling rand v0.9.2
   Compiling futures-core v0.3.31
   Compiling spin v0.10.0
   Compiling indexmap v2.12.0
   Compiling num-traits v0.2.19
   Compiling concurrent-queue v2.5.0
   Compiling unicode-xid v0.2.6
   Compiling futures-io v0.3.31
   Compiling typeid v1.0.3
   Compiling foldhash v0.2.0
   Compiling fastrand v2.3.0
   Compiling event-listener v5.4.1
   Compiling slotmap v1.0.7
   Compiling thread_local v1.1.9
   Compiling erased-serde v0.4.8
   Compiling futures-lite v2.6.1
   Compiling event-listener-strategy v0.5.4
   Compiling async-task v4.7.1
   Compiling fixedbitset v0.5.7
   Compiling atomic-waker v1.1.2
   Compiling syn v2.0.108
   Compiling disqualified v1.0.0
   Compiling async-channel v2.5.0
   Compiling inventory v0.3.21
   Compiling zerocopy v0.8.27
   Compiling bevy_ptr v0.17.2
   Compiling downcast-rs v2.0.2
   Compiling toml_parser v1.0.4
   Compiling crossbeam-queue v0.3.12
   Compiling slab v0.4.11
   Compiling nonmax v0.5.5
   Compiling memchr v2.7.6
   Compiling bumpalo v3.19.0
   Compiling tracing-core v0.1.34
   Compiling windows_x86_64_msvc v0.52.6
   Compiling either v1.15.0
   Compiling toml_edit v0.23.7
   Compiling async-executor v1.13.3
   Compiling aho-corasick v1.1.4
   Compiling regex-syntax v0.8.8
   Compiling crossbeam-channel v0.5.15
   Compiling find-msvc-tools v0.1.4
   Compiling cfg_aliases v0.2.1
   Compiling windows-targets v0.52.6
   Compiling shlex v1.3.0
   Compiling itertools v0.14.0
   Compiling rand_distr v0.5.1
   Compiling crc32fast v1.5.0
   Compiling simd-adler32 v0.3.7
   Compiling piper v0.2.4
   Compiling cc v1.2.44
   Compiling adler2 v2.0.1
   Compiling async-lock v3.4.1
   Compiling ctrlc v3.5.1
   Compiling miniz_oxide v0.8.9
   Compiling blocking v1.6.2
   Compiling constant_time_eq v0.3.1
   Compiling lazy_static v1.5.0
   Compiling arrayref v0.3.9
   Compiling base64 v0.22.1
   Compiling fdeflate v0.3.7
   Compiling pxfm v0.1.25
   Compiling async-broadcast v0.7.2
   Compiling typewit v1.14.2
   Compiling async-fs v2.2.0
   Compiling flate2 v1.1.5
   Compiling stackfuture v0.3.0
   Compiling atomicow v1.1.0
   Compiling const_panic v0.2.15
   Compiling sharded-slab v0.1.7
   Compiling nu-ansi-term v0.50.3
   Compiling tracing-log v0.2.0
   Compiling raw-window-handle v0.6.2
   Compiling byteorder-lite v0.1.0
   Compiling euclid v0.22.11
   Compiling svg_fmt v0.4.5
   Compiling twox-hash v2.1.2
   Compiling windows-link v0.1.3
   Compiling rectangle-pack v0.4.2
   Compiling const_soft_float v0.1.4
   Compiling rustc-hash v1.1.0
   Compiling regex-automata v0.4.13
   Compiling windows-strings v0.4.2
   Compiling ruzstd v0.8.2
   Compiling windows-result v0.3.4
   Compiling winapi-util v0.1.11
   Compiling bevy_mikktspace v0.17.0-dev
   Compiling naga v26.0.0
   Compiling constgebra v0.1.4
   Compiling blake3 v1.8.2
   Compiling termcolor v1.4.1
   Compiling windows-result v0.2.0
   Compiling bit-vec v0.8.0
   Compiling unicode-width v0.2.2
   Compiling windows-threading v0.1.0
   Compiling thiserror v1.0.69
   Compiling guillotiere v0.6.2
   Compiling bit-set v0.8.0
   Compiling windows-strings v0.1.0
   Compiling codespan-reporting v0.12.0
   Compiling hexf-parse v0.2.1
   Compiling profiling v1.0.17
   Compiling ash v0.38.0+1.3.281
   Compiling wgpu-hal v26.0.6
   Compiling libloading v0.8.9
   Compiling presser v0.3.1
   Compiling bevy_macro_utils v0.17.2
   Compiling encase_derive_impl v0.11.2
   Compiling synstructure v0.13.2
   Compiling winapi v0.3.9
   Compiling ordered-float v5.0.0
   Compiling libc v0.2.177
   Compiling ntapi v0.4.1
   Compiling static_assertions v1.1.0
   Compiling moxcms v0.7.9
   Compiling renderdoc-sys v1.1.0
   Compiling range-alloc v0.1.4
   Compiling wgpu-core v26.0.1
   Compiling tinyvec_macros v0.1.1
   Compiling stable_deref_trait v1.2.1
   Compiling litrs v1.0.0
   Compiling tinyvec v1.10.0
   Compiling wgpu v26.0.1
   Compiling unicode-segmentation v1.12.0
   Compiling data-encoding v2.9.0
   Compiling radsort v0.1.1
   Compiling const-fnv1a-hash v1.1.0
   Compiling offset-allocator v0.2.0
   Compiling unicode-script v0.5.7
   Compiling unicode-properties v0.1.4
   Compiling zeno v0.3.3
   Compiling ttf-parser v0.21.1
   Compiling unicode-ccc v0.2.0
   Compiling unicode-bidi-mirroring v0.2.0
   Compiling ttf-parser v0.20.0
   Compiling document-features v0.2.12
   Compiling yazi v0.2.1
   Compiling memmap2 v0.9.9
   Compiling windows-result v0.1.2
   Compiling rangemap v1.6.0
   Compiling unicode-bidi v0.3.18
   Compiling matchers v0.2.0
   Compiling regex v1.12.2
   Compiling unicode-linebreak v0.1.5
   Compiling self_cell v1.2.1
   Compiling sys-locale v0.3.2
   Compiling accesskit v0.21.1
   Compiling bytemuck_derive v1.10.2
   Compiling serde_derive v1.0.228
   Compiling thiserror-impl v2.0.17
   Compiling derive_more-impl v2.0.1
   Compiling assert_type_match v0.1.1
   Compiling bevy_reflect_derive v0.17.2
   Compiling variadics_please v1.1.0
   Compiling zerocopy-derive v0.8.27
   Compiling bevy_ecs_macros v0.17.2
   Compiling bevy_derive v0.17.2
   Compiling tracing-attributes v0.1.30
   Compiling bevy_asset_macros v0.17.2
   Compiling encase_derive v0.11.2
   Compiling windows-interface v0.59.3
   Compiling windows-implement v0.60.2
   Compiling bytemuck v1.24.0
   Compiling windows-interface v0.58.0
   Compiling bitflags v2.10.0
   Compiling glam v0.30.9
   Compiling wgpu-types v26.0.0
   Compiling png v0.18.0
   Compiling tracing v0.1.41
   Compiling ktx2 v0.4.0
   Compiling windows-core v0.61.2
   Compiling tracing-subscriber v0.3.20
   Compiling spirv v0.3.0+sdk-1.3.268.0
   Compiling windows-implement v0.58.0
   Compiling thiserror-impl v1.0.69
   Compiling windows-future v0.2.1
   Compiling image v0.25.8
   Compiling windows-collections v0.2.0
   Compiling windows-numerics v0.2.0
   Compiling half v2.7.1
   Compiling windows-core v0.58.0
   Compiling windows v0.61.3
   Compiling derive_more v2.0.1
   Compiling gpu-alloc-types v0.3.0
   Compiling gpu-descriptor-types v0.2.0
   Compiling zerofrom-derive v0.1.6
   Compiling smol_str v0.2.2
   Compiling uuid v1.18.1
   Compiling ron v0.10.1
   Compiling windows v0.58.0
   Compiling gpu-descriptor v0.3.2
   Compiling gpu-alloc v0.6.0
   Compiling yoke-derive v0.8.1
   Compiling font-types v0.10.0
   Compiling zerovec-derive v0.11.2
   Compiling bevy_encase_derive v0.17.2
   Compiling bevy_platform v0.17.2
   Compiling bevy_utils v0.17.2
   Compiling bevy_tasks v0.17.2
   Compiling read-fonts v0.35.0
   Compiling bevy_render_macros v0.17.2
   Compiling petgraph v0.8.3
   Compiling zerofrom v0.1.6
   Compiling displaydoc v0.2.5
   Compiling yoke v0.8.1
   Compiling rustybuzz v0.14.1
   Compiling fontdb v0.16.2
   Compiling windows-core v0.54.0
   Compiling litemap v0.8.1
   Compiling zerovec v0.11.5
   Compiling writeable v0.6.2
   Compiling windows_x86_64_msvc v0.53.1
   Compiling tinystr v0.8.2
   Compiling zerotrie v0.2.3
   Compiling potential_utf v0.1.4
   Compiling windows v0.54.0
   Compiling icu_locale_core v2.1.1
   Compiling icu_properties_data v2.1.1
   Compiling byteorder v1.5.0
   Compiling icu_normalizer_data v2.1.1
   Compiling icu_collections v2.1.1
   Compiling winit v0.30.12
   Compiling icu_provider v2.1.1
   Compiling percent-encoding v2.3.2
   Compiling serde_json v1.0.145
   Compiling bevy_reflect v0.17.2
   Compiling encase v0.11.2
   Compiling hexasphere v16.0.0
   Compiling accesskit_consumer v0.31.0
   Compiling windows-sys v0.52.0
   Compiling cpal v0.15.3
   Compiling grid v0.15.0
   Compiling ryu v1.0.20
   Compiling cursor-icon v1.2.0
   Compiling inflections v1.1.1
   Compiling dpi v0.1.2
   Compiling itoa v1.0.15
   Compiling gltf-derive v1.4.1
   Compiling taffy v0.7.7
   Compiling icu_properties v2.1.1
   Compiling windows-targets v0.53.5
   Compiling skrifa v0.37.0
   Compiling icu_normalizer v2.1.1
   Compiling ogg v0.8.0
   Compiling ahash v0.8.12
   Compiling dasp_sample v0.11.0
   Compiling gilrs v0.11.0
   Compiling ttf-parser v0.25.1
   Compiling gpu-allocator v0.27.0
   Compiling lewton v0.10.2
   Compiling gltf-json v1.4.1
   Compiling idna_adapter v1.2.1
   Compiling windows-sys v0.60.2
   Compiling swash v0.2.6
   Compiling emath v0.33.0
   Compiling naga_oil v0.19.1
   Compiling bevy_ecs v0.17.2
   Compiling bevy_math v0.17.2
   Compiling owned_ttf_parser v0.25.1
   Compiling cosmic-text v0.14.2
   Compiling bevy_animation_macros v0.17.2
   Compiling approx v0.5.1
   Compiling utf8_iter v1.0.4
   Compiling ab_glyph_rasterizer v0.1.10
   Compiling windows_x86_64_msvc v0.42.2
   Compiling vec_map v0.8.2
   Compiling fnv v1.0.7
   Compiling ab_glyph v0.2.32
   Compiling idna v1.1.0
   Compiling wgpu-core-deps-windows-linux-android v26.0.0
   Compiling gltf v1.4.1
   Compiling ecolor v0.33.0
   Compiling form_urlencoded v1.2.2
   Compiling bevy_gizmos_macros v0.17.2
   Compiling bevy_state_macros v0.17.2
   Compiling getrandom v0.2.16
   Compiling nohash-hasher v0.2.0
   Compiling error-code v3.3.2
   Compiling epaint_default_fonts v0.33.0
   Compiling rand_core v0.6.4
   Compiling epaint v0.33.0
   Compiling clipboard-win v5.4.1
   Compiling url v2.5.7
   Compiling ppv-lite86 v0.2.21
   Compiling crossbeam-epoch v0.9.18
   Compiling anyhow v1.0.100
   Compiling arboard v3.6.1
   Compiling rand_chacha v0.3.1
   Compiling webbrowser v1.0.6
   Compiling egui v0.33.0
   Compiling bevy_color v0.17.2
   Compiling crossbeam-deque v0.8.6
   Compiling windows v0.43.0
   Compiling socket2 v0.6.1
   Compiling tokio-macros v2.6.0
   Compiling mio v1.1.0
   Compiling is-terminal v0.4.17
   Compiling gilrs-core v0.6.6
   Compiling bytes v1.10.1
   Compiling bevy_app v0.17.2
   Compiling rodio v0.20.1
   Compiling humantime v2.3.0
   Compiling bitflags v1.3.2
   Compiling bevy_asset v0.17.2
   Compiling bevy_log v0.17.2
   Compiling bevy_input v0.17.2
   Compiling bevy_time v0.17.2
   Compiling bevy_transform v0.17.2
   Compiling bevy_a11y v0.17.2
   Compiling bevy_state v0.17.2
   Compiling env_logger v0.10.2
   Compiling tokio v1.48.0
   Compiling midir v0.9.1
   Compiling crossbeam v0.8.4
   Compiling rand v0.8.5
   Compiling chrono v0.4.42
   Compiling async-trait v0.1.89
   Compiling bevy_gilrs v0.17.2
   Compiling wmidi v4.0.10
   Compiling hound v3.5.1
   Compiling bevy_image v0.17.2
   Compiling bevy_shader v0.17.2
   Compiling bevy_audio v0.17.2
   Compiling bevy_mesh v0.17.2
   Compiling bevy_window v0.17.2
   Compiling sysinfo v0.37.2
   Compiling bevy_text v0.17.2
   Compiling bevy_diagnostic v0.17.2
   Compiling accesskit_windows v0.29.2
   Compiling accesskit_winit v0.29.2
   Compiling bevy_camera v0.17.2
   Compiling bevy_animation v0.17.2
   Compiling bevy_render v0.17.2
   Compiling bevy_picking v0.17.2
   Compiling bevy_light v0.17.2
   Compiling bevy_scene v0.17.2
   Compiling bevy_sprite v0.17.2
   Compiling bevy_input_focus v0.17.2
   Compiling bevy_winit v0.17.2
   Compiling bevy_ui v0.17.2
   Compiling bevy_core_pipeline v0.17.2
   Compiling bevy_sprite_render v0.17.2
   Compiling bevy_pbr v0.17.2
   Compiling bevy_post_process v0.17.2
   Compiling bevy_anti_alias v0.17.2
   Compiling bevy_ui_render v0.17.2
   Compiling bevy_egui v0.38.0
   Compiling bevy_gltf v0.17.2
   Compiling bevy_gizmos v0.17.2
   Compiling bevy_internal v0.17.2
   Compiling bevy v0.17.2
   Compiling hexodsp-daw v0.1.0 (C:\Users\kapil\compiling\hexodsp-vst3)
error[E0432]: unresolved import `uuid`
 --> src\modular_patch_system.rs:3:5
  |
3 | use uuid::Uuid;
  |     ^^^^ use of unresolved module or unlinked crate `uuid`
  |
  = help: if you wanted to use a crate named `uuid`, use `cargo add uuid` to add it to your `Cargo.toml`
System.Management.Automation.RemoteException
error[E0432]: unresolved import `uuid`
 --> src\piano_roll_editor.rs:3:5
  |
3 | use uuid::Uuid;
  |     ^^^^ use of unresolved module or unlinked crate `uuid`
  |
  = help: if you wanted to use a crate named `uuid`, use `cargo add uuid` to add it to your `Cargo.toml`
System.Management.Automation.RemoteException
error[E0432]: unresolved import `uuid`
 --> src\midi_control_system.rs:3:5
  |
3 | use uuid::Uuid;
  |     ^^^^ use of unresolved module or unlinked crate `uuid`
  |
  = help: if you wanted to use a crate named `uuid`, use `cargo add uuid` to add it to your `Cargo.toml`
System.Management.Automation.RemoteException
warning: use of deprecated type alias `bevy::prelude::EventReader`: Renamed to `MessageReader`.
   --> src\ui\bevy_egui_ui.rs:109:22
    |
109 |     mut exit_events: EventReader<bevy::app::AppExit>,
    |                      ^^^^^^^^^^^
    |
    = note: `#[warn(deprecated)]` on by default
System.Management.Automation.RemoteException
For more information about this error, try `rustc --explain E0432`.
warning: `hexodsp-daw` (lib) generated 1 warning
error: could not compile `hexodsp-daw` (lib) due to 3 previous errors; 1 warning emitted
warning: build failed, waiting for other jobs to finish...
```

## Tests
Status: Failed or not present
Output (truncated):
```
error[E0432]: unresolved import `uuid`
 --> src\modular_patch_system.rs:3:5
  |
3 | use uuid::Uuid;
  |     ^^^^ use of unresolved module or unlinked crate `uuid`
  |
  = help: if you wanted to use a crate named `uuid`, use `cargo add uuid` to add it to your `Cargo.toml`
System.Management.Automation.RemoteException
error[E0432]: unresolved import `uuid`
 --> src\piano_roll_editor.rs:3:5
  |
3 | use uuid::Uuid;
  |     ^^^^ use of unresolved module or unlinked crate `uuid`
  |
  = help: if you wanted to use a crate named `uuid`, use `cargo add uuid` to add it to your `Cargo.toml`
System.Management.Automation.RemoteException
error[E0432]: unresolved import `uuid`
 --> src\midi_control_system.rs:3:5
  |
3 | use uuid::Uuid;
  |     ^^^^ use of unresolved module or unlinked crate `uuid`
  |
  = help: if you wanted to use a crate named `uuid`, use `cargo add uuid` to add it to your `Cargo.toml`
System.Management.Automation.RemoteException
warning: use of deprecated type alias `bevy::prelude::EventReader`: Renamed to `MessageReader`.
   --> src\ui\bevy_egui_ui.rs:109:22
    |
109 |     mut exit_events: EventReader<bevy::app::AppExit>,
    |                      ^^^^^^^^^^^
    |
    = note: `#[warn(deprecated)]` on by default
System.Management.Automation.RemoteException
For more information about this error, try `rustc --explain E0432`.
error: could not compile `hexodsp-daw` (lib) due to 3 previous errors; 1 warning emitted
error: could not compile `hexodsp-daw` (lib test) due to 3 previous errors; 1 warning emitted
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
