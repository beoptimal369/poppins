// src/bootstrap.rs

use std::{fs, error::Error, path::Path};


/// Bootstraps (Initializes) a model directory with the following files for training:
/// - `train.xml`: XML configuration file defining the training data structure and model parameters
/// - `train.xsd`: XML Schema Definition file for validating the train.xml structure
/// - `math.xml`: Math domain training data
/// - `english.xml`: English domain training data
///
/// The function will create the entire directory path if it doesn't exist, and will overwrite
/// any existing files with the default templates.
///
/// # Arguments
/// * `output_dir` - The directory path where the model files will be created (typically `.poppins/{model_name}/`)
///
/// # Errors
/// Returns an error if directory creation or any file write operation fails
pub fn bootstrap(output_dir: &Path) -> Result<(), Box<dyn Error>> {
    // Create directory
    fs::create_dir_all(output_dir)?;

    write_file(output_dir, TRAIN_XML_CONTENT, "train.xml")?;
    write_file(output_dir, TRAIN_XSD_CONTENT, "train.xsd")?;
    write_file(output_dir, MATH_XML_CONTENT, "math.xml")?;
    write_file(output_dir, ENGLISH_XML_CONTENT, "english.xml")?;

    Ok(())
}


const TRAIN_XML_CONTENT: &str = include_str!("train/train.xml");
const TRAIN_XSD_CONTENT: &str = include_str!("train/train.xsd");
const MATH_XML_CONTENT: &str = include_str!("train/math.xml");
const ENGLISH_XML_CONTENT: &str = include_str!("train/english.xml");


fn write_file(output_dir: &Path, src_content: &str, dest_file_name: &str) -> Result<(), Box<dyn Error>> {
    let dest_path_puf = output_dir.join(dest_file_name);
    fs::write(&dest_path_puf, src_content)?;
    println!("✅ Wrote {}", dest_path_puf.display());

    Ok(())
}



#[cfg(test)]
mod tests {
    use super::bootstrap;
    use tempfile::tempdir;
    use std::{fs, error::Error, path::Path};

    #[test]
    fn test_bootstrap_creates_directory() -> Result<(), Box<dyn Error>> {
        let temp_dir = tempdir()?;
        let output_dir = temp_dir.path().join("test_model");
        
        assert!(!output_dir.exists(), "Directory should not exist before bootstrap");
        
        bootstrap(&output_dir)?;
        
        assert!(output_dir.exists(), "Directory should be created by bootstrap");
        assert!(output_dir.is_dir(), "Path should be a directory");
        
        Ok(())
    }

    #[test]
    fn test_bootstrap_creates_train_xml() -> Result<(), Box<dyn Error>> {
        let temp_dir = tempdir()?;
        let output_dir = temp_dir.path().join("test_model");
        
        bootstrap(&output_dir)?;
        
        let xml_path = output_dir.join("train.xml");
        assert!(xml_path.exists(), "train.xml should be created");
        assert!(xml_path.is_file(), "train.xml should be a file");
        
        let content = fs::read_to_string(&xml_path)?;
        assert!(!content.is_empty(), "train.xml should not be empty");
        assert!(content.contains("<?xml"), "train.xml should contain XML declaration");
        assert!(content.contains("<train"), "train.xml should contain <train> root element");
        
        Ok(())
    }

    #[test]
    fn test_bootstrap_creates_english_xml() -> Result<(), Box<dyn Error>> {
        let temp_dir = tempdir()?;
        let output_dir = temp_dir.path().join("test_model");
        
        bootstrap(&output_dir)?;
        
        let xml_path = output_dir.join("english.xml");
        assert!(xml_path.exists(), "english.xml should be created");
        assert!(xml_path.is_file(), "english.xml should be a file");
        
        let content = fs::read_to_string(&xml_path)?;
        assert!(!content.is_empty(), "english.xml should not be empty");
        assert!(content.contains("<?xml"), "english.xml should contain XML declaration");
        assert!(content.contains("<train"), "english.xml should contain <train> root element");
        
        Ok(())
    }

    #[test]
    fn test_bootstrap_creates_math_xml() -> Result<(), Box<dyn Error>> {
        let temp_dir = tempdir()?;
        let output_dir = temp_dir.path().join("test_model");
        
        bootstrap(&output_dir)?;
        
        let xml_path = output_dir.join("math.xml");
        assert!(xml_path.exists(), "math.xml should be created");
        assert!(xml_path.is_file(), "math.xml should be a file");
        
        let content = fs::read_to_string(&xml_path)?;
        assert!(!content.is_empty(), "math.xml should not be empty");
        assert!(content.contains("<?xml"), "math.xml should contain XML declaration");
        assert!(content.contains("<train"), "math.xml should contain <train> root element");
        
        Ok(())
    }

    #[test]
    fn test_bootstrap_creates_train_xsd() -> Result<(), Box<dyn Error>> {
        let temp_dir = tempdir()?;
        let output_dir = temp_dir.path().join("test_model");
        
        bootstrap(&output_dir)?;
        
        let xsd_path = output_dir.join("train.xsd");
        assert!(xsd_path.exists(), "train.xsd should be created");
        assert!(xsd_path.is_file(), "train.xsd should be a file");
        
        let content = fs::read_to_string(&xsd_path)?;
        assert!(!content.is_empty(), "train.xsd should not be empty");
        assert!(content.contains("<?xml"), "train.xsd should contain XML declaration");
        assert!(content.contains("<xs:schema"), "train.xsd should contain schema element");
        
        Ok(())
    }

    #[test]
    fn test_bootstrap_overwrites_existing_files() -> Result<(), Box<dyn Error>> {
        let temp_dir = tempdir()?;
        let output_dir = temp_dir.path().join("test_model");
        
        bootstrap(&output_dir)?;
        
        let xml_path = output_dir.join("train.xml");
        let original_content = fs::read_to_string(&xml_path)?;
        
        fs::write(&xml_path, "modified content")?;
        
        bootstrap(&output_dir)?;
        
        let new_content = fs::read_to_string(&xml_path)?;
        assert_eq!(new_content, original_content, "bootstrap should overwrite existing files");
        assert_ne!(new_content, "modified content", "File should be restored to original");
        
        Ok(())
    }

    #[test]
    fn test_bootstrap_creates_nested_directories() -> Result<(), Box<dyn Error>> {
        let temp_dir = tempdir()?;
        let output_dir = temp_dir.path().join("deeply").join("nested").join("path").join("test_model");
        
        assert!(!output_dir.exists(), "Nested directory should not exist before bootstrap");
        
        bootstrap(&output_dir)?;
        
        assert!(output_dir.exists(), "Nested directory should be created");
        assert!(output_dir.join("train.xml").exists(), "train.xml should be created in nested path");
        assert!(output_dir.join("train.xsd").exists(), "train.xsd should be created in nested path");
        
        Ok(())
    }

    #[test]
    fn test_bootstrap_handles_existing_directory() -> Result<(), Box<dyn Error>> {
        let temp_dir = tempdir()?;
        let output_dir = temp_dir.path().join("existing_model");
        
        fs::create_dir_all(&output_dir)?;
        assert!(output_dir.exists(), "Directory should exist before bootstrap");
        
        bootstrap(&output_dir)?;
        
        assert!(output_dir.join("train.xml").exists(), "train.xml should be created in existing directory");
        assert!(output_dir.join("train.xsd").exists(), "train.xsd should be created in existing directory");
        
        Ok(())
    }

    #[test]
    fn test_bootstrap_file_sizes() -> Result<(), Box<dyn Error>> {
        let temp_dir = tempdir()?;
        let output_dir = temp_dir.path().join("test_model");
        
        bootstrap(&output_dir)?;
        
        let xml_path = output_dir.join("train.xml");
        let xsd_path = output_dir.join("train.xsd");
        
        let xml_size = fs::metadata(&xml_path)?.len();
        let xsd_size = fs::metadata(&xsd_path)?.len();
        
        assert!(xml_size > 0, "train.xml should have content (size > 0 bytes)");
        assert!(xsd_size > 0, "train.xsd should have content (size > 0 bytes)");
        
        Ok(())
    }

    #[test]
    fn test_bootstrap_idempotent() -> Result<(), Box<dyn Error>> {
        let temp_dir = tempdir()?;
        let output_dir = temp_dir.path().join("test_model");
        
        for i in 0..3 {
            bootstrap(&output_dir)?;
            assert!(output_dir.join("train.xml").exists(), "Iteration {}: train.xml should exist", i);
            assert!(output_dir.join("train.xsd").exists(), "Iteration {}: train.xsd should exist", i);
        }
        
        let xml_content = fs::read_to_string(output_dir.join("train.xml"))?;
        let xsd_content = fs::read_to_string(output_dir.join("train.xsd"))?;
        
        bootstrap(&output_dir)?;
        
        let new_xml_content = fs::read_to_string(output_dir.join("train.xml"))?;
        let new_xsd_content = fs::read_to_string(output_dir.join("train.xsd"))?;
        
        assert_eq!(xml_content, new_xml_content, "XML content should be identical across runs");
        assert_eq!(xsd_content, new_xsd_content, "XSD content should be identical across runs");
        
        Ok(())
    }

    #[test]
    fn test_bootstrap_with_relative_path() -> Result<(), Box<dyn Error>> {
        let temp_dir = tempdir()?;
        let original_dir = std::env::current_dir()?;
        
        std::env::set_current_dir(temp_dir.path())?;
        
        let relative_path = Path::new("relative_model");
        bootstrap(relative_path)?;
        
        assert!(relative_path.exists(), "Relative path should be created");
        assert!(relative_path.join("train.xml").exists(), "train.xml should be created in relative path");
        assert!(relative_path.join("train.xsd").exists(), "train.xsd should be created in relative path");
        
        std::env::set_current_dir(original_dir)?;
        
        Ok(())
    }
}
