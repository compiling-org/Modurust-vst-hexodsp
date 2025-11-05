<#
Purpose: Run the desktop-only application with ZERO web UI code.

Usage examples:
- ./scripts/desktop_run.ps1              # run debug build, desktop-only
- ./scripts/desktop_run.ps1 -Release     # run release build, desktop-only
- ./scripts/desktop_run.ps1 -Release -- --arg1 value

This script enforces `--no-default-features` so the `webui` feature is NOT compiled or run.
#>

[CmdletBinding()]
param(
    [switch]$Release,
    [Parameter(ValueFromRemainingArguments=$true)]
    [string[]]$Args
)

$ErrorActionPreference = "Stop"

try {
    $root = Resolve-Path (Join-Path $PSScriptRoot "..")
    Set-Location $root

    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-Error "Rust Cargo not found. Install Rust toolchain (https://rustup.rs)."
        exit 1
    }

    $buildType = if ($Release) { "--release" } else { "" }
    $argsJoined = if ($Args) { $Args -join " " } else { "" }

    Write-Host "Running DESKTOP-ONLY (no web) with $buildType" -ForegroundColor Cyan
    $cmd = "cargo run $buildType --no-default-features -- $argsJoined".Trim()
    Write-Host $cmd
    iex $cmd
}
catch {
    Write-Error "Desktop-only run failed: $($_.Exception.Message)"
    exit 1
}