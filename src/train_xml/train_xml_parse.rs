// src/train_xml/train_xml_parse.rs

use std::fs;
use std::path::Path;
use crate::train_xml::TrainXML;


/// Parse train.xml and all its imports from the given output directory
///
/// The first element in the vector is the main train.xml, followed by its imports in order.
///
/// # Arguments
/// * `output_dir` - Directory containing the main train.xml file
///
/// # Returns
/// * `Result<Vec<TrainXML>, Box<dyn std::error::Error>>` - Vector of parsed TrainXML structs
pub fn train_xml_parse(
    output_dir: &Path,
) -> Result<Vec<TrainXML>, Box<dyn std::error::Error>> {
    let train_xml_path = output_dir.join("train.xml");
    
    // Check if train.xml exists
    if !train_xml_path.exists() {
        return Err(format!("train.xml not found in directory: {}", output_dir.display()).into());
    }
    
    // Read the main train.xml
    let train_content = fs::read_to_string(&train_xml_path)?;
    
    let mut result = Vec::new();
    
    // Parse the main XML
    let main_train_xml: TrainXML = quick_xml::de::from_str(&train_content)?;
    result.push(main_train_xml);
    
    // Process imports recursively using the directory as base path
    process_imports(&mut result, output_dir)?;
    
    Ok(result)
}


/// Recursively process imports and add them to the result vector
fn process_imports(
    result: &mut Vec<TrainXML>,
    base_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    // Get the index of the last item BEFORE we start processing
    let current_len = result.len();
    
    // Get the imports from the last item (clone the imports to avoid borrow issues)
    let imports = result[current_len - 1].imports.clone();
    
    if let Some(imports) = imports {
        for import in imports.import {
            // Validate import path
            let import_path = Path::new(&import.path);
            
            // Check if path is absolute or relative
            let full_path = if import_path.is_absolute() {
                import_path.to_path_buf()
            } else {
                base_path.join(import_path)
            };
            
            // Validate file exists
            if !full_path.exists() {
                return Err(format!("Import file does not exist: {}", full_path.display()).into());
            }
            
            // Validate it's a file, not a directory
            if !full_path.is_file() {
                return Err(format!("Import path is not a file: {}", full_path.display()).into());
            }
            
            // Read and parse the imported file
            let import_content = fs::read_to_string(&full_path)?;
            let imported_train_xml: TrainXML = quick_xml::de::from_str(&import_content)?;
            
            // Add to result vector
            result.push(imported_train_xml);
            
            // Recursively process imports from this imported file
            // Use the directory of the imported file as the new base path
            let import_dir = full_path.parent().unwrap_or(base_path);
            process_imports(result, import_dir)?;
        }
    }
    
    Ok(())
}



#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile::tempdir;
    use crate::train_xml::{
        train_xml_parse,
        TrainXMLSamplesSampleChildren,
    };

    #[test]
    fn test_train_xml_parse_success() {
        let temp_dir = tempdir().unwrap();
        
        // Create train.xml
        let xml_content = r#"
            <train>
                <constants>
                    <aim-train-gb>3.0</aim-train-gb>
                    <batch-size>32</batch-size>
                    <mixed-precision>true</mixed-precision>
                    <bpe-requested-tokens>
                        <value>function</value>
                        <value>console.log</value>
                    </bpe-requested-tokens>
                </constants>
                <samples>
                    <sample>
                        <prompt id="pr::aum::test" />
                        <response-ids response="re::aum::test" source="so::aum::test" />
                    </sample>
                </samples>
            </train>
        "#;
        
        let train_xml_path = temp_dir.path().join("train.xml");
        fs::write(&train_xml_path, xml_content).unwrap();
        
        let result = train_xml_parse(temp_dir.path());
        
        // Assertions
        assert!(result.is_ok(), "Parser failed on valid XML: {:?}", result.err());
        let train_xmls = result.unwrap();
        assert_eq!(train_xmls.len(), 1);
        let train_xml = &train_xmls[0];
        
        // Verify Constants - now accessing fields directly
        let constants = train_xml.constants.as_ref().unwrap();
        assert_eq!(constants.aim_train_gb, Some(3.0));
        assert_eq!(constants.batch_size, Some(32));
        assert_eq!(constants.mixed_precision, Some(true));
        
        // Verify BPE requested tokens
        let bpe_tokens = constants.bpe_requested_tokens.as_ref().unwrap();
        assert_eq!(bpe_tokens.values.len(), 2);
        assert_eq!(bpe_tokens.values[0], "function");
        assert_eq!(bpe_tokens.values[1], "console.log");

        // Verify nested Sample and ResponseIds via children
        let samples = train_xml.samples.as_ref().unwrap();
        let first_sample = &samples.sample.as_ref().unwrap()[0];
        
        // Find the ResponseIds in the children
        let resp_ids = first_sample.children.iter().find_map(|child| {
            if let TrainXMLSamplesSampleChildren::ResponseIds(ids) = child {
                Some(ids)
            } else {
                None
            }
        }).expect("Should find ResponseIds in children");
        
        assert_eq!(resp_ids.response, "re::aum::test");
        assert_eq!(resp_ids.source.as_deref(), Some("so::aum::test"));
    }

    #[test]
    fn test_train_xml_parse_with_defaults() {
        let temp_dir = tempdir().unwrap();
        
        // Create train.xml with minimal content
        let xml_content = r#"
            <train>
                <constants>
                    <aim-train-gb>4.5</aim-train-gb>
                </constants>
                <samples>
                    <sample>
                        <prompt id="pr::aum:;test" />
                    </sample>
                </samples>
            </train>
        "#;
        
        let train_xml_path = temp_dir.path().join("train.xml");
        fs::write(&train_xml_path, xml_content).unwrap();
        
        let result = train_xml_parse(temp_dir.path());
        assert!(result.is_ok());
        let train_xmls = result.unwrap();
        assert_eq!(train_xmls.len(), 1);
        let train_xml = &train_xmls[0];
        
        let constants = train_xml.constants.as_ref().unwrap();
        assert_eq!(constants.aim_train_gb, Some(4.5));
        // Other fields should be None (will use defaults later)
        assert_eq!(constants.batch_size, None);
        assert_eq!(constants.learning_rate, None);
    }

    #[test]
    fn test_train_xml_parse_empty_constants() {
        let temp_dir = tempdir().unwrap();
        
        // Create train.xml with no constants section
        let xml_content = r#"
            <train>
                <samples>
                    <sample>
                        <prompt id="pr::aum::test" />
                    </sample>
                </samples>
            </train>
        "#;
        
        let train_xml_path = temp_dir.path().join("train.xml");
        fs::write(&train_xml_path, xml_content).unwrap();
        
        let result = train_xml_parse(temp_dir.path());
        assert!(result.is_ok());
        let train_xmls = result.unwrap();
        assert_eq!(train_xmls.len(), 1);
        let train_xml = &train_xmls[0];
        
        // Constants should be None (will use all defaults)
        assert!(train_xml.constants.is_none());
    }

    #[test]
    fn test_train_xml_parse_fail_malformed() {
        let temp_dir = tempdir().unwrap();
        
        // Create malformed XML
        let malformed_xml = r#"
            <train>
                <constants>
                    <aim_train_gb>3.0
                </constants>
            </train>
        "#;
        
        let train_xml_path = temp_dir.path().join("train.xml");
        fs::write(&train_xml_path, malformed_xml).unwrap();
        
        let result = train_xml_parse(temp_dir.path());
        assert!(result.is_err(), "Expected parsing to fail for malformed XML");
    }

    #[test]
    fn test_train_xml_parse_type_mismatch() {
        let temp_dir = tempdir().unwrap();
        
        // Create XML with wrong type for a float field
        let xml_content = r#"
            <train>
                <constants>
                    <aim-train-gb>not_a_number</aim-train-gb>
                </constants>
                <samples>
                    <sample>
                        <prompt id="pr::aum::test" />
                    </sample>
                </samples>
            </train>
        "#;
        
        let train_xml_path = temp_dir.path().join("train.xml");
        fs::write(&train_xml_path, xml_content).unwrap();
        
        let result = train_xml_parse(temp_dir.path());
        
        // Should fail because string can't be parsed as f32
        assert!(result.is_err(), "Expected parsing to fail for type mismatch");
    }

    #[test]
    fn test_train_xml_parse_missing_train_xml() {
        let temp_dir = tempdir().unwrap();
        
        // Don't create train.xml
        let result = train_xml_parse(temp_dir.path());
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("train.xml not found"));
    }

    #[test]
    fn test_train_xml_parse_with_imports() {
        let temp_dir = tempdir().unwrap();
        
        // Create an imported XML file
        let import_content = r#"
            <train>
                <prompts>
                    <prompt id="pr::aum::imported">What is Rust?</prompt>
                </prompts>
                <samples>
                    <sample-ids prompt="pr::aum::imported" />
                </samples>
            </train>
        "#;
        
        let import_path = temp_dir.path().join("imported.xml");
        fs::write(&import_path, import_content).unwrap();
        
        // Create main train.xml with import
        let main_content = format!(r#"
            <train>
                <imports>
                    <import path="{}" />
                </imports>
                <prompts>
                    <prompt id="pr::aum::main">What is Poppins?</prompt>
                </prompts>
                <samples>
                    <sample-ids prompt="pr::aum::main" />
                </samples>
            </train>
        "#, import_path.display());
        
        let train_xml_path = temp_dir.path().join("train.xml");
        fs::write(&train_xml_path, main_content).unwrap();
        
        let result = train_xml_parse(temp_dir.path());
        assert!(result.is_ok());
        let train_xmls = result.unwrap();
        
        // Should have 2 TrainXML structs: main + import
        assert_eq!(train_xmls.len(), 2);
        
        // Main file should have its prompt
        let main = &train_xmls[0];
        let main_prompts = main.prompts.as_ref().unwrap();
        assert_eq!(main_prompts.prompt[0].id, "pr::aum::main");
        
        // Imported file should have its prompt
        let imported = &train_xmls[1];
        let imported_prompts = imported.prompts.as_ref().unwrap();
        assert_eq!(imported_prompts.prompt[0].id, "pr::aum::imported");
    }

    #[test]
    fn test_train_xml_parse_with_nonexistent_import() {
        let temp_dir = tempdir().unwrap();
        
        // Create main train.xml with non-existent import
        let main_content = r#"
            <train>
                <imports>
                    <import path="./nonexistent.xml" />
                </imports>
            </train>
        "#;
        
        let train_xml_path = temp_dir.path().join("train.xml");
        fs::write(&train_xml_path, main_content).unwrap();
        
        let result = train_xml_parse(temp_dir.path());
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }

    #[test]
    fn test_train_xml_parse_with_nested_imports() {
        let temp_dir = tempdir().unwrap();
        
        // Create level 2 import (deepest)
        let level2_content = r#"
            <train>
                <prompts>
                    <prompt id="pr::aum::level_two">Level 2 Prompt</prompt>
                </prompts>
            </train>
        "#;
        let level2_path = temp_dir.path().join("level2.xml");
        fs::write(&level2_path, level2_content).unwrap();
        
        // Create level 1 import (imports level2)
        let level1_content = format!(r#"
            <train>
                <imports>
                    <import path="{}" />
                </imports>
                <prompts>
                    <prompt id="pr::aum::level_one">Level 1 Prompt</prompt>
                </prompts>
            </train>
        "#, level2_path.display());
        let level1_path = temp_dir.path().join("level1.xml");
        fs::write(&level1_path, level1_content).unwrap();
        
        // Create main train.xml (imports level1)
        let main_content = format!(r#"
            <train>
                <imports>
                    <import path="{}" />
                </imports>
                <prompts>
                    <prompt id="pr::aum::main">Main Prompt</prompt>
                </prompts>
            </train>
        "#, level1_path.display());
        
        let train_xml_path = temp_dir.path().join("train.xml");
        fs::write(&train_xml_path, main_content).unwrap();
        
        let result = train_xml_parse(temp_dir.path());
        assert!(result.is_ok());
        let train_xmls = result.unwrap();
        
        // Should have 3 TrainXML structs: main, level1, level2
        assert_eq!(train_xmls.len(), 3);
        
        // Verify order
        assert_eq!(train_xmls[0].prompts.as_ref().unwrap().prompt[0].id, "pr::aum::main");
        assert_eq!(train_xmls[1].prompts.as_ref().unwrap().prompt[0].id, "pr::aum::level_one");
        assert_eq!(train_xmls[2].prompts.as_ref().unwrap().prompt[0].id, "pr::aum::level_two");
    }
}
