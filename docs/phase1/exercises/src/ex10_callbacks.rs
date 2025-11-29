// Exercise 10: Callback Pattern
// Goal: Understand function pointers (used for driver callbacks)

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
    !path.to_lowercase().contains("rahasia")
}

fn main() {
    let mut callbacks = FilterCallbacks::new();
    callbacks.set_pre_create(my_pre_create_callback);

    // Simulate file accesses
    let paths = vec![
        "C:\\test.txt",
        "C:\\file_rahasia.txt",
        "C:\\RAHASIA.doc",
        "C:\\normal.doc",
    ];

    println!("File Access Simulation:\n");

    for path in paths {
        let allowed = callbacks.invoke_pre_create(path);
        let status = if allowed { "✓ ALLOW" } else { "✗ BLOCK" };
        println!("{}: {}", status, path);
    }

    println!("\n✅ Exercise complete! This is exactly how driver callbacks work.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_rahasia() {
        assert!(!my_pre_create_callback("file_rahasia.txt"));
        assert!(!my_pre_create_callback("FILE_RAHASIA.TXT"));
    }

    #[test]
    fn test_allow_normal() {
        assert!(my_pre_create_callback("normal.txt"));
        assert!(my_pre_create_callback("document.doc"));
    }
}
