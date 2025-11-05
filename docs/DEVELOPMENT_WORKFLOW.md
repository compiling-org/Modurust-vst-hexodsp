# Development Workflow (Script-Based SOP)

This workflow defines the repeatable, non-interactive steps we follow for GUI and desktop updates to avoid unnecessary app launches and reduce instruction repetition.

## Core Principles
- Prefer non-interactive checks first: compile and lint before running the UI.
- Harden UI code: avoid `unwrap()`/`expect()` in handlers; use safe locking and log errors.
- Search broadly before patching: run multiple semantic searches to understand context.
- Only preview when visual changes are made (use preview URL when applicable).
- Keep changes minimal and focused on the task; do not fix unrelated issues.

## Standard Operating Procedure
1. Update todos to track tasks and mark progress in real-time.
2. Run non-interactive checks via script:
   - `powershell -NoProfile -ExecutionPolicy Bypass -File ./scripts/dev_workflow.ps1 -All`
   - Or run targeted checks: `-Check`, `-Clippy`, `-AuditUI`, `-AuditEgui`, `-AuditZoom`.
3. Review generated reports in `automation_reports/` and `cargo_*_output.txt`.
4. Patch unsafe UI code:
   - Replace `br.lock().unwrap().send_param(...)` with safe locking and error logging.
   - Keep egui API usage consistent (rounding, stroke kinds, painter calls).
5. Verify with `cargo check` (fast feedback without launching).
6. If changes affect visuals, open preview and inspect.

## Script Overview (scripts/dev_workflow.ps1)
- Cargo checks: compiles and saves output.
- Clippy: lints and saves output.
- UI audit: lists `unwrap()`/`expect()` occurrences in `src/ui`.
- egui API audit: flags painter calls and potentially outdated APIs.
- Zoom audit: finds `timeline_zoom`/`zoom_level` and slider ranges.

## Error-Handling Policy
- UI→Audio bridge: never `unwrap()` mutex locks; log errors on failure.
- Transport controls, sliders, and buttons: use non-panicking flows.

## Visual Changes
- For any painter/layout tweaks, open a preview to validate rendering.

## Notes
- Do not auto-run `cargo fix`; review suggestions first.
- Avoid broad refactors unless explicitly requested.