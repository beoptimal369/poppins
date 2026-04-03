// src/bpe/bpe_infer_tokenize.rs

use crate::bpe::BPETokenizer;
use std::collections::{HashSet, HashMap};


/// Converts a string to a sequence of token IDs using the trained BPE tokenizer
pub fn bpe_infer_tokenize(tokenizer: &BPETokenizer, text: &str) -> Vec<u32> {
    let mut result = Vec::new();

    let special_tokens: HashSet<&str> = tokenizer.vocab[..tokenizer.special_token_count as usize]
        .iter()
        .map(|s| s.as_str())
        .collect();
    
    let tokens = split_into_tokens(text, &special_tokens);

    for token in tokens {
        if special_tokens.contains(token.as_str()) {
            if let Some(&id) = tokenizer.token_to_id.get(&token) {
                result.push(id);
            } else {
                result.push(0);
            }
        } else {
            let merged = apply_merges_fast(&token, &tokenizer.merges, &tokenizer.token_to_id);
            result.extend(merged);
        }
    }
    
    result
}


/// Splits text into initial tokens (words and special tokens)
fn split_into_tokens(text: &str, special_tokens: &HashSet<&str>) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = text.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '<' {
            let mut potential = String::new();
            potential.push(c);
            
            while let Some(&next) = chars.peek() {
                potential.push(next);
                chars.next();
                if next == '>' {
                    break;
                }
            }
            
            if special_tokens.contains(potential.as_str()) {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }

                tokens.push(potential);
            } else {
                current.push_str(&potential);
            }
        } else {
            current.push(c);
        }
    }
    
    if !current.is_empty() {
        tokens.push(current);
    }
    
    tokens
}



/// Simple but fast BPE merge application
fn apply_merges_fast(
    token: &str,
    merges: &[(String, String)],
    token_to_id: &HashMap<String, u32>,
) -> Vec<u32> {
    // Build a map from (a,b) to merged result for O(1) lookup
    let merge_map: HashMap<(&str, &str), String> = merges
        .iter()
        .map(|(a, b)| ((a.as_str(), b.as_str()), format!("{}{}", a, b)))
        .collect();
    
    let mut word: Vec<String> = token.chars().map(|c| c.to_string()).collect();
    
    // Single pass merging - keep trying until no more merges
    loop {
        let mut i = 0;
        let mut merged = false;
        
        while i < word.len() - 1 {
            let a = word[i].as_str();
            let b = word[i + 1].as_str();
            
            if let Some(merged_str) = merge_map.get(&(a, b)) {
                word[i] = merged_str.clone();
                word.remove(i + 1);
                merged = true;
                // Stay at same index to check new merge
            } else {
                i += 1;
            }
        }
        
        if !merged {
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
    use super::*;
    use std::collections::HashMap;

    fn create_test_tokenizer() -> BPETokenizer {
        let mut vocab = vec![
            "<unknown>".to_string(),
            "<sample>".to_string(),
            "</sample>".to_string(),
            "<system>".to_string(),
            "</system>".to_string(),
            "<prompt>".to_string(),
            "</prompt>".to_string(),
            "<ai>".to_string(),
            "</ai>".to_string(),
            "<text>".to_string(),
            "</text>".to_string(),
            "<source>".to_string(),
            "</source>".to_string(),
            "<line-break />".to_string(),
            "<line-break count=\"2\" />".to_string(),
            "<js>".to_string(),
            "</js>".to_string(),
            "H".to_string(),
            "e".to_string(),
            "l".to_string(),
            "o".to_string(),
            " ".to_string(),
            "W".to_string(),
            "r".to_string(),
            "d".to_string(),
            "!".to_string(),
            "L".to_string(),
            "i".to_string(),
            "n".to_string(),
            "1".to_string(),
            "2".to_string(),
        ];
        
        // Add merged tokens for testing
        vocab.push("He".to_string());
        vocab.push("ll".to_string());
        vocab.push("Hello".to_string());
        vocab.push("World".to_string());
        vocab.push("Line1".to_string());
        vocab.push("Line2".to_string());
        
        let mut token_to_id = HashMap::new();
        for (id, token) in vocab.iter().enumerate() {
            token_to_id.insert(token.clone(), id as u32);
        }
        
        let merges = vec![
            ("H".to_string(), "e".to_string()),
            ("l".to_string(), "l".to_string()),
            ("He".to_string(), "ll".to_string()),
            ("Hell".to_string(), "o".to_string()),
            ("W".to_string(), "o".to_string()),
            ("Wo".to_string(), "r".to_string()),
            ("Wor".to_string(), "l".to_string()),
            ("Worl".to_string(), "d".to_string()),
            ("L".to_string(), "i".to_string()),
            ("Li".to_string(), "n".to_string()),
            ("Lin".to_string(), "e".to_string()),
            ("Line".to_string(), "1".to_string()),
        ];
        
        BPETokenizer {
            vocab,
            token_to_id,
            merges,
            special_token_count: 15, // All special tags up to and including <line-break count="2" />
            initial_token_count: 15,
        }
    }

    #[test]
    fn test_split_into_tokens_basic() {
        let tokenizer = create_test_tokenizer();
        let special_tokens: HashSet<&str> = tokenizer.vocab[..tokenizer.special_token_count as usize]
            .iter()
            .map(|s| s.as_str())
            .collect();
        
        let text = "Hello World";
        let tokens = split_into_tokens(text, &special_tokens);
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], "Hello World");
    }

    #[test]
    fn test_split_into_tokens_with_special_tags() {
        let tokenizer = create_test_tokenizer();
        let special_tokens: HashSet<&str> = tokenizer.vocab[..tokenizer.special_token_count as usize]
            .iter()
            .map(|s| s.as_str())
            .collect();
        
        let text = "<prompt>Hello</prompt>";
        let tokens = split_into_tokens(text, &special_tokens);
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], "<prompt>");
        assert_eq!(tokens[1], "Hello");
        assert_eq!(tokens[2], "</prompt>");
    }

    #[test]
    fn test_split_into_tokens_with_nested_tags() {
        let tokenizer = create_test_tokenizer();
        let special_tokens: HashSet<&str> = tokenizer.vocab[..tokenizer.special_token_count as usize]
            .iter()
            .map(|s| s.as_str())
            .collect();
        
        let text = "<ai><text>Hello</text></ai>";
        let tokens = split_into_tokens(text, &special_tokens);
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], "<ai>");
        assert_eq!(tokens[1], "<text>");
        assert_eq!(tokens[2], "Hello");
        assert_eq!(tokens[3], "</text>");
        assert_eq!(tokens[4], "</ai>");
    }

    #[test]
    fn test_split_into_tokens_with_line_break() {
        let tokenizer = create_test_tokenizer();
        let special_tokens: HashSet<&str> = tokenizer.vocab[..tokenizer.special_token_count as usize]
            .iter()
            .map(|s| s.as_str())
            .collect();
        
        let text = "Line1<line-break />Line2";
        let tokens = split_into_tokens(text, &special_tokens);
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], "Line1");
        assert_eq!(tokens[1], "<line-break />");
        assert_eq!(tokens[2], "Line2");
    }

    #[test]
    fn test_split_into_tokens_with_unknown_tag() {
        let tokenizer = create_test_tokenizer();
        let special_tokens: HashSet<&str> = tokenizer.vocab[..tokenizer.special_token_count as usize]
            .iter()
            .map(|s| s.as_str())
            .collect();
        
        // Use a tag that's definitely not in special tokens
        let text = "<xyz>text</xyz>";
        let tokens = split_into_tokens(text, &special_tokens);
        // Unknown tags are not recognized as special, so they remain as one token
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], "<xyz>text</xyz>");
    }

    #[test]
    fn test_apply_merges_fast_basic() {
        let tokenizer = create_test_tokenizer();
        let token = "Hello".to_string();
        let result = apply_merges_fast(&token, &tokenizer.merges, &tokenizer.token_to_id);
        
        // Should merge "Hello" into one token
        assert_eq!(result.len(), 1);
        assert_eq!(tokenizer.vocab[result[0] as usize], "Hello");
    }

    #[test]
    fn test_apply_merges_fast_partial() {
        let tokenizer = create_test_tokenizer();
        // Create a custom merge set that doesn't fully merge
        let merges = vec![
            ("H".to_string(), "e".to_string()),
            ("l".to_string(), "l".to_string()),
        ];
        
        let token = "Hello".to_string();
        let result = apply_merges_fast(&token, &merges, &tokenizer.token_to_id);
        
        // Should merge "He" and "ll", but not fully to "Hello"
        assert_eq!(result.len(), 3);
        assert_eq!(tokenizer.vocab[result[0] as usize], "He");
        assert_eq!(tokenizer.vocab[result[1] as usize], "ll");
        assert_eq!(tokenizer.vocab[result[2] as usize], "o");
    }

    #[test]
    fn test_apply_merges_fast_no_merges() {
        let tokenizer = create_test_tokenizer();
        let token = "xyz".to_string();
        let result = apply_merges_fast(&token, &tokenizer.merges, &tokenizer.token_to_id);
        
        // No merges possible, so each character becomes unknown (0)
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], 0);
        assert_eq!(result[1], 0);
        assert_eq!(result[2], 0);
    }

    #[test]
    fn test_apply_merges_fast_unicode() {
        let tokenizer = create_test_tokenizer();
        let mut vocab = tokenizer.vocab.clone();
        let mut token_to_id = tokenizer.token_to_id.clone();
        
        // Add Unicode characters
        let unicode_chars = ['é', 'á', 'ñ'];
        for c in unicode_chars {
            let token = c.to_string();
            let id = vocab.len() as u32;
            vocab.push(token.clone());
            token_to_id.insert(token, id);
        }
        
        let merges = vec![
            ("c".to_string(), "a".to_string()),
            ("f".to_string(), "é".to_string()),
        ];
        
        let tokenizer = BPETokenizer {
            vocab,
            token_to_id,
            merges,
            special_token_count: tokenizer.special_token_count,
            initial_token_count: tokenizer.initial_token_count,
        };
        
        let token = "café".to_string();
        let result = apply_merges_fast(&token, &tokenizer.merges, &tokenizer.token_to_id);
        
        // Should merge "ca" but "é" remains
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_bpe_infer_tokenize_simple_text() {
        let tokenizer = create_test_tokenizer();
        let text = "Hello";
        let result = bpe_infer_tokenize(&tokenizer, text);
        
        assert!(!result.is_empty());
        // "Hello" should merge to a single token
        assert_eq!(result.len(), 1);
        assert_eq!(tokenizer.vocab[result[0] as usize], "Hello");
    }

    #[test]
    fn test_bpe_infer_tokenize_with_special_tags() {
        let tokenizer = create_test_tokenizer();
        let text = "<prompt>Hello</prompt>";
        let result = bpe_infer_tokenize(&tokenizer, text);
        
        // Should have: <prompt>, Hello (merged), </prompt>
        assert_eq!(result.len(), 3);
        assert_eq!(tokenizer.vocab[result[0] as usize], "<prompt>");
        assert_eq!(tokenizer.vocab[result[1] as usize], "Hello");
        assert_eq!(tokenizer.vocab[result[2] as usize], "</prompt>");
    }

    #[test]
    fn test_bpe_infer_tokenize_with_line_break() {
        let tokenizer = create_test_tokenizer();
        let text = "Line1<line-break />Line2";
        let result = bpe_infer_tokenize(&tokenizer, text);
        
        // Should have: Line1 (characters), <line-break />, Line2 (characters)
        assert!(result.len() >= 3);
        // Find the line break token
        let line_break_id = tokenizer.token_to_id.get("<line-break />").unwrap();
        assert!(result.contains(line_break_id));
    }

    #[test]
    fn test_bpe_infer_tokenize_empty_string() {
        let tokenizer = create_test_tokenizer();
        let text = "";
        let result = bpe_infer_tokenize(&tokenizer, text);
        
        assert!(result.is_empty());
    }

    #[test]
    fn test_bpe_infer_tokenize_unknown_token() {
        let tokenizer = create_test_tokenizer();
        let text = "xyz";
        let result = bpe_infer_tokenize(&tokenizer, text);
        
        // Each unknown character becomes <unknown> (0)
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], 0);
        assert_eq!(result[1], 0);
        assert_eq!(result[2], 0);
    }

    #[test]
    fn test_bpe_infer_tokenize_multiple_merges() {
        let tokenizer = create_test_tokenizer();
        let text = "Hello World";
        let result = bpe_infer_tokenize(&tokenizer, text);
        
        // Should have: Hello (merged), space, World (merged)
        assert_eq!(result.len(), 3);
        assert_eq!(tokenizer.vocab[result[0] as usize], "Hello");
        assert_eq!(tokenizer.vocab[result[1] as usize], " ");
        assert_eq!(tokenizer.vocab[result[2] as usize], "World");
    }

    #[test]
    fn test_bpe_infer_tokenize_nested_tags() {
        let tokenizer = create_test_tokenizer();
        let text = "<ai><text>Hello</text></ai>";
        let result = bpe_infer_tokenize(&tokenizer, text);
        
        // Should have: <ai>, <text>, Hello, </text>, </ai>
        assert_eq!(result.len(), 5);
        assert_eq!(tokenizer.vocab[result[0] as usize], "<ai>");
        assert_eq!(tokenizer.vocab[result[1] as usize], "<text>");
        assert_eq!(tokenizer.vocab[result[2] as usize], "Hello");
        assert_eq!(tokenizer.vocab[result[3] as usize], "</text>");
        assert_eq!(tokenizer.vocab[result[4] as usize], "</ai>");
    }

    #[test]
    fn test_bpe_infer_tokenize_with_unknown_tag() {
        let tokenizer = create_test_tokenizer();
        // Use a tag that's definitely not in special tokens
        let text = "<xyz>Hello</xyz>";
        let result = bpe_infer_tokenize(&tokenizer, text);
        
        // Unknown tags should be tokenized as characters
        // The entire "<xyz>Hello</xyz>" becomes one text token that gets character-tokenized
        assert!(result.len() > 1);
        // The first character '<' is unknown and maps to 0
        assert_eq!(result[0], 0);
    }

    #[test]
    fn test_bpe_infer_tokenize_unicode() {
        let tokenizer = create_test_tokenizer();
        let mut vocab = tokenizer.vocab.clone();
        let mut token_to_id = tokenizer.token_to_id.clone();
        
        // Add Unicode characters
        let unicode_chars = ['é', 'á', 'ñ'];
        for c in unicode_chars {
            let token = c.to_string();
            let id = vocab.len() as u32;
            vocab.push(token.clone());
            token_to_id.insert(token, id);
        }
        
        let tokenizer = BPETokenizer {
            vocab,
            token_to_id,
            merges: tokenizer.merges,
            special_token_count: tokenizer.special_token_count,
            initial_token_count: tokenizer.initial_token_count,
        };
        
        let text = "café";
        let result = bpe_infer_tokenize(&tokenizer, text);
        
        // Should tokenize each character
        assert_eq!(result.len(), 4);
        assert!(result[0] < tokenizer.vocab.len() as u32);
        assert!(result[1] < tokenizer.vocab.len() as u32);
        assert!(result[2] < tokenizer.vocab.len() as u32);
        assert!(result[3] < tokenizer.vocab.len() as u32);
    }

    #[test]
    fn test_bpe_infer_tokenize_consistent() {
        let tokenizer = create_test_tokenizer();
        let text1 = "Hello World";
        let text2 = "Hello World";
        
        let result1 = bpe_infer_tokenize(&tokenizer, text1);
        let result2 = bpe_infer_tokenize(&tokenizer, text2);
        
        assert_eq!(result1, result2);
    }
}
