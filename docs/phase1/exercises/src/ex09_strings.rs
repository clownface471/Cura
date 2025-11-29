// Exercise 9: String Comparison (No-Std)
// Goal: Compare strings without String type (kernel environment)

fn main() {
    println!("=== Exercise 9: String Comparison (No-Std) ===\n");

    // Test cases
    let test_cases = vec![
        (b"file_rahasia.txt" as &[u8], b"rahasia" as &[u8], true),
        (b"FILE_RAHASIA.TXT" as &[u8], b"rahasia" as &[u8], true),
        (b"normal.txt" as &[u8], b"rahasia" as &[u8], false),
        (b"C:\\Users\\Nora\\secret_document.docx" as &[u8], b"secret" as &[u8], true),
        (b"test" as &[u8], b"testing" as &[u8], false),
    ];

    println!("Testing str_contains_ignore_case:");
    for (i, (haystack, needle, expected)) in test_cases.iter().enumerate() {
        let result = str_contains_ignore_case(haystack, needle);
        let status = if result == *expected { "✓" } else { "✗" };

        println!("  {} Test {}: \"{}\" contains \"{}\"? {}",
            status,
            i + 1,
            String::from_utf8_lossy(haystack),
            String::from_utf8_lossy(needle),
            result
        );
    }

    println!();

    // Demonstrate other string operations
    demonstrate_case_conversion();
    demonstrate_path_operations();

    println!("\n✅ Key Lesson: Kernel can't use String type!");
    println!("   Work with byte slices (&[u8])");
    println!("   Implement your own string algorithms");
}

/// Check if haystack contains needle (case-insensitive)
fn str_contains_ignore_case(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.len() > haystack.len() {
        return false;
    }

    // Slide a window across haystack
    for window in haystack.windows(needle.len()) {
        if bytes_equal_ignore_case(window, needle) {
            return true;
        }
    }

    false
}

/// Compare two byte slices (case-insensitive)
fn bytes_equal_ignore_case(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    a.iter()
        .zip(b.iter())
        .all(|(a_byte, b_byte)| {
            a_byte.to_ascii_lowercase() == b_byte.to_ascii_lowercase()
        })
}

/// Check if byte slice starts with prefix (case-insensitive)
fn starts_with_ignore_case(haystack: &[u8], prefix: &[u8]) -> bool {
    if prefix.len() > haystack.len() {
        return false;
    }

    bytes_equal_ignore_case(&haystack[..prefix.len()], prefix)
}

/// Check if byte slice ends with suffix (case-insensitive)
fn ends_with_ignore_case(haystack: &[u8], suffix: &[u8]) -> bool {
    if suffix.len() > haystack.len() {
        return false;
    }

    let start = haystack.len() - suffix.len();
    bytes_equal_ignore_case(&haystack[start..], suffix)
}

fn demonstrate_case_conversion() {
    println!("Case conversion:");

    let text = b"Hello, CURA System!";
    println!("  Original: {}", String::from_utf8_lossy(text));

    // Convert to lowercase
    let mut lower = [0u8; 32];
    for (i, &byte) in text.iter().enumerate() {
        lower[i] = byte.to_ascii_lowercase();
    }
    println!("  Lowercase: {}", String::from_utf8_lossy(&lower[..text.len()]));

    // Convert to uppercase
    let mut upper = [0u8; 32];
    for (i, &byte) in text.iter().enumerate() {
        upper[i] = byte.to_ascii_uppercase();
    }
    println!("  Uppercase: {}\n", String::from_utf8_lossy(&upper[..text.len()]));
}

fn demonstrate_path_operations() {
    println!("Path operations:");

    let path1 = b"C:\\Windows\\System32\\notepad.exe";
    let path2 = b"c:\\windows\\system32\\NOTEPAD.EXE";

    println!("  Path 1: {}", String::from_utf8_lossy(path1));
    println!("  Path 2: {}", String::from_utf8_lossy(path2));
    println!("  Are equal (case-insensitive)? {}",
        bytes_equal_ignore_case(path1, path2));

    println!();

    // Check file extensions
    let files = vec![
        (b"document.txt" as &[u8], b".txt" as &[u8]),
        (b"image.PNG" as &[u8], b".png" as &[u8]),
        (b"script.ps1" as &[u8], b".exe" as &[u8]),
    ];

    println!("  Extension checking:");
    for (file, ext) in files {
        let has_ext = ends_with_ignore_case(file, ext);
        println!("    {} has extension {}? {}",
            String::from_utf8_lossy(file),
            String::from_utf8_lossy(ext),
            has_ext
        );
    }

    println!();

    // Check path prefixes
    let system_paths = vec![
        b"C:\\Windows\\System32\\file.dll" as &[u8],
        b"c:\\windows\\temp\\data.tmp" as &[u8],
        b"D:\\Users\\Nora\\document.txt" as &[u8],
    ];

    println!("  Windows path checking:");
    for path in system_paths {
        let is_windows = starts_with_ignore_case(path, b"C:\\Windows");
        println!("    {}: {}",
            String::from_utf8_lossy(path),
            if is_windows { "Windows path" } else { "Other path" }
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_case_insensitive() {
        assert!(str_contains_ignore_case(b"file_RAHASIA.txt", b"rahasia"));
        assert!(str_contains_ignore_case(b"SECRET_Document", b"secret"));
        assert!(!str_contains_ignore_case(b"normal.txt", b"rahasia"));
    }

    #[test]
    fn test_bytes_equal_ignore_case() {
        assert!(bytes_equal_ignore_case(b"Hello", b"hello"));
        assert!(bytes_equal_ignore_case(b"CURA", b"cura"));
        assert!(!bytes_equal_ignore_case(b"test", b"testing"));
    }

    #[test]
    fn test_starts_with() {
        assert!(starts_with_ignore_case(b"C:\\Windows\\file.dll", b"c:\\windows"));
        assert!(!starts_with_ignore_case(b"D:\\Users", b"C:\\Windows"));
    }

    #[test]
    fn test_ends_with() {
        assert!(ends_with_ignore_case(b"document.TXT", b".txt"));
        assert!(ends_with_ignore_case(b"image.PNG", b".png"));
        assert!(!ends_with_ignore_case(b"script.ps1", b".exe"));
    }
}
