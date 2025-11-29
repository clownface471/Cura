// Exercise 1: String Ownership
// Goal: Understand ownership transfer

fn main() {
    println!("=== Exercise 1: String Ownership ===\n");

    let message = String::from("CURA initializing");

    // Version 1: Using reference (borrowing)
    print_message_ref(&message);
    print_message_ref(&message); // This works! We're borrowing, not taking ownership

    println!();

    // Version 2: Taking ownership (original problem)
    let message2 = String::from("CURA system check");
    print_message_owned(message2);
    // print_message_owned(message2); // ERROR! Ownership was moved

    println!();

    // Version 3: Using clone
    let message3 = String::from("CURA ready");
    print_message_owned(message3.clone());
    print_message_owned(message3); // This works! We cloned it

    println!("\n✅ Key Lesson: Use & to borrow, or clone() to duplicate");
    println!("   In kernel code, we rarely clone (no heap). We use references!");
}

// This function borrows the string (doesn't take ownership)
fn print_message_ref(s: &String) {
    println!("Message (borrowed): {}", s);
}

// This function takes ownership
fn print_message_owned(s: String) {
    println!("Message (owned): {}", s);
    // s is dropped here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_borrowing() {
        let msg = String::from("test");
        print_message_ref(&msg);
        // Should still be usable
        assert_eq!(msg, "test");
    }

    #[test]
    fn test_ownership_transfer() {
        let msg = String::from("test");
        print_message_owned(msg);
        // msg is no longer valid here
    }
}
