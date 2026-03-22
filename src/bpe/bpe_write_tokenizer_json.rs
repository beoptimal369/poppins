// src/bpe/bpe_write_tokenizer_json.rs

use crate::bpe::BPETokenizer;
use std::{fs::File, io::Write, path::Path};


/// Get the package version from Cargo.toml at compile time
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Write tokenizer vocabulary and metadata to tokenizer.json
///
/// This function creates a JSON file containing:
/// - Full vocabulary list
/// - Merge operations in order
/// - Special tokens metadata (ids, tokens, token_to_id, count)
/// - Configuration metadata
/// - Version information
///
/// # Arguments
/// * `tokenizer` - Reference to the trained tokenizer
/// * `output_dir` - Directory path where tokenizer.json will be written
/// * `model_version` - Version of the model being created (semver)
///
/// # Returns
/// * `std::io::Result<()>` - Ok on success, Err on file write failure
pub fn bpe_write_tokenizer_json(
    tokenizer: &BPETokenizer,
    output_dir: &Path,
    model_version: &str,
) -> std::io::Result<()> {
    // Build output file path
    let file_path = output_dir.join("tokenizer.json");
    
    // Collect special tokens (first special_token_count entries in vocab)
    let special_tokens: Vec<String> = tokenizer.vocab
        .iter()
        .take(tokenizer.special_token_count as usize)
        .cloned()
        .collect();
    
    // Build special token ID mapping
    let special_token_ids: std::collections::HashMap<String, u32> = special_tokens
        .iter()
        .enumerate()
        .map(|(id, token)| (token.clone(), id as u32))
        .collect();
    
    // Format merges as strings - join with the actual concatenation result
    let merges_strings: Vec<String> = tokenizer.merges
        .iter()
        .map(|(a, b)| format!("{}{}", a, b))
        .collect();
    
    // Build JSON structure with clean, non-redundant format
    let tokenizer_json = serde_json::json!({
        "version": model_version,
        "model_type": "bpe",
        "vocab": tokenizer.vocab,
        "merges": merges_strings,
        "special_tokens": {
            "ids": special_tokens.iter().enumerate().map(|(id, _)| id as u32).collect::<Vec<u32>>(),
            "tokens": special_tokens,
            "token_to_id": special_token_ids,
            "count": tokenizer.special_token_count
        },
        "config": {
            "vocab_size": tokenizer.vocab.len(),
            "merge_count": tokenizer.merges.len(),
            "normalizer": "none",
            "pre_tokenizer": "whitespace",
            "add_prefix_space": false,
            "model_version": model_version,
            "poppins_version": PACKAGE_VERSION,
            "created_at": {
                "timestamp": std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0)
            }
        }
    });
    
    // Write to file
    let json_string = serde_json::to_string_pretty(&tokenizer_json)?;
    let mut file = File::create(file_path)?;
    file.write_all(json_string.as_bytes())?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::{fs, path::Path};
    use std::collections::HashMap;

    const TEST_MODEL_VERSION: &str = "1.0.0";

    fn create_test_tokenizer() -> BPETokenizer {
        let mut token_to_id = HashMap::new();
        
        // Create vocab with special tokens and regular tokens
        let vocab = vec![
            "<unknown>".to_string(),
            "<sample>".to_string(),
            "</sample>".to_string(),
            "hello".to_string(),
            "world".to_string(),
            " ".to_string(),
            "a".to_string(),
            "b".to_string(),
        ];
        
        for (id, token) in vocab.iter().enumerate() {
            token_to_id.insert(token.clone(), id as u32);
        }
        
        let merges = vec![
            (" ".to_string(), "a".to_string()),
            ("a".to_string(), "b".to_string()),
            ("hello".to_string(), " ".to_string()),
        ];
        
        BPETokenizer {
            vocab,
            token_to_id,
            merges,
            special_token_count: 3, // <unknown>, <sample>, </sample>
        }
    }

    #[test]
    fn test_write_tokenizer_json_success() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();
        
        let tokenizer = create_test_tokenizer();
        
        let result = bpe_write_tokenizer_json(&tokenizer, output_dir, TEST_MODEL_VERSION);
        assert!(result.is_ok());
        
        // Verify file was created
        let file_path = output_dir.join("tokenizer.json");
        assert!(file_path.exists());
        
        // Read and parse the file
        let content = fs::read_to_string(file_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        
        // Verify structure
        assert_eq!(
            json.get("version"), 
            Some(&serde_json::Value::String(TEST_MODEL_VERSION.to_string()))
        );
        assert_eq!(
            json.get("model_type"), 
            Some(&serde_json::Value::String("bpe".to_string()))
        );
        assert!(json.get("vocab").is_some());
        assert!(json.get("merges").is_some());
        assert!(json.get("special_tokens").is_some());
        assert!(json.get("config").is_some());
        
        // Verify special_tokens structure (should NOT have count at top level)
        let special_tokens = json.get("special_tokens").unwrap();
        assert!(special_tokens.get("ids").is_some());
        assert!(special_tokens.get("tokens").is_some());
        assert!(special_tokens.get("token_to_id").is_some());
        assert!(special_tokens.get("count").is_some());
        
        // Verify content
        let vocab = json.get("vocab").unwrap().as_array().unwrap();
        assert_eq!(vocab.len(), 8);
        assert_eq!(vocab[0], "<unknown>");
        assert_eq!(vocab[1], "<sample>");
        assert_eq!(vocab[2], "</sample>");
        
        let merges = json.get("merges").unwrap().as_array().unwrap();
        assert_eq!(merges.len(), 3);
        assert_eq!(merges[0], " a");
        assert_eq!(merges[1], "ab");
        assert_eq!(merges[2], "hello ");
        
        let special_tokens_obj = json.get("special_tokens").unwrap();
        let special_tokens_list = special_tokens_obj.get("tokens").unwrap().as_array().unwrap();
        assert_eq!(special_tokens_list.len(), 3);
        assert_eq!(special_tokens_list[0], "<unknown>");
        assert_eq!(special_tokens_list[1], "<sample>");
        assert_eq!(special_tokens_list[2], "</sample>");
        
        let special_token_count = special_tokens_obj.get("count").unwrap().as_u64().unwrap();
        assert_eq!(special_token_count, 3);
        
        let config = json.get("config").unwrap();
        assert_eq!(config.get("vocab_size").unwrap().as_u64().unwrap(), 8);
        assert_eq!(config.get("merge_count").unwrap().as_u64().unwrap(), 3);
        assert_eq!(config.get("normalizer").unwrap().as_str().unwrap(), "none");
        assert_eq!(config.get("pre_tokenizer").unwrap().as_str().unwrap(), "whitespace");
        assert_eq!(config.get("add_prefix_space").unwrap().as_bool().unwrap(), false);
        assert_eq!(config.get("model_version").unwrap().as_str().unwrap(), TEST_MODEL_VERSION);
        assert_eq!(config.get("poppins_version").unwrap().as_str().unwrap(), PACKAGE_VERSION);
        assert!(config.get("created_at").is_some());
        
        // Verify config does NOT have special_token_count
        assert!(config.get("special_token_count").is_none());
    }

    #[test]
    fn test_write_tokenizer_json_empty_tokenizer() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();
        
        let tokenizer = BPETokenizer {
            vocab: vec![],
            token_to_id: HashMap::new(),
            merges: vec![],
            special_token_count: 0,
        };
        
        let result = bpe_write_tokenizer_json(&tokenizer, output_dir, TEST_MODEL_VERSION);
        assert!(result.is_ok());
        
        let file_path = output_dir.join("tokenizer.json");
        let content = fs::read_to_string(file_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        
        let vocab = json.get("vocab").unwrap().as_array().unwrap();
        assert!(vocab.is_empty());
        
        let merges = json.get("merges").unwrap().as_array().unwrap();
        assert!(merges.is_empty());
        
        let special_tokens_obj = json.get("special_tokens").unwrap();
        let special_tokens_list = special_tokens_obj.get("tokens").unwrap().as_array().unwrap();
        assert!(special_tokens_list.is_empty());
        
        assert_eq!(special_tokens_obj.get("count").unwrap().as_u64().unwrap(), 0);
        
        let config = json.get("config").unwrap();
        assert_eq!(config.get("vocab_size").unwrap().as_u64().unwrap(), 0);
        assert_eq!(config.get("merge_count").unwrap().as_u64().unwrap(), 0);
        assert!(config.get("special_token_count").is_none());
    }

    #[test]
    fn test_write_tokenizer_json_no_special_tokens() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();
        
        let mut token_to_id = HashMap::new();
        let vocab = vec!["hello".to_string(), "world".to_string()];
        for (id, token) in vocab.iter().enumerate() {
            token_to_id.insert(token.clone(), id as u32);
        }
        
        let tokenizer = BPETokenizer {
            vocab,
            token_to_id,
            merges: vec![],
            special_token_count: 0,
        };
        
        let result = bpe_write_tokenizer_json(&tokenizer, output_dir, TEST_MODEL_VERSION);
        assert!(result.is_ok());
        
        let file_path = output_dir.join("tokenizer.json");
        let content = fs::read_to_string(file_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        
        let special_tokens_obj = json.get("special_tokens").unwrap();
        let special_tokens_list = special_tokens_obj.get("tokens").unwrap().as_array().unwrap();
        assert!(special_tokens_list.is_empty());
        
        let token_to_id_obj = special_tokens_obj.get("token_to_id").unwrap().as_object().unwrap();
        assert!(token_to_id_obj.is_empty());
        
        assert_eq!(special_tokens_obj.get("count").unwrap().as_u64().unwrap(), 0);
    }

    #[test]
    fn test_write_tokenizer_json_with_merges() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();
        
        let tokenizer = create_test_tokenizer();
        
        bpe_write_tokenizer_json(&tokenizer, output_dir, TEST_MODEL_VERSION).unwrap();
        
        let file_path = output_dir.join("tokenizer.json");
        let content = fs::read_to_string(file_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        
        let merges = json.get("merges").unwrap().as_array().unwrap();
        assert_eq!(merges.len(), 3);
        assert_eq!(merges[0], " a");
        assert_eq!(merges[1], "ab");
        assert_eq!(merges[2], "hello ");
    }

    #[test]
    fn test_write_tokenizer_json_custom_version() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();
        
        let tokenizer = create_test_tokenizer();
        let custom_version = "2.3.1-rc.0";
        
        let result = bpe_write_tokenizer_json(&tokenizer, output_dir, custom_version);
        assert!(result.is_ok());
        
        let file_path = output_dir.join("tokenizer.json");
        let content = fs::read_to_string(file_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        
        // Verify custom version is used
        assert_eq!(
            json.get("version").unwrap().as_str().unwrap(),
            custom_version
        );
        
        let config = json.get("config").unwrap();
        assert_eq!(
            config.get("model_version").unwrap().as_str().unwrap(),
            custom_version
        );
        assert_eq!(
            config.get("poppins_version").unwrap().as_str().unwrap(),
            PACKAGE_VERSION
        );
    }

    #[test]
    fn test_write_tokenizer_json_invalid_directory() {
        let tokenizer = create_test_tokenizer();
        let invalid_dir = Path::new("/nonexistent/directory/that/should/not/exist");
        
        let result = bpe_write_tokenizer_json(&tokenizer, invalid_dir, TEST_MODEL_VERSION);
        assert!(result.is_err());
    }

    #[test]
    fn test_write_tokenizer_json_version_matches() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();
        
        let tokenizer = create_test_tokenizer();
        
        bpe_write_tokenizer_json(&tokenizer, output_dir, TEST_MODEL_VERSION).unwrap();
        
        let file_path = output_dir.join("tokenizer.json");
        let content = fs::read_to_string(file_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        
        // Verify top-level version matches config.model_version
        let top_version = json.get("version").unwrap().as_str().unwrap();
        let config_version = json.get("config").unwrap().get("model_version").unwrap().as_str().unwrap();
        assert_eq!(top_version, config_version);
        assert_eq!(top_version, TEST_MODEL_VERSION);
        
        // Verify poppins_version is set correctly
        let poppins_version = json.get("config").unwrap().get("poppins_version").unwrap().as_str().unwrap();
        assert_eq!(poppins_version, PACKAGE_VERSION);
        
        // Verify config does NOT have special_token_count
        assert!(json.get("config").unwrap().get("special_token_count").is_none());
    }
}
