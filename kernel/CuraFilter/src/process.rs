// process.rs - Process monitoring
//
// Track process creation and termination

/// Process information
pub struct ProcessInfo {
    pub process_id: u32,
    pub parent_process_id: u32,
    pub image_name: &'static str,
}

impl ProcessInfo {
    pub fn new(pid: u32, parent_pid: u32, name: &'static str) -> Self {
        Self {
            process_id: pid,
            parent_process_id: parent_pid,
            image_name: name,
        }
    }
}

/// Process create callback
///
/// Phase 2+: This will send telemetry to CuraCore service
pub fn process_create_callback(process: &ProcessInfo) {
    // TODO Phase 2:
    // - Send to CuraCore via IOCTL
    // - Track process lineage (parent -> child)
    // - Detect suspicious process chains

    kernel_log!(
        "CURA: Process created: {} (PID: {})",
        process.image_name,
        process.process_id
    );
}

/// Process terminate callback
pub fn process_terminate_callback(process_id: u32) {
    kernel_log!("CURA: Process terminated: PID {}", process_id);

    // TODO Phase 2:
    // - Clean up per-process resources
    // - Report to CuraCore
}

/// Check if process is suspicious based on name
///
/// Phase 2: This will be replaced by AI-based detection
pub fn is_suspicious_process(name: &str) -> bool {
    let suspicious_names = [
        "cryptolocker",
        "ransomware",
        "malware",
    ];

    let name_lower = name.to_ascii_lowercase();
    suspicious_names.iter()
        .any(|&pattern| name_lower.contains(pattern))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suspicious_process_detection() {
        assert!(is_suspicious_process("cryptolocker.exe"));
        assert!(is_suspicious_process("RANSOMWARE.EXE"));
        assert!(!is_suspicious_process("notepad.exe"));
        assert!(!is_suspicious_process("chrome.exe"));
    }
}
