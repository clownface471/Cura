// Exercise 4: Result Type
// Goal: Master Result<T, E> for error handling

use std::fs::File;
use std::io::{Read, Write};

fn main() {
    println!("=== Exercise 4: Result Type ===\n");

    // Try to read a config file
    match read_config("config.txt") {
        Ok(contents) => println!("Config loaded:\n{}\n", contents),
        Err(e) => println!("Failed to read config: {}\n", e),
    }

    // Create a test config file
    if let Err(e) = create_test_config("test_config.txt") {
        println!("Failed to create config: {}", e);
    } else {
        println!("Test config created successfully");

        // Now read it
        match read_config("test_config.txt") {
            Ok(contents) => println!("Test config:\n{}", contents),
            Err(e) => println!("Error: {}", e),
        }
    }

    println!();

    // Using the ? operator
    demonstrate_question_mark();

    println!("\n✅ Key Lesson: Result<T, E> forces you to handle errors");
    println!("   In kernel: NTSTATUS codes work similarly!");
}

fn read_config(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?; // ? propagates errors
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn create_test_config(path: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;
    file.write_all(b"[CURA Configuration]\n")?;
    file.write_all(b"mode=turbo\n")?;
    file.write_all(b"protection_level=high\n")?;
    Ok(())
}

fn demonstrate_question_mark() {
    println!("The ? operator demonstration:");

    // Without ?
    fn old_style() -> Result<String, std::io::Error> {
        let mut file = match File::open("test.txt") {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => {},
            Err(e) => return Err(e),
        }

        Ok(contents)
    }

    // With ? (much cleaner!)
    fn new_style() -> Result<String, std::io::Error> {
        let mut file = File::open("test.txt")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    println!("  Old style: verbose match expressions");
    println!("  New style: clean ? operator");
    println!("  Both do the same thing!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_create_and_read_config() {
        let test_file = "test_exercise4.txt";

        // Create
        assert!(create_test_config(test_file).is_ok());

        // Read
        let contents = read_config(test_file).unwrap();
        assert!(contents.contains("CURA Configuration"));
        assert!(contents.contains("mode=turbo"));

        // Cleanup
        let _ = fs::remove_file(test_file);
    }

    #[test]
    fn test_read_nonexistent_file() {
        let result = read_config("this_file_does_not_exist.txt");
        assert!(result.is_err());
    }
}
