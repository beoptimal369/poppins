// src/bootstrap.rs

use std::fs;
use std::path::Path;

pub fn bootstrap(output_dir_path: Option<&Path>) {
    let output_dir = output_dir_path.unwrap_or(Path::new("."));
    
    // Ensure the directory exists
    if let Err(e) = fs::create_dir_all(output_dir) {
        eprintln!("Failed to create directory: {}", e);
        return;
    }

    let dest_path = output_dir.join("train.xml");
    let xml_content = include_str!("train/train.xml");

    match fs::write(&dest_path, xml_content) {
        Ok(_) => println!("Successfully wrote {}", dest_path.display()),
        Err(e) => eprintln!("Failed to write train.xml: {}", e),
    }
}



#[cfg(test)]
mod tests {
    use std::fs;
    use super::bootstrap;
    use tempfile::tempdir;

    #[test]
    fn test_bootstrap_creates_file_in_temp_dir() {
        let dir = tempdir().expect("Failed to create temp dir");
        let path = dir.path();
        
        bootstrap(Some(path));

        let expected_file = path.join("train.xml");
        assert!(expected_file.exists(), "train.xml should exist in temp dir");
        
        let content = fs::read_to_string(expected_file).expect("Should read back file");

        assert!(content.contains("<train"), "XML should start with the train tag");
        assert!(content.contains("</train>"), "XML should have a closing train tag");
    }

    #[test]
    fn test_bootstrap_logic_defaults_to_dot() {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp dir");
        let original_cwd = std::env::current_dir().expect("Failed to get CWD");
        
        // We wrap this in a way that ensures we ALWAYS return to the original CWD even if the test panics
        struct CwdGuard(std::path::PathBuf);

        impl Drop for CwdGuard {
            fn drop(&mut self) {
                std::env::set_current_dir(&self.0).expect("Failed to drop");
            }
        }
        
        CwdGuard(original_cwd);
        std::env::set_current_dir(temp_dir.path()).expect("Failed to set CWD");

        // Act
        bootstrap(None);

        // Assert
        let expected_file = temp_dir.path().join("train.xml");
        assert!(expected_file.exists(), "Should have written to the temporary CWD");
    }
}
