# build_all.ps1 - Build all CURA components
# Usage: .\build_all.ps1 [-Configuration Release|Debug]

param(
    [ValidateSet("Debug", "Release")]
    [string]$Configuration = "Release"
)

$ErrorActionPreference = "Stop"
$ScriptDir = $PSScriptRoot
$ProjectRoot = (Get-Item $ScriptDir).Parent.Parent.FullName

Write-Host "=== Building CURA ===" -ForegroundColor Cyan
Write-Host "Configuration: $Configuration" -ForegroundColor Yellow
Write-Host "Project Root: $ProjectRoot`n" -ForegroundColor Gray

# Track build results
$BuildResults = @{
    Kernel = $false
    Service = $false
    Exercises = $false
}

# ============================================================================
# Build 1: Kernel Driver (Rust)
# ============================================================================
Write-Host "[1/3] Building Kernel Driver (CuraFilter.sys)..." -ForegroundColor Yellow

Push-Location "$ProjectRoot\kernel\CuraFilter"

try {
    if ($Configuration -eq "Release") {
        cargo build --release 2>&1 | Write-Host
    } else {
        cargo build 2>&1 | Write-Host
    }

    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ Kernel driver build successful`n" -ForegroundColor Green
        $BuildResults.Kernel = $true
    } else {
        Write-Host "✗ Kernel driver build failed`n" -ForegroundColor Red
    }
} catch {
    Write-Host "✗ Error building kernel driver: $_`n" -ForegroundColor Red
} finally {
    Pop-Location
}

# ============================================================================
# Build 2: Service (C++)
# ============================================================================
Write-Host "[2/3] Building Service (CuraCore.exe)..." -ForegroundColor Yellow

Push-Location "$ProjectRoot\service\CuraCore"

try {
    # Create build directory if it doesn't exist
    if (-not (Test-Path "build")) {
        Write-Host "Creating build directory..." -ForegroundColor Gray
        New-Item -ItemType Directory -Path "build" | Out-Null
    }

    Push-Location "build"

    # Configure with CMake (only if CMakeCache doesn't exist)
    if (-not (Test-Path "CMakeCache.txt")) {
        Write-Host "Configuring with CMake..." -ForegroundColor Gray
        cmake .. -G "Visual Studio 17 2022" -A x64 2>&1 | Write-Host

        if ($LASTEXITCODE -ne 0) {
            throw "CMake configuration failed"
        }
    }

    # Build
    Write-Host "Building..." -ForegroundColor Gray
    cmake --build . --config $Configuration 2>&1 | Write-Host

    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ Service build successful`n" -ForegroundColor Green
        $BuildResults.Service = $true
    } else {
        Write-Host "✗ Service build failed`n" -ForegroundColor Red
    }

    Pop-Location
} catch {
    Write-Host "✗ Error building service: $_`n" -ForegroundColor Red
} finally {
    Pop-Location
}

# ============================================================================
# Build 3: Rust Exercises (for learning)
# ============================================================================
Write-Host "[3/3] Building Rust Exercises..." -ForegroundColor Yellow

Push-Location "$ProjectRoot\docs\phase1\exercises"

try {
    cargo build 2>&1 | Write-Host

    if ($LASTEXITCODE -eq 0) {
        Write-Host "✓ Exercises build successful`n" -ForegroundColor Green
        $BuildResults.Exercises = $true
    } else {
        Write-Host "✗ Exercises build failed`n" -ForegroundColor Red
    }
} catch {
    Write-Host "✗ Error building exercises: $_`n" -ForegroundColor Red
} finally {
    Pop-Location
}

# ============================================================================
# Summary
# ============================================================================
Write-Host "=== Build Summary ===" -ForegroundColor Cyan

$TotalBuilds = $BuildResults.Count
$SuccessfulBuilds = ($BuildResults.Values | Where-Object { $_ -eq $true }).Count

Write-Host "Kernel Driver:  " -NoNewline
if ($BuildResults.Kernel) {
    Write-Host "✓ SUCCESS" -ForegroundColor Green
} else {
    Write-Host "✗ FAILED" -ForegroundColor Red
}

Write-Host "Service:        " -NoNewline
if ($BuildResults.Service) {
    Write-Host "✓ SUCCESS" -ForegroundColor Green
} else {
    Write-Host "✗ FAILED" -ForegroundColor Red
}

Write-Host "Exercises:      " -NoNewline
if ($BuildResults.Exercises) {
    Write-Host "✓ SUCCESS" -ForegroundColor Green
} else {
    Write-Host "✗ FAILED" -ForegroundColor Red
}

Write-Host "`nTotal: $SuccessfulBuilds/$TotalBuilds builds successful" -ForegroundColor $(if ($SuccessfulBuilds -eq $TotalBuilds) { "Green" } else { "Yellow" })

if ($SuccessfulBuilds -eq $TotalBuilds) {
    Write-Host "`n🎉 All builds successful!" -ForegroundColor Green
    exit 0
} else {
    Write-Host "`n⚠️  Some builds failed. Check the output above for details." -ForegroundColor Yellow
    exit 1
}
