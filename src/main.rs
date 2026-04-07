// src/main.rs

mod cli;

use crate::cli::{Cli, CliCommand};
use poppins::{train, infer, bootstrap};

fn main() {
    let cli = Cli::parse_args();

    match cli.command {
        CliCommand::Bootstrap { model_name } => {
            bootstrap(Cli::get_model_path(&model_name).as_path()).expect("❌ Failed to bootstrap:");
        },
        CliCommand::Train { model_name, device } => {
            let device = &Cli::get_device(device);
            let output_dir = Cli::get_model_path(&model_name);
            train(output_dir.as_path(), model_name, device).expect("❌ Failed to train:");
        },
        CliCommand::Infer { model_name, temperature, prompt, device } => {
            let device = &Cli::get_device(device);
            let output_dir = Cli::get_model_path(&model_name);
            infer(output_dir.as_path(), prompt.join(" "), temperature, device);
        },
    }
}



#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile::tempdir;
    use poppins::{train, infer, bootstrap, Device};

    /// Helper to create a minimal valid train.xml for testing
    fn create_test_train_xml(dir: &std::path::Path) {
        let train_xml_content = r#"<?xml version="1.0" encoding="UTF-8"?>
<train xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:noNamespaceSchemaLocation="./train.xsd">
  <prompts>
    <prompt id="pr::aum::test">What is a test?</prompt>
  </prompts>
  <responses>
    <response id="pr::aum::response">A test is a validation check.</response>
  </responses>
  <samples>
    <sample-ids prompt="pr::aum::test" response="pr::aum::response" />
  </samples>
</train>"#;
        
        fs::write(dir.join("train.xml"), train_xml_content).unwrap();
        fs::write(dir.join("train.xsd"), r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema"/>"#).unwrap();
        fs::write(dir.join("math.xml"), r#"<train/>"#).unwrap();
        fs::write(dir.join("english.xml"), r#"<train/>"#).unwrap();
    }

    #[test]
    fn test_bootstrap_happy_path() {
        let temp_dir = tempdir().unwrap();
        let model_path = temp_dir.path().join("test_model");
        
        // Bootstrap should succeed
        let result = bootstrap(&model_path);
        assert!(result.is_ok(), "Bootstrap failed: {:?}", result.err());
        
        // Verify files were created
        assert!(model_path.join("train.xml").exists(), "train.xml not created");
        assert!(model_path.join("train.xsd").exists(), "train.xsd not created");
        assert!(model_path.join("math.xml").exists(), "math.xml not created");
        assert!(model_path.join("english.xml").exists(), "english.xml not created");
    }

    #[test]
    fn test_bootstrap_error() {
        // Create a read-only directory
        let temp_dir = tempdir().unwrap();
        let read_only_dir = temp_dir.path().join("readonly");
        std::fs::create_dir(&read_only_dir).unwrap();
        
        // Make directory read-only
        let mut perms = std::fs::metadata(&read_only_dir).unwrap().permissions();
        perms.set_readonly(true);
        std::fs::set_permissions(&read_only_dir, perms).unwrap();
        
        // Try to bootstrap into read-only directory (should fail)
        let result = bootstrap(&read_only_dir);
        assert!(result.is_err(), "Bootstrap should fail with read-only directory");
        
        // Clean up (remove read-only flag first)
        let mut perms = std::fs::metadata(&read_only_dir).unwrap().permissions();
        perms.set_readonly(false);
        std::fs::set_permissions(&read_only_dir, perms).unwrap();
    }

    #[test]
    fn test_train_happy_path() {
        let temp_dir = tempdir().unwrap();
        let model_path = temp_dir.path().join("test_model");
        
        // First bootstrap to create necessary files
        bootstrap(&model_path).unwrap();
        
        // Create a valid train.xml with proper structure
        create_test_train_xml(&model_path);
        
        let device = Device::Cpu;
        let model_name = "test_model".to_string();
        
        // Train should succeed
        let result = train(&model_path, model_name, &device);
        assert!(result.is_ok(), "Training failed: {:?}", result.err());
        
        // Verify output files were created
        assert!(model_path.join("train_corpus.txt").exists(), "train_corpus.txt not created");
        assert!(model_path.join("val_corpus.txt").exists(), "val_corpus.txt not created");
        assert!(model_path.join("tokenizer.json").exists(), "tokenizer.json not created");
        assert!(model_path.join("config_poppins.json").exists(), "config_poppins.json not created");
        assert!(model_path.join("config.json").exists(), "config.json not created");
    }

    #[test]
    fn test_train_error() {
        let temp_dir = tempdir().unwrap();
        let model_path = temp_dir.path().join("nonexistent_model");
        let model_name = "nonexistent_model".to_string();
        let device = Device::Cpu;
        
        // Train should fail because model doesn't exist
        let result = train(&model_path, model_name, &device);
        assert!(result.is_err(), "Training should fail with nonexistent model");
    }

    #[test]
    fn test_infer_happy_path() {
        let temp_dir = tempdir().unwrap();
        let model_path = temp_dir.path().join("test_model");
        
        // First bootstrap and train to create a model
        bootstrap(&model_path).unwrap();
        create_test_train_xml(&model_path);
        
        let device = Device::Cpu;
        let model_name = "test_model".to_string();
        
        // Train first
        train(&model_path, model_name, &device).unwrap();
        
        // Then infer should not panic
        let prompt = "What is a test?".to_string();
        let temperature = Some(0.7);
        
        // Infer should complete without error
        infer(&model_path, prompt, temperature, &device);
    }
}
