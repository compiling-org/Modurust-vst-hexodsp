param(
  [int]$IntervalMinutes = 60
)

$repoDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$scriptPath = Join-Path $repoDir 'taskflow.ps1'

Write-Host "[hourly] Starting automation taskflow every $IntervalMinutes minutes. Press Ctrl+C to stop."

while ($true) {
  try {
    if (Test-Path $scriptPath) {
      # Prefer Windows PowerShell, fall back to PowerShell Core
      if (Get-Command powershell -ErrorAction SilentlyContinue) {
        powershell -NoProfile -ExecutionPolicy Bypass -File $scriptPath -ErrorAction SilentlyContinue *> $null
      } elseif (Get-Command pwsh -ErrorAction SilentlyContinue) {
        pwsh -NoProfile -ExecutionPolicy Bypass -File $scriptPath -ErrorAction SilentlyContinue *> $null
      } else {
        Write-Host "[hourly] PowerShell not found; skipping run." -ForegroundColor Yellow
      }
    } else {
      Write-Host "[hourly] Script not found: $scriptPath" -ForegroundColor Yellow
    }
  } catch {
    Write-Host "[hourly] Error: $($_.Exception.Message)" -ForegroundColor Red
  }

  Start-Sleep -Seconds ($IntervalMinutes * 60)
}