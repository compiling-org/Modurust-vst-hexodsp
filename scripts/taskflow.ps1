Param(
    [string]$ScanDir = "src/ui",
    [string]$ReportPath = "automation_report.md"
)

# Initialize report
"# Automation Report" | Out-File -FilePath $ReportPath -Encoding UTF8
Add-Content -Path $ReportPath -Value "Generated: $(Get-Date -Format o)"

# Environment
Add-Content -Path $ReportPath -Value "`n## Environment"
try {
    $rustcVer = (& rustc -V 2>&1)
    Add-Content -Path $ReportPath -Value "- rustc: $rustcVer"
} catch { Add-Content -Path $ReportPath -Value "- rustc: unavailable" }
try {
    $cargoVer = (& cargo -V 2>&1)
    Add-Content -Path $ReportPath -Value "- cargo: $cargoVer"
} catch { Add-Content -Path $ReportPath -Value "- cargo: unavailable" }
Add-Content -Path $ReportPath -Value "- PowerShell: $($PSVersionTable.PSVersion)"

# Concurrency lock to avoid overlapping runs
$lockDir = "automation_tmp"
$lockFile = Join-Path $lockDir "taskflow.lock"
try { if (-not (Test-Path $lockDir)) { New-Item -ItemType Directory -Path $lockDir -Force | Out-Null } } catch { }
$now = Get-Date
$staleMinutes = 90
if (Test-Path $lockFile) {
    $age = $now - (Get-Item $lockFile).LastWriteTime
    if ($age.TotalMinutes -lt $staleMinutes) {
        Add-Content -Path $ReportPath -Value "`n## Run Skipped"
        Add-Content -Path $ReportPath -Value "Another automation run is active; skipping to avoid overlap."
        Add-Content -Path $ReportPath -Value "Lock age: $([int]$age.TotalMinutes) minutes"
        try {
            $archiveDir = "automation_reports"
            if (-not (Test-Path $archiveDir)) { New-Item -ItemType Directory -Path $archiveDir -Force | Out-Null }
            $tsName = "automation_report_" + (Get-Date -Format "yyyyMMdd_HHmmss") + ".md"
            $tsPath = Join-Path $archiveDir $tsName
            Copy-Item -Path $ReportPath -Destination $tsPath -Force
        } catch { }
        return
    } else {
        # Purge stale lock
        Remove-Item -Path $lockFile -Force -ErrorAction SilentlyContinue
    }
}
Set-Content -Path $lockFile -Value $now.ToString("o") -ErrorAction SilentlyContinue

# Build
Add-Content -Path $ReportPath -Value "`n## Build"
$build = & cargo build --color never 2>&1
if ($LASTEXITCODE -eq 0) {
    Add-Content -Path $ReportPath -Value "Status: Success"
    # Truncated build warnings for quick visibility
    $buildWarnings = $build | Select-String -Pattern '^\s*warning:'
    if ($buildWarnings) {
        Add-Content -Path $ReportPath -Value "`n### Build Warnings (truncated)"
        Add-Content -Path $ReportPath -Value '```'
        ($buildWarnings | Select-Object -First 20 | ForEach-Object { $_.Line }) | ForEach-Object { Add-Content -Path $ReportPath -Value $_ }
        Add-Content -Path $ReportPath -Value '```'
    }
} else {
    Add-Content -Path $ReportPath -Value "Status: Failed"
    Add-Content -Path $ReportPath -Value "Output:"
    Add-Content -Path $ReportPath -Value '```'
    Add-Content -Path $ReportPath -Value $build
    Add-Content -Path $ReportPath -Value '```'
}

# Test (optional)
Add-Content -Path $ReportPath -Value "`n## Tests"
$tests = & cargo test --quiet --color never 2>&1
if ($LASTEXITCODE -eq 0) {
    Add-Content -Path $ReportPath -Value "Status: Success"
} else {
    Add-Content -Path $ReportPath -Value "Status: Failed or not present"
    # Do not fail the flow for missing/failed tests; capture output
    Add-Content -Path $ReportPath -Value "Output (truncated):"
    Add-Content -Path $ReportPath -Value '```'
    ($tests | Select-Object -First 50) | ForEach-Object { Add-Content -Path $ReportPath -Value $_ }
    Add-Content -Path $ReportPath -Value '```'
}

# Scan UI for clicked handlers without audio bridge sends
Add-Content -Path $ReportPath -Value "`n## Unwired UI Click Handlers"
$clicked = Select-String -Path (Join-Path $ScanDir "**/*.rs") -Pattern 'clicked\(\)' -ErrorAction SilentlyContinue
$unwired = @()
foreach ($m in $clicked) {
    $file = $m.Path
    $line = $m.LineNumber
    $content = Get-Content -Path $file -Encoding UTF8
    $start = [Math]::Max(0, $line - 6)
    $end = [Math]::Min($content.Length - 1, $line + 6)
    $slice = $content[$start..$end]
    $hasSend = ($slice | Where-Object { $_ -match 'send_param\(' }).Count -gt 0
    if (-not $hasSend) {
        $unwired += [PSCustomObject]@{ File=$file; Line=$line; Text=$m.Line.Trim() }
    }
}
if ($unwired.Count -gt 0) {
    foreach ($u in $unwired) { Add-Content -Path $ReportPath -Value "- $($u.File):$($u.Line) -> $($u.Text)" }
} else { Add-Content -Path $ReportPath -Value "None found (heuristic)." }

# Enumerate AudioParamMessage variants in both bridges
Add-Content -Path $ReportPath -Value "`n## AudioParamMessage Variants"
$variants = @()
$bridgeFiles = @(
    "src/audio_engine/bridge.rs",
    "src/ui/audio_engine_bridge.rs"
)
foreach ($bf in $bridgeFiles) {
    if (Test-Path $bf) {
        $defs = Select-String -Path $bf -Pattern 'enum\s+AudioParamMessage' -Context 0,60 -ErrorAction SilentlyContinue
        foreach ($d in $defs) {
            $ctx = $d.Context.PostContext
            foreach ($line in $ctx) {
                if ($line -match '^\s*([A-Za-z0-9_]+)\s*(\(|,|})') {
                    $name = $Matches[1]
                    if ($name -ne 'enum' -and $name -ne 'pub') { $variants += $name }
                }
                if ($line -match '^\s*}') { break }
            }
        }
    }
}
$variants = ($variants | Sort-Object -Unique)
if ($variants.Count -gt 0) {
    foreach ($v in $variants) { Add-Content -Path $ReportPath -Value "- $v" }
} else { Add-Content -Path $ReportPath -Value "No variants found (check parsing)." }

# Variant coverage diff
Add-Content -Path $ReportPath -Value "`n### Variant Coverage Diff"
$engineVariants = @()
$uiVariants = @()
foreach ($bf in $bridgeFiles) {
    if (Test-Path $bf) {
        $defs = Select-String -Path $bf -Pattern 'enum\s+AudioParamMessage' -Context 0,60 -ErrorAction SilentlyContinue
        foreach ($d in $defs) {
            $ctx = $d.Context.PostContext
            foreach ($line in $ctx) {
                if ($line -match '^\s*([A-Za-z0-9_]+)\s*(\(|,|})') {
                    $name = $Matches[1]
                    if ($name -ne 'enum' -and $name -ne 'pub') {
                        if ($bf -like '*src/audio_engine/*') { $engineVariants += $name } else { $uiVariants += $name }
                    }
                }
                if ($line -match '^\s*}') { break }
            }
        }
    }
}
$engineVariants = ($engineVariants | Sort-Object -Unique)
$uiVariants = ($uiVariants | Sort-Object -Unique)
$missingInUI = $engineVariants | Where-Object { $_ -notin $uiVariants }
$missingInEngine = $uiVariants | Where-Object { $_ -notin $engineVariants }
if ($missingInUI.Count -gt 0) {
    Add-Content -Path $ReportPath -Value "- Present in engine, missing in UI:"
    foreach ($m in $missingInUI) { Add-Content -Path $ReportPath -Value "  - $m" }
}
if ($missingInEngine.Count -gt 0) {
    Add-Content -Path $ReportPath -Value "- Present in UI, missing in engine:"
    foreach ($m in $missingInEngine) { Add-Content -Path $ReportPath -Value "  - $m" }
}
if ($missingInUI.Count -eq 0 -and $missingInEngine.Count -eq 0) {
    Add-Content -Path $ReportPath -Value "- Engine and UI variants appear aligned."
}

# Summarize
Add-Content -Path $ReportPath -Value "`n## Next Actions"
Add-Content -Path $ReportPath -Value "- Wire listed unwired UI handlers to appropriate AudioParamMessage variants."
Add-Content -Path $ReportPath -Value "- Prefer using existing variants before proposing new ones."
Add-Content -Path $ReportPath -Value "- Re-run this script after changes to update the report."

Add-Content -Path $ReportPath -Value "`nDone."

# Archive the report with timestamp
try {
    $archiveDir = "automation_reports"
    if (-not (Test-Path $archiveDir)) { New-Item -ItemType Directory -Path $archiveDir -Force | Out-Null }
    $tsName = "automation_report_" + (Get-Date -Format "yyyyMMdd_HHmmss") + ".md"
    $tsPath = Join-Path $archiveDir $tsName
    Copy-Item -Path $ReportPath -Destination $tsPath -Force
} catch { }

# Release lock
Remove-Item -Path $lockFile -Force -ErrorAction SilentlyContinue