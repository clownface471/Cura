cmake_minimum_required(VERSION 3.20)
project(CuraCore VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# CURA Core Service - The Brain
# AI Inference Engine + Snapshot Manager + Cloud Sync

# Platform check
if(NOT WIN32)
    message(FATAL_ERROR "CuraCore is Windows-only (requires Win32 API)")
endif()

# Output directories
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)

# Options
option(CURA_BUILD_TESTS "Build unit tests" ON)
option(CURA_ENABLE_GPU "Enable NVIDIA GPU acceleration" ON)
option(CURA_ENABLE_NPU "Enable Intel NPU support" ON)

# Dependencies (to be added in Phase 2)
# find_package(ONNXRuntime REQUIRED)
# find_package(OpenVINO QUIET)  # For NPU
# find_package(CUDA QUIET)       # For RTX

# Source files
set(CURA_SOURCES
    src/main.cpp
    # Phase 2+:
    # src/inference_engine.cpp
    # src/snapshot_manager.cpp
    # src/kernel_comm.cpp
    # src/cloud_sync.cpp
    # src/gear_shifter.cpp
)

# Headers
set(CURA_HEADERS
    include/cura_core.h
    # Phase 2+:
    # include/inference_engine.h
    # include/snapshot_manager.h
)

# Main executable
add_executable(CuraCore ${CURA_SOURCES} ${CURA_HEADERS})

target_include_directories(CuraCore PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/include
)

# Compiler flags
if(MSVC)
    target_compile_options(CuraCore PRIVATE
        /W4              # Warning level 4
        /WX              # Treat warnings as errors
        /permissive-     # Standards conformance
        /MP              # Multi-processor compilation
        $<$<CONFIG:Release>:/O2>     # Optimize for speed
        $<$<CONFIG:Release>:/GL>     # Whole program optimization
    )
    target_link_options(CuraCore PRIVATE
        $<$<CONFIG:Release>:/LTCG>   # Link-time code generation
    )
endif()

# Windows-specific libraries
target_link_libraries(CuraCore PRIVATE
    # Windows API
    # kernel32
    # advapi32    # For service management
    # vssapi      # For Volume Shadow Copy
    # winsock2    # For cloud communication
)

# Installation
install(TARGETS CuraCore
    RUNTIME DESTINATION bin
)

# Testing (Phase 3+)
if(CURA_BUILD_TESTS)
    # enable_testing()
    # add_subdirectory(tests)
endif()
