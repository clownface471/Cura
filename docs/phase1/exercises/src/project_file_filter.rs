// Practice Project: Mini File Filter
// Goal: Build a usermode file filter simulator

use std::path::Path;

struct FileFilter {
    blocked_patterns: Vec<String>,
}

impl FileFilter {
    fn new() -> Self {
        Self {
            blocked_patterns: vec![
                "rahasia".to_string(),
                "secret".to_string(),
                "confidential".to_string(),
            ],
        }
    }

    fn add_pattern(&mut self, pattern: String) {
        self.blocked_patterns.push(pattern.to_lowercase());
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
                    return Err(format!("Access denied: {}", filename_str));
                }
            }
        }
        Ok(())
    }
}

struct FilterStatistics {
    allowed: usize,
    blocked: usize,
}

impl FilterStatistics {
    fn new() -> Self {
        Self { allowed: 0, blocked: 0 }
    }

    fn record_allow(&mut self) {
        self.allowed += 1;
    }

    fn record_block(&mut self) {
        self.blocked += 1;
    }

    fn print_summary(&self) {
        println!("\n=== Filter Statistics ===");
        println!("Allowed: {}", self.allowed);
        println!("Blocked: {}", self.blocked);
        println!("Total:   {}", self.allowed + self.blocked);

        if self.allowed + self.blocked > 0 {
            let block_rate = (self.blocked as f64 / (self.allowed + self.blocked) as f64) * 100.0;
            println!("Block rate: {:.1}%", block_rate);
        }
    }
}

fn main() {
    println!("=== CURA Mini File Filter ===\n");
    println!("This simulates what the kernel driver will do!\n");

    let filter = FileFilter::new();
    let mut stats = FilterStatistics::new();

    // Test files
    let test_files = vec![
        "C:\\Users\\Nora\\Documents\\test.txt",
        "C:\\Users\\Nora\\Documents\\file_rahasia.txt",
        "C:\\Users\\Nora\\Desktop\\presentation.pptx",
        "C:\\Users\\Nora\\Downloads\\secret_document.doc",
        "C:\\Program Files\\app.exe",
        "C:\\Windows\\System32\\confidential.dat",
        "C:\\normal_file.txt",
    ];

    println!("Testing file access:\n");

    for file in test_files {
        let path = Path::new(file);
        match filter.check_file(path) {
            Ok(_) => {
                println!("✓ ALLOW: {}", file);
                stats.record_allow();
            }
            Err(e) => {
                println!("✗ BLOCK: {} ({})", file, e);
                stats.record_block();
            }
        }
    }

    stats.print_summary();

    println!("\n✅ Practice project complete!");
    println!("Next step: Implement this in kernel mode (Ring 0)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_patterns() {
        let filter = FileFilter::new();

        assert!(filter.should_block("file_rahasia.txt"));
        assert!(filter.should_block("SECRET.doc"));
        assert!(filter.should_block("confidential_data.xlsx"));
    }

    #[test]
    fn test_allow_normal() {
        let filter = FileFilter::new();

        assert!(!filter.should_block("normal.txt"));
        assert!(!filter.should_block("presentation.pptx"));
        assert!(!filter.should_block("photo.jpg"));
    }

    #[test]
    fn test_case_insensitive() {
        let filter = FileFilter::new();

        assert!(filter.should_block("RAHASIA.txt"));
        assert!(filter.should_block("RaHaSiA.TXT"));
        assert!(filter.should_block("file_RAHASIA_2024.doc"));
    }

    #[test]
    fn test_add_pattern() {
        let mut filter = FileFilter::new();
        filter.add_pattern("classified".to_string());

        assert!(filter.should_block("classified.pdf"));
    }
}
