Param(
    [switch]$All,
    [switch]$Check,
    [switch]$Clippy,
    [switch]$AuditUI,
    [switch]$AuditEgui,
    [switch]$AuditZoom
)

$ErrorActionPreference = 'Stop'

function Get-RepoRoot {
    $scriptDir = Split-Path -Parent $PSCommandPath
    return (Split-Path -Parent $scriptDir)
}

function New-ReportPath {
    param([string]$Title)
    $root = Get-RepoRoot
    $reportsDir = Join-Path $root 'automation_reports'
    if (-not (Test-Path $reportsDir)) { New-Item -ItemType Directory -Path $reportsDir | Out-Null }
    $ts = Get-Date -Format 'yyyyMMdd_HHmmss'
    $safeTitle = ($Title -replace '[^a-zA-Z0-9_-]','_')
    return Join-Path $reportsDir "automation_report_${ts}_${safeTitle}.md"
}

function Write-Section {
    param([string]$Path, [string]$Header, [string]$Body)
    "# $Header`n" | Out-File -FilePath $Path -Append -Encoding UTF8
    $Body | Out-File -FilePath $Path -Append -Encoding UTF8
    "`n" | Out-File -FilePath $Path -Append -Encoding UTF8
}

function Run-CargoCheck {
    $root = Get-RepoRoot
    Push-Location $root
    try {
        Write-Host 'Running cargo check...'
        $checkOut = cargo check 2>&1
        $outFile = Join-Path $root 'cargo_check_output.txt'
        $checkOut | Out-File -FilePath $outFile -Encoding UTF8
        $report = New-ReportPath -Title 'cargo_check'
        Write-Section -Path $report -Header 'Cargo Check Output' -Body ($checkOut | Out-String)
        Write-Host "cargo check complete. Output saved to $outFile"
    } finally { Pop-Location }
}

function Run-CargoClippy {
    $root = Get-RepoRoot
    Push-Location $root
    try {
        Write-Host 'Running cargo clippy...'
        $clippyOut = cargo clippy 2>&1
        $outFile = Join-Path $root 'cargo_clippy_output.txt'
        $clippyOut | Out-File -FilePath $outFile -Encoding UTF8
        $report = New-ReportPath -Title 'cargo_clippy'
        Write-Section -Path $report -Header 'Cargo Clippy Output' -Body ($clippyOut | Out-String)
        Write-Host "cargo clippy complete. Output saved to $outFile"
    } finally { Pop-Location }
}

function Audit-UnwrapExpectUI {
    $root = Get-RepoRoot
    $uiDir = Join-Path $root 'src/ui'
    $matches = Get-ChildItem -Path $uiDir -Recurse -Filter '*.rs' |
        Select-String -Pattern 'unwrap\(', 'expect\(' |
        Sort-Object Path, LineNumber
    $report = New-ReportPath -Title 'ui_unwrap_expect_audit'
    $body = if ($matches) { $matches | ForEach-Object { "$($_.Path):$($_.LineNumber): $($_.Line.Trim())" } | Out-String } else { 'No unwrap()/expect() found under src/ui.' }
    Write-Section -Path $report -Header 'UI unwrap/expect audit' -Body $body
    Write-Host 'UI unwrap/expect audit complete.'
}

function Audit-EguiApis {
    $root = Get-RepoRoot
    $uiDir = Join-Path $root 'src/ui'
    $patterns = @('CornerRadius::same', 'StrokeKind::', 'rect_stroke', 'rect_filled')
    $report = New-ReportPath -Title 'egui_api_audit'
    foreach ($pat in $patterns) {
        $matches = Get-ChildItem -Path $uiDir -Recurse -Filter '*.rs' | Select-String -Pattern $pat | Sort-Object Path, LineNumber
        $header = "Pattern: $pat"
        $body = if ($matches) { $matches | ForEach-Object { "$($_.Path):$($_.LineNumber): $($_.Line.Trim())" } | Out-String } else { 'No matches.' }
        Write-Section -Path $report -Header $header -Body $body
    }
    Write-Host 'egui API audit complete.'
}

function Audit-ZoomControls {
    $root = Get-RepoRoot
    $uiDir = Join-Path $root 'src/ui'
    $report = New-ReportPath -Title 'zoom_controls_audit'
    $patterns = @('timeline_zoom', 'zoom_level', 'Slider', 'DragValue')
    foreach ($pat in $patterns) {
        $matches = Get-ChildItem -Path $uiDir -Recurse -Filter '*.rs' | Select-String -Pattern $pat | Sort-Object Path, LineNumber
        $header = "Pattern: $pat"
        $body = if ($matches) { $matches | ForEach-Object { "$($_.Path):$($_.LineNumber): $($_.Line.Trim())" } | Out-String } else { 'No matches.' }
        Write-Section -Path $report -Header $header -Body $body
    }
    Write-Host 'Zoom controls audit complete.'
}

function Run-All {
    Run-CargoCheck
    Run-CargoClippy
    Audit-UnwrapExpectUI
    Audit-EguiApis
    Audit-ZoomControls
}

if ($All -or (-not ($Check -or $Clippy -or $AuditUI -or $AuditEgui -or $AuditZoom))) {
    Run-All
} else {
    if ($Check) { Run-CargoCheck }
    if ($Clippy) { Run-CargoClippy }
    if ($AuditUI) { Audit-UnwrapExpectUI }
    if ($AuditEgui) { Audit-EguiApis }
    if ($AuditZoom) { Audit-ZoomControls }
}