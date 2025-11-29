# run_exercises.ps1 - Run Rust learning exercises
# Usage: .\run_exercises.ps1 [exercise_number]

param(
    [int]$ExerciseNumber = 0
)

$ErrorActionPreference = "Stop"
$ScriptDir = $PSScriptRoot
$ProjectRoot = (Get-Item $ScriptDir).Parent.Parent.FullName
$ExercisesDir = Join-Path $ProjectRoot "docs\phase1\exercises"

# Exercise list
$Exercises = @(
    @{ Number = 1; Name = "ex01_ownership"; Description = "String Ownership" },
    @{ Number = 2; Name = "ex02_borrowing"; Description = "Mutable Borrowing" },
    @{ Number = 3; Name = "ex03_lifetimes"; Description = "Lifetime Annotations" },
    @{ Number = 4; Name = "ex04_result"; Description = "Result Type" },
    @{ Number = 5; Name = "ex05_errors"; Description = "Custom Error Types" },
    @{ Number = 6; Name = "ex06_pointers"; Description = "Raw Pointers" },
    @{ Number = 7; Name = "ex07_validation"; Description = "Pointer Validation" },
    @{ Number = 8; Name = "ex08_buffer"; Description = "No-Std Buffer" },
    @{ Number = 9; Name = "ex09_strings"; Description = "String Comparison (No-Std)" },
    @{ Number = 10; Name = "ex10_callbacks"; Description = "Callback Pattern" },
    @{ Number = 11; Name = "project_file_filter"; Description = "Practice Project: File Filter" }
)

Push-Location $ExercisesDir

try {
    if ($ExerciseNumber -eq 0) {
        # Run all exercises
        Write-Host "=== Running All Exercises ===" -ForegroundColor Cyan
        Write-Host ""

        foreach ($ex in $Exercises) {
            Write-Host "[$($ex.Number)/11] $($ex.Description)" -ForegroundColor Yellow
            Write-Host "Running $($ex.Name)...`n" -ForegroundColor Gray

            cargo run --bin $ex.Name 2>&1 | Write-Host

            if ($LASTEXITCODE -eq 0) {
                Write-Host "✓ Exercise $($ex.Number) complete`n" -ForegroundColor Green
            } else {
                Write-Host "✗ Exercise $($ex.Number) failed`n" -ForegroundColor Red
            }
        }

        # Run tests
        Write-Host "`n=== Running Tests ===" -ForegroundColor Cyan
        cargo test 2>&1 | Write-Host

        if ($LASTEXITCODE -eq 0) {
            Write-Host "`n✓ All tests passed!" -ForegroundColor Green
        } else {
            Write-Host "`n✗ Some tests failed" -ForegroundColor Red
        }

    } else {
        # Run specific exercise
        $ex = $Exercises | Where-Object { $_.Number -eq $ExerciseNumber }

        if ($null -eq $ex) {
            Write-Host "Error: Exercise $ExerciseNumber not found" -ForegroundColor Red
            Write-Host "Available exercises: 1-11" -ForegroundColor Yellow
            exit 1
        }

        Write-Host "=== Exercise $($ex.Number): $($ex.Description) ===" -ForegroundColor Cyan
        Write-Host ""

        cargo run --bin $ex.Name 2>&1 | Write-Host

        if ($LASTEXITCODE -eq 0) {
            Write-Host "`n✓ Exercise complete!" -ForegroundColor Green
        } else {
            Write-Host "`n✗ Exercise failed" -ForegroundColor Red
        }
    }

} finally {
    Pop-Location
}
