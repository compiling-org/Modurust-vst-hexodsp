# Project Planner Import Guide

This folder contains ready-to-use files to populate your GitHub Project and create linked repository issues.

## Files
- `issues.csv`: Full backlog with Title, Body, and suggested Labels (area, priority, status).
- `project-items.csv`: Simplified CSV (Title, Body) for quick paste into GitHub Projects.

## Import into GitHub Project (Draft Items)
1. Open your project: `https://github.com/orgs/compiling-org/projects/3`.
2. Click `+ Add items` → `Add from spreadsheet`.
3. Open `scripts/project-items.csv`, copy all rows, and paste into the project.
4. This creates Draft items with titles and descriptions.

## Convert Drafts to Repo Issues
1. Select the new rows in the project board.
2. `Bulk actions` → `Create issues`.
3. Choose repo: `compiling-org/Modurust-vst-hexodsp`.

## Apply Labels (Recommended Scheme)
- Area: `area:UI`, `area:Audio`, `area:Plugins`, `area:Infra`.
- Priority: `priority:P1`, `priority:P2`, `priority:P3`.
- Status: `status:Backlog`, `status:In Progress`, `status:Done`.

### Option A: Manual (UI)
- Open each issue in the repo and apply the labels as needed.

### Option B: CLI (Bulk with labels from `issues.csv`)
Prereqs:
- `gh auth login`
- `gh repo set-default compiling-org/Modurust-vst-hexodsp`

PowerShell one-liner:
```
Import-Csv scripts/issues.csv | ForEach-Object {
  $labels = $_.Labels -split ';'
  gh issue create --title $_.Title --body $_.Body --label $labels
}
```

This creates issues with labels applied. If your project has automation enabled, the issues will appear in the project automatically.

## Auto-Add Issues to Project
1. In Project #3 → `Settings` → `Workflows`.
2. Enable “Auto-add newly created issues”.
3. Scope: `compiling-org/Modurust-vst-hexodsp`.
4. Destination column: `To do` (or your default).
5. Optional filter: only add when `label:status:Backlog`.

## Troubleshooting
- If org permissions block automation, convert draft items to issues via the project UI.
- Ensure repo is linked to the project in Project Settings.
- If CLI fails on labels, create issues first, then apply labels via bulk edit in the repo.

## Sources
- Items trace back to `docs/ROADMAP.md`, `docs/frontend-status.md`, and related files; acceptance criteria embedded in descriptions.