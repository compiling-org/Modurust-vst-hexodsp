# Automation Taskflow

This document defines a persistent, repeatable flow to drive UI wiring, audio-engine message integration, and ongoing maintenance without losing context across sessions.

## Goals

- Keep a living backlog of UI tasks that map to audio engine messages.
- Automate routine operations: build, test, scan for unwired UI handlers, and produce a concise report.
- Maintain continuity across conversations by storing status in the repo.

## Flow Overview

- Build project: `cargo build`
- Run tests: `cargo test --quiet` (if present)
- Scan UI for clickable elements not sending `AudioParamMessage` via the bridge.
- Enumerate used `AudioParamMessage` variants and highlight gaps.
- Generate `automation_report.md` with findings and next actions.

## Usage

- Run the PowerShell script: `powershell -ExecutionPolicy Bypass -File scripts/taskflow.ps1`
- Open `automation_report.md` in the repo root for the latest results.

## Backlog Conventions

- Prefer wiring UI interactions to existing variants in `src/audio_engine/bridge.rs` or `src/ui/audio_engine_bridge.rs`.
- When a control lacks a corresponding message, propose adding one in a dedicated PR and reference it from the report.

## Files

- `scripts/taskflow.ps1`: Orchestrates build/test/scan and writes a report.
- `automation_report.md`: Generated status summary to persist progress across sessions.