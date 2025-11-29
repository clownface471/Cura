# test_driver.ps1 - Load and test kernel driver in VM
# ⚠️  WARNING: Only run this inside a Virtual Machine!
# NEVER run on host Windows!

param(
    [switch]$Load,
    [switch]$Unload,
    [switch]$Status,
    [switch]$Logs
)

$ErrorActionPreference = "Stop"

# Check if running in VM (basic check)
function Test-IsVirtualMachine {
    $computerSystem = Get-WmiObject Win32_ComputerSystem
    $manufacturer = $computerSystem.Manufacturer

    return $manufacturer -match "VMware|Microsoft Corporation|QEMU|VirtualBox"
}

# Safety check
if (-not (Test-IsVirtualMachine)) {
    Write-Host "⚠️  WARNING: This doesn't appear to be a Virtual Machine!" -ForegroundColor Red
    Write-Host "Driver testing should ONLY be done in VMs!" -ForegroundColor Red
    Write-Host ""
    $confirm = Read-Host "Are you ABSOLUTELY sure you want to continue? (type 'YES' to proceed)"

    if ($confirm -ne "YES") {
        Write-Host "Aborting for safety." -ForegroundColor Yellow
        exit 1
    }
}

$DriverName = "CuraFilter"
$DriverPath = "C:\CURA\CuraFilter.sys"  # Adjust this path

# Check if running as administrator
$isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    Write-Host "Error: This script must be run as Administrator" -ForegroundColor Red
    exit 1
}

# Load driver
if ($Load) {
    Write-Host "Loading CuraFilter driver..." -ForegroundColor Cyan

    # Check if driver file exists
    if (-not (Test-Path $DriverPath)) {
        Write-Host "Error: Driver file not found: $DriverPath" -ForegroundColor Red
        Write-Host "Build the driver first and copy it to the VM" -ForegroundColor Yellow
        exit 1
    }

    # Create service
    sc.exe create $DriverName type=filesys binPath=$DriverPath 2>&1 | Write-Host

    if ($LASTEXITCODE -ne 0) {
        Write-Host "Warning: Service may already exist" -ForegroundColor Yellow
    }

    # Start service
    sc.exe start $DriverName 2>&1 | Write-Host

    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ Driver loaded successfully" -ForegroundColor Green
    } else {
        Write-Host "✗ Failed to load driver" -ForegroundColor Red
        exit 1
    }
}

# Unload driver
if ($Unload) {
    Write-Host "Unloading CuraFilter driver..." -ForegroundColor Cyan

    # Stop service
    sc.exe stop $DriverName 2>&1 | Write-Host

    # Delete service
    sc.exe delete $DriverName 2>&1 | Write-Host

    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ Driver unloaded successfully" -ForegroundColor Green
    } else {
        Write-Host "✗ Failed to unload driver" -ForegroundColor Red
    }
}

# Check status
if ($Status) {
    Write-Host "Checking driver status..." -ForegroundColor Cyan
    Write-Host ""

    # Check service status
    sc.exe query $DriverName 2>&1 | Write-Host

    # Check loaded filters
    Write-Host "`nLoaded minifilters:" -ForegroundColor Yellow
    fltmc.exe filters 2>&1 | Select-String -Pattern $DriverName
}

# Show logs
if ($Logs) {
    Write-Host "Showing driver logs..." -ForegroundColor Cyan
    Write-Host "Use DebugView to see kernel debug output" -ForegroundColor Yellow
    Write-Host "Download: https://learn.microsoft.com/en-us/sysinternals/downloads/debugview" -ForegroundColor Gray
}

# If no parameters, show usage
if (-not ($Load -or $Unload -or $Status -or $Logs)) {
    Write-Host "Usage: .\test_driver.ps1 [options]" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Options:" -ForegroundColor Cyan
    Write-Host "  -Load      Load the driver"
    Write-Host "  -Unload    Unload the driver"
    Write-Host "  -Status    Check driver status"
    Write-Host "  -Logs      Show how to view driver logs"
    Write-Host ""
    Write-Host "Example:" -ForegroundColor Yellow
    Write-Host "  .\test_driver.ps1 -Load"
    Write-Host "  .\test_driver.ps1 -Status"
    Write-Host "  .\test_driver.ps1 -Unload"
}
