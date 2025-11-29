// Exercise 3: Lifetime Annotations
// Goal: Understand lifetimes (critical for kernel development)

fn main() {
    println!("=== Exercise 3: Lifetime Annotations ===\n");

    let path1 = String::from("C:\\Windows\\System32");
    let path2 = String::from("C:\\Program Files");

    let result = longest_path(&path1, &path2);
    println!("Path 1: {}", path1);
    println!("Path 2: {}", path2);
    println!("Longest: {}\n", result);

    // Demonstrate lifetime scope
    demonstrate_lifetime_scope();

    // Kernel-relevant example
    kernel_example();

    println!("\n✅ Key Lesson: Lifetimes ensure references don't outlive data");
    println!("   In kernel: critical for driver contexts, IRPs, buffers!");
}

// Lifetime 'a means: the returned reference lives as long as the shortest input
fn longest_path<'a>(path1: &'a str, path2: &'a str) -> &'a str {
    if path1.len() > path2.len() {
        path1
    } else {
        path2
    }
}

fn demonstrate_lifetime_scope() {
    println!("Lifetime scope demonstration:");

    let string1 = String::from("long string");
    let result;

    {
        let string2 = String::from("short");
        result = longest_path(&string1, &string2);
        println!("  Inside scope: {}", result);
        // string2 is still alive here
    }
    // string2 is dropped here - result would be invalid if it pointed to string2!

    // This would be a compile error if result could point to string2:
    // println!("  Outside scope: {}", result);
}

// Kernel-like example: File path validation
struct FileRequest<'a> {
    path: &'a str,
    process_id: u32,
}

impl<'a> FileRequest<'a> {
    fn new(path: &'a str, pid: u32) -> Self {
        Self {
            path,
            process_id: pid,
        }
    }

    fn should_block(&self) -> bool {
        self.path.to_lowercase().contains("rahasia")
    }
}

fn kernel_example() {
    println!("Kernel-like example:");

    let path = String::from("C:\\file_rahasia.txt");
    let request = FileRequest::new(&path, 1234);

    if request.should_block() {
        println!("  BLOCK: {} (PID: {})", request.path, request.process_id);
    } else {
        println!("  ALLOW: {} (PID: {})", request.path, request.process_id);
    }

    // path is still valid here because request only borrows it
    println!("  Original path still accessible: {}", path);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_path() {
        assert_eq!(
            longest_path("short", "much longer path"),
            "much longer path"
        );
    }

    #[test]
    fn test_file_request() {
        let path = String::from("C:\\rahasia.txt");
        let req = FileRequest::new(&path, 100);
        assert!(req.should_block());
    }

    #[test]
    fn test_file_request_allow() {
        let path = String::from("C:\\normal.txt");
        let req = FileRequest::new(&path, 100);
        assert!(!req.should_block());
    }
}
