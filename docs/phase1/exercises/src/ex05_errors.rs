// Exercise 5: Custom Error Types
// Goal: Create driver-specific error types

use std::fmt;

fn main() {
    println!("=== Exercise 5: Custom Error Types ===\n");

    // Test various error scenarios
    let test_cases = vec![
        ("", "Empty path"),
        ("C:\\secret\\file.txt", "Contains 'secret'"),
        ("C:\\normal.txt", "Normal file"),
        ("\\\\invalid\\\\path", "Invalid path format"),
    ];

    for (path, description) in test_cases {
        println!("Test: {}", description);
        match validate_path(path) {
            Ok(_) => println!("  ✓ ALLOWED: {}\n", path),
            Err(e) => println!("  ✗ BLOCKED: {}\n     Error: {}\n", path, e),
        }
    }

    // Demonstrate converting to NTSTATUS (kernel status codes)
    demonstrate_kernel_status();

    println!("✅ Key Lesson: Custom errors provide context for failures");
    println!("   In kernel: Map to NTSTATUS codes (0xC0000001, etc.)");
}

#[derive(Debug)]
enum DriverError {
    InvalidHandle,
    AccessDenied,
    BufferTooSmall,
    NotSupported,
    InvalidPath,
}

impl fmt::Display for DriverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DriverError::InvalidHandle => write!(f, "Invalid handle provided"),
            DriverError::AccessDenied => write!(f, "Access denied"),
            DriverError::BufferTooSmall => write!(f, "Buffer too small for operation"),
            DriverError::NotSupported => write!(f, "Operation not supported"),
            DriverError::InvalidPath => write!(f, "Invalid file path"),
        }
    }
}

impl std::error::Error for DriverError {}

// Validate a file path
fn validate_path(path: &str) -> Result<(), DriverError> {
    // Check for empty path
    if path.is_empty() {
        return Err(DriverError::InvalidHandle);
    }

    // Check for invalid characters
    if path.contains("\\\\") {
        return Err(DriverError::InvalidPath);
    }

    // Check for blocked patterns
    let path_lower = path.to_lowercase();
    if path_lower.contains("secret") || path_lower.contains("rahasia") {
        return Err(DriverError::AccessDenied);
    }

    Ok(())
}

// Simulate kernel NTSTATUS codes
#[repr(u32)]
#[allow(dead_code)]
enum NtStatus {
    Success = 0x00000000,
    AccessDenied = 0xC0000022,
    InvalidParameter = 0xC000000D,
    NotSupported = 0xC00000BB,
    BufferTooSmall = 0xC0000023,
}

impl From<DriverError> for NtStatus {
    fn from(error: DriverError) -> Self {
        match error {
            DriverError::InvalidHandle => NtStatus::InvalidParameter,
            DriverError::AccessDenied => NtStatus::AccessDenied,
            DriverError::BufferTooSmall => NtStatus::BufferTooSmall,
            DriverError::NotSupported => NtStatus::NotSupported,
            DriverError::InvalidPath => NtStatus::InvalidParameter,
        }
    }
}

fn demonstrate_kernel_status() {
    println!("Kernel NTSTATUS mapping:");

    let errors = vec![
        DriverError::AccessDenied,
        DriverError::InvalidHandle,
        DriverError::NotSupported,
    ];

    for error in errors {
        let status: NtStatus = error.into();
        println!("  {} → 0x{:08X}",
            match status {
                NtStatus::Success => "SUCCESS",
                NtStatus::AccessDenied => "ACCESS_DENIED",
                NtStatus::InvalidParameter => "INVALID_PARAMETER",
                NtStatus::NotSupported => "NOT_SUPPORTED",
                NtStatus::BufferTooSmall => "BUFFER_TOO_SMALL",
            },
            status as u32
        );
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_empty_path() {
        assert!(matches!(validate_path(""), Err(DriverError::InvalidHandle)));
    }

    #[test]
    fn test_validate_secret_path() {
        assert!(matches!(
            validate_path("C:\\secret\\file.txt"),
            Err(DriverError::AccessDenied)
        ));
    }

    #[test]
    fn test_validate_normal_path() {
        assert!(validate_path("C:\\Users\\test\\file.txt").is_ok());
    }

    #[test]
    fn test_validate_invalid_path() {
        assert!(matches!(
            validate_path("\\\\invalid\\\\path"),
            Err(DriverError::InvalidPath)
        ));
    }

    #[test]
    fn test_error_to_ntstatus() {
        let status: NtStatus = DriverError::AccessDenied.into();
        assert_eq!(status as u32, 0xC0000022);
    }
}
