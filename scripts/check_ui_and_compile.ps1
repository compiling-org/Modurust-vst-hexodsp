Param(
    [switch]$Build
)

$ErrorActionPreference = 'Stop'

Write-Host "== HexoDSP UI Diagnostics ==" -ForegroundColor Cyan

$logPath = Join-Path (Get-Location) 'logs/ui_panic.log'
if (Test-Path $logPath) {
    Write-Host "Showing last 50 lines of $logPath" -ForegroundColor Yellow
    Get-Content -Path $logPath -Tail 50 | ForEach-Object { $_ }
} else {
    Write-Host "No UI panic log found at $logPath" -ForegroundColor Green
}

Write-Host "Running cargo check (desktop-only)" -ForegroundColor Cyan
$check = Start-Process -FilePath cargo -ArgumentList 'check','--no-default-features' -NoNewWindow -PassThru -Wait
if ($check.ExitCode -ne 0) {
    Write-Host "cargo check failed with exit code $($check.ExitCode)" -ForegroundColor Red
    exit $check.ExitCode
} else {
    Write-Host "cargo check completed successfully" -ForegroundColor Green
}

if ($Build) {
    Write-Host "Running cargo build (desktop-only)" -ForegroundColor Cyan
    $build = Start-Process -FilePath cargo -ArgumentList 'build','--no-default-features' -NoNewWindow -PassThru -Wait
    if ($build.ExitCode -ne 0) {
        Write-Host "cargo build failed with exit code $($build.ExitCode)" -ForegroundColor Red
        exit $build.ExitCode
    } else {
        Write-Host "cargo build completed successfully" -ForegroundColor Green
    }
}

Write-Host "Diagnostics complete" -ForegroundColor Cyan