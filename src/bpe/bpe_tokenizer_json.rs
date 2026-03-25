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
        
        // Write to file
        let json_string = serde_json::to_string_pretty(&tokenizer_json)?;
        let mut file = File::create(output_dir.join("tokenizer.json"))?;
        file.write_all(json_string.as_bytes())?;
        
        Ok(())
    }
    
    /// Load the tokenizer from a JSON file
    ///
    /// # Arguments
    /// * `input_dir` - Path to the tokenizer.json file
    ///
    /// # Returns
    /// * `Result<Self, Box<dyn std::error::Error>>` - The loaded tokenizer JSON
    pub fn load(input_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(input_dir.join("tokenizer.json"))?;
        let tokenizer_json: BPETokenizerJSON = serde_json::from_str(&content)?;
        Ok(tokenizer_json)
    }
    
    /// Convert the JSON representation back to a BPETokenizer
    ///
    /// This reconstructs the full tokenizer state including:
    /// - Vocabulary
    /// - Token-to-ID mapping
    /// - Merge operations (reconstructed from merged strings)
    /// - Special token count
    /// - Initial token count
    ///
    /// # Returns
    /// * `BPETokenizer` - The reconstructed tokenizer
    pub fn to_tokenizer(&self) -> BPETokenizer {
        let mut token_to_id = HashMap::new();
        
        // Build token-to-id mapping
        for (id, token) in self.vocab.iter().enumerate() {
            token_to_id.insert(token.clone(), id as u32);
        }
        
        // Reconstruct merges from merged strings
        // We need to find the original pair that produced each merge
        // This is a simplified reconstruction - in practice, we'd store the pairs directly
        let mut merges = Vec::new();
        for merged in &self.merges {
            // Find the split that produced this merge
            // For now, we'll use a simple approach: find the longest prefix that exists in vocab
            // This is a placeholder - in production, you'd store the actual pairs
            if let Some((a, b)) = self.find_merge_pair(merged) {
                merges.push((a, b));
            }
        }
        
        BPETokenizer {
            vocab: self.vocab.clone(),
            token_to_id,
            merges,
            special_token_count: self.special_tokens.count,
            initial_token_count: self.special_tokens.count + self.requested_tokens.count,
        }
    }
    
    /// Helper to find the original pair that produced a merged token
    fn find_merge_pair(&self, merged: &str) -> Option<(String, String)> {
        // Try all possible splits to find one where both parts are in vocabulary
        for split_pos in 1..merged.len() {
            let a = &merged[..split_pos];
            let b = &merged[split_pos..];
            
            if self.vocab.contains(&a.to_string()) && self.vocab.contains(&b.to_string()) {
                return Some((a.to_string(), b.to_string()));
            }
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use tempfile::tempdir;
    use std::{collections::HashMap, path::Path};
    use super::{PACKAGE_VERSION, BPETokenizer, BPETokenizerJSON};

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

    #[test]
    fn test_load_tokenizer_json() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();
        
        let original_tokenizer = create_test_tokenizer();
        BPETokenizerJSON::save(&original_tokenizer, output_dir, TEST_MODEL_VERSION).unwrap();
        
        // Load from the same directory
        let loaded = BPETokenizerJSON::load(output_dir).unwrap();
        
        // Verify loaded data matches original
        assert_eq!(loaded.version, TEST_MODEL_VERSION);
        assert_eq!(loaded.model_type, "bpe");
        assert_eq!(loaded.vocab.len(), 10);
        assert_eq!(loaded.special_tokens.count, 3);
        assert_eq!(loaded.requested_tokens.count, 2);
        assert_eq!(loaded.config.vocab_size, 10);
        assert_eq!(loaded.config.merge_count, 3);
        
        // Verify special tokens
        assert_eq!(loaded.special_tokens.tokens[0], "<unknown>");
        assert_eq!(loaded.special_tokens.tokens[1], "<sample>");
        assert_eq!(loaded.special_tokens.tokens[2], "</sample>");
        
        // Verify requested tokens
        assert_eq!(loaded.requested_tokens.tokens[0], "console.log");
        assert_eq!(loaded.requested_tokens.tokens[1], "HelloWorld");
        
        // Verify token_to_id mappings
        assert_eq!(*loaded.special_tokens.token_to_id.get("<unknown>").unwrap(), 0);
        assert_eq!(*loaded.special_tokens.token_to_id.get("<sample>").unwrap(), 1);
        assert_eq!(*loaded.requested_tokens.token_to_id.get("console.log").unwrap(), 3);
        assert_eq!(*loaded.requested_tokens.token_to_id.get("HelloWorld").unwrap(), 4);
    }

    #[test]
    fn test_to_tokenizer() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();
        
        let original_tokenizer = create_test_tokenizer();
        BPETokenizerJSON::save(&original_tokenizer, output_dir, TEST_MODEL_VERSION).unwrap();
        
        let loaded_json = BPETokenizerJSON::load(output_dir).unwrap();
        let reconstructed_tokenizer = loaded_json.to_tokenizer();
        
        // Verify reconstructed tokenizer matches original
        assert_eq!(reconstructed_tokenizer.vocab, original_tokenizer.vocab);
        assert_eq!(reconstructed_tokenizer.special_token_count, original_tokenizer.special_token_count);
        assert_eq!(reconstructed_tokenizer.initial_token_count, original_tokenizer.initial_token_count);
        assert_eq!(reconstructed_tokenizer.merges.len(), original_tokenizer.merges.len());
        
        // Verify token_to_id mappings
        for (token, id) in &original_tokenizer.token_to_id {
            assert_eq!(reconstructed_tokenizer.token_to_id.get(token), Some(id));
        }
    }

    #[test]
    fn test_save_tokenizer_json_empty_tokenizer() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();
        
        let tokenizer = BPETokenizer {
            vocab: vec![],
            token_to_id: HashMap::new(),
            merges: vec![],
            special_token_count: 0,
            initial_token_count: 0,
        };
        
        let result = BPETokenizerJSON::save(&tokenizer, output_dir, TEST_MODEL_VERSION);
        assert!(result.is_ok());
        
        // Load from the directory, not from the file path
        let loaded = BPETokenizerJSON::load(output_dir).unwrap();
        assert!(loaded.vocab.is_empty());
        assert_eq!(loaded.special_tokens.count, 0);
        assert_eq!(loaded.requested_tokens.count, 0);
        assert_eq!(loaded.config.vocab_size, 0);
        assert_eq!(loaded.config.merge_count, 0);
    }

    #[test]
    fn test_save_tokenizer_json_no_requested_tokens() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();
        
        let mut tokenizer = create_test_tokenizer();
        tokenizer.initial_token_count = tokenizer.special_token_count;
        
        let result = BPETokenizerJSON::save(&tokenizer, output_dir, TEST_MODEL_VERSION);
        assert!(result.is_ok());
        
        // Load from the directory, not from the file path
        let loaded = BPETokenizerJSON::load(output_dir).unwrap();
        assert_eq!(loaded.requested_tokens.count, 0);
        assert!(loaded.requested_tokens.tokens.is_empty());
        assert!(loaded.requested_tokens.ids.is_empty());
        assert!(loaded.requested_tokens.token_to_id.is_empty());
    }

    #[test]
    fn test_save_tokenizer_json_invalid_directory() {
        let tokenizer = create_test_tokenizer();
        let invalid_path = Path::new("/nonexistent/directory/that/should/not/exist");
        
        let result = BPETokenizerJSON::save(&tokenizer, invalid_path, TEST_MODEL_VERSION);
        assert!(result.is_err());
    }

    #[test]
    fn test_load_nonexistent_directory() {
        let path = Path::new("/nonexistent/directory");
        let result = BPETokenizerJSON::load(path);
        assert!(result.is_err());
    }

    #[test]
    fn test_version_consistency() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();
        
        let tokenizer = create_test_tokenizer();
        BPETokenizerJSON::save(&tokenizer, output_dir, TEST_MODEL_VERSION).unwrap();
        
        let loaded = BPETokenizerJSON::load(output_dir).unwrap();
        
        assert_eq!(loaded.version, TEST_MODEL_VERSION);
        assert_eq!(loaded.config.model_version, TEST_MODEL_VERSION);
        assert_eq!(loaded.config.poppins_version, PACKAGE_VERSION);
    }
}
