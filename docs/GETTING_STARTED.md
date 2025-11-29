# Getting Started with CURA Development

**Welcome to the most challenging project of your life.**

## Prerequisites

### Hardware Requirements
- **Windows PC** (Windows 10/11)
- **Minimum 16GB RAM** (32GB recommended)
- **SSD** (kernel debugging generates lots of I/O)
- **NVIDIA GPU** (for Phase 2+, RTX preferred)
- **Second monitor** (optional but extremely helpful for debugging)

### Software Requirements

#### Phase 1 (Kernel Development)
1. **Windows 11 Pro/Enterprise** (Hyper-V for VM)
   - Or VMware Workstation / VirtualBox
2. **Visual Studio 2022**
   - Workload: "Desktop development with C++"
   - Workload: "Windows Driver Kit"
3. **Windows Driver Kit (WDK)**
   - Download: https://learn.microsoft.com/en-us/windows-hardware/drivers/download-the-wdk
4. **Rust** (latest stable)
   - Download: https://rustup.rs/
5. **Git** (version control)

#### Phase 2 (AI Service)
1. **CMake** (3.20+)
2. **ONNX Runtime** (will be linked later)
3. **Python 3.10+** (for model training)
   - PyTorch
   - ONNX export tools

#### Phase 3 (UI)
1. **Flutter SDK** (stable channel)
   - Download: https://docs.flutter.dev/get-started/install/windows

## Development Environment Setup

### Step 1: Clone Repository
```bash
git clone <your-repo-url>
cd Cura
```

### Step 2: Install Rust
```powershell
# Install Rust
# Visit https://rustup.rs/ and run the installer

# Verify installation
rustc --version
cargo --version

# Add Windows targets
rustup target add x86_64-pc-windows-msvc
```

### Step 3: Install WDK
```powershell
# Install Visual Studio 2022 first
# Then install WDK from Microsoft

# Verify WDK installation
# Check if these exist:
#   C:\Program Files (x86)\Windows Kits\10\Include\wdf
#   C:\Program Files (x86)\Windows Kits\10\build
```

### Step 4: Set Up Virtual Machine

**CRITICAL**: NEVER test kernel drivers on your host Windows installation!

#### Option A: Hyper-V (Recommended for Windows 11 Pro)
```powershell
# Enable Hyper-V
Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V -All

# Create VM
# 1. Open Hyper-V Manager
# 2. New -> Virtual Machine
# 3. Allocate 4GB RAM, 2 CPUs, 40GB disk
# 4. Install Windows 10/11

# Enable test signing in VM
bcdedit /set testsigning on

# Enable kernel debugging
bcdedit /debug on
bcdedit /dbgsettings serial debugport:1 baudrate:115200
```

#### Option B: VMware Workstation
```
1. Create new VM
   - OS: Windows 10/11
   - RAM: 4GB
   - CPU: 2 cores
   - Disk: 40GB

2. Add serial port
   - Use named pipe
   - Server: \\.\pipe\com_1

3. In VM, enable test signing:
   bcdedit /set testsigning on
```

### Step 5: Install WinDbg (Debugger)
```powershell
# Install from Windows Store
# Search: "WinDbg Preview"

# Or download SDK
# https://developer.microsoft.com/en-us/windows/downloads/windows-sdk/
```

## Project Structure Overview

```
Cura/
├── kernel/CuraFilter/       # Phase 1: Rust kernel driver
│   ├── src/lib.rs           # Driver entry point
│   └── Cargo.toml
│
├── service/CuraCore/        # Phase 2: C++ AI service
│   ├── src/main.cpp
│   ├── include/cura_core.h
│   └── CMakeLists.txt
│
├── ui/CuraUI/               # Phase 3: Flutter UI
│   └── (Created in Phase 3)
│
├── docs/                    # Documentation
│   ├── ARCHITECTURE.md
│   ├── GETTING_STARTED.md (this file)
│   └── phase1/
│
├── research/                # AI research & datasets
│   ├── datasets/
│   └── models/
│
└── tools/                   # Build scripts & utilities
```

## Your First Build

### Test Rust Setup
```bash
cd kernel/CuraFilter
cargo build

# Should compile (but won't do anything yet)
# This is just a skeleton
```

### Test C++ Setup
```bash
cd service/CuraCore
mkdir build
cd build
cmake .. -G "Visual Studio 17 2022"
cmake --build . --config Release

# Run skeleton service
.\bin\Release\CuraCore.exe
```

## Phase 1: Your Journey Begins

### Months 1-2: Learn Rust

**Goal**: Understand Rust fundamentals

**Resources**:
1. [The Rust Book](https://doc.rust-lang.org/book/)
   - Chapters 1-10 (ownership, borrowing, lifetimes)
2. [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
3. [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises

**Exercises**:
```bash
# Clone rustlings
git clone https://github.com/rust-lang/rustlings
cd rustlings
cargo install --force --path .

# Start learning
rustlings watch
```

**Milestone**: Write a Rust program that:
- Manages memory safely (no garbage collector)
- Uses `Result<T, E>` for error handling
- Implements a simple file I/O utility

### Months 3-4: Windows Kernel Fundamentals

**Goal**: Understand how Windows kernel works

**Essential Reading**:
1. **Windows Internals (7th Edition)** by Mark Russinovich
   - Part 1, Chapter 5: Processes & Threads
   - Part 1, Chapter 8: I/O System
2. **Windows Kernel Programming** by Pavel Yosifovich
   - Chapters 1-5

**Key Concepts to Master**:
- [ ] IRPs (I/O Request Packets)
- [ ] IRQL (Interrupt Request Levels)
- [ ] Filter Manager architecture
- [ ] Kernel memory management (paged vs non-paged pool)
- [ ] Synchronization (spinlocks, mutexes)

**Online Resources**:
- [OSR Online](https://www.osronline.com/) - Forum for driver developers
- [Microsoft Learn - Driver Development](https://learn.microsoft.com/en-us/windows-hardware/drivers/)

**Milestone**: Understand this code:
```c
NTSTATUS DriverEntry(PDRIVER_OBJECT DriverObject, PUNICODE_STRING RegistryPath) {
    DriverObject->DriverUnload = DriverUnload;
    return STATUS_SUCCESS;
}
```

### Months 5-6: First Driver

**Goal**: Build a driver that loads and unloads

**Steps**:
1. Create empty driver project
2. Implement `DriverEntry` and `DriverUnload`
3. Build and sign driver
4. Load in VM using `sc create`
5. Verify with WinDbg

**Resources**:
- [windows-kernel-rs](https://github.com/not-matthias/windows-kernel-rs) - Rust WDK bindings

**Milestone**: See your driver in WinDbg:
```
kd> !drvobj YourDriver
Driver object (ffffXXXX) is for:
 \Driver\YourDriver
```

### Months 7-8: File Filter Implementation

**Goal**: Block access to "file_rahasia.txt"

**Implementation**:
```rust
// In CuraFilter/src/lib.rs
fn pre_create_callback(path: &str) -> FltStatus {
    if path.contains("file_rahasia.txt") {
        return FltStatus::AccessDenied; // BLOCK
    }
    FltStatus::Success // ALLOW
}
```

**Testing**:
```powershell
# In VM:
1. Load CuraFilter.sys
2. Try to create file_rahasia.txt
3. Should be BLOCKED
4. Try to create other files
5. Should SUCCEED
```

**Milestone**: Working minifilter driver!

## Daily Development Workflow

### Morning Routine
```powershell
# 1. Start VM
# (In Hyper-V Manager or VMware)

# 2. Pull latest changes
git pull origin main

# 3. Check what you're working on
# Review Phase 1 checklist in docs/phase1/
```

### Development Cycle
```powershell
# 1. Write code (in host Windows)
# Edit kernel/CuraFilter/src/lib.rs

# 2. Build (in host Windows)
cd kernel/CuraFilter
cargo build --release

# 3. Copy driver to VM
# Use shared folder or network share

# 4. Load in VM
sc stop CuraFilter
sc delete CuraFilter
sc create CuraFilter type=filesys binPath=C:\Path\cura-filter.sys
sc start CuraFilter

# 5. Test & debug
# Use DebugView or WinDbg

# 6. Unload (if stable)
sc stop CuraFilter
```

### Evening Routine
```powershell
# 1. Commit progress
git add .
git commit -m "feat: implemented PreCreate callback"

# 2. Push to backup
git push origin main

# 3. Update learning journal
# Document what you learned today
```

## Debugging Tips

### Using WinDbg

**Connect to VM**:
```
File -> Kernel Debug
COM -> Pipe: \\.\pipe\com_1
```

**Essential Commands**:
```
!analyze -v          # Analyze crash
bp CuraFilter!DriverEntry  # Set breakpoint
g                    # Go (continue)
k                    # Stack trace
!process 0 0         # List processes
!drvobj CuraFilter 7 # Inspect driver
```

### Using DbgView (Simpler)

```powershell
# In host Windows, download DbgView from Sysinternals
# Configure to capture kernel debug output
# Your driver's DbgPrint will appear here
```

### Common Issues

**Issue**: "Driver could not be loaded"
**Solution**: Enable test signing:
```powershell
bcdedit /set testsigning on
```

**Issue**: BSOD (Blue Screen of Death)
**Solution**:
1. Don't panic - this is normal
2. Restart VM
3. Check crash dump with WinDbg
4. Fix the bug
5. Try again

**Issue**: Driver won't unload
**Solution**: Reboot VM (it's faster than debugging why)

## Learning Resources

### Books
1. **The Rust Programming Language** - Free online
2. **Windows Internals (Part 1)** - Essential
3. **Windows Kernel Programming** - Practical examples

### Online Courses
1. [Rust Course (Official)](https://www.rust-lang.org/learn)
2. [OSR Driver Development](https://www.osronline.com/seminars/)

### Communities
1. [Rust Discord](https://discord.gg/rust-lang)
2. [OSR Online Forums](https://community.osr.com/)
3. [r/rust](https://reddit.com/r/rust)
4. [r/kernel](https://reddit.com/r/kernel)

### YouTube
1. Search: "Windows Kernel Driver Development"
2. Search: "Rust for System Programming"

## Time Management (For Students)

You're balancing school with this ambitious project. Here's a realistic schedule:

### Weekdays (School Days)
- **After school (1-2 hours)**: Reading/Learning
  - Read Rust book
  - Watch kernel dev tutorials
  - Small coding exercises

### Weekends
- **Saturday (4-6 hours)**: Focused development
  - Deep work on driver code
  - VM testing
  - Debugging

- **Sunday (2-3 hours)**: Review and planning
  - Document progress
  - Plan next week
  - Update learning journal

### Realistic Progress
- **Phase 1**: 8 months (as planned)
  - Don't rush
  - Understand deeply
  - Build muscle memory

## Staying Motivated

### Milestones Celebration
- ✅ First Rust program compiles
- ✅ First driver loads
- ✅ First file blocked
- ✅ First behavioral detection

### When You Feel Stuck
1. Take a break (go for a walk)
2. Ask for help (OSR forums, Rust Discord)
3. Simplify the problem
4. Sleep on it (seriously, your brain works while sleeping)

### Remember Why
You're not just learning to code. You're building:
- **Security expertise** (valuable skill)
- **System programming mastery** (rare skill)
- **Product from scratch** (entrepreneurship)

## Next Steps

1. ✅ Read this document
2. ⬜ Set up development environment
3. ⬜ Create VM for testing
4. ⬜ Install Rust and WDK
5. ⬜ Build the skeleton projects
6. ⬜ Start Phase 1 Month 1: Learn Rust

## Getting Help

### When to Ask for Help
- After you've spent 2+ hours stuck
- After you've researched online
- After you've read the docs

### Where to Ask
1. **OSR Online** - Kernel driver questions
2. **Rust Discord** - Rust language questions
3. **Stack Overflow** - General programming

### How to Ask
- Show your code
- Explain what you tried
- Provide error messages
- Be specific

## Final Words

**This is hard. That's the point.**

You're attempting something that most senior engineers wouldn't dare:
- Kernel programming (where bugs crash the system)
- AI integration (cutting-edge tech)
- Full product development (end-to-end)

**But you can do it.**

One line of code at a time.
One day at a time.
One phase at a time.

Welcome to CURA.

---

**Now go set up your environment and let's build something amazing.**
