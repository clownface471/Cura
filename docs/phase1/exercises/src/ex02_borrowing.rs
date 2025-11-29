// Exercise 2: Mutable Borrowing
// Goal: Understand mutable vs immutable references

fn main() {
    println!("=== Exercise 2: Mutable Borrowing ===\n");

    let mut buffer = vec![0u8; 10];

    println!("Initial buffer: {:?}", buffer);

    // Fill buffer with 0xFF
    fill_buffer(&mut buffer, 0xFF);
    println!("After fill:     {:?}", buffer);

    // Read buffer
    read_buffer(&buffer);

    println!();

    // Demonstrate borrowing rules
    demonstrate_borrowing_rules();

    println!("\n✅ Key Lesson: One &mut OR many & at a time, never both!");
    println!("   This prevents data races at compile time.");
}

fn fill_buffer(buf: &mut [u8], value: u8) {
    for byte in buf.iter_mut() {
        *byte = value;
    }
}

fn read_buffer(buf: &[u8]) {
    println!("Buffer contents: {:?}", buf);
    println!("Buffer length: {}", buf.len());
}

fn demonstrate_borrowing_rules() {
    let mut data = vec![1, 2, 3, 4, 5];

    println!("Borrowing rules demonstration:");

    // Multiple immutable borrows - OK!
    let r1 = &data;
    let r2 = &data;
    println!("  Immutable borrow 1: {:?}", r1);
    println!("  Immutable borrow 2: {:?}", r2);

    // One mutable borrow - OK!
    let r3 = &mut data;
    r3[0] = 99;
    println!("  Mutable borrow: {:?}", r3);

    // After mutable borrow is done, we can borrow again
    println!("  Final data: {:?}", data);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill_buffer() {
        let mut buf = vec![0u8; 5];
        fill_buffer(&mut buf, 0xAA);
        assert_eq!(buf, vec![0xAA; 5]);
    }

    #[test]
    fn test_multiple_immutable_borrows() {
        let data = vec![1, 2, 3];
        let r1 = &data;
        let r2 = &data;
        assert_eq!(r1.len(), r2.len());
    }
}
