# clean_all.ps1 - Clean all build artifacts

param(
    [switch]$Force
)

$ErrorActionPreference = "Stop"
$ScriptDir = $PSScriptRoot
$ProjectRoot = (Get-Item $ScriptDir).Parent.Parent.FullName

Write-Host "=== Cleaning CURA Build Artifacts ===" -ForegroundColor Cyan

if (-not $Force) {
    $confirm = Read-Host "This will delete all build artifacts. Continue? (Y/N)"
    if ($confirm -ne "Y") {
        Write-Host "Aborted." -ForegroundColor Yellow
        exit 0
    }
}

$CleanedDirs = 0
$CleanedSize = 0

# Function to remove directory and track stats
function Remove-BuildDir {
    param([string]$Path)

    if (Test-Path $Path) {
        $size = (Get-ChildItem $Path -Recurse | Measure-Object -Property Length -Sum).Sum
        $sizeMB = [math]::Round($size / 1MB, 2)

        Write-Host "Removing: $Path ($sizeMB MB)" -ForegroundColor Gray
        Remove-Item -Path $Path -Recurse -Force
        $script:CleanedDirs++
        $script:CleanedSize += $size
    }
}

# Clean Rust targets
Write-Host "`n[1/3] Cleaning Rust builds..." -ForegroundColor Yellow
Remove-BuildDir "$ProjectRoot\kernel\CuraFilter\target"
Remove-BuildDir "$ProjectRoot\docs\phase1\exercises\target"

# Clean C++ builds
Write-Host "`n[2/3] Cleaning C++ builds..." -ForegroundColor Yellow
Remove-BuildDir "$ProjectRoot\service\CuraCore\build"

# Clean other artifacts
Write-Host "`n[3/3] Cleaning other artifacts..." -ForegroundColor Yellow

# Find and remove all Cargo.lock files (except in project roots)
Get-ChildItem -Path $ProjectRoot -Filter "Cargo.lock" -Recurse | Where-Object {
    $_.FullName -notlike "*\kernel\CuraFilter\Cargo.lock"
} | ForEach-Object {
    Write-Host "Removing: $($_.FullName)" -ForegroundColor Gray
    Remove-Item $_.FullName -Force
}

# Summary
Write-Host "`n=== Clean Summary ===" -ForegroundColor Cyan
Write-Host "Directories removed: $CleanedDirs" -ForegroundColor Green

if ($CleanedSize -gt 0) {
    $cleanedMB = [math]::Round($CleanedSize / 1MB, 2)
    $cleanedGB = [math]::Round($CleanedSize / 1GB, 2)

    if ($cleanedGB -gt 1) {
        Write-Host "Space freed: $cleanedGB GB" -ForegroundColor Green
    } else {
        Write-Host "Space freed: $cleanedMB MB" -ForegroundColor Green
    }
}

Write-Host "`n✓ Clean complete!" -ForegroundColor Green
