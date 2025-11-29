# Phase 1 Roadmap: THE SKELETON

**Duration**: Months 1-8
**Goal**: Build a working kernel driver that blocks access to a specific file
**Language**: Rust
**Target**: Windows Kernel Mode

## Overview

Phase 1 is pure foundation. No AI, no UI, no fancy features. Just:
1. Learn Rust
2. Understand Windows kernel
3. Build a minifilter driver
4. Block a file

**Why this matters**: If you can't get this working, the rest is impossible.

---

## Month 1-2: Rust Fundamentals

### Week 1-2: Basic Syntax
**Goal**: Understand Rust ownership and borrowing

**Tasks**:
- [ ] Install Rust (`rustup.rs`)
- [ ] Complete "The Rust Programming Language" Chapters 1-4
- [ ] Write hello world
- [ ] Understand `let`, `mut`, and variables
- [ ] Practice with basic data types

**Code Challenge**:
```rust
// Write a function that:
// 1. Takes ownership of a String
// 2. Converts to uppercase
// 3. Returns the String
fn to_uppercase(s: String) -> String {
    // Your code here
}
```

**Resources**:
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rustlings](https://github.com/rust-lang/rustlings)

---

### Week 3-4: Ownership Deep Dive
**Goal**: Master borrowing, references, and lifetimes

**Tasks**:
- [ ] Complete Rust Book Chapters 5-10
- [ ] Understand `&` vs `&mut`
- [ ] Learn about slices
- [ ] Practice with structs and enums
- [ ] Understand `Result<T, E>` and `Option<T>`

**Code Challenge**:
```rust
// Write a function that:
// - Takes a reference to a vector
// - Returns the longest string
// - WITHOUT taking ownership
fn find_longest<'a>(words: &'a Vec<String>) -> Option<&'a str> {
    // Your code here
}
```

**Resources**:
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

---

### Week 5-6: Advanced Concepts
**Goal**: Traits, error handling, and unsafe

**Tasks**:
- [ ] Complete Rust Book Chapters 11-16
- [ ] Understand traits and generics
- [ ] Practice with `Result` propagation (`?` operator)
- [ ] Learn about `unsafe` blocks
- [ ] Understand raw pointers

**Code Challenge**:
```rust
// Implement a custom Error type
#[derive(Debug)]
struct FileError {
    message: String,
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FileError: {}", self.message)
    }
}

impl std::error::Error for FileError {}
```

**Why unsafe matters**: Kernel programming is ALL unsafe. You need to understand it deeply.

---

### Week 7-8: No-Std Environment
**Goal**: Work without the standard library

**Tasks**:
- [ ] Understand `#![no_std]`
- [ ] Learn about `core` vs `std`
- [ ] Practice allocation without heap
- [ ] Understand `#[panic_handler]`

**Code Challenge**:
```rust
#![no_std]

// Write a no_std program that compares two byte arrays
fn bytes_equal(a: &[u8], b: &[u8]) -> bool {
    // Your code here (no Vec, no String)
}
```

**Resources**:
- [The Embedded Rust Book](https://docs.rust-embedded.org/book/)

**Milestone**: Write a Rust program that manipulates memory safely without `std`

---

## Month 3-4: Windows Kernel Internals

### Week 9-10: I/O System Architecture
**Goal**: Understand how Windows handles files

**Reading**:
- [ ] Windows Internals Part 1, Chapter 8
- [ ] Focus on:
  - I/O Request Packets (IRPs)
  - I/O Manager
  - Filter Manager
  - File System Drivers

**Key Concepts**:
```
Application (CreateFile)
    ↓
Kernel32.dll (CreateFileW)
    ↓
NtCreateFile (syscall)
    ↓
I/O Manager
    ↓
Filter Manager
    ↓ [YOUR DRIVER SITS HERE]
File System Driver (NTFS)
    ↓
Storage Driver
    ↓
Disk
```

**Exercise**:
- [ ] Draw the I/O stack on paper
- [ ] Explain IRP flow in your own words
- [ ] Research: What is IRP_MJ_CREATE?

---

### Week 11-12: Filter Manager
**Goal**: Understand minifilter architecture

**Reading**:
- [ ] [Filter Manager Concepts](https://learn.microsoft.com/en-us/windows-hardware/drivers/ifs/filter-manager-concepts)
- [ ] [Minifilter Design Guide](https://learn.microsoft.com/en-us/windows-hardware/drivers/ifs/filter-manager-and-minifilter-driver-architecture)

**Key Concepts**:
- **Altitude**: Driver loading order (CURA should use 360000-389999 range for anti-virus)
- **Callbacks**: PreCreate, PostCreate, PreWrite, etc.
- **FltRegisterFilter**: How to register your minifilter

**Diagram**:
```
┌──────────────────┐
│  Application     │
└────────┬─────────┘
         ↓
┌────────┴─────────┐
│  Filter Manager  │
├──────────────────┤
│  CuraFilter      │ ← Altitude 365000
│  (Your Driver)   │
├──────────────────┤
│  Other Filters   │
└────────┬─────────┘
         ↓
┌────────┴─────────┐
│  File System     │
└──────────────────┘
```

**Exercise**:
- [ ] List all IRP_MJ_* types
- [ ] Identify which ones CURA needs to intercept

---

### Week 13-14: IRQL and Synchronization
**Goal**: Avoid BSOD from IRQL violations

**Reading**:
- [ ] Windows Internals Part 1, Chapter 5 (Threads)
- [ ] [IRQL Levels](https://learn.microsoft.com/en-us/windows-hardware/drivers/kernel/managing-hardware-priorities)

**Key Concepts**:
```
PASSIVE_LEVEL (0):   Normal user/kernel code
APC_LEVEL (1):       Async procedure calls
DISPATCH_LEVEL (2):  DPCs, cannot page fault
HIGH_LEVEL (31):     Critical interrupts
```

**Rules**:
- ❌ Cannot call pageable code at DISPATCH_LEVEL
- ❌ Cannot allocate paged pool at DISPATCH_LEVEL
- ✅ Must use spinlocks carefully

**Exercise**:
- [ ] Research: What happens if you page fault at DISPATCH_LEVEL?
- [ ] Answer: IRQL_NOT_LESS_OR_EQUAL BSOD

---

### Week 15-16: Memory Management
**Goal**: Understand kernel memory

**Reading**:
- [ ] Windows Internals Part 1, Chapter 5 (Memory)
- [ ] [Kernel Memory](https://learn.microsoft.com/en-us/windows-hardware/drivers/kernel/managing-memory-for-drivers)

**Key Concepts**:
- **Paged Pool**: Can be swapped to disk (use for large buffers)
- **Non-Paged Pool**: Always in RAM (use for critical data)
- **Pool Tags**: 4-character identifiers ('CURA')

**Code Pattern**:
```c
// Allocate non-paged memory
void* buffer = ExAllocatePool2(POOL_FLAG_NON_PAGED, 1024, 'CURA');

// Use buffer...

// FREE IT! (or memory leak)
ExFreePoolWithTag(buffer, 'CURA');
```

**Milestone**: Explain paged vs non-paged pool to a friend

---

## Month 5-6: First Driver

### Week 17-18: Empty Driver
**Goal**: Load a driver that does nothing

**Tasks**:
- [ ] Set up WDK environment
- [ ] Create driver project
- [ ] Implement `DriverEntry` and `DriverUnload`
- [ ] Build driver (targeting x64)
- [ ] Sign driver with test certificate

**Code**:
```rust
#![no_std]

#[no_mangle]
pub extern "system" fn DriverEntry(
    _driver_object: *mut DriverObject,
    _registry_path: *const UnicodeString,
) -> i32 {
    0 // STATUS_SUCCESS
}

#[no_mangle]
pub extern "system" fn DriverUnload(_driver_object: *mut DriverObject) {
    // Cleanup
}
```

**Commands**:
```powershell
# Build
cargo build --target x86_64-pc-windows-msvc

# Sign (test mode)
signtool sign /v /fd SHA256 /s "TestStore" /n "CURA Test" cura-filter.sys

# Load
sc create CuraFilter type=filesys binPath=C:\Path\cura-filter.sys
sc start CuraFilter

# Check status
sc query CuraFilter

# Unload
sc stop CuraFilter
sc delete CuraFilter
```

**Milestone**: See your driver in Device Manager!

---

### Week 19-20: Debug Output
**Goal**: Print messages from kernel

**Tasks**:
- [ ] Add debug output (DbgPrint equivalent in Rust)
- [ ] Use DebugView to see messages
- [ ] Connect WinDbg for debugging

**Code**:
```rust
// In your driver
kernel_print!("CURA: Driver loaded!\n");
```

**Verification**:
```
[CURA] Driver entry point called
[CURA] Initializing...
[CURA] Driver loaded successfully
```

**Debugging Setup**:
```powershell
# In VM
bcdedit /debug on
bcdedit /dbgsettings serial debugport:1 baudrate:115200

# In host WinDbg
File -> Kernel Debug -> COM -> Pipe: \\.\pipe\com_1
```

---

### Week 21-22: Filter Registration
**Goal**: Register as a minifilter

**Tasks**:
- [ ] Call `FltRegisterFilter`
- [ ] Specify altitude (365000 for AV)
- [ ] Implement callback registration (empty callbacks)

**Code**:
```rust
const ALTITUDE: &str = "365000"; // Anti-virus range

fn register_filter() -> Result<(), Error> {
    let registration = FLT_REGISTRATION {
        size: size_of::<FLT_REGISTRATION>(),
        version: FLT_REGISTRATION_VERSION,
        callbacks: &CALLBACKS,
        // ...
    };

    let status = unsafe {
        FltRegisterFilter(driver_object, &registration, &mut filter_handle)
    };

    // ...
}
```

**Verification**:
```powershell
# Check registered filters
fltmc filters

# Should see:
Filter Name       Altitude
CuraFilter        365000
```

---

### Week 23-24: Stability Testing
**Goal**: Ensure driver doesn't crash

**Tasks**:
- [ ] Load/unload driver 100 times
- [ ] Monitor for memory leaks (`!poolused`)
- [ ] Verify clean unload (no BSOD)

**Testing Script**:
```powershell
for ($i=1; $i -le 100; $i++) {
    Write-Host "Iteration $i"
    sc create CuraFilter type=filesys binPath=C:\test\cura.sys
    sc start CuraFilter
    Start-Sleep -Seconds 2
    sc stop CuraFilter
    sc delete CuraFilter
    Start-Sleep -Seconds 1
}
```

**Milestone**: Driver loads and unloads reliably!

---

## Month 7-8: File Blocking

### Week 25-26: PreCreate Callback
**Goal**: Intercept file creation

**Tasks**:
- [ ] Implement PreCreate callback
- [ ] Log all file paths being accessed
- [ ] Return FLT_PREOP_SUCCESS_NO_CALLBACK

**Code**:
```rust
fn pre_create_callback(
    data: &FLT_CALLBACK_DATA,
    flt_objects: &FLT_RELATED_OBJECTS,
) -> FLT_PREOP_CALLBACK_STATUS {
    // Get file path
    let path = get_file_path(data);

    kernel_print!("CURA: File accessed: {:?}\n", path);

    FLT_PREOP_SUCCESS_NO_CALLBACK
}
```

**Verification**:
```
[CURA] File accessed: C:\Users\Nora\test.txt
[CURA] File accessed: C:\Windows\System32\notepad.exe
```

---

### Week 27-28: Path Matching
**Goal**: Detect target filename

**Tasks**:
- [ ] Extract filename from path
- [ ] Compare against "file_rahasia.txt"
- [ ] Handle Unicode strings safely

**Code**:
```rust
fn should_block(path: &UnicodeString) -> bool {
    // Convert to Rust string
    let path_str = path.to_string().unwrap_or_default();

    // Check if contains target file
    path_str.to_lowercase().contains("file_rahasia.txt")
}
```

---

### Week 29-30: Blocking Logic
**Goal**: Actually block file access

**Tasks**:
- [ ] Return `STATUS_ACCESS_DENIED` for target file
- [ ] Allow all other files
- [ ] Test with real file operations

**Final Code**:
```rust
fn pre_create_callback(
    data: &FLT_CALLBACK_DATA,
    flt_objects: &FLT_RELATED_OBJECTS,
) -> FLT_PREOP_CALLBACK_STATUS {
    let path = get_file_path(data);

    if should_block(&path) {
        kernel_print!("CURA: BLOCKED: {:?}\n", path);

        // Block access
        data.IoStatus.Status = STATUS_ACCESS_DENIED;
        data.IoStatus.Information = 0;

        return FLT_PREOP_COMPLETE;
    }

    FLT_PREOP_SUCCESS_NO_CALLBACK
}
```

**Testing**:
```powershell
# In VM with driver loaded
New-Item -Path "C:\test\file_rahasia.txt" -ItemType File
# Should fail: Access denied

New-Item -Path "C:\test\normal.txt" -ItemType File
# Should succeed
```

---

### Week 31-32: Final Polish
**Goal**: Clean, stable, production-quality code

**Tasks**:
- [ ] Add comprehensive logging
- [ ] Handle edge cases (NULL pointers, etc.)
- [ ] Code review and cleanup
- [ ] Documentation

**Edge Cases**:
```rust
// Handle NULL pointers
if path_buffer.is_null() {
    return FLT_PREOP_SUCCESS_NO_CALLBACK;
}

// Handle empty paths
if path_length == 0 {
    return FLT_PREOP_SUCCESS_NO_CALLBACK;
}

// Handle invalid Unicode
let path = match unicode_to_string(path_buffer) {
    Ok(p) => p,
    Err(_) => return FLT_PREOP_SUCCESS_NO_CALLBACK,
};
```

---

## Phase 1 Milestone Checklist

### Knowledge
- [ ] Understand Rust ownership and borrowing
- [ ] Explain IRPs and filter manager
- [ ] Know IRQL levels and implications
- [ ] Understand kernel memory management

### Skills
- [ ] Write safe Rust code
- [ ] Work in `#![no_std]` environment
- [ ] Use WinDbg for kernel debugging
- [ ] Build and sign drivers

### Deliverables
- [ ] Kernel driver that loads/unloads cleanly
- [ ] Driver registers as minifilter
- [ ] PreCreate callback intercepts file I/O
- [ ] Successfully blocks "file_rahasia.txt"
- [ ] No crashes or BSODs during testing

### Demo
```
1. Boot VM
2. Load CuraFilter.sys
3. Attempt: echo "test" > file_rahasia.txt
   Result: Access denied
4. Attempt: echo "test" > normal.txt
   Result: Success
5. Unload driver
6. Attempt: echo "test" > file_rahasia.txt
   Result: Success (driver not running)
```

---

## Success Criteria

You can move to Phase 2 when:
1. ✅ Driver loads without errors
2. ✅ Driver blocks target file reliably
3. ✅ Driver allows all other files
4. ✅ No memory leaks (verified with `!poolused`)
5. ✅ No BSODs during normal operation
6. ✅ Code is clean and well-documented

---

## Common Pitfalls

### 1. Rushing Through Rust
**Mistake**: "I'll learn Rust while coding the driver"
**Reality**: You'll spend 10x longer debugging pointer issues
**Solution**: Spend full 2 months on Rust fundamentals

### 2. Testing on Host Windows
**Mistake**: "Just one quick test on my main PC"
**Reality**: BSOD and potential data corruption
**Solution**: ALWAYS use VM

### 3. Ignoring IRQL
**Mistake**: Calling pageable code at DISPATCH_LEVEL
**Reality**: Random BSODs that are hard to debug
**Solution**: Always check IRQL before allocations

### 4. Memory Leaks
**Mistake**: Forgetting to free allocations
**Reality**: System runs out of memory
**Solution**: Every `ExAllocatePool` needs `ExFreePool`

### 5. Skipping Documentation
**Mistake**: "I'll document later"
**Reality**: You forget how your own code works
**Solution**: Comment as you code

---

## Resources Summary

### Books (Buy These!)
1. **The Rust Programming Language** (Free online)
2. **Windows Internals Part 1** (Essential - ~$50)
3. **Windows Kernel Programming** by Pavel Yosifovich (~$40)

### Online Resources (Free)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Microsoft Learn - Driver Development](https://learn.microsoft.com/en-us/windows-hardware/drivers/)
- [OSR Online](https://www.osronline.com/)
- [windows-kernel-rs](https://github.com/not-matthias/windows-kernel-rs)

### Tools (Free)
- Visual Studio 2022 Community
- Windows Driver Kit (WDK)
- WinDbg Preview (Windows Store)
- DebugView (Sysinternals)

---

## Motivation

**8 months seems like forever.**

It is. But consider:
- Most people never start
- Most who start never finish
- You're building something extraordinary

**Break it down**:
- 8 months = 32 weeks
- 32 weeks = 224 days
- 1 day at a time

**Track progress**:
```
Week 1:  ▓░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 3%
Week 16: ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓░░░░░░░░░░░░░░░░ 50%
Week 32: ▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓ 100%
```

**You've got this.**

---

## Next Steps

1. Read this roadmap completely
2. Set up your environment (GETTING_STARTED.md)
3. Start Week 1: Install Rust
4. Begin Rustlings exercises
5. Update your progress weekly

**Good luck, Nora. Let's build CURA.**
