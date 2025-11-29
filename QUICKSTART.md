# CURA Quick Start Guide

**Get started with CURA development in 5 minutes**

## Prerequisites

- Windows 10/11
- Git installed
- Terminal (PowerShell or Windows Terminal)

## Step 1: Install Rust

```powershell
# Open PowerShell and run:
# Visit https://rustup.rs/ and follow instructions
# Or use winget:
winget install Rustlang.Rustup
```

Verify installation:
```powershell
rustc --version
cargo --version
```

## Step 2: Clone Repository

```powershell
git clone <your-repo-url>
cd Cura
```

## Step 3: Build Everything

```powershell
# Run the build script
.\tools\scripts\build_all.ps1
```

This will build:
- ✅ Kernel driver (Rust)
- ✅ Service skeleton (C++)
- ✅ Learning exercises (Rust)

## Step 4: Start Learning

### Option A: Run Exercises (Recommended)

```powershell
# Run all exercises
.\tools\scripts\run_exercises.ps1

# Or run specific exercise
.\tools\scripts\run_exercises.ps1 -ExerciseNumber 1
```

### Option B: Read Documentation

1. **Start Here**: [docs/GETTING_STARTED.md](docs/GETTING_STARTED.md)
2. **Phase 1 Roadmap**: [docs/phase1/ROADMAP.md](docs/phase1/ROADMAP.md)
3. **Exercises**: [docs/phase1/EXERCISES.md](docs/phase1/EXERCISES.md)

## Step 5: Learn Rust Fundamentals

### Week 1-2 Goals
- [ ] Complete [The Rust Book](https://doc.rust-lang.org/book/) Chapters 1-4
- [ ] Run exercises 1-3 (ownership, borrowing, lifetimes)
- [ ] Write your first Rust program

### Practice:
```powershell
cd docs/phase1/exercises

# Run specific exercise
cargo run --bin ex01_ownership
cargo run --bin ex10_callbacks
cargo run --bin project_file_filter

# Run tests
cargo test
```

## Phase 1 Path

```
Month 1-2:  Learn Rust fundamentals
Month 3-4:  Study Windows kernel internals
Month 5-6:  Build first driver (loads/unloads)
Month 7-8:  Implement file blocking ← MILESTONE
```

## Useful Commands

### Build
```powershell
# Build everything
.\tools\scripts\build_all.ps1

# Build specific component
cd kernel/CuraFilter
cargo build --release

# Build C++ service
cd service/CuraCore
mkdir build; cd build
cmake .. -G "Visual Studio 17 2022"
cmake --build . --config Release
```

### Clean
```powershell
# Clean all build artifacts
.\tools\scripts\clean_all.ps1
```

### Test
```powershell
# Run Rust tests
cargo test

# Run exercises
.\tools\scripts\run_exercises.ps1
```

## Current Status

You are here: **Phase 1, Month 1 - Rust Learning**

### Completed:
- ✅ Project structure
- ✅ Documentation
- ✅ Build scripts
- ✅ Learning exercises

### Next Steps:
1. Install Rust and build tools
2. Run exercises 1-3
3. Read Rust Book chapters 1-4
4. Move to Month 2

## Get Help

- **Rust Help**: [Rust Discord](https://discord.gg/rust-lang) #beginners channel
- **Driver Development**: [OSR Online Forums](https://community.osr.com/)
- **Issues**: Check `docs/` directory for detailed guides

## Tips for Success

1. **Don't skip fundamentals** - Master Rust before kernel work
2. **Use VM for testing** - NEVER test drivers on host Windows
3. **Track progress** - Update README checkboxes as you learn
4. **Take breaks** - This is a marathon, not a sprint

## Resources

### Essential Reading
- [The Rust Book](https://doc.rust-lang.org/book/) - FREE
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises - FREE
- [docs/phase1/RESOURCES.md](docs/phase1/RESOURCES.md) - Curated list

### Books (Optional, ~$150 total)
- Windows Internals Part 1 - $60
- Windows Kernel Programming - $40
- Programming Rust - $50

## Common Issues

### "cargo: command not found"
- Rust not installed or not in PATH
- Solution: Install Rust from https://rustup.rs/

### "CMake not found"
- Visual Studio or CMake not installed
- Solution: Install Visual Studio 2022 with C++ workload

### Build errors in kernel driver
- This is normal! It's a skeleton.
- You'll implement actual code in Phase 1 Months 5-8

## Project Structure

```
Cura/
├── kernel/CuraFilter/       ← Rust kernel driver (Phase 1 Month 5+)
├── service/CuraCore/        ← C++ service (Phase 2)
├── ui/CuraUI/               ← Flutter UI (Phase 3)
├── docs/
│   ├── phase1/
│   │   ├── ROADMAP.md       ← Detailed 8-month plan
│   │   ├── EXERCISES.md     ← Start here!
│   │   └── RESOURCES.md
│   ├── ARCHITECTURE.md
│   └── GETTING_STARTED.md   ← Full setup guide
└── tools/scripts/           ← Build scripts
```

## What's Next?

After mastering Rust (Months 1-2):
1. Study Windows Internals (Months 3-4)
2. Set up VM for testing
3. Install Windows Driver Kit (WDK)
4. Build your first kernel driver!

---

**Remember**: "One line of code at a time. One day at a time. One phase at a time."

Ready to start? Run:
```powershell
.\tools\scripts\run_exercises.ps1 -ExerciseNumber 1
```

Good luck! 🚀
