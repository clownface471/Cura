# CURA System Architecture

**The Digital Immune System**

## Philosophy

CURA is not an antivirus. CURA is a **Digital Immune System**.

- **The Curator's Eye**: Contextual behavior understanding, not pattern matching
- **The Curator's Hand**: Instant restoration over mere blocking
- **The Curator's Vault**: Active isolation and analysis

## The Stack

CURA operates across two privilege levels:

```
┌─────────────────────────────────────────────────────────┐
│                  USER SPACE (Ring 3)                    │
│                                                         │
│  ┌──────────────────┐         ┌──────────────────┐     │
│  │    CuraUI.exe    │◄───────►│  CuraCore.exe    │     │
│  │   (Flutter)      │         │  (C++ Service)   │     │
│  │                  │  Named  │                  │     │
│  │  - Dashboard     │  Pipe   │  - AI Inference  │     │
│  │  - Vault View    │         │  - Snapshots     │     │
│  │  - Settings      │         │  - Cloud Sync    │     │
│  └──────────────────┘         └────────┬─────────┘     │
│                                        │ IOCTL         │
├────────────────────────────────────────┼───────────────┤
│                  KERNEL SPACE (Ring 0) │               │
│                                        │               │
│                          ┌─────────────▼─────────┐     │
│                          │  CuraFilter.sys       │     │
│                          │  (Rust Driver)        │     │
│                          │                       │     │
│                          │  - File I/O Monitor   │     │
│                          │  - Process Monitor    │     │
│                          │  - Self-Defense       │     │
│                          └─────────┬─────────────┘     │
│                                    │                   │
│                          ┌─────────▼─────────┐         │
│                          │  Filter Manager   │         │
│                          └─────────┬─────────┘         │
│                                    │                   │
│                          ┌─────────▼─────────┐         │
│                          │  File System      │         │
│                          │  (NTFS/FAT32)     │         │
│                          └───────────────────┘         │
└─────────────────────────────────────────────────────────┘
```

## Three Layers

### Layer 1: The Sentinel (Kernel)

**Technology**: Rust, Windows Kernel Mode Driver
**File**: `CuraFilter.sys`
**Responsibility**: High-speed interception and telemetry

#### Capabilities
- Intercept all file operations (Create, Read, Write, Delete)
- Monitor process creation/termination
- Self-defense (prevent CURA from being killed)
- Zero AI - only Boolean logic

#### Data Flow
```
File Access Attempt
    ↓
CuraFilter PreCreate Callback
    ↓
Fast Boolean Check (Known Safe/Malicious)
    ↓
├─ If Known Good → ALLOW immediately
├─ If Known Bad  → BLOCK immediately
└─ If Unknown    → REPORT to CuraCore + Allow/Block pending decision
```

### Layer 2: The Brain (Service)

**Technology**: C++20, ONNX Runtime
**File**: `CuraCore.exe`
**Responsibility**: AI-powered threat analysis

#### Capabilities
- Receive telemetry from kernel driver
- Run ML inference on behavior sequences
- Manage snapshots (VSS integration)
- Cloud synchronization (licensing, model updates)
- Gear shifting (NPU ↔ RTX)

#### Threat Assessment Pipeline
```
Kernel Event Stream
    ↓
Feature Extraction
  (System call sequence: [OpenFile, WriteFile, NetworkConnect])
    ↓
Gear Selection
  ├─ Battery + Low CPU → NPU (Eco)
  └─ Gaming/Scanning   → RTX (Turbo)
    ↓
AI Inference (ONNX Runtime)
  (Transformer model trained on malware behaviors)
    ↓
Threat Score (0-100%)
  ├─ 0-20%:   Clean       → ALLOW
  ├─ 20-50%:  Suspicious  → MONITOR
  ├─ 50-70%:  Risky       → VAULT (sandbox)
  └─ 70-100%: Malicious   → BLOCK + RESTORE
```

### Layer 3: The Face (UI)

**Technology**: Flutter (Dart)
**File**: `CuraUI.exe`
**Responsibility**: User interaction and visualization

#### Capabilities
- Real-time system health visualization
- Threat history and analysis
- Vault (quarantine) management
- Settings and configuration
- Windows Toast notifications

## The Trinity Features

### 1. Contextual Anomaly Detection (The Eye)

**Principle**: Behavior > Signature

Traditional antivirus matches code signatures. CURA understands **context**.

**Example**:
```
Scenario A:
  Word.exe → OpenFile("document.docx") → ReadFile()
  Analysis: NORMAL

Scenario B:
  Calculator.exe → OpenFile("document.docx") → WriteFile() → NetworkConnect("russia.evil.com")
  Analysis: ANOMALY - Calculator should never touch documents or network
  Action: BLOCK
```

**Implementation**:
- Small Language Model (SLM) trained on system call sequences
- Input: Vector of recent operations
- Output: Anomaly score

### 2. Micro-Snapshot Restoration (The Hand)

**Principle**: Undo button for ransomware

Traditional antivirus blocks threats. CURA **reverses damage**.

**Trigger Detection**:
```
Monitor file entropy and bulk changes:
  - If process modifies >10 files in <5 seconds
  - AND entropy increases (encrypted = random)
  → RANSOMWARE DETECTED
```

**Response**:
1. **Suspend** malicious process
2. **Terminate** process
3. **Restore** files from VSS snapshot
4. **Notify** user: "Attack neutralized. Data restored."

**Technology**:
- Windows Volume Shadow Copy Service (VSS)
- Controlled by CuraFilter.sys (kernel)
- Hidden snapshots (user never sees them)

### 3. The Vault (Active Containment)

**Principle**: Glass prison for suspicious files

Traditional antivirus deletes suspicious files. CURA **observes them safely**.

**Flow**:
```
Unknown File (Threat: 50-70%)
    ↓
Do NOT block, redirect to Vault
    ↓
Lightweight VM/Sandbox
    ↓
Allow file to execute in isolation
    ↓
Monitor behavior:
  - Registry changes?
  - Network connections?
  - File modifications?
    ↓
After analysis:
  ├─ If benign  → Release to system, add to whitelist
  └─ If malicious → Create block rule, delete from vault
```

**Rendering**: RTX GPU used to accelerate sandbox simulation

## Hybrid Hardware Strategy

### Gear Shifting

CURA adapts to hardware state:

| Mode | Condition | Hardware | Framework | Impact |
|------|-----------|----------|-----------|--------|
| **Eco** | Battery, idle, browsing | NPU (Intel AI Boost) | OpenVINO | Low power, silent |
| **Turbo** | Charging, gaming, scanning | RTX 5060 (Tensor Cores) | TensorRT/CUDA | Max performance |

**Auto-Switching Logic**:
```cpp
if (on_battery && cpu_usage < 30% && !user_gaming) {
    SetGear(Eco);   // NPU
} else if (deep_scan_requested || anomaly_detected) {
    SetGear(Turbo); // RTX
}
```

### Performance Targets

| Metric | Eco (NPU) | Turbo (RTX) |
|--------|-----------|-------------|
| Inference Time | <100ms | <50ms |
| Power Draw | ~3W | ~60W |
| Model Size | INT8 (150MB) | INT8 (150MB) |
| RAM Usage | <400MB | <400MB |
| VRAM Usage | 0 (uses system RAM) | ~500MB |

## Security & IP Protection

### Model Encryption

Models are valuable IP. They must be protected.

**Threat**: User extracts `model.onnx` and shares it

**Solution**:
```
Original Model (model.onnx)
    ↓
Encrypt with AES-256 (key stored on cloud)
    ↓
Encrypted Model (model.enc)
    ↓
Runtime: Decrypt in TEE (Trusted Execution Environment)
  - Intel SGX (if available)
  - Windows VBS (Virtualization Based Security)
  - Or obfuscated decryption routine
    ↓
Load into ONNX Runtime (in-memory only)
```

**Key Management**:
- Decryption key tied to **Hardware ID**
- Requires **license server token** (renewed every 7 days)
- No internet for >7 days → AI stops working (kernel continues)

### Anti-Piracy

**The Heartbeat**:
```
Every 7 days:
  1. CuraCore connects to license server
  2. Sends: Hardware ID + License Key
  3. Receives: Encrypted token (valid for 7 days)
  4. Stores token (encrypted in registry)

If token expires:
  - AI inference disabled
  - Kernel driver continues (basic protection)
  - User gets warning: "Reconnect to internet to restore full protection"
```

## Data Flow Example

### Scenario: User downloads suspicious file

```
1. Browser saves "crack.exe" to Downloads/
      ↓
2. CuraFilter.sys intercepts CreateFile()
      ↓
3. Quick check: Not in whitelist/blacklist
      ↓
4. Send to CuraCore via IOCTL
      ↓
5. CuraCore extracts features:
   - File entropy: High (packed/obfuscated)
   - Digital signature: None
   - Download source: Untrusted domain
      ↓
6. Run AI inference (Turbo mode, RTX GPU)
      ↓
7. Result: Threat score 65% (RISKY)
      ↓
8. Decision: Send to VAULT
      ↓
9. CuraCore instructs kernel:
   - Create isolated directory
   - Redirect file access to vault
      ↓
10. File executes in sandbox
      ↓
11. Behavior observed:
    - Attempts to modify Registry
    - Connects to C2 server
    - Scans for other .exe files
      ↓
12. Analysis: CONFIRMED MALWARE
      ↓
13. Actions:
    - Terminate process in vault
    - Add signature to blacklist
    - Delete file
    - Notify user: "Malware blocked in Vault"
      ↓
14. UI shows notification
```

## Memory Budget (16GB RAM)

Total system RAM: **16GB**

Allocation strategy:
```
Windows OS:          ~4GB
Gaming (LoL/Valorant): ~6GB
Chrome (tabs):       ~2GB
─────────────────────────
Available for CURA:  ~4GB
```

CURA budget:
```
CuraFilter.sys (kernel): ~50MB   (non-paged pool)
CuraCore.exe (service):  ~600MB
  ├─ Base application:   ~100MB
  ├─ AI model:           ~400MB  (INT8 quantized)
  └─ Telemetry buffers:  ~100MB
CuraUI.exe (Flutter):    ~200MB
─────────────────────────────
Total:                   ~850MB (~5% of system RAM)
```

**Justification**: Even during gaming, CURA uses <1GB, leaving plenty for games.

## File System Layout

### Installation Directory
```
C:\Program Files\CURA\
├── CuraCore.exe          # Service
├── CuraUI.exe            # UI
├── models\
│   └── classifier.enc    # Encrypted AI model
├── licenses\
│   └── license.key
└── config\
    └── settings.json
```

### Driver Directory
```
C:\Windows\System32\drivers\
└── CuraFilter.sys        # Kernel driver
```

### User Data
```
C:\ProgramData\CURA\
├── logs\
│   ├── kernel.log
│   └── service.log
├── snapshots\            # VSS snapshot metadata
├── vault\                # Quarantined files
│   └── suspicious.exe
└── telemetry\
    └── events.db         # Local event database
```

## Development Principles

1. **Security First**
   - Memory safety (Rust for kernel)
   - Input validation everywhere
   - Least privilege

2. **Performance Matters**
   - <50ms inference time (Turbo)
   - Zero UI lag
   - Minimal battery impact

3. **User Experience**
   - Invisible when working
   - Clear when acting
   - Never annoying

4. **Fail Safe**
   - If AI fails → kernel continues (basic protection)
   - If license expires → AI stops, kernel continues
   - If crash → system remains stable (no BSOD)

## Future Enhancements

### Post-Launch (Year 2+)
- **Cloud Vault**: Upload suspicious files to cloud for analysis
- **Community Intelligence**: Share threat signatures (privacy-preserved)
- **Zero-Day Detection**: Detect unknown malware using behavioral analysis
- **Mobile Support**: Extend to Android (kernel module + ONNX Runtime)

---

**Remember**: CURA is not just software. It's a living system that learns, adapts, and protects.
