# CuraCore - The Brain

**Layer: User Space Service (Ring 3)**

## Overview

CuraCore is the intelligence center of CURA. It receives telemetry from the kernel driver, runs AI inference, and makes decisions about threats.

```
┌─────────────────────────────────────┐
│  CuraCore.exe (Protected Service)   │
├─────────────────────────────────────┤
│                                     │
│  ┌────────────────────────────┐    │
│  │   Inference Engine         │    │
│  │   ┌──────┐    ┌─────────┐  │    │
│  │   │ NPU  │◄──►│   RTX   │  │    │
│  │   │(Eco) │    │ (Turbo) │  │    │
│  │   └──────┘    └─────────┘  │    │
│  │   ONNX Runtime             │    │
│  └────────────────────────────┘    │
│               ▲                     │
│               │                     │
│  ┌────────────┼─────────────┐      │
│  │  Snapshot Manager         │      │
│  │  (VSS Integration)        │      │
│  └───────────────────────────┘      │
│               ▲                     │
│               │                     │
│  ┌────────────┼─────────────┐      │
│  │  Kernel Communicator      │      │
│  │  (IOCTL Handler)          │      │
│  └───────────────────────────┘      │
│               ▲                     │
└───────────────┼─────────────────────┘
                │
        ┌───────┴───────┐
        │ CuraFilter.sys │
        └───────────────┘
```

## Responsibilities

### 1. AI Inference
- Receive behavior sequences from kernel
- Run threat classification model
- Return verdict: Clean / Suspicious / Risky / Malicious

### 2. Gear Shifting (Hybrid Hardware)
- **Eco Mode**: Use NPU (Intel AI Boost) for battery savings
- **Turbo Mode**: Use RTX 5060 for maximum performance
- Auto-switch based on power state and workload

### 3. Snapshot Management
- Integrate with Windows VSS (Volume Shadow Copy)
- Create micro-snapshots before risky operations
- Instant rollback if ransomware detected

### 4. Cloud Communication
- License validation heartbeat (every 7 days)
- Model updates (encrypted)
- Threat intelligence sync

## Phase 2 Goals (Months 9-16)

### Month 9-10: Dataset Research
- [ ] Download EMBER or SOREL-20M dataset
- [ ] Study malware behavior patterns
- [ ] Prepare training data (system call sequences)

### Month 11-12: Model Training
- [ ] Design small Transformer architecture
- [ ] Train on malware behaviors (PyTorch)
- [ ] Quantize to INT8 for low memory (<400MB)

### Month 13-14: ONNX Integration
- [ ] Export model to ONNX format
- [ ] Integrate ONNX Runtime in C++
- [ ] Implement NPU backend (OpenVINO)
- [ ] Implement RTX backend (TensorRT/CUDA)

### Month 15-16: Kernel Communication
- [ ] Create IOCTL interface
- [ ] Receive events from CuraFilter.sys
- [ ] Send verdicts back to kernel
- [ ] **MILESTONE**: CLI app that detects malware using GPU

## Architecture Details

### Threat Classification Pipeline

```
Kernel Event → Feature Extraction → Model Inference → Verdict
     ↓                 ↓                    ↓            ↓
FileCreate      [OpenFile,         NPU/RTX         Clean (0-20%)
ProcessCreate    WriteFile,        Inference       Suspicious (20-50%)
NetworkConn      Connect]                          Risky (50-70%)
                                                   Malicious (70-100%)
```

### Gear Shifting Logic

```cpp
// Auto gear shifting
if (on_battery && cpu_usage < 30%) {
    SetGearMode(GearMode::Eco);   // NPU
} else if (gaming_detected || deep_scan) {
    SetGearMode(GearMode::Turbo); // RTX
}
```

### Model Encryption (Phase 4)

Models are encrypted with AES-256:
```
model.onnx (plain) → Encrypt with key → model.enc
                                           ↓
                              Decrypt at runtime (TEE)
                                           ↓
                              Load into ONNX Runtime
```

## Dependencies

### Phase 2
- **ONNX Runtime** (AI inference)
  - `onnxruntime-win-x64-gpu` (CUDA/TensorRT)
  - `onnxruntime-openvino` (NPU support)

### Phase 3
- **Windows VSS API** (snapshots)
- **WinHTTP** (cloud communication)

### Phase 4
- **Intel SGX** or **Windows VBS** (model decryption in TEE)

## Build Instructions

### Prerequisites
- Visual Studio 2022 (C++ Desktop Development)
- CMake 3.20+
- Windows SDK 10.0.22621.0+

### Build
```powershell
# Create build directory
mkdir build
cd build

# Configure
cmake .. -G "Visual Studio 17 2022" -A x64

# Build
cmake --build . --config Release

# Run
.\bin\Release\CuraCore.exe
```

## Model Specifications

### Target Model Size
- **Disk**: ~150MB (quantized INT8)
- **RAM**: <400MB during inference
- **VRAM**: ~500MB (when using RTX)

### Inference Speed Targets
- **NPU (Eco)**: <100ms per event
- **RTX (Turbo)**: <50ms per event

### Accuracy Targets (Phase 2)
- **True Positive Rate**: >95% (catch malware)
- **False Positive Rate**: <1% (don't block legitimate apps)

## Service Architecture (Phase 3)

CuraCore will run as a Windows service:

```cpp
// Service main
SERVICE_TABLE_ENTRY ServiceTable[] = {
    {L"CuraCore", ServiceMain},
    {NULL, NULL}
};

StartServiceCtrlDispatcher(ServiceTable);
```

Features:
- Auto-start on boot
- Restart on failure
- Protected from termination (self-defense)

## Communication Protocols

### Kernel → Service (IOCTL)
```cpp
// Open driver handle
HANDLE hDriver = CreateFile(L"\\\\.\\CuraFilter",
    GENERIC_READ | GENERIC_WRITE, 0, NULL,
    OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, NULL);

// Receive event
DWORD bytesReturned;
KernelEvent event;
DeviceIoControl(hDriver, IOCTL_CURA_GET_EVENT,
    NULL, 0, &event, sizeof(event),
    &bytesReturned, NULL);
```

### Service → UI (Named Pipe)
```cpp
// Create pipe
HANDLE hPipe = CreateNamedPipe(L"\\\\.\\pipe\\CuraUI",
    PIPE_ACCESS_DUPLEX, PIPE_TYPE_MESSAGE,
    1, 4096, 4096, 0, NULL);

// Send status
WriteFile(hPipe, &status, sizeof(status), &written, NULL);
```

## Testing Strategy

### Unit Tests (Phase 2)
- Test inference engine with known malware samples
- Validate gear switching logic
- Test IOCTL communication (loopback)

### Integration Tests (Phase 3)
- End-to-end: Kernel → Service → UI
- Snapshot creation/restoration
- License validation

### Performance Tests (Phase 3)
- Measure inference latency
- GPU vs NPU benchmarks
- Memory usage profiling

## File Structure

```
CuraCore/
├── include/
│   ├── cura_core.h           # Main service class
│   ├── inference_engine.h    # AI inference
│   ├── snapshot_manager.h    # VSS integration
│   ├── kernel_comm.h         # IOCTL handler
│   └── cloud_sync.h          # License & updates
├── src/
│   ├── main.cpp              # Entry point
│   ├── inference_engine.cpp
│   ├── snapshot_manager.cpp
│   ├── kernel_comm.cpp
│   ├── cloud_sync.cpp
│   └── gear_shifter.cpp      # NPU/RTX switching
├── models/
│   └── threat_classifier.enc # Encrypted model
├── tests/
│   └── unit_tests.cpp
├── CMakeLists.txt
└── README.md (this file)
```

## Security Considerations

1. **Model Protection**
   - Never store model in plaintext
   - Decrypt only in memory (TEE if available)

2. **Service Protection**
   - Run with SYSTEM privileges
   - Implement anti-tampering checks
   - Validate all input from kernel

3. **Memory Safety**
   - Use smart pointers (`std::unique_ptr`, `std::shared_ptr`)
   - Avoid raw `new`/`delete`
   - RAII everywhere

## Resources

### ONNX Runtime
- [Documentation](https://onnxruntime.ai/docs/)
- [C++ API](https://onnxruntime.ai/docs/api/c/)

### VSS (Volume Shadow Copy)
- [VSS API Reference](https://learn.microsoft.com/en-us/windows/win32/vss/volume-shadow-copy-service-portal)

### Windows Services
- [Service Programs](https://learn.microsoft.com/en-us/windows/win32/services/service-programs)

---

**Remember**: The Brain must be fast (<50ms), accurate (>95% detection), and secure (encrypted models).
