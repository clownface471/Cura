// Exercise 7: Pointer Validation
// Goal: Check pointers before dereferencing (avoid BSOD!)

fn main() {
    println!("=== Exercise 7: Pointer Validation ===\n");

    // Test with valid pointer
    let value = 42u32;
    let valid_ptr = &value as *const u32;

    unsafe {
        if let Some(v) = read_from_pointer(valid_ptr) {
            println!("Valid pointer read: {}", v);
        }
    }

    // Test with null pointer
    let null_ptr = std::ptr::null::<u32>();

    unsafe {
        if let Some(v) = read_from_pointer(null_ptr) {
            println!("This shouldn't print: {}", v);
        } else {
            println!("Correctly detected null pointer");
        }
    }

    println!();

    // Demonstrate alignment checking
    demonstrate_alignment();

    // Kernel-like buffer validation
    kernel_validation_example();

    println!("\n✅ Key Lesson: ALWAYS validate before dereferencing!");
    println!("   In kernel: Use MmIsAddressValid, ProbeForRead, ProbeForWrite");
    println!("   Invalid dereference = BSOD (0x50 PAGE_FAULT_IN_NONPAGED_AREA)");
}

fn is_valid_pointer<T>(ptr: *const T) -> bool {
    // Basic validation (in kernel, you'd use MmIsAddressValid)

    // Check 1: Not null
    if ptr.is_null() {
        return false;
    }

    // Check 2: Aligned properly
    if (ptr as usize) % std::mem::align_of::<T>() != 0 {
        return false;
    }

    // In kernel, you'd also check:
    // - Is address in valid range?
    // - Is page present in memory?
    // - Does process have access rights?

    true
}

unsafe fn read_from_pointer(ptr: *const u32) -> Option<u32> {
    if is_valid_pointer(ptr) {
        Some(*ptr)
    } else {
        None
    }
}

fn demonstrate_alignment() {
    println!("Alignment checking:");

    let value = 1000u32;
    let aligned_ptr = &value as *const u32;

    println!("  u32 requires {} byte alignment", std::mem::align_of::<u32>());
    println!("  Pointer address: {:p}", aligned_ptr);
    println!("  Address value: 0x{:x}", aligned_ptr as usize);
    println!("  Is aligned? {}", (aligned_ptr as usize) % std::mem::align_of::<u32>() == 0);

    // Create misaligned pointer (demonstration only - don't do this in real code!)
    let misaligned_ptr = (aligned_ptr as usize + 1) as *const u32;
    println!("  Misaligned pointer: {:p}", misaligned_ptr);
    println!("  Is aligned? {}\n", is_valid_pointer(misaligned_ptr));
}

// Kernel-like buffer validation
struct BufferDescriptor {
    ptr: *const u8,
    len: usize,
}

impl BufferDescriptor {
    fn new(ptr: *const u8, len: usize) -> Self {
        Self { ptr, len }
    }

    fn is_valid(&self) -> bool {
        // Check pointer
        if !is_valid_pointer(self.ptr) {
            return false;
        }

        // Check length is reasonable
        if self.len == 0 || self.len > 1024 * 1024 {
            // Empty or > 1MB is suspicious
            return false;
        }

        // In kernel, you'd also:
        // - ProbeForRead to ensure all bytes are readable
        // - Check if buffer crosses page boundaries
        // - Verify buffer doesn't overlap kernel space

        true
    }

    unsafe fn read_bytes(&self) -> Option<Vec<u8>> {
        if !self.is_valid() {
            return None;
        }

        let slice = std::slice::from_raw_parts(self.ptr, self.len);
        Some(slice.to_vec())
    }
}

fn kernel_validation_example() {
    println!("Kernel buffer validation:");

    let data = vec![0x41, 0x42, 0x43, 0x44]; // "ABCD"

    // Valid buffer
    let valid_buf = BufferDescriptor::new(data.as_ptr(), data.len());

    if valid_buf.is_valid() {
        unsafe {
            if let Some(bytes) = valid_buf.read_bytes() {
                println!("  Valid buffer read: {:?}", bytes);
                println!("  As ASCII: {}", String::from_utf8_lossy(&bytes));
            }
        }
    }

    // Invalid buffer (null pointer)
    let invalid_buf = BufferDescriptor::new(std::ptr::null(), 100);
    println!("  Invalid buffer valid? {}", invalid_buf.is_valid());

    // Invalid buffer (too large)
    let huge_buf = BufferDescriptor::new(data.as_ptr(), 10 * 1024 * 1024);
    println!("  Huge buffer valid? {}\n", huge_buf.is_valid());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_pointer() {
        let value = 42u32;
        let ptr = &value as *const u32;
        assert!(is_valid_pointer(ptr));
    }

    #[test]
    fn test_null_pointer() {
        let ptr = std::ptr::null::<u32>();
        assert!(!is_valid_pointer(ptr));
    }

    #[test]
    fn test_buffer_descriptor_valid() {
        let data = vec![1u8, 2, 3, 4];
        let buf = BufferDescriptor::new(data.as_ptr(), data.len());
        assert!(buf.is_valid());
    }

    #[test]
    fn test_buffer_descriptor_null() {
        let buf = BufferDescriptor::new(std::ptr::null(), 10);
        assert!(!buf.is_valid());
    }

    #[test]
    fn test_buffer_descriptor_too_large() {
        let data = vec![1u8];
        let buf = BufferDescriptor::new(data.as_ptr(), 10 * 1024 * 1024);
        assert!(!buf.is_valid());
    }
}
