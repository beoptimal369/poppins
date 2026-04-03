// src/bootstrap.rs

use std::fs;
use std::path::Path;


/// Bootstraps (Initializes) a model directory with the following files for training:
/// - `train.xml`: XML configuration file defining the training data structure and model parameters
/// - `train.xsd`: XML Schema Definition file for validating the train.xml structure
///
/// The function will create the entire directory path if it doesn't exist, and will overwrite
/// any existing train.xml and train.xsd files with the default templates.
///
/// # Arguments
/// * `output_dir` - The directory path where the model files will be created (typically `.poppins/{model_name}/`)
///
/// # Behavior
/// Overwrites existing files without warning
///
/// # Panics
/// This function does not panic. All errors are handled gracefully and printed to stderr.
pub fn bootstrap(output_dir: &Path) {
    // Ensure the directory exists
    if let Err(e) = fs::create_dir_all(output_dir) {
        eprintln!("Failed to create directory: {}", e);
        return;
    }

    let xml_dest = output_dir.join("train.xml");
    let xml_content = include_str!("train/train.xml");

    match fs::write(&xml_dest, xml_content) {
        Ok(_) => println!("✅ Wrote {}", xml_dest.display()),
        Err(e) => eprintln!("❌ Failed writing train.xml: {}", e),
    }

    let xsd_dest = output_dir.join("train.xsd");
    let xsd_content = include_str!("train/train.xsd");

    match fs::write(&xsd_dest, xsd_content) {
        Ok(_) => println!("✅ Wrote {}", xsd_dest.display()),
        Err(e) => eprintln!("❌ Failed writing train.xsd: {}", e),
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_bootstrap_creates_directory() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path().join("test_model");
        
        assert!(!output_dir.exists(), "Directory should not exist before bootstrap");
        
        bootstrap(&output_dir);
        
        assert!(output_dir.exists(), "Directory should be created by bootstrap");
        assert!(output_dir.is_dir(), "Path should be a directory");
    }

    #[test]
    fn test_bootstrap_creates_train_xml() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path().join("test_model");
        
        bootstrap(&output_dir);
        
        let xml_path = output_dir.join("train.xml");
        assert!(xml_path.exists(), "train.xml should be created");
        assert!(xml_path.is_file(), "train.xml should be a file");
        
        // Verify content is not empty
        let content = fs::read_to_string(&xml_path).unwrap();
        assert!(!content.is_empty(), "train.xml should not be empty");
        
        // Basic XML validation - should contain root element
        assert!(content.contains("<?xml"), "train.xml should contain XML declaration");
        assert!(content.contains("<train"), "train.xml should contain <train> root element");
    }

    #[test]
    fn test_bootstrap_creates_train_xsd() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path().join("test_model");
        
        bootstrap(&output_dir);
        
        let xsd_path = output_dir.join("train.xsd");
        assert!(xsd_path.exists(), "train.xsd should be created");
        assert!(xsd_path.is_file(), "train.xsd should be a file");
        
        // Verify content is not empty
        let content = fs::read_to_string(&xsd_path).unwrap();
        assert!(!content.is_empty(), "train.xsd should not be empty");
        
        // Basic XSD validation
        assert!(content.contains("<?xml"), "train.xsd should contain XML declaration");
        assert!(content.contains("<xs:schema"), "train.xsd should contain schema element");
    }

    #[test]
    fn test_bootstrap_overwrites_existing_files() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path().join("test_model");
        
        // First bootstrap
        bootstrap(&output_dir);
        
        let xml_path = output_dir.join("train.xml");
        let original_content = fs::read_to_string(&xml_path).unwrap();
        
        // Modify the file
        fs::write(&xml_path, "modified content").unwrap();
        
        // Second bootstrap should overwrite
        bootstrap(&output_dir);
        
        let new_content = fs::read_to_string(&xml_path).unwrap();
        assert_eq!(new_content, original_content, "bootstrap should overwrite existing files");
        assert_ne!(new_content, "modified content", "File should be restored to original");
    }

    #[test]
    fn test_bootstrap_creates_nested_directories() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path().join("deeply").join("nested").join("path").join("test_model");
        
        assert!(!output_dir.exists(), "Nested directory should not exist before bootstrap");
        
        bootstrap(&output_dir);
        
        assert!(output_dir.exists(), "Nested directory should be created");
        assert!(output_dir.join("train.xml").exists(), "train.xml should be created in nested path");
        assert!(output_dir.join("train.xsd").exists(), "train.xsd should be created in nested path");
    }

    #[test]
    fn test_bootstrap_handles_existing_directory() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path().join("existing_model");
        
        // Create directory first
        fs::create_dir_all(&output_dir).unwrap();
        assert!(output_dir.exists(), "Directory should exist before bootstrap");
        
        // Bootstrap should still work
        bootstrap(&output_dir);
        
        assert!(output_dir.join("train.xml").exists(), "train.xml should be created in existing directory");
        assert!(output_dir.join("train.xsd").exists(), "train.xsd should be created in existing directory");
    }

    #[test]
    fn test_bootstrap_file_permissions() {
        #[cfg(unix)]
        {
            let temp_dir = tempdir().unwrap();
            let output_dir = temp_dir.path().join("test_model");
            
            bootstrap(&output_dir);
            
            use std::os::unix::fs::PermissionsExt;
            let xml_path = output_dir.join("train.xml");
            let metadata = fs::metadata(&xml_path).unwrap();
            let permissions = metadata.permissions();
            
            // Check that file is readable and writable by owner
            assert!(permissions.mode() & 0o600 != 0, "File should have read/write permissions for owner");
        }
        
        // For Windows, we just check that files exist (permissions are more complex)
        #[cfg(windows)]
        {
            let temp_dir = tempdir().unwrap();
            let output_dir = temp_dir.path().join("test_model");
            
            bootstrap(&output_dir);
            
            let xml_path = output_dir.join("train.xml");
            assert!(xml_path.exists(), "File should exist on Windows");
        }
    }

    #[test]
    fn test_bootstrap_with_relative_path() {
        // Create a temp directory and change to it
        let temp_dir = tempdir().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        
        std::env::set_current_dir(temp_dir.path()).unwrap();
        
        let relative_path = Path::new("relative_model");
        bootstrap(relative_path);
        
        assert!(relative_path.exists(), "Relative path should be created");
        assert!(relative_path.join("train.xml").exists(), "train.xml should be created in relative path");
        assert!(relative_path.join("train.xsd").exists(), "train.xsd should be created in relative path");
        
        // Clean up - change back to original directory
        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_bootstrap_idempotent() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path().join("test_model");
        
        // Run bootstrap multiple times
        for i in 0..3 {
            bootstrap(&output_dir);
            assert!(output_dir.join("train.xml").exists(), "Iteration {}: train.xml should exist", i);
            assert!(output_dir.join("train.xsd").exists(), "Iteration {}: train.xsd should exist", i);
        }
        
        // Verify content is consistent across runs
        let xml_content = fs::read_to_string(output_dir.join("train.xml")).unwrap();
        let xsd_content = fs::read_to_string(output_dir.join("train.xsd")).unwrap();
        
        // Run bootstrap again
        bootstrap(&output_dir);
        
        let new_xml_content = fs::read_to_string(output_dir.join("train.xml")).unwrap();
        let new_xsd_content = fs::read_to_string(output_dir.join("train.xsd")).unwrap();
        
        assert_eq!(xml_content, new_xml_content, "XML content should be identical across runs");
        assert_eq!(xsd_content, new_xsd_content, "XSD content should be identical across runs");
    }

    #[test]
    fn test_bootstrap_handles_invalid_characters_in_path() {
        #[cfg(unix)]
        {
            let temp_dir = tempdir().unwrap();
            // Unix allows most characters except null and slash
            let output_dir = temp_dir.path().join("model-with-dots.and_underscores-123");
            bootstrap(&output_dir);
            assert!(output_dir.exists(), "Should handle valid special characters");
            assert!(output_dir.join("train.xml").exists());
        }
        
        #[cfg(windows)]
        {
            let temp_dir = tempdir().unwrap();
            // Windows has more restrictions, but we'll test with allowed characters
            let output_dir = temp_dir.path().join("model-with-dots.and_underscores-123");
            bootstrap(&output_dir);
            assert!(output_dir.exists(), "Should handle valid special characters");
            assert!(output_dir.join("train.xml").exists());
        }
    }

    #[test]
    fn test_bootstrap_file_sizes() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path().join("test_model");
        
        bootstrap(&output_dir);
        
        let xml_path = output_dir.join("train.xml");
        let xsd_path = output_dir.join("train.xsd");
        
        let xml_size = fs::metadata(&xml_path).unwrap().len();
        let xsd_size = fs::metadata(&xsd_path).unwrap().len();
        
        assert!(xml_size > 0, "train.xml should have content (size > 0 bytes)");
        assert!(xsd_size > 0, "train.xsd should have content (size > 0 bytes)");
        
        // Optional: log sizes for debugging
        println!("train.xml size: {} bytes", xml_size);
        println!("train.xsd size: {} bytes", xsd_size);
    }
}
