# Build Scripts & Utilities

Automation tools for CURA development.

## Available Scripts

### Build Scripts

**`build_all.ps1`** - Build all CURA components

```powershell
.\build_all.ps1                # Build in Release mode
.\build_all.ps1 -Configuration Debug
```

Builds:
- Kernel driver (CuraFilter.sys)
- Service (CuraCore.exe)
- Rust exercises

**`clean_all.ps1`** - Clean all build artifacts

```powershell
.\clean_all.ps1        # Prompts for confirmation
.\clean_all.ps1 -Force # Skip confirmation
```

Removes:
- Rust target directories
- C++ build directories
- Shows space freed

### Testing Scripts

**`test_driver.ps1`** - Load/test kernel driver in VM

⚠️ **WARNING: Only run in Virtual Machine!**

```powershell
.\test_driver.ps1 -Load      # Load driver
.\test_driver.ps1 -Status    # Check driver status
.\test_driver.ps1 -Unload    # Unload driver
.\test_driver.ps1 -Logs      # Show how to view logs
```

**`run_exercises.ps1`** - Run Rust learning exercises

```powershell
.\run_exercises.ps1                    # Run all exercises + tests
.\run_exercises.ps1 -ExerciseNumber 1  # Run specific exercise
```

Exercises:
1. Ownership
2. Borrowing
3. Lifetimes
4. Result types
5. Error handling
6. Raw pointers
7. Pointer validation
8. No-std buffers
9. String comparison
10. Callbacks
11. Practice project (file filter)

### Future Scripts
- `build_kernel.ps1` - Build kernel driver only
- `build_service.ps1` - Build service only
- `sign_driver.ps1` - Sign driver with test certificate
- `stress_test.ps1` - Stability testing
- `benchmark.ps1` - Performance benchmarks
- `package.ps1` - Create installer
- `deploy_vm.ps1` - Deploy to test VM
- `setup_dev_env.ps1` - One-click dev environment setup
- `create_test_cert.ps1` - Generate test signing certificate

## Example: Build All Script (Future)

```powershell
# build_all.ps1
param(
    [ValidateSet("Debug", "Release")]
    [string]$Configuration = "Release"
)

Write-Host "Building CURA - $Configuration" -ForegroundColor Cyan

# Build kernel driver
Write-Host "`n[1/3] Building CuraFilter.sys..." -ForegroundColor Yellow
Set-Location "$PSScriptRoot\..\..\kernel\CuraFilter"
cargo build --release
if ($LASTEXITCODE -ne 0) { exit 1 }

# Build service
Write-Host "`n[2/3] Building CuraCore.exe..." -ForegroundColor Yellow
Set-Location "$PSScriptRoot\..\..\service\CuraCore"
cmake --build build --config $Configuration
if ($LASTEXITCODE -ne 0) { exit 1 }

# Build UI (Phase 3)
# Write-Host "`n[3/3] Building CuraUI.exe..." -ForegroundColor Yellow
# ...

Write-Host "`nBuild complete!" -ForegroundColor Green
```

## Example: Sign Driver Script

```powershell
# sign_driver.ps1
param(
    [Parameter(Mandatory=$true)]
    [string]$DriverPath
)

$CertName = "CURA Test Certificate"

Write-Host "Signing driver: $DriverPath" -ForegroundColor Cyan

# Sign with test certificate
signtool sign `
    /v `
    /fd SHA256 `
    /s "TestCertStore" `
    /n "$CertName" `
    $DriverPath

if ($LASTEXITCODE -eq 0) {
    Write-Host "Driver signed successfully!" -ForegroundColor Green
} else {
    Write-Host "Signing failed!" -ForegroundColor Red
    exit 1
}
```

## Example: VM Deploy Script

```powershell
# deploy_vm.ps1
$VMName = "CURA-Test-VM"
$VMPath = "\\$VMName\C$\CURA"

Write-Host "Deploying to VM: $VMName" -ForegroundColor Cyan

# Copy driver
Copy-Item "kernel\CuraFilter\target\release\cura_filter.sys" `
          "$VMPath\CuraFilter.sys" -Force

# Copy service
Copy-Item "service\CuraCore\build\Release\CuraCore.exe" `
          "$VMPath\CuraCore.exe" -Force

Write-Host "Deployment complete!" -ForegroundColor Green
Write-Host "Now load the driver in the VM." -ForegroundColor Yellow
```

---

**Phase 1**: Scripts not needed yet. Manual builds are fine.
**Phase 2+**: Create automation scripts as needed.
