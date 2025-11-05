Param(
    [string]$ScanDir = "src/ui",
    [string]$ReportPath = "automation_report.md"
)

function Write-Section {
    Param([string]$Title)
    "`n## $Title`" | Out-File -FilePath $ReportPath -Append -Encoding UTF8
}

function Write-Line {
    Param([string]$Text)
    $Text | Out-File -FilePath $ReportPath -Append -Encoding UTF8
}

# Initialize report
"# Automation Report" | Out-File -FilePath $ReportPath -Encoding UTF8
Write-Line "Generated: $(Get-Date -Format o)"

# Build
Write-Section "Build"
$build = & cargo build 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Line "Status: Success"
} else {
    Write-Line "Status: Failed"
    Write-Line "Output:"; Write-Line "```"; Write-Line $build; Write-Line "```"
}

# Test (optional)
Write-Section "Tests"
$tests = & cargo test --quiet 2>&1
if ($LASTEXITCODE -eq 0) {
    Write-Line "Status: Success"
} else {
    Write-Line "Status: Failed or not present"
    # Do not fail the flow for missing/failed tests; capture output
    Write-Line "Output (truncated):"; Write-Line "```"; Write-Line ($tests | Select-Object -First 50); Write-Line "```"
}

# Scan UI for clicked handlers without audio bridge sends
Write-Section "Unwired UI Click Handlers"
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
    foreach ($u in $unwired) { Write-Line "- $($u.File):$($u.Line) -> $($u.Text)" }
} else { Write-Line "None found (heuristic)." }

# Enumerate AudioParamMessage variants in both bridges
Write-Section "AudioParamMessage Variants"
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
    foreach ($v in $variants) { Write-Line "- $v" }
} else { Write-Line "No variants found (check parsing)." }

# Summarize
Write-Section "Next Actions"
Write-Line "- Wire listed unwired UI handlers to appropriate AudioParamMessage variants."
Write-Line "- Prefer using existing variants before proposing new ones."
Write-Line "- Re-run this script after changes to update the report."

Write-Line "`nDone."
