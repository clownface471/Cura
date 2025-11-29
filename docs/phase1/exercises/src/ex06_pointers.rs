// Exercise 6: Raw Pointers
// Goal: Work with raw pointers safely

fn main() {
    println!("=== Exercise 6: Raw Pointers ===\n");

    // Basic pointer usage
    let mut value = 42u32;
    let ptr: *mut u32 = &mut value;

    println!("Original value: {}", value);
    println!("Pointer address: {:p}", ptr);

    unsafe {
        // Dereference and modify
        *ptr = 100;
    }

    println!("Modified value: {}\n", value);

    // Demonstrate const and mut pointers
    demonstrate_pointer_types();

    // Null pointer handling
    demonstrate_null_handling();

    // Kernel-like scenario
    kernel_buffer_example();

    println!("\n✅ Key Lesson: Raw pointers need 'unsafe', validation is YOUR job!");
    println!("   In kernel: ALL pointers are raw! Windows provides MmIsAddressValid.");
}

fn demonstrate_pointer_types() {
    println!("Pointer types:");

    let data = vec![1u8, 2, 3, 4, 5];

    // Immutable raw pointer
    let const_ptr: *const u8 = data.as_ptr();

    unsafe {
        println!("  First element (const ptr): {}", *const_ptr);
        println!("  Second element: {}", *const_ptr.offset(1));
    }

    // Mutable raw pointer
    let mut data_mut = vec![10u8, 20, 30];
    let mut_ptr: *mut u8 = data_mut.as_mut_ptr();

    unsafe {
        *mut_ptr = 99;
        *mut_ptr.offset(1) = 88;
    }

    println!("  Modified data: {:?}\n", data_mut);
}

fn demonstrate_null_handling() {
    println!("Null pointer handling:");

    // Create null pointer
    let null_ptr: *const u32 = std::ptr::null();

    println!("  Null pointer: {:p}", null_ptr);
    println!("  Is null? {}", null_ptr.is_null());

    // Safe check before dereference
    unsafe {
        if !null_ptr.is_null() {
            println!("  Value: {}", *null_ptr);
        } else {
            println!("  Skipping dereference - pointer is null!");
        }
    }

    // Valid pointer
    let value = 42u32;
    let valid_ptr: *const u32 = &value;

    unsafe {
        if !valid_ptr.is_null() {
            println!("  Valid pointer value: {}\n", *valid_ptr);
        }
    }
}

// Simulate kernel buffer manipulation
fn kernel_buffer_example() {
    println!("Kernel-like buffer example:");

    const BUFFER_SIZE: usize = 16;
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

    // Get raw pointer to buffer
    let buffer_ptr: *mut u8 = buffer.as_mut_ptr();

    unsafe {
        // Fill buffer (like memset in C)
        for i in 0..BUFFER_SIZE {
            *buffer_ptr.offset(i as isize) = (i * 16) as u8;
        }
    }

    println!("  Buffer: {:?}", buffer);

    // Read from buffer using pointer arithmetic
    unsafe {
        let first_four = std::slice::from_raw_parts(buffer_ptr, 4);
        println!("  First 4 bytes: {:?}", first_four);
    }
    println!();
}

// Helper: Safe wrapper around pointer dereference
unsafe fn read_from_pointer(ptr: *const u32) -> Option<u32> {
    if ptr.is_null() {
        None
    } else {
        Some(*ptr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pointer_modification() {
        let mut value = 10u32;
        let ptr: *mut u32 = &mut value;

        unsafe {
            *ptr = 20;
        }

        assert_eq!(value, 20);
    }

    #[test]
    fn test_null_pointer_detection() {
        let null_ptr: *const u32 = std::ptr::null();
        assert!(null_ptr.is_null());

        let value = 42;
        let valid_ptr = &value as *const u32;
        assert!(!valid_ptr.is_null());
    }

    #[test]
    fn test_read_from_pointer() {
        let value = 42u32;
        let ptr = &value as *const u32;

        unsafe {
            assert_eq!(read_from_pointer(ptr), Some(42));
            assert_eq!(read_from_pointer(std::ptr::null()), None);
        }
    }
}
