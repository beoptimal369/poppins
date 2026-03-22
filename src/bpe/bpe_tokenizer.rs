// src/bpe/bpe_tokenizer.rs

use std::collections::HashMap;


/// BPE Tokenizer
///
/// Implements Byte Pair Encoding tokenization with support for:
/// - Special tokens (protected from merging)
/// - Requested tokens (pre-inserted into vocabulary)
/// - Configurable merge frequency threshold
/// - Vocabulary size limits
#[derive(Debug, Clone)]
pub struct BPETokenizer {
    /// Vocabulary mapping token ID to token string
    pub vocab: Vec<String>,
    
    /// Reverse mapping from token string to token ID
    pub token_to_id: HashMap<String, u32>,
    
    /// List of merge operations performed during training
    /// Each entry is (left_token, right_token) that were merged
    pub merges: Vec<(String, String)>,
    
    /// Number of special tokens at the start of vocab
    /// These tokens are never merged with other tokens
    pub special_token_count: u32,
}



#[cfg(test)]
mod tests {
    use super::BPETokenizer;

    #[test]
    fn test_bpe_tokenizer_fields() {
        let tokenizer = BPETokenizer {
            vocab: vec!["<unknown>".to_string(), "test".to_string()],
            token_to_id: {
                let mut map = std::collections::HashMap::new();
                map.insert("<unknown>".to_string(), 0);
                map.insert("test".to_string(), 1);
                map
            },
            merges: vec![("a".to_string(), "b".to_string())],
            special_token_count: 1,
        };
        
        assert_eq!(tokenizer.vocab.len(), 2);
        assert_eq!(tokenizer.token_to_id.len(), 2);
        assert_eq!(tokenizer.merges.len(), 1);
        assert_eq!(tokenizer.special_token_count, 1);
    }
}
