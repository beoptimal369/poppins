// src/bpe/bpe_tokenizer_json.rs

use crate::bpe::BPETokenizer;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::{fs::File, io::Write, path::Path};


/// Get the package version from Cargo.toml at compile time
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// JSON representation of the BPE tokenizer for serialization
///
/// This struct handles saving and loading the tokenizer state to/from JSON files.
/// It includes:
/// - Full vocabulary
/// - Merge operations
/// - Special tokens metadata (IDs, tokens, mapping, count)
/// - Requested tokens metadata (IDs, tokens, mapping, count)
/// - Configuration metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct BPETokenizerJSON {
    /// Model version (semver)
    pub version: String,
    
    /// Model type identifier (always "bpe")
    pub model_type: String,
    
    /// Full vocabulary list
    pub vocab: Vec<String>,
    
    /// Merge operations (as merged strings)
    pub merges: Vec<String>,
    
    /// Special tokens metadata
    pub special_tokens: TokenMetadata,
    
    /// Requested tokens metadata
    pub requested_tokens: TokenMetadata,
    
    /// Configuration metadata
    pub config: TokenizerConfig,
}

/// Metadata for a set of tokens (special or requested)
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenMetadata {
    /// Token IDs (position in vocabulary)
    pub ids: Vec<u32>,
    
    /// Token strings
    pub tokens: Vec<String>,
    
    /// Mapping from token string to ID
    pub token_to_id: HashMap<String, u32>,
    
    /// Number of tokens in this set
    pub count: u32,
}

/// Configuration metadata for the tokenizer
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenizerConfig {
    /// Total vocabulary size
    pub vocab_size: usize,
    
    /// Number of merge operations
    pub merge_count: usize,
    
    /// Normalizer type (none for BPE)
    pub normalizer: String,
    
    /// Pre-tokenizer type
    pub pre_tokenizer: String,
    
    /// Whether to add prefix space
    pub add_prefix_space: bool,
    
    /// Model version (same as top-level)
    pub model_version: String,
    
    /// Poppins framework version
    pub poppins_version: String,
    
    /// Creation timestamp (Unix seconds)
    pub created_at: u64,
}

impl BPETokenizerJSON {
    /// Save the tokenizer to a JSON file
    ///
    /// # Arguments
    /// * `tokenizer` - Reference to the trained tokenizer
    /// * `output_dir` - Path where tokenizer.json will be written
    /// * `model_version` - Version of the model being created (semver)
    ///
    /// # Returns
    /// * `std::io::Result<()>` - Ok on success, Err on file write failure
    pub fn save(
        tokenizer: &BPETokenizer,
        output_dir: &Path,
        model_version: &str,
    ) -> std::io::Result<()> {
        // Collect special tokens (first special_token_count entries in vocab)
        let special_tokens: Vec<String> = tokenizer.vocab
            .iter()
            .take(tokenizer.special_token_count as usize)
            .cloned()
            .collect();
        
        // Build special token ID mapping
        let special_token_ids: HashMap<String, u32> = special_tokens
            .iter()
            .enumerate()
            .map(|(id, token)| (token.clone(), id as u32))
            .collect();
        
        // Collect requested tokens (special_token_count to initial_token_count - 1)
        let requested_tokens: Vec<String> = tokenizer.vocab
            .iter()
            .skip(tokenizer.special_token_count as usize)
            .take((tokenizer.initial_token_count - tokenizer.special_token_count) as usize)
            .cloned()
            .collect();
        
        // Build requested token ID mapping (IDs are their actual positions in vocab)
        let requested_token_ids: HashMap<String, u32> = requested_tokens
            .iter()
            .map(|token| {
                let id = tokenizer.token_to_id.get(token).unwrap();
                (token.clone(), *id)
            })
            .collect();
        
        // Format merges as strings - join with the actual concatenation result
        let merges_strings: Vec<String> = tokenizer.merges
            .iter()
            .map(|(a, b)| format!("{}{}", a, b))
            .collect();
        
        let tokenizer_json = BPETokenizerJSON {
            version: model_version.to_string(),
            model_type: "bpe".to_string(),
            vocab: tokenizer.vocab.clone(),
            merges: merges_strings,
            special_tokens: TokenMetadata {
                ids: (0..special_tokens.len()).map(|i| i as u32).collect(),
                tokens: special_tokens,
                token_to_id: special_token_ids,
                count: tokenizer.special_token_count,
            },
            requested_tokens: TokenMetadata {
                ids: requested_tokens.iter()
                    .map(|token| *tokenizer.token_to_id.get(token).unwrap())
                    .collect(),
                tokens: requested_tokens,
                token_to_id: requested_token_ids,
                count: tokenizer.initial_token_count - tokenizer.special_token_count,
            },
            config: TokenizerConfig {
                vocab_size: tokenizer.vocab.len(),
                merge_count: tokenizer.merges.len(),
                normalizer: "none".to_string(),
                pre_tokenizer: "whitespace".to_string(),
                add_prefix_space: false,
                model_version: model_version.to_string(),
                poppins_version: PACKAGE_VERSION.to_string(),
                created_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0),
            },
        };

        let path_buf = output_dir.join("tokenizer.json");
        
        // Write to file
        let json_string = serde_json::to_string_pretty(&tokenizer_json)?;
        let mut file = File::create(&path_buf)?;
        file.write_all(json_string.as_bytes())?;

        println!("✅ Wrote {:?}", &path_buf);

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use tempfile::tempdir;
    use std::{collections::HashMap};
    use super::{BPETokenizer, BPETokenizerJSON};

    const TEST_MODEL_VERSION: &str = "1.0.0";

    fn create_test_tokenizer() -> BPETokenizer {
        let mut token_to_id = HashMap::new();
        
        // Create vocab with special tokens, requested tokens, and regular tokens
        let vocab = vec![
            "<unknown>".to_string(),
            "<sample>".to_string(),
            "</sample>".to_string(),
            "console.log".to_string(),  // requested token
            "HelloWorld".to_string(),   // requested token
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
            initial_token_count: 5, // + console.log, HelloWorld
        }
    }

    #[test]
    fn test_save_tokenizer_json_success() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();
        
        let tokenizer = create_test_tokenizer();
        
        let result = BPETokenizerJSON::save(&tokenizer, output_dir, TEST_MODEL_VERSION);
        assert!(result.is_ok());
        
        // Verify file was created in the output directory
        let file_path = output_dir.join("tokenizer.json");
        assert!(file_path.exists());
        
        // Read and parse the file
        let content = std::fs::read_to_string(&file_path).unwrap();
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
        assert!(json.get("requested_tokens").is_some());
        assert!(json.get("config").is_some());
        
        // Verify special_tokens structure
        let special_tokens = json.get("special_tokens").unwrap();
        assert!(special_tokens.get("ids").is_some());
        assert!(special_tokens.get("tokens").is_some());
        assert!(special_tokens.get("token_to_id").is_some());
        assert!(special_tokens.get("count").is_some());
        
        // Verify requested_tokens structure
        let requested_tokens = json.get("requested_tokens").unwrap();
        assert!(requested_tokens.get("ids").is_some());
        assert!(requested_tokens.get("tokens").is_some());
        assert!(requested_tokens.get("token_to_id").is_some());
        assert!(requested_tokens.get("count").is_some());
        
        // Verify content
        let vocab = json.get("vocab").unwrap().as_array().unwrap();
        assert_eq!(vocab.len(), 10);
        assert_eq!(vocab[0], "<unknown>");
        assert_eq!(vocab[1], "<sample>");
        assert_eq!(vocab[2], "</sample>");
        assert_eq!(vocab[3], "console.log");
        assert_eq!(vocab[4], "HelloWorld");
        
        // Verify special tokens
        let special_tokens_obj = json.get("special_tokens").unwrap();
        let special_tokens_list = special_tokens_obj.get("tokens").unwrap().as_array().unwrap();
        assert_eq!(special_tokens_list.len(), 3);
        assert_eq!(special_tokens_list[0], "<unknown>");
        assert_eq!(special_tokens_list[1], "<sample>");
        assert_eq!(special_tokens_list[2], "</sample>");
        assert_eq!(special_tokens_obj.get("count").unwrap().as_u64().unwrap(), 3);
        
        // Verify requested tokens
        let requested_tokens_obj = json.get("requested_tokens").unwrap();
        let requested_tokens_list = requested_tokens_obj.get("tokens").unwrap().as_array().unwrap();
        assert_eq!(requested_tokens_list.len(), 2);
        assert_eq!(requested_tokens_list[0], "console.log");
        assert_eq!(requested_tokens_list[1], "HelloWorld");
        assert_eq!(requested_tokens_obj.get("count").unwrap().as_u64().unwrap(), 2);
        
        let config = json.get("config").unwrap();
        assert_eq!(config.get("vocab_size").unwrap().as_u64().unwrap(), 10);
        assert_eq!(config.get("merge_count").unwrap().as_u64().unwrap(), 3);
    }
}
