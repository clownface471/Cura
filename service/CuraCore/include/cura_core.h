/**
 * CURA Core - The Brain
 *
 * Layer: User Space Service (Ring 3)
 * Language: C++20
 * Purpose: AI Inference + Snapshot Management + Cloud Sync
 *
 * Architecture:
 *   Kernel (CuraFilter.sys)
 *        ↓ IOCTL
 *   Service (CuraCore.exe) ← YOU ARE HERE
 *        ↓ Named Pipe
 *   UI (CuraUI.exe)
 */

#pragma once

#include <cstdint>
#include <string>
#include <memory>

namespace cura {

// Version
constexpr uint32_t VERSION_MAJOR = 0;
constexpr uint32_t VERSION_MINOR = 1;
constexpr uint32_t VERSION_PATCH = 0;

/**
 * Hardware acceleration mode
 */
enum class GearMode {
    Eco,        // NPU/iGPU - Low power, web browsing
    Turbo       // RTX GPU - High power, gaming/scanning
};

/**
 * Threat assessment result
 */
enum class ThreatLevel {
    Clean = 0,      // 0-20%: Safe
    Suspicious = 1, // 20-50%: Monitor
    Risky = 2,      // 50-70%: Send to Vault
    Malicious = 3   // 70-100%: Block immediately
};

/**
 * Event from kernel driver
 */
struct KernelEvent {
    uint64_t timestamp;
    uint32_t process_id;
    uint32_t thread_id;

    enum class Type {
        FileCreate,
        FileWrite,
        FileDelete,
        ProcessCreate,
        ProcessTerminate,
        NetworkConnect
    } type;

    wchar_t path[260];  // MAX_PATH
    uint8_t data[512];  // Additional context
};

/**
 * Main CURA Core service
 * Runs as a protected Windows service
 */
class CuraService {
public:
    CuraService();
    ~CuraService();

    // Service lifecycle
    bool Initialize();
    void Run();
    void Shutdown();

    // Gear shifting (NPU ↔ RTX)
    void SetGearMode(GearMode mode);
    GearMode GetCurrentGear() const;

private:
    // Phase 2: AI Inference
    // std::unique_ptr<InferenceEngine> inference_engine_;

    // Phase 2: Snapshot management
    // std::unique_ptr<SnapshotManager> snapshot_manager_;

    // Phase 2: Communication with kernel driver
    // std::unique_ptr<KernelCommunicator> kernel_comm_;

    // Phase 3: Cloud sync
    // std::unique_ptr<CloudSync> cloud_sync_;

    GearMode current_gear_ = GearMode::Eco;
    bool is_running_ = false;
};

} // namespace cura
