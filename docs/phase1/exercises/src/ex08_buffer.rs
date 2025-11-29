// Exercise 8: No-Std Buffer
// Goal: Work without the standard library (kernel environment)

#![allow(dead_code)]

fn main() {
    println!("=== Exercise 8: No-Std Buffer ===\n");

    let mut buffer = FixedBuffer::<64>::new();

    println!("Buffer capacity: {}", buffer.capacity());
    println!("Buffer length: {}", buffer.len());
    println!("Is empty? {}\n", buffer.is_empty());

    // Push some data
    println!("Pushing bytes 1-10...");
    for i in 1..=10 {
        match buffer.push(i) {
            Ok(_) => print!("{} ", i),
            Err(_) => println!("\nError: buffer full"),
        }
    }
    println!("\n");

    println!("Buffer now: {:?}", buffer.as_slice());
    println!("Length: {}\n", buffer.len());

    // Fill buffer
    println!("Filling rest of buffer...");
    for i in 11..=70 {
        if buffer.push(i).is_err() {
            println!("Buffer full at byte {}", i);
            break;
        }
    }

    println!("Final length: {}/{}\n", buffer.len(), buffer.capacity());

    // Demonstrate clear
    buffer.clear();
    println!("After clear:");
    println!("  Length: {}", buffer.len());
    println!("  Is empty? {}\n", buffer.is_empty());

    // Kernel-like usage
    demonstrate_kernel_usage();

    println!("\n✅ Key Lesson: Kernel can't use Vec or String!");
    println!("   Must use fixed-size buffers allocated from pools");
    println!("   No dynamic allocation in many kernel contexts");
}

/// Fixed-size buffer (no heap allocation)
/// This is what you use in kernel mode!
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

    fn capacity(&self) -> usize {
        N
    }

    fn len(&self) -> usize {
        self.len
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn is_full(&self) -> bool {
        self.len >= N
    }

    fn push(&mut self, byte: u8) -> Result<(), ()> {
        if self.is_full() {
            return Err(()); // Buffer full
        }

        self.data[self.len] = byte;
        self.len += 1;
        Ok(())
    }

    fn as_slice(&self) -> &[u8] {
        &self.data[..self.len]
    }

    fn clear(&mut self) {
        self.len = 0;
        // In kernel, you might want to zero the buffer too
        // for i in 0..N {
        //     self.data[i] = 0;
        // }
    }

    fn extend_from_slice(&mut self, slice: &[u8]) -> Result<(), ()> {
        if self.len + slice.len() > N {
            return Err(()); // Not enough space
        }

        for &byte in slice {
            self.push(byte)?;
        }

        Ok(())
    }
}

fn demonstrate_kernel_usage() {
    println!("Kernel-like usage example:");

    // Typical kernel scenario: copying a file path
    let mut path_buffer = FixedBuffer::<260>::new(); // MAX_PATH

    let path = b"C:\\Users\\Nora\\Documents\\file_rahasia.txt";

    match path_buffer.extend_from_slice(path) {
        Ok(_) => {
            println!("  Path stored: {} bytes", path_buffer.len());
            println!("  Path: {}", String::from_utf8_lossy(path_buffer.as_slice()));

            // Check if path contains "rahasia"
            let haystack = path_buffer.as_slice();
            let needle = b"rahasia";

            if contains_bytes(haystack, needle) {
                println!("  ✗ BLOCK: Path contains 'rahasia'");
            } else {
                println!("  ✓ ALLOW: Path is safe");
            }
        }
        Err(_) => println!("  Error: Path too long!"),
    }
}

// Helper: Check if haystack contains needle (no String/Vec!)
fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.len() > haystack.len() {
        return false;
    }

    for window in haystack.windows(needle.len()) {
        if window == needle {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut buf = FixedBuffer::<10>::new();
        assert_eq!(buf.len(), 0);

        assert!(buf.push(1).is_ok());
        assert_eq!(buf.len(), 1);
        assert_eq!(buf.as_slice(), &[1]);
    }

    #[test]
    fn test_buffer_full() {
        let mut buf = FixedBuffer::<3>::new();

        assert!(buf.push(1).is_ok());
        assert!(buf.push(2).is_ok());
        assert!(buf.push(3).is_ok());
        assert!(buf.push(4).is_err()); // Should fail - buffer full
    }

    #[test]
    fn test_clear() {
        let mut buf = FixedBuffer::<10>::new();
        buf.push(1).unwrap();
        buf.push(2).unwrap();

        assert_eq!(buf.len(), 2);

        buf.clear();
        assert_eq!(buf.len(), 0);
        assert!(buf.is_empty());
    }

    #[test]
    fn test_extend_from_slice() {
        let mut buf = FixedBuffer::<10>::new();
        assert!(buf.extend_from_slice(&[1, 2, 3]).is_ok());
        assert_eq!(buf.as_slice(), &[1, 2, 3]);
    }

    #[test]
    fn test_contains_bytes() {
        assert!(contains_bytes(b"hello world", b"world"));
        assert!(contains_bytes(b"file_rahasia.txt", b"rahasia"));
        assert!(!contains_bytes(b"normal.txt", b"rahasia"));
    }
}
