// src/bpe/bpe_infer_tokenize.rs

use crate::bpe::BPETokenizer;
use std::collections::{HashSet, HashMap};


/// Converts a string to a sequence of token IDs using the trained BPE tokenizer
///
/// This function is used **after** BPE training is complete, for:
/// - Converting samples to token IDs when writing binary corpus files
/// - Tokenizing input text during inference
/// - Any scenario where the trained BPE merges should be applied
///
/// # Arguments
/// * `tokenizer` - The trained BPE tokenizer (with merges and full vocabulary)
/// * `text` - The input text to tokenize
///
/// # Returns
/// * `Vec<u32>` - Vector of token IDs (0 = `<unknown>` for out-of-vocabulary tokens)
pub fn bpe_infer_tokenize(tokenizer: &BPETokenizer, text: &str) -> Vec<u32> {
    // Create a set of special token strings for quick lookup
    // Special tokens are the first `special_token_count` entries in the vocabulary
    // These tokens are never passed through the BPE merge process
    let special_tokens: HashSet<&str> = tokenizer.vocab[..tokenizer.special_token_count as usize]
        .iter()
        .map(|s| s.as_str())
        .collect();
    
    // Split text into words and special tokens
    let tokens = split_into_tokens(text, &special_tokens);
    
    let mut result = Vec::new();

    for token in tokens {
        // Check if this is a special token
        if special_tokens.contains(token.as_str()) {
            // Special token: look up directly in vocabulary
            if let Some(&id) = tokenizer.token_to_id.get(&token) {
                result.push(id);
            } else {
                result.push(0); // <unknown>
            }
        } else {
            // Regular text: apply BPE merges
            let merged = apply_merges(token, &tokenizer.merges, &tokenizer.token_to_id);
            result.extend(merged);
        }
    }
    
    result
}


/// Splits text into initial tokens (words and special tokens)
fn split_into_tokens(text: &str, special_tokens: &HashSet<&str>) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let bytes = text.as_bytes();
    let mut i = 0;
    
    while i < bytes.len() {
        if bytes[i] == b'<' {
            // Find matching '>' without re-scanning from start each time
            let start = i;
            let mut j = i + 1;

            while j < bytes.len() && bytes[j] != b'>' {
                j += 1;
            }
            
            if j < bytes.len() {
                let potential_special = &text[start..=j];

                if special_tokens.contains(potential_special) {
                    if !current.is_empty() {
                        tokens.push(std::mem::take(&mut current));
                    }
                    tokens.push(potential_special.to_string());
                    i = j + 1;
                    continue;
                }
            }
        }
        
        current.push(text.chars().nth(i).unwrap()); // O(n) - even worse!
        i += 1;
    }
    
    if !current.is_empty() {
        tokens.push(current);
    }
    
    tokens
}


/// Applies BPE merges to a token, splitting it into vocabulary tokens
fn apply_merges(
    token: String,
    merges: &[(String, String)],
    token_to_id: &HashMap<String, u32>,
) -> Vec<u32> {
    // Build merge lookup with references
    let merge_map: HashMap<(&str, &str), String> = merges
        .iter()
        .map(|(a, b)| ((a.as_str(), b.as_str()), format!("{}{}", a, b)))
        .collect();
    
    let mut word: Vec<String> = token.chars().map(|c| c.to_string()).collect();
    
    loop {
        let mut merged_any = false;
        
        for i in 0..word.len().saturating_sub(1) {
            let key = (word[i].as_str(), word[i + 1].as_str());
            if let Some(merged) = merge_map.get(&key) {
                word[i] = merged.clone();
                word.remove(i + 1);
                merged_any = true;
                break;
            }
        }
        
        if !merged_any {
            break;
        }
    }
    
    // Convert to IDs
    let mut result = Vec::with_capacity(word.len());
    for w in word {
        result.push(*token_to_id.get(&w).unwrap_or(&0));
    }
    result
}



#[cfg(test)]
mod tests {
    use super::{HashSet, HashMap, apply_merges, split_into_tokens};
    use crate::bpe::{
        bpe_train,
        BPETokenizer,
        bpe_infer_tokenize,
        bpe_get_special_tokens,
    };
    use crate::sample::{
        Sample,
        SampleCode,
        SampleAiEnum,
        SampleIndent,
        SampleLanguage,
        SamplePromptEnum,
    };

    fn create_test_tokenizer() -> BPETokenizer {
        let special_tokens = bpe_get_special_tokens();
        
        let sample = Sample {
            prompt_section: vec![
                SamplePromptEnum::Text("Hello world".to_string()),
                SamplePromptEnum::Code(SampleCode {
                    lang: SampleLanguage::Js,
                    inline: false,
                    indent: SampleIndent::Zero,
                    content: "console.log('test')".to_string(),
                }),
            ],
            ai_section: vec![
                SampleAiEnum::Text("Response".to_string()),
            ],
        };
        
        bpe_train(&[sample], &special_tokens, &[], 2).unwrap()
    }

    #[test]
    fn test_bpe_infer_tokenize_line_break() {
        let tokenizer = create_test_tokenizer();
        
        let text = "<line-break />";
        let ids = bpe_infer_tokenize(&tokenizer, text);
        
        // Should be a single token for the line break
        assert_eq!(ids.len(), 1);
        
        // ID should be within vocabulary range
        assert!(ids[0] < tokenizer.vocab.len() as u32);
    }

    #[test]
    fn test_bpe_infer_tokenize_preserves_special_token_boundaries() {
        let tokenizer = create_test_tokenizer();
        
        let text = "<sample>Hello</sample>";
        let ids = bpe_infer_tokenize(&tokenizer, text);
        
        // Find the positions of special tokens
        let mut found_sample_start = false;
        let mut found_sample_end = false;
        
        for &id in &ids {
            let token_str = &tokenizer.vocab[id as usize];
            if token_str == "<sample>" {
                found_sample_start = true;
            }
            if token_str == "</sample>" {
                found_sample_end = true;
            }
        }
        
        assert!(found_sample_start);
        assert!(found_sample_end);
    }

    #[test]
    fn test_split_into_tokens_with_incomplete_special_token() {
        let tokenizer = create_test_tokenizer();
        let special_tokens: HashSet<&str> = tokenizer.vocab[..tokenizer.special_token_count as usize]
            .iter()
            .map(|s| s.as_str())
            .collect();
        
        let text = "<sample>Hello<not-a-token";
        let tokens = split_into_tokens(text, &special_tokens);
        
        // Should treat "<not-a-token" as regular text since no closing '>'
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], "<sample>");
        assert_eq!(tokens[1], "Hello<not-a-token");
    }

    #[test]
    fn test_bpe_infer_tokenize_special_tokens() {
        let tokenizer = create_test_tokenizer();
        
        let text = "<sample><prompt>Hello</prompt></sample>";
        let ids = bpe_infer_tokenize(&tokenizer, text);
        
        assert!(!ids.is_empty());
    }

    #[test]
    fn test_bpe_infer_tokenize_simple_text() {
        let tokenizer = create_test_tokenizer();
        
        let text = "Hello";
        let ids = bpe_infer_tokenize(&tokenizer, text);
        
        assert!(!ids.is_empty());
        
        for &id in &ids {
            assert!(id < tokenizer.vocab.len() as u32);
        }
    }

    #[test]
    fn test_bpe_infer_tokenize_multiple_words() {
        let tokenizer = create_test_tokenizer();
        
        let text = "Hello world";
        let ids = bpe_infer_tokenize(&tokenizer, text);
        
        assert!(!ids.is_empty());
        
        for &id in &ids {
            assert!(id < tokenizer.vocab.len() as u32);
        }
    }

    #[test]
    fn test_bpe_infer_tokenize_empty_string() {
        let tokenizer = create_test_tokenizer();
        
        let text = "";
        let ids = bpe_infer_tokenize(&tokenizer, text);
        
        assert!(ids.is_empty());
    }

    #[test]
    fn test_bpe_infer_tokenize_mixed_special_and_text() {
        let tokenizer = create_test_tokenizer();
        
        let text = "<text>Hello</text>";
        let ids = bpe_infer_tokenize(&tokenizer, text);
        
        assert!(!ids.is_empty());
        
        for &id in &ids {
            assert!(id < tokenizer.vocab.len() as u32);
        }
    }

    #[test]
    fn test_bpe_infer_tokenize_code_tag() {
        let tokenizer = create_test_tokenizer();
        
        let text = "<js>console.log('test');</js>";
        let ids = bpe_infer_tokenize(&tokenizer, text);
        
        assert!(!ids.is_empty());
        
        for &id in &ids {
            assert!(id < tokenizer.vocab.len() as u32);
        }
    }

    #[test]
    fn test_bpe_infer_tokenize_consistent_tokenization() {
        let tokenizer = create_test_tokenizer();
        
        let text1 = "Hello world";
        let text2 = "Hello world";
        
        let ids1 = bpe_infer_tokenize(&tokenizer, text1);
        let ids2 = bpe_infer_tokenize(&tokenizer, text2);
        
        assert_eq!(ids1, ids2);
    }

    #[test]
    fn test_bpe_infer_tokenize_different_texts() {
        let tokenizer = create_test_tokenizer();
        
        let ids1 = bpe_infer_tokenize(&tokenizer, "Hello");
        let ids2 = bpe_infer_tokenize(&tokenizer, "World");
        
        assert_ne!(ids1, ids2);
    }

    #[test]
    fn test_split_into_tokens_with_special_tokens() {
        let tokenizer = create_test_tokenizer();
        let special_tokens: HashSet<&str> = tokenizer.vocab[..tokenizer.special_token_count as usize]
            .iter()
            .map(|s| s.as_str())
            .collect();
        
        let text = "<sample>Hello<text>world</text></sample>";
        let tokens = split_into_tokens(text, &special_tokens);
        
        assert_eq!(tokens.len(), 6);
        assert_eq!(tokens[0], "<sample>");
        assert_eq!(tokens[2], "<text>");
        assert_eq!(tokens[4], "</text>");
        assert_eq!(tokens[5], "</sample>");
    }

    #[test]
    fn test_split_into_tokens_without_special_tokens() {
        let tokenizer = create_test_tokenizer();
        let special_tokens: HashSet<&str> = tokenizer.vocab[..tokenizer.special_token_count as usize]
            .iter()
            .map(|s| s.as_str())
            .collect();
        
        let text = "Hello world";
        let tokens = split_into_tokens(text, &special_tokens);
        
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], "Hello world");
    }

    #[test]
    fn test_apply_merges_basic() {
        let mut token_to_id = HashMap::new();
        token_to_id.insert("H".to_string(), 1);
        token_to_id.insert("e".to_string(), 2);
        token_to_id.insert("l".to_string(), 3);
        token_to_id.insert("o".to_string(), 4);
        token_to_id.insert("He".to_string(), 5);
        token_to_id.insert("llo".to_string(), 6);
        token_to_id.insert("Hello".to_string(), 7);
        
        let merges = vec![
            ("H".to_string(), "e".to_string()),
            ("l".to_string(), "l".to_string()),
            ("He".to_string(), "ll".to_string()),
            ("Hell".to_string(), "o".to_string()),
        ];
        
        let token = "Hello".to_string();
        let ids = apply_merges(token, &merges, &token_to_id);
        
        assert_eq!(ids, vec![7]);
    }

    #[test]
    fn test_apply_merges_no_merges_available() {
        let token_to_id = std::collections::HashMap::new();
        let merges: Vec<(String, String)> = vec![];
        
        let token = "abc".to_string();
        let ids = apply_merges(token, &merges, &token_to_id);
        
        assert_eq!(ids.len(), 3);
        assert_eq!(ids, vec![0, 0, 0]);
    }

    #[test]
    fn test_apply_merges_partial_merge() {
        let mut token_to_id = std::collections::HashMap::new();
        token_to_id.insert("H".to_string(), 1);
        token_to_id.insert("e".to_string(), 2);
        token_to_id.insert("l".to_string(), 3);
        token_to_id.insert("o".to_string(), 4);
        token_to_id.insert("He".to_string(), 5);
        
        let merges = vec![
            ("H".to_string(), "e".to_string()),
        ];
        
        let token = "Hello".to_string();
        let ids = apply_merges(token, &merges, &token_to_id);
        
        assert_eq!(ids, vec![5, 3, 3, 4]);
    }
}
