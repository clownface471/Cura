/**
 * CURA Core - Service Entry Point
 *
 * This runs as a Windows service (not a console app)
 * It receives telemetry from the kernel driver and runs AI inference
 */

#include "cura_core.h"
#include <iostream>
#include <Windows.h>

// Phase 1: Skeleton only
// You will implement the actual service logic in Phase 2

namespace cura {

CuraService::CuraService() {
    // TODO Phase 2: Initialize components
}

CuraService::~CuraService() {
    // TODO Phase 2: Cleanup
}

bool CuraService::Initialize() {
    std::cout << "[CURA Core] Initializing...\n";
    std::cout << "[CURA Core] Version: "
              << VERSION_MAJOR << "."
              << VERSION_MINOR << "."
              << VERSION_PATCH << "\n";

    // TODO Phase 2:
    // [ ] Load AI model (encrypted)
    // [ ] Initialize ONNX Runtime
    // [ ] Connect to kernel driver (DeviceIoControl)
    // [ ] Initialize VSS for snapshots
    // [ ] Start heartbeat to license server

    return true;
}

void CuraService::Run() {
    is_running_ = true;

    std::cout << "[CURA Core] Service running...\n";
    std::cout << "[CURA Core] Current gear: "
              << (current_gear_ == GearMode::Eco ? "Eco" : "Turbo") << "\n";

    // TODO Phase 2:
    // Main event loop
    // while (is_running_) {
    //     1. Receive event from kernel driver
    //     2. Run AI inference
    //     3. Decide: Allow / Block / Send to Vault
    //     4. Send response back to kernel
    //     5. Update UI via named pipe
    // }
}

void CuraService::Shutdown() {
    std::cout << "[CURA Core] Shutting down...\n";
    is_running_ = false;

    // TODO Phase 2: Cleanup
}

void CuraService::SetGearMode(GearMode mode) {
    if (current_gear_ == mode) return;

    std::cout << "[CURA Core] Switching gear: "
              << (mode == GearMode::Eco ? "Eco" : "Turbo") << "\n";

    current_gear_ = mode;

    // TODO Phase 2:
    // if (mode == GearMode::Eco) {
    //     inference_engine_->UseNPU();
    // } else {
    //     inference_engine_->UseRTX();
    // }
}

GearMode CuraService::GetCurrentGear() const {
    return current_gear_;
}

} // namespace cura

// Entry point
int main(int argc, char* argv[]) {
    std::cout << R"(
   ____  _   _ ____      _
  / ___|  | | |  _ \    / \
 | |  | | | | |_) |  / _ \
 | |__| |_| |  _ <  / ___ \
  \____\___/|_| \_\/_/   \_\

The Digital Immune System
    )" << "\n";

    // Phase 1: Just a console app for now
    // Phase 3: Will become actual Windows service

    cura::CuraService service;

    if (!service.Initialize()) {
        std::cerr << "[ERROR] Failed to initialize\n";
        return 1;
    }

    // For now, just test basic functionality
    service.SetGearMode(cura::GearMode::Eco);

    std::cout << "\n[CURA Core] Phase 1 Skeleton - Press Enter to exit\n";
    std::cin.get();

    service.Shutdown();

    return 0;
}
