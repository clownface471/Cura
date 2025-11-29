# Rust Learning Exercises for Kernel Development

These exercises build the specific Rust skills you need for kernel driver development.

## Week 1-2: Ownership & Memory

### Exercise 1: String Ownership
**Goal**: Master ownership transfer

```rust
// TODO: Fix the compilation errors
fn main() {
    let message = String::from("CURA initializing");

    print_message(message);
    print_message(message); // Error! Why?
}

fn print_message(s: String) {
    println!("{}", s);
}
```

**Solution Hint**: Use references (`&String`) or clone the string.

---

### Exercise 2: Mutable Borrowing
**Goal**: Understand mutable vs immutable references

```rust
// TODO: Make this compile
fn main() {
    let mut buffer = vec![0u8; 10];

    fill_buffer(&mut buffer, 0xFF);
    read_buffer(&buffer);
}

fn fill_buffer(buf: &mut [u8], value: u8) {
    for byte in buf.iter_mut() {
        *byte = value;
    }
}

fn read_buffer(buf: &[u8]) {
    println!("Buffer: {:?}", buf);
}
```

**Challenge**: What happens if you try to call `read_buffer` before `fill_buffer` finishes?

---

### Exercise 3: Lifetime Annotations
**Goal**: Understand lifetimes (critical for kernel development)

```rust
// TODO: Add lifetime annotations
fn longest_path<'a>(path1: &'a str, path2: &'a str) -> &'a str {
    if path1.len() > path2.len() {
        path1
    } else {
        path2
    }
}

fn main() {
    let p1 = String::from("C:\\Windows\\System32");
    let p2 = String::from("C:\\Program Files");

    let result = longest_path(&p1, &p2);
    println!("Longest: {}", result);
}
```

**Why this matters**: Kernel code constantly works with references that must outlive certain operations.

---

## Week 3-4: Error Handling

### Exercise 4: Result Type
**Goal**: Master `Result<T, E>` for error handling

```rust
use std::fs::File;
use std::io::Read;

// TODO: Implement this function
fn read_config(path: &str) -> Result<String, std::io::Error> {
    // Read file and return contents
    // Handle errors properly
    todo!()
}

fn main() {
    match read_config("config.txt") {
        Ok(contents) => println!("Config: {}", contents),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

**Kernel version**: Kernel code can't panic! Everything must return a status code.

---

### Exercise 5: Custom Error Types
**Goal**: Create driver-specific error types

```rust
#[derive(Debug)]
enum DriverError {
    InvalidHandle,
    AccessDenied,
    BufferTooSmall,
    NotSupported,
}

impl std::fmt::Display for DriverError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DriverError::InvalidHandle => write!(f, "Invalid handle"),
            DriverError::AccessDenied => write!(f, "Access denied"),
            DriverError::BufferTooSmall => write!(f, "Buffer too small"),
            DriverError::NotSupported => write!(f, "Operation not supported"),
        }
    }
}

// TODO: Implement a function that returns DriverError
fn validate_path(path: &str) -> Result<(), DriverError> {
    if path.is_empty() {
        return Err(DriverError::InvalidHandle);
    }

    if path.contains("secret") {
        return Err(DriverError::AccessDenied);
    }

    Ok(())
}
```

---

## Week 5-6: Unsafe Rust

### Exercise 6: Raw Pointers
**Goal**: Work with raw pointers safely

```rust
fn main() {
    let mut value = 42u32;
    let ptr: *mut u32 = &mut value;

    unsafe {
        // TODO: Dereference and modify the value
        *ptr = 100;
    }

    println!("Value: {}", value);
}
```

**Kernel reality**: All driver code works with raw pointers from Windows kernel!

---

### Exercise 7: Pointer Validation
**Goal**: Check pointers before dereferencing

```rust
fn is_valid_pointer<T>(ptr: *const T) -> bool {
    // In kernel, you'd use MmIsAddressValid or ProbeForRead
    // For now, just check for null
    !ptr.is_null()
}

unsafe fn read_from_pointer(ptr: *const u32) -> Option<u32> {
    // TODO: Validate pointer before reading
    if is_valid_pointer(ptr) {
        Some(*ptr)
    } else {
        None
    }
}

fn main() {
    let value = 42u32;
    let ptr = &value as *const u32;

    unsafe {
        if let Some(v) = read_from_pointer(ptr) {
            println!("Read value: {}", v);
        }
    }

    // Try with null pointer
    let null_ptr = std::ptr::null::<u32>();
    unsafe {
        if let Some(v) = read_from_pointer(null_ptr) {
            println!("This shouldn't print");
        } else {
            println!("Correctly detected null pointer");
        }
    }
}
```

**Why this matters**: Dereferencing invalid pointers = BSOD!

---

## Week 7-8: No-Std Environment

### Exercise 8: No-Std Buffer
**Goal**: Work without the standard library

```rust
#![no_std]

// No Vec, no String, no heap allocation!

// TODO: Implement a fixed-size buffer
struct FixedBuffer<const N: usize> {
    data: [u8; N],
    len: usize,
}

impl<const N: usize> FixedBuffer<N> {
    const fn new() -> Self {
        Self {
            data: [0; N],
            len: 0,
        }
    }

    fn push(&mut self, byte: u8) -> Result<(), ()> {
        if self.len >= N {
            return Err(()); // Buffer full
        }

        self.data[self.len] = byte;
        self.len += 1;
        Ok(())
    }

    fn as_slice(&self) -> &[u8] {
        &self.data[..self.len]
    }
}

// Test in main.rs with std
fn main() {
    let mut buffer = FixedBuffer::<10>::new();

    for i in 0..5 {
        buffer.push(i).unwrap();
    }

    println!("Buffer: {:?}", buffer.as_slice());
}
```

---

### Exercise 9: String Comparison (No-Std)
**Goal**: Compare paths without String type

```rust
#![no_std]

// TODO: Implement case-insensitive comparison
fn str_contains_ignore_case(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.len() > haystack.len() {
        return false;
    }

    for window in haystack.windows(needle.len()) {
        if window.iter()
            .zip(needle.iter())
            .all(|(a, b)| a.to_ascii_lowercase() == b.to_ascii_lowercase())
        {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        assert!(str_contains_ignore_case(b"file_RAHASIA.txt", b"rahasia"));
        assert!(!str_contains_ignore_case(b"normal.txt", b"rahasia"));
    }
}
```

**Kernel usage**: Exactly how you'll check filenames in the driver!

---

## Advanced Exercises

### Exercise 10: Callback Pattern
**Goal**: Understand function pointers (used for driver callbacks)

```rust
type PreCreateCallback = fn(&str) -> bool;

struct FilterCallbacks {
    pre_create: Option<PreCreateCallback>,
}

impl FilterCallbacks {
    fn new() -> Self {
        Self { pre_create: None }
    }

    fn set_pre_create(&mut self, callback: PreCreateCallback) {
        self.pre_create = Some(callback);
    }

    fn invoke_pre_create(&self, path: &str) -> bool {
        if let Some(callback) = self.pre_create {
            callback(path)
        } else {
            true // Allow by default
        }
    }
}

// TODO: Implement the callback
fn my_pre_create_callback(path: &str) -> bool {
    // Return false to block, true to allow
    !path.contains("rahasia")
}

fn main() {
    let mut callbacks = FilterCallbacks::new();
    callbacks.set_pre_create(my_pre_create_callback);

    // Simulate file accesses
    let paths = vec![
        "C:\\test.txt",
        "C:\\file_rahasia.txt",
        "C:\\normal.doc",
    ];

    for path in paths {
        let allowed = callbacks.invoke_pre_create(path);
        println!("{}: {}", path, if allowed { "ALLOW" } else { "BLOCK" });
    }
}
```

---

## Practice Project: Mini File Filter

**Goal**: Build a usermode file filter simulator

Create a program that:
1. Monitors a directory for file creation
2. Blocks files matching a pattern
3. Logs all file operations

This simulates what your kernel driver will do!

```rust
use std::fs;
use std::path::Path;

struct FileFilter {
    blocked_patterns: Vec<String>,
}

impl FileFilter {
    fn new() -> Self {
        Self {
            blocked_patterns: vec!["rahasia".to_string()],
        }
    }

    fn should_block(&self, filename: &str) -> bool {
        let filename_lower = filename.to_lowercase();
        self.blocked_patterns.iter()
            .any(|pattern| filename_lower.contains(pattern))
    }

    fn check_file(&self, path: &Path) -> Result<(), String> {
        if let Some(filename) = path.file_name() {
            if let Some(filename_str) = filename.to_str() {
                if self.should_block(filename_str) {
                    return Err(format!("BLOCKED: {}", filename_str));
                }
            }
        }
        Ok(())
    }
}

fn main() {
    let filter = FileFilter::new();

    // Test files
    let test_files = vec![
        "C:\\test.txt",
        "C:\\file_rahasia.txt",
        "C:\\secret_document.doc",
    ];

    for file in test_files {
        let path = Path::new(file);
        match filter.check_file(path) {
            Ok(_) => println!("✓ ALLOW: {}", file),
            Err(e) => println!("✗ {}", e),
        }
    }
}
```

---

## Solutions

All exercise solutions are in `exercises/solutions/` directory.

Don't peek until you've tried them yourself!

---

## Validation

Run this to check your progress:

```bash
cargo test --package rust-exercises
```

Each exercise has unit tests. Aim for 100% passing before moving to kernel development.

---

## Next Steps

After completing these exercises:
1. ✅ You understand Rust ownership
2. ✅ You can work with unsafe code
3. ✅ You're comfortable with no_std
4. ✅ You understand error handling

**You're ready for Month 3: Windows Kernel Internals!**
