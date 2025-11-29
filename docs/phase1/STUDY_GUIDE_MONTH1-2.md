# Month 1-2 Study Guide: Rust Fundamentals

**Detailed week-by-week plan for mastering Rust before kernel development**

Duration: 8 weeks (2 months)
Time commitment: 10-15 hours/week
Goal: Master Rust to the point where you can confidently write kernel-safe code

---

## Overview

```
Week 1-2:  Basics (ownership, borrowing)
Week 3-4:  Advanced (lifetimes, traits, error handling)
Week 5-6:  Unsafe Rust (pointers, no_std)
Week 7-8:  Practice (exercises, mini-projects)
```

---

## Week 1: Ownership & Basic Syntax

### Goals
- Understand ownership model
- Write basic Rust programs
- Complete exercises 1-2

### Monday: Getting Started

**Reading:**
- Rust Book Chapter 1: Getting Started
- Rust Book Chapter 2: Guessing Game

**Practice:**
```bash
# Install Rust (if not done)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify
rustc --version
cargo --version

# Create first project
cargo new hello_cura
cd hello_cura
cargo run
```

**Exercise:**
```rust
// Modify src/main.rs
fn main() {
    println!("CURA - The Digital Immune System");

    let system_name = String::from("CURA");
    let version = "0.1.0";

    println!("System: {}", system_name);
    println!("Version: {}", version);
}
```

---

### Tuesday-Wednesday: Ownership

**Reading:**
- Rust Book Chapter 4: Understanding Ownership
- Focus on:
  - What is ownership?
  - The Stack and the Heap
  - Ownership rules
  - Move semantics

**Key Concepts:**
```rust
// Ownership transfer
let s1 = String::from("hello");
let s2 = s1;  // s1 is no longer valid!

// Clone for copy
let s3 = s2.clone();  // Both s2 and s3 valid

// Functions take ownership
fn takes_ownership(s: String) {
    println!("{}", s);
}  // s dropped here
```

**Practice:**
- Run Exercise 1: `cargo run --bin ex01_ownership`
- Complete Rustlings: `intro`, `move_semantics`

**Mini-Project:**
```rust
// File: ownership_practice.rs
// Create a program that manages file paths
// - Store 5 different file paths
// - Print them without losing ownership
// - Demonstrate clone vs reference
```

---

### Thursday-Friday: Borrowing & References

**Reading:**
- Rust Book Chapter 4.2: References and Borrowing
- Understanding &T and &mut T

**Key Rules:**
1. One &mut OR many & at a time
2. References must be valid
3. No dangling references

**Practice:**
- Run Exercise 2: `cargo run --bin ex02_borrowing`
- Complete Rustlings: `references`

**Coding Practice:**
```rust
// Practice mutable vs immutable borrowing
fn main() {
    let mut buffer = vec![0u8; 100];

    // Fill it (needs &mut)
    fill_buffer(&mut buffer);

    // Read it (needs &)
    print_buffer(&buffer);
}

fn fill_buffer(buf: &mut [u8]) {
    for (i, byte) in buf.iter_mut().enumerate() {
        *byte = (i % 256) as u8;
    }
}

fn print_buffer(buf: &[u8]) {
    println!("First 10 bytes: {:?}", &buf[..10]);
}
```

---

### Saturday-Sunday: Week 1 Review

**Tasks:**
- [ ] Review all Week 1 code
- [ ] Complete all Rustlings exercises related to ownership
- [ ] Write a summary of ownership rules in your own words
- [ ] Complete Exercise 1 and 2 successfully

**Challenge Project:**
```rust
// File path manager
// Requirements:
// - Store list of file paths
// - Add/remove paths
// - Search for path containing keyword
// - No String::clone() allowed!
// Use references only.
```

**Self-Assessment:**
- Can you explain ownership without looking at notes?
- Do you understand when to use & vs &mut?
- Can you predict when code won't compile due to ownership?

If not, review this week's materials before continuing.

---

## Week 2: Structs, Enums, Pattern Matching

### Monday-Tuesday: Structs

**Reading:**
- Rust Book Chapter 5: Structs

**Practice:**
```rust
// Kernel-relevant example: File information
struct FileInfo {
    path: String,
    size: u64,
    is_system: bool,
}

impl FileInfo {
    fn new(path: String, size: u64) -> Self {
        Self {
            path,
            size,
            is_system: false,
        }
    }

    fn should_scan(&self) -> bool {
        !self.is_system && self.size > 0
    }
}
```

**Mini-Project:**
```rust
// Create a FilterRule struct
// - Pattern to match
// - Action (Allow/Block)
// - Priority
// Implement methods to evaluate a file path
```

---

### Wednesday-Thursday: Enums & Pattern Matching

**Reading:**
- Rust Book Chapter 6: Enums and Pattern Matching

**Key Concept:**
```rust
enum ThreatLevel {
    Clean,
    Suspicious(String),  // with reason
    Malicious { score: u8, source: String },
}

fn handle_threat(level: ThreatLevel) {
    match level {
        ThreatLevel::Clean => println!("Safe"),
        ThreatLevel::Suspicious(reason) => {
            println!("Warning: {}", reason);
        }
        ThreatLevel::Malicious { score, source } => {
            println!("BLOCK! Score: {}, Source: {}", score, source);
        }
    }
}
```

**Practice:**
- Complete Rustlings: `enums`, `pattern_matching`

---

### Friday: Option & Result

**Reading:**
- Rust Book Chapter 6.1: Option
- Rust Book Chapter 9: Error Handling

**Critical for kernel development!**

```rust
// Option - value may not exist
fn find_process(name: &str) -> Option<u32> {
    if name == "explorer.exe" {
        Some(1234)
    } else {
        None
    }
}

// Result - operation may fail
fn open_file(path: &str) -> Result<FileHandle, IoError> {
    // ...
}
```

**Practice:**
- Run Exercise 4: `cargo run --bin ex04_result`
- Run Exercise 5: `cargo run --bin ex05_errors`

---

### Saturday-Sunday: Week 2 Review

**Project:**
```rust
// File scanner simulator
// Requirements:
// - Scan a list of file paths
// - Each file has a threat level (enum)
// - Return Result with scan summary
// - Use Option for optional metadata
// - Pattern match on all cases
```

---

## Week 3: Lifetimes & Advanced Concepts

### Monday-Wednesday: Lifetimes

**Reading:**
- Rust Book Chapter 10.3: Validating References with Lifetimes

**This is CRITICAL for kernel drivers!**

**Why lifetimes matter in kernel:**
- IRPs have lifetimes
- File objects have lifetimes
- Buffers have lifetimes
- **Accessing freed memory = BSOD**

**Practice:**
- Run Exercise 3: `cargo run --bin ex03_lifetimes`

**Coding Practice:**
```rust
// Kernel-like scenario
struct FileRequest<'a> {
    path: &'a str,
    buffer: &'a [u8],
}

impl<'a> FileRequest<'a> {
    fn process(&self) -> bool {
        // Process while path and buffer are valid
        true
    }
}

fn main() {
    let path = String::from("C:\\test.txt");
    let data = vec![0u8; 100];

    let request = FileRequest {
        path: &path,
        buffer: &data,
    };

    request.process();
    // path and data still valid here
}
```

---

### Thursday-Friday: Traits

**Reading:**
- Rust Book Chapter 10.2: Traits

**Practice:**
```rust
// Define behavior for different filter types
trait FilterBehavior {
    fn should_block(&self, path: &str) -> bool;
    fn priority(&self) -> u8;
}

struct PatternFilter {
    pattern: String,
}

impl FilterBehavior for PatternFilter {
    fn should_block(&self, path: &str) -> bool {
        path.to_lowercase().contains(&self.pattern.to_lowercase())
    }

    fn priority(&self) -> u8 {
        10
    }
}
```

---

### Saturday-Sunday: Week 3 Practice

**Project:**
```rust
// Threat analyzer system
// - Multiple threat detection strategies (traits)
// - File info with lifetimes
// - Result-based error handling
// - Pattern matching on outcomes
```

---

## Week 4: Collections & Iteration

### Monday-Tuesday: Vectors & Strings

**Reading:**
- Rust Book Chapter 8: Common Collections

**Practice:**
```rust
// Manage list of blocked files
fn main() {
    let mut blocked_files = Vec::new();

    blocked_files.push("virus.exe".to_string());
    blocked_files.push("malware.dll".to_string());

    // Iterate
    for file in &blocked_files {
        println!("Blocked: {}", file);
    }

    // Filter
    let exe_files: Vec<_> = blocked_files
        .iter()
        .filter(|f| f.ends_with(".exe"))
        .collect();

    println!("EXE files: {:?}", exe_files);
}
```

---

### Wednesday-Thursday: Iterators

**Reading:**
- Rust Book Chapter 13.2: Iterators

**Powerful for data processing:**
```rust
let files = vec!["a.txt", "b.exe", "c.dll"];

let executables = files
    .iter()
    .filter(|f| f.ends_with(".exe") || f.ends_with(".dll"))
    .map(|f| f.to_uppercase())
    .collect::<Vec<_>>();
```

---

### Friday: Closures

**Reading:**
- Rust Book Chapter 13.1: Closures

**Used in callbacks (important for drivers!):**
```rust
fn apply_filter<F>(path: &str, filter: F) -> bool
where
    F: Fn(&str) -> bool,
{
    filter(path)
}

fn main() {
    let blocked = apply_filter("virus.exe", |p| p.contains("virus"));
    println!("Blocked: {}", blocked);
}
```

---

### Saturday-Sunday: Week 4 Review

**Comprehensive Project:**
```rust
// File filter system v2
// - Store rules in Vec
// - Use iterators to check rules
// - Closures for custom filters
// - Traits for different rule types
```

---

## Week 5: Unsafe Rust

### Monday-Wednesday: Raw Pointers

**Reading:**
- Rust Book Chapter 19.1: Unsafe Rust

**⚠️ This is 90% of kernel code!**

**Practice:**
- Run Exercise 6: `cargo run --bin ex06_pointers`
- Run Exercise 7: `cargo run --bin ex07_validation`

**Key Concepts:**
```rust
// Raw pointers (*const T, *mut T)
let mut value = 42;
let ptr: *mut i32 = &mut value;

unsafe {
    *ptr = 100;  // Dereference requires unsafe
}

println!("Value: {}", value);  // 100
```

**Kernel Reality:**
- ALL Windows kernel pointers are raw
- YOU must validate before dereferencing
- Invalid dereference = BSOD (0x50)

---

### Thursday-Friday: Pointer Validation

**Critical skill:**
```rust
fn is_valid_pointer<T>(ptr: *const T) -> bool {
    // 1. Not null
    if ptr.is_null() {
        return false;
    }

    // 2. Aligned
    if (ptr as usize) % std::mem::align_of::<T>() != 0 {
        return false;
    }

    // In kernel: Also check with MmIsAddressValid
    true
}

unsafe fn read_safe(ptr: *const u32) -> Option<u32> {
    if is_valid_pointer(ptr) {
        Some(*ptr)
    } else {
        None
    }
}
```

**Practice:**
- Complete all pointer exercises
- Write validation functions

---

### Saturday-Sunday: Unsafe Practice

**Project:**
```rust
// Kernel-style buffer manager
// - Allocate fixed buffer (array)
// - Get raw pointer
// - Validate before access
// - Implement safe wrappers around unsafe code
```

---

## Week 6: No-Std Environment

### Monday-Wednesday: Working Without std

**Reading:**
- Rust Book Chapter 19.1: no_std
- [The Embedded Rust Book](https://docs.rust-embedded.org/book/)

**Practice:**
- Run Exercise 8: `cargo run --bin ex08_buffer`
- Run Exercise 9: `cargo run --bin ex09_strings`

**Key Differences:**
```rust
#![no_std]

// ❌ Can't use:
// - String
// - Vec
// - Box
// - println! (uses std::io)
// - Most collections

// ✅ Can use:
// - Arrays [T; N]
// - Slices &[T]
// - Option, Result
// - Iterators
// - Traits
```

---

### Thursday-Friday: Fixed Buffers & String Operations

**Practice:**
```rust
// No Vec - use arrays
let mut buffer: [u8; 256] = [0; 256];

// No String - use &str or &[u8]
fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    haystack.windows(needle.len())
        .any(|window| window == needle)
}
```

**Complete:**
- Exercise 8 (No-std buffer)
- Exercise 9 (String comparison)

---

### Saturday-Sunday: No-Std Practice

**Project:**
```rust
#![no_std]

// Path filter (kernel-style)
// - Fixed-size buffer for paths
// - No String/Vec
// - Byte slice operations
// - Pattern matching
```

---

## Week 7-8: Integration & Practice

### Goals
- Complete all 11 exercises
- Build practice projects
- Review all concepts
- Prepare for kernel work

### Daily Schedule

**Monday-Wednesday:**
- Complete Exercise 10 (callbacks)
- Complete Exercise 11 (file filter project)
- Run all tests: `cargo test`

**Thursday-Friday:**
- Build comprehensive project combining all concepts
- Review Week 1-6 materials
- Identify weak areas

**Saturday-Sunday:**
- **Final Project:** Build a complete file filter simulator
  - Uses all Rust concepts learned
  - No-std compatible
  - Proper error handling
  - Safe wrappers around unsafe code
  - Trait-based architecture

---

## Final Project Specification

**Requirements:**
1. **No-std compatible** (can compile with `#![no_std]`)
2. **File path filtering** (block patterns)
3. **Fixed-size buffers** (no Vec, no String)
4. **Trait-based rules** (multiple filter types)
5. **Proper error handling** (Result types)
6. **Unsafe operations** (raw pointers with validation)
7. **Comprehensive tests**

**This will be your kernel driver in miniature!**

---

## Week-by-Week Checklist

### Week 1
- [ ] Install Rust
- [ ] Complete Rust Book Ch 1-4
- [ ] Complete Exercise 1-2
- [ ] Complete Rustlings: intro, move_semantics, references
- [ ] Can explain ownership rules

### Week 2
- [ ] Complete Rust Book Ch 5-6, 9
- [ ] Complete Exercise 4-5
- [ ] Complete Rustlings: structs, enums, error_handling
- [ ] Can write error-handling code

### Week 3
- [ ] Complete Rust Book Ch 10
- [ ] Complete Exercise 3
- [ ] Complete Rustlings: lifetimes, traits
- [ ] Understand lifetime annotations

### Week 4
- [ ] Complete Rust Book Ch 8, 13
- [ ] Complete Rustlings: collections, iterators, closures
- [ ] Can use iterators fluently

### Week 5
- [ ] Complete Rust Book Ch 19.1
- [ ] Complete Exercise 6-7
- [ ] Complete Rustlings: unsafe
- [ ] Can validate pointers

### Week 6
- [ ] Read Embedded Rust Book
- [ ] Complete Exercise 8-9
- [ ] Can work in no_std
- [ ] Can manipulate byte slices

### Week 7-8
- [ ] Complete Exercise 10-11
- [ ] All tests passing
- [ ] Final project complete
- [ ] Ready for kernel work!

---

## Self-Assessment

Before moving to Month 3, ensure you can:

1. **Explain ownership** without looking at notes
2. **Write code with lifetimes** confidently
3. **Use unsafe correctly** with proper validation
4. **Work in no_std** without struggling
5. **Handle errors properly** with Result
6. **Validate pointers** before dereferencing
7. **Manipulate byte slices** for string operations

If you can't do all of these, review before proceeding to kernel work!

---

## Resources

### Books
- [The Rust Book](https://doc.rust-lang.org/book/) - Primary resource
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) - Unsafe Rust

### Practice
- [Rustlings](https://github.com/rust-lang/rustlings)
- [Exercism Rust Track](https://exercism.org/tracks/rust)

### Communities
- [Rust Discord](https://discord.gg/rust-lang) - #beginners channel
- [/r/rust](https://reddit.com/r/rust)
- [Rust Users Forum](https://users.rust-lang.org/)

---

## Time Management Tips

**For students balancing school:**

**Weekdays (1-2 hours/day):**
- 30 min: Reading
- 30-60 min: Coding practice
- 15 min: Review and notes

**Weekends (4-6 hours/day):**
- 2 hours: Deep reading
- 2-3 hours: Projects
- 1 hour: Review week

**Total: ~10-15 hours/week**

**Adjust based on your schedule, but be consistent!**

---

## What's Next?

After completing Month 1-2:
- **Month 3-4**: Windows Kernel Internals
- **Month 5-6**: First driver implementation
- **Month 7-8**: File blocking implementation

**You're building the foundation. Take your time. Get it right.**

---

**Good luck! You've got this! 🦀**
