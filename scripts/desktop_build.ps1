<#
Purpose: Build the desktop-only application with ZERO web UI code.

Usage examples:
- ./scripts/desktop_build.ps1            # debug build, desktop-only
- ./scripts/desktop_build.ps1 -Release   # release build, desktop-only
- ./scripts/desktop_build.ps1 -Clean -Release

This script enforces `--no-default-features` so the `webui` feature is NOT compiled.
#>

[CmdletBinding()]
param(
    [switch]$Clean,
    [switch]$Release
)

$ErrorActionPreference = "Stop"

try {
    $root = Resolve-Path (Join-Path $PSScriptRoot "..")
    Set-Location $root

    if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
        Write-Error "Rust Cargo not found. Install Rust toolchain (https://rustup.rs)."
        exit 1
    }

    if ($Clean) {
        Write-Host "Cleaning target..." -ForegroundColor Yellow
        cargo clean
    }

    $buildType = if ($Release) { "--release" } else { "" }

    Write-Host "Building DESKTOP-ONLY (no web) with $buildType" -ForegroundColor Cyan
    $cmd = "cargo build $buildType --no-default-features"
    Write-Host $cmd
    iex $cmd

    Write-Host "Desktop-only build completed successfully." -ForegroundColor Green
}
catch {
    Write-Error "Desktop-only build failed: $($_.Exception.Message)"
    exit 1
}