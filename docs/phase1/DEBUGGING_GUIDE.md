# Kernel Driver Debugging Guide

**Essential guide for debugging Windows kernel drivers**

⚠️ **CRITICAL**: All debugging must be done in a Virtual Machine. NEVER on host Windows!

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [VM Setup](#vm-setup)
3. [WinDbg Setup](#windbg-setup)
4. [Debugging Workflow](#debugging-workflow)
5. [Common BSOD Codes](#common-bsod-codes)
6. [Debugging Commands](#debugging-commands)
7. [Troubleshooting](#troubleshooting)

---

## Prerequisites

### Required Software

**On Host Machine:**
- WinDbg Preview (from Microsoft Store)
- VMware Workstation / Hyper-V / VirtualBox
- Visual Studio 2022 + WDK

**On VM:**
- Windows 10/11 (any edition)
- Test signing enabled
- Kernel debugging enabled

---

## VM Setup

### Step 1: Create Virtual Machine

**Recommended Specs:**
- OS: Windows 10/11
- RAM: 4GB minimum
- CPU: 2 cores
- Disk: 40GB
- Network: NAT or Bridged

### Step 2: Enable Test Signing

**In VM, run as Administrator:**

```powershell
# Enable test signing (allows loading unsigned drivers)
bcdedit /set testsigning on

# Reboot
shutdown /r /t 0
```

After reboot, you'll see "Test Mode" watermark in bottom-right corner.

### Step 3: Enable Kernel Debugging

Choose one method:

#### Option A: Serial Port Debugging (VMware/VirtualBox)

**In VM:**
```powershell
# Enable debugging
bcdedit /debug on

# Configure serial port
bcdedit /dbgsettings serial debugport:1 baudrate:115200

# Reboot
shutdown /r /t 0
```

**In VMware:**
1. VM Settings → Add → Serial Port
2. Use named pipe: `\\.\pipe\com_1`
3. This end is server
4. The other end is application

**In VirtualBox:**
1. VM Settings → Serial Ports → Port 1
2. Enable Serial Port
3. Port Mode: Host Pipe
4. Path: `\\.\pipe\com_1`

#### Option B: Network Debugging (Hyper-V)

**In VM:**
```powershell
# Generate a key (one time)
bcdedit /dbgsettings net hostip:192.168.1.100 port:50000

# It will output a key like: 1.2.3.4
# Save this key!

# Enable debugging
bcdedit /debug on

shutdown /r /t 0
```

---

## WinDbg Setup

### Install WinDbg Preview

**On Host Machine:**
1. Open Microsoft Store
2. Search "WinDbg Preview"
3. Install

OR download from:
https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/

### Connect to VM

#### Serial Port Connection

1. Launch WinDbg Preview
2. File → Attach to kernel
3. Select "COM" tab
4. Port: `com1` or pipe `\\.\pipe\com_1`
5. Baud rate: `115200`
6. Click OK

#### Network Connection

1. Launch WinDbg Preview
2. File → Attach to kernel
3. Select "NET" tab
4. Port: `50000`
5. Key: (paste the key from bcdedit command)
6. Click OK

###Connection Status

When connected, you'll see:
```
Microsoft (R) Windows Debugger Version 10.0...
Connected to Windows 10 ... Kernel Version ...
Symbol search path is: srv*
```

Press Ctrl+Break or click "Break" to pause the kernel and get a command prompt:
```
kd>
```

---

## Debugging Workflow

### Basic Workflow

```
1. Build driver on host
2. Copy .sys file to VM
3. Connect WinDbg to VM
4. Load driver in VM
5. Set breakpoints in WinDbg
6. Trigger driver code
7. Debug in WinDbg
8. Unload driver
```

### Example Session

**1. In WinDbg (host):**

```
kd> .reload                      # Reload symbols
kd> lm m CuraFilter              # List CuraFilter module
kd> bp CuraFilter!DriverEntry    # Set breakpoint
kd> g                             # Continue execution
```

**2. In VM:**

```powershell
# Load driver
sc create CuraFilter type=filesys binPath=C:\CURA\CuraFilter.sys
sc start CuraFilter
```

**3. WinDbg breaks at DriverEntry:**

```
Breakpoint 0 hit
CuraFilter!DriverEntry:
fffff800`12345678  mov rax, rsp
```

**4. Step through code:**

```
kd> p         # Step over
kd> t         # Step into
kd> g         # Continue
kd> k         # Stack trace
```

---

## Common BSOD Codes

### 0x0000000A: IRQL_NOT_LESS_OR_EQUAL

**Cause:** Accessed paged memory at too high IRQL

**Common causes:**
- Called pageable function at DISPATCH_LEVEL
- Touched paged pool at DISPATCH_LEVEL
- Dereferenced invalid pointer

**Debug:**
```
kd> !analyze -v
kd> k                      # Check call stack
kd> !irql                  # Check current IRQL
```

**Fix:** Lower IRQL or use non-paged memory

---

### 0x00000050: PAGE_FAULT_IN_NONPAGED_AREA

**Cause:** Accessed invalid memory address

**Common causes:**
- Dereferenced NULL pointer
- Freed memory then accessed it
- Pointer arithmetic error
- Uninitialized pointer

**Debug:**
```
kd> !analyze -v
kd> k                      # Call stack
kd> r                      # Registers
kd> u @rip                 # Disassemble at crash
```

**Fix:** Validate all pointers before dereferencing

---

### 0x000000C4: DRIVER_VERIFIER_DETECTED_VIOLATION

**Cause:** Driver Verifier found a bug

**Common causes:**
- Memory leak
- Double free
- Incorrect IRQL
- Invalid handle

**Debug:**
```
kd> !analyze -v
kd> !verifier 3            # Show verifier details
```

**Fix:** Fix the specific violation reported

---

### 0x000000D1: DRIVER_IRQL_NOT_LESS_OR_EQUAL

**Cause:** Driver accessed memory at wrong IRQL

**Similar to 0x0A but specifically for drivers**

**Debug:**
```
kd> !analyze -v
kd> k
kd> !irql
```

---

## Debugging Commands

### Essential Commands

#### Module Commands
```
lm                          # List all loaded modules
lm m CuraFilter             # List CuraFilter module
.reload /f CuraFilter.sys   # Force reload CuraFilter symbols
```

#### Breakpoint Commands
```
bp CuraFilter!DriverEntry            # Set breakpoint
bp CuraFilter!PreCreateCallback      # Set breakpoint on function
bl                                    # List breakpoints
bc *                                  # Clear all breakpoints
bd 0                                  # Disable breakpoint 0
be 0                                  # Enable breakpoint 0
```

#### Execution Commands
```
g                           # Go (continue execution)
p                           # Step over
t                           # Step into
gu                          # Step out
```

#### Inspection Commands
```
k                           # Stack trace
kv                          # Stack trace (verbose)
r                           # Show registers
r rax                       # Show RAX register
u @rip                      # Disassemble at instruction pointer
u CuraFilter!DriverEntry    # Disassemble function
```

#### Memory Commands
```
db ADDRESS                  # Display bytes
dd ADDRESS                  # Display dwords
dq ADDRESS                  # Display qwords
du ADDRESS                  # Display Unicode string
da ADDRESS                  # Display ASCII string
```

####Driver-Specific Commands
```
!drvobj CuraFilter 7        # Inspect driver object
!devobj DEVICE              # Inspect device object
!irp ADDRESS                # Inspect IRP
!process 0 0                # List all processes
!thread ADDRESS             # Inspect thread
!pool ADDRESS               # Inspect pool allocation
!poolused                   # Show pool usage
```

### Advanced Commands

#### Filter Manager
```
!fltkd.filters              # List all minifilters
!fltkd.filter CuraFilter    # Show CuraFilter details
!fltkd.volumes              # List volumes
```

#### Crash Analysis
```
!analyze -v                 # Analyze crash (verbose)
.bugcheck                   # Show bugcheck code
.lastevent                  # Show last event
```

#### Symbol Commands
```
.sympath                    # Show symbol path
.reload                     # Reload symbols
ld CuraFilter               # Load symbols for module
x CuraFilter!*              # List all symbols in module
```

---

## Debugging Scenarios

### Scenario 1: Driver Won't Load

**Symptoms:** `sc start` fails with error

**Debug:**
```powershell
# In VM, check error
sc query CuraFilter

# In WinDbg
kd> !drvobj CuraFilter 7
```

**Common causes:**
- DriverEntry returned error
- Registration failed
- Missing dependencies

---

### Scenario 2: System Hangs

**Symptoms:** VM freezes, no BSOD

**Debug:**
1. Break into WinDbg (Ctrl+Break)
2. Check what's running:
```
kd> !process 0 0
kd> !thread -t
kd> k
```

**Common causes:**
- Infinite loop
- Deadlock (waiting for lock forever)
- Spinlock held too long

---

### Scenario 3: Memory Corruption

**Symptoms:** Random crashes, data corruption

**Debug:**
```
kd> !pool ADDRESS          # Check pool corruption
kd> !poolused              # Check for leaks
kd> !verifier 3            # Enable Driver Verifier
```

**Tools:**
- Enable Driver Verifier (detects memory issues)
- Use pool tagging to track allocations

---

## Using DebugView

**DebugView** is simpler than WinDbg - shows kernel debug output in real-time.

### Setup

1. Download from Sysinternals:
   https://learn.microsoft.com/en-us/sysinternals/downloads/debugview

2. Run as Administrator

3. Capture → Capture Kernel
4. Capture → Enable Verbose Kernel Output

### In Your Driver

```rust
// In kernel driver (pseudo-code)
DbgPrint("[CURA] Driver loaded\n");
DbgPrint("[CURA] File access: %wZ\n", &fileName);
```

### Output

You'll see in DebugView:
```
[CURA] Driver loaded
[CURA] File access: C:\test.txt
```

---

## Troubleshooting

### "Cannot connect to kernel"

**Cause:** VM not configured for debugging

**Fix:**
1. Check bcdedit settings in VM
2. Verify serial port / network settings
3. Reboot VM
4. Try different connection method

---

### "Symbols not loading"

**Cause:** Symbol path not configured

**Fix:**
```
kd> .sympath srv*c:\symbols*https://msdl.microsoft.com/download/symbols
kd> .reload
```

---

### "Breakpoint not hit"

**Cause:** Code not executing or symbols wrong

**Fix:**
1. Check module loaded: `lm m CuraFilter`
2. Reload symbols: `.reload /f CuraFilter.sys`
3. Verify function name: `x CuraFilter!*`
4. Check if code path executed

---

## Safety Checklist

Before debugging:

- [ ] Using VM (not host Windows!)
- [ ] Have VM snapshot (can restore if broken)
- [ ] WinDbg connected successfully
- [ ] Symbols loading correctly
- [ ] Driver signed (test signing enabled)

---

## Quick Reference Card

```
┌─────────────────────────────────────┐
│   WinDbg Quick Reference            │
├─────────────────────────────────────┤
│ g              Continue              │
│ p              Step over             │
│ t              Step into             │
│ k              Stack trace           │
│ r              Registers             │
│ u @rip         Disassemble           │
│ bp ADDRESS     Breakpoint            │
│ !analyze -v    Analyze crash         │
│ !process 0 0   List processes        │
│ !drvobj NAME 7 Inspect driver        │
│ .reload        Reload symbols        │
└─────────────────────────────────────┘
```

---

## Resources

### Official Documentation
- [WinDbg Documentation](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/)
- [Driver Debugging Techniques](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/debugging-techniques)

### Videos
- Search YouTube: "WinDbg kernel debugging tutorial"
- OSR Online videos

### Communities
- [OSR Online](https://community.osr.com/)
- [/r/kernel](https://reddit.com/r/kernel)

---

**Remember**: Kernel debugging is an essential skill. Master it early!

You'll spend 50% of Phase 1 Month 5-8 in WinDbg. Get comfortable with it now.
