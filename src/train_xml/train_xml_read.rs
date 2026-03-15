// src/train_xml/train_xml_read.rs

use std::fs;
use std::path::Path;


pub fn train_xml_read(path_option: Option<&Path>) -> Result<String, Box<dyn std::error::Error>> {
    let path = path_option.unwrap_or(Path::new("./train.xml"));
    
    if !path.exists() {
        return Err(format!("❌ Error: File not found: {}", path.display()).into());
    }

    let train_xml_content = fs::read_to_string(path)?;

    Ok(train_xml_content)
}



#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::path::Path;
    use tempfile::NamedTempFile;
    use crate::train_xml::train_xml_read;

    #[test]
    fn test_train_read_xml_success() -> Result<(), Box<dyn std::error::Error>> {
        let mut temp_file = NamedTempFile::new()?; // Cleanup is automatic! Even if an assertion fails, the file is deleted
        let expected_content = "<train>optimally</train>";
        
        write!(temp_file, "{}", expected_content)?;

        let result = train_xml_read(Some(temp_file.path()));

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_content);
        
        Ok(())
    }

    #[test]
    fn test_train_read_xml_file_not_found() {
        let result = train_xml_read(Some(Path::new("not_real.xml")));
        assert!(result.is_err());
    }
}
