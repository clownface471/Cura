# CuraFilter - Kernel Mode Driver

**The Sentinel Layer (Ring 0)**

## ⚠️ CRITICAL WARNINGS

1. **NEVER test this driver on your host Windows installation!**
   - Always use a Virtual Machine (VMware/VirtualBox/Hyper-V)
   - A crash here = Blue Screen of Death (BSOD)
   - Potential data loss if tested on host

2. **Memory Safety is Non-Negotiable**
   - Kernel crashes affect the entire system
   - Use Rust's safety features religiously
   - Validate every pointer before dereferencing

3. **Development Requirements**
   - Windows Driver Kit (WDK)
   - Visual Studio 2022 (with Windows Driver development workload)
   - Test signing enabled on VM
   - Debugger setup (WinDbg + VM debugging)

## Architecture

```
┌─────────────────────────────────────┐
│     User Mode (Ring 3)              │
│  ┌──────────────┐  ┌─────────────┐  │
│  │  CuraCore    │  │   CuraUI    │  │
│  │  (Service)   │  │  (Flutter)  │  │
│  └──────┬───────┘  └─────────────┘  │
│         │ IOCTL                      │
├─────────┼──────────────────────────────
│         ↓                            │
│  ┌──────────────────┐                │
│  │   CuraFilter.sys │ ← YOU ARE HERE │
│  │  (Minifilter)    │                │
│  └────────┬─────────┘                │
│           │                          │
│  ┌────────↓─────────┐                │
│  │  Filter Manager  │                │
│  └────────┬─────────┘                │
│           │                          │
│  ┌────────↓─────────┐                │
│  │   File System    │                │
│  │   (NTFS/FAT32)   │                │
│  └──────────────────┘                │
│     Kernel Mode (Ring 0)             │
└─────────────────────────────────────┘
```

## Phase 1 Goals (Months 1-8)

### Month 1-2: Foundation
- [ ] Learn Rust ownership, borrowing, lifetimes
- [ ] Study `#![no_std]` environment
- [ ] Understand unsafe Rust

### Month 3-4: Windows Kernel Basics
- [ ] Read "Windows Internals" (Part 1, Chapter 8: I/O System)
- [ ] Understand IRPs (I/O Request Packets)
- [ ] Learn about IRQL (Interrupt Request Levels)
- [ ] Study Filter Manager architecture

### Month 5-6: First Driver
- [ ] Set up WDK environment
- [ ] Create empty driver that loads/unloads
- [ ] Implement DriverEntry and Unload routines
- [ ] Test in VM with DbgView

### Month 7-8: File Blocking
- [ ] Register as minifilter
- [ ] Implement PreCreate callback
- [ ] Block access to "file_rahasia.txt"
- [ ] **MILESTONE**: Working driver that prevents opening specific file

## Zero-AI Philosophy

**No AI inference in kernel space!**

The driver only does:
- ✅ Boolean logic (Allow/Block/Report)
- ✅ Fast pattern matching
- ✅ Telemetry collection
- ❌ Machine learning
- ❌ Complex heuristics

All intelligence lives in CuraCore (usermode service).

## Key Responsibilities

1. **File I/O Interception**
   - PreCreate: Before file is opened
   - PreWrite: Before data is written
   - PreSetInformation: Before file is renamed/deleted

2. **Process Monitoring**
   - PsSetCreateProcessNotifyRoutineEx
   - Track process creation/termination
   - Collect command line arguments

3. **Self-Defense**
   - Protect driver from being unloaded
   - Prevent CURA processes from being killed
   - Block tampering with CURA files

4. **Telemetry Pipeline**
   - Send events to CuraCore via IOCTL
   - Minimal buffering (kernel memory is precious)
   - No blocking waits

## Development Workflow

```bash
# 1. Build driver (in VM)
cargo build --release

# 2. Sign driver (test signing)
signtool sign /v /fd SHA256 /s TestCertStore /n "CURA Test" cura-filter.sys

# 3. Install driver
sc create CuraFilter type= filesys binPath= C:\Path\cura-filter.sys
fltmc load CuraFilter

# 4. Monitor with DebugView
# Watch for your debug prints

# 5. Unload
fltmc unload CuraFilter
sc delete CuraFilter
```

## Resources

### Essential Reading
1. **Windows Internals (7th Edition)** - Part 1, Chapters 5 & 8
2. **Windows Kernel Programming** - Pavel Yosifovich
3. [OSR Online](https://www.osronline.com/) - Driver development community

### Rust for Kernel
1. [windows-kernel-rs](https://github.com/not-matthias/windows-kernel-rs)
2. [Kernel Driver with Rust](https://github.com/Trantect/win_driver_example)

### Official Documentation
1. [Filter Manager Concepts](https://learn.microsoft.com/en-us/windows-hardware/drivers/ifs/filter-manager-concepts)
2. [Minifilter Best Practices](https://learn.microsoft.com/en-us/windows-hardware/drivers/ifs/development-and-testing-tools)

## Debugging Tips

```
# Enable boot debugging in VM
bcdedit /debug on
bcdedit /dbgsettings serial debugport:1 baudrate:115200

# Connect WinDbg from host
# In VMware: Add Serial Port -> Use named pipe

# Essential WinDbg commands
!analyze -v          # Analyze crash
!process 0 0         # List processes
!drvobj CuraFilter 7 # Inspect driver object
bp CuraFilter!DriverEntry  # Set breakpoint
```

## Common Pitfalls

1. **IRQL Issues**
   - Don't call pageable code at DISPATCH_LEVEL
   - Use `KeGetCurrentIrql()` to check

2. **Memory Leaks**
   - Every `ExAllocatePool` needs `ExFreePool`
   - Use pool tags for tracking

3. **Deadlocks**
   - Never hold a spinlock too long
   - Respect lock hierarchy

4. **Pointer Validation**
   ```rust
   // NEVER do this:
   let value = *ptr;  // BSOD if ptr is invalid

   // DO this:
   if is_valid_user_address(ptr) {
       let value = unsafe { *ptr };
   }
   ```

## File Structure (Future)

```
CuraFilter/
├── src/
│   ├── lib.rs                 # Entry point
│   ├── filter_callbacks.rs    # File I/O handlers
│   ├── process_monitor.rs     # Process tracking
│   ├── self_defense.rs        # Anti-tampering
│   ├── telemetry.rs           # Send to usermode
│   └── utils.rs               # Helper functions
├── Cargo.toml
└── README.md (this file)
```

---

**Remember**: Start small, test often, and NEVER skip VM testing. One bug here can ruin your day (and your data).
