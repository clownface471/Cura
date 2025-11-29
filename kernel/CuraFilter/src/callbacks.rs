// callbacks.rs - Filter Manager callback implementations
//
// This module implements the actual file system callbacks

/// Callback result
#[repr(i32)]
pub enum CallbackStatus {
    Allow = 0,          // FLT_PREOP_SUCCESS_NO_CALLBACK
    Block = 1,          // FLT_PREOP_COMPLETE
    Pending = 2,        // FLT_PREOP_PENDING (for async operations)
}

/// File operation types
#[derive(Debug, Copy, Clone)]
pub enum FileOperation {
    Create,
    Read,
    Write,
    Delete,
    SetInfo,
    Close,
}

/// File operation context
pub struct FileContext {
    pub operation: FileOperation,
    pub path: &'static str,  // In real driver, this would be UNICODE_STRING
    pub process_id: u32,
}

impl FileContext {
    pub fn new(operation: FileOperation, path: &'static str, process_id: u32) -> Self {
        Self {
            operation,
            path,
            process_id,
        }
    }
}

/// PreCreate callback - called before file is created/opened
///
/// This is where we decide: Allow or Block?
pub fn pre_create_callback(context: &FileContext) -> CallbackStatus {
    // Phase 1 Month 7-8 Goal: Block "file_rahasia.txt"

    // TODO: Implement actual blocking logic
    // For now, this is a skeleton

    // Extract filename from path
    if path_contains_pattern(context.path, "rahasia") {
        kernel_log!("CURA: BLOCKED file access: {}", context.path);
        return CallbackStatus::Block;
    }

    // Allow by default
    CallbackStatus::Allow
}

/// PreWrite callback - called before data is written
pub fn pre_write_callback(context: &FileContext) -> CallbackStatus {
    // Phase 2+: Detect ransomware by monitoring bulk encryption

    // For Phase 1, just allow
    CallbackStatus::Allow
}

/// PreSetInformation callback - called before file is renamed/deleted
pub fn pre_setinfo_callback(context: &FileContext) -> CallbackStatus {
    // Phase 2+: Detect file deletion patterns

    CallbackStatus::Allow
}

/// Helper: Check if path contains a pattern (case-insensitive)
fn path_contains_pattern(path: &str, pattern: &str) -> bool {
    // Convert to lowercase for case-insensitive comparison
    let path_lower = path.to_ascii_lowercase();
    let pattern_lower = pattern.to_ascii_lowercase();

    path_lower.contains(&pattern_lower)
}

/// Macro for kernel logging (to be implemented with actual DbgPrint)
#[macro_export]
macro_rules! kernel_log {
    ($($arg:tt)*) => {
        // TODO Phase 1 Month 5: Replace with actual DbgPrint
        // For now, this is a no-op in kernel
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_contains_pattern() {
        assert!(path_contains_pattern("C:\\file_rahasia.txt", "rahasia"));
        assert!(path_contains_pattern("C:\\FILE_RAHASIA.TXT", "rahasia"));
        assert!(!path_contains_pattern("C:\\normal.txt", "rahasia"));
    }

    #[test]
    fn test_pre_create_blocks_rahasia() {
        let context = FileContext::new(
            FileOperation::Create,
            "C:\\Users\\test\\file_rahasia.txt",
            1234,
        );

        match pre_create_callback(&context) {
            CallbackStatus::Block => {}, // Expected
            _ => panic!("Should have blocked rahasia file"),
        }
    }

    #[test]
    fn test_pre_create_allows_normal() {
        let context = FileContext::new(
            FileOperation::Create,
            "C:\\Users\\test\\normal.txt",
            1234,
        );

        match pre_create_callback(&context) {
            CallbackStatus::Allow => {}, // Expected
            _ => panic!("Should have allowed normal file"),
        }
    }
}
