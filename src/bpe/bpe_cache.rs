// src/bpe/bpe_cache.rs

use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::bpe::{BPETokenizer, bpe_infer_tokenize};


static BPE_CACHE: Lazy<Mutex<HashMap<String, Vec<u32>>>> = Lazy::new(|| Mutex::new(HashMap::new()));


pub struct BPECache {
    pub sample_open: Vec<u32>,
    pub sample_close: Vec<u32>,
    pub system_open: Vec<u32>,
    pub system_close: Vec<u32>,
    pub thought_open: Vec<u32>,
    pub thought_close: Vec<u32>,
    pub prompt_open: Vec<u32>,
    pub prompt_close: Vec<u32>,
    pub ai_open: Vec<u32>,
    pub ai_close: Vec<u32>,
    pub text_open: Vec<u32>,
    pub text_close: Vec<u32>,
    pub source_open: Vec<u32>,
    pub source_close: Vec<u32>,
    pub line_break_single: Vec<u32>,
    pub line_break_double: Vec<u32>,
}


pub fn create_bpe_cache(tokenizer: &BPETokenizer) -> BPECache {
    BPECache {
        sample_open: get_bpe_cache_tokens(tokenizer, "<sample>"),
        sample_close: get_bpe_cache_tokens(tokenizer, "</sample>"),
        system_open: get_bpe_cache_tokens(tokenizer, "<system>"),
        system_close: get_bpe_cache_tokens(tokenizer, "</system>"),
        thought_open: get_bpe_cache_tokens(tokenizer, "<thought>"),
        thought_close: get_bpe_cache_tokens(tokenizer, "</thought>"),
        prompt_open: get_bpe_cache_tokens(tokenizer, "<prompt>"),
        prompt_close: get_bpe_cache_tokens(tokenizer, "</prompt>"),
        ai_open: get_bpe_cache_tokens(tokenizer, "<ai>"),
        ai_close: get_bpe_cache_tokens(tokenizer, "</ai>"),
        text_open: get_bpe_cache_tokens(tokenizer, "<text>"),
        text_close: get_bpe_cache_tokens(tokenizer, "</text>"),
        source_open: get_bpe_cache_tokens(tokenizer, "<source>"),
        source_close: get_bpe_cache_tokens(tokenizer, "</source>"),
        line_break_single: get_bpe_cache_tokens(tokenizer, "<line-break />"),
        line_break_double: get_bpe_cache_tokens(tokenizer, "<line-break count=\"2\" />"),
    }
}


pub fn get_bpe_cache_tokens(tokenizer: &BPETokenizer, s: &str) -> Vec<u32> {
    let mut cache = BPE_CACHE.lock().unwrap_or_else(|poisoned| poisoned.into_inner());

    if let Some(tokens) = cache.get(s) {
        return tokens.clone();
    }

    let tokens = bpe_infer_tokenize(tokenizer, s);
    cache.insert(s.to_string(), tokens.clone());
    tokens
}



#[cfg(test)]
mod tests {
    use super::BPE_CACHE;
    use once_cell::sync::Lazy;
    use crate::sample::{Sample, SamplePromptEnum, SampleAiEnum};
    use crate::bpe::{bpe_train, create_bpe_cache, get_bpe_cache_tokens, BPETokenizer};


    fn clear_bpe_cache() {
        let mut cache = BPE_CACHE.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        cache.clear();
    }

    
    static TEST_TOKENIZER: Lazy<BPETokenizer> = Lazy::new(|| {
        let samples = vec![
            Sample {
                system: Some(String::new()),
                thought: None,
                prompt_section: vec![
                    SamplePromptEnum::Text("Hello world".to_string()),
                ],
                ai_section: vec![
                    SampleAiEnum::Text("Hi there".to_string()),
                ],
            },
        ];
        
        let special_tokens = vec![
            "<unknown>".to_string(),
            "<sample>".to_string(),
            "</sample>".to_string(),
            "<system>".to_string(),
            "</system>".to_string(),
            "<thought>".to_string(),
            "</thought>".to_string(),
            "<prompt>".to_string(),
            "</prompt>".to_string(),
            "<ai>".to_string(),
            "</ai>".to_string(),
            "<text>".to_string(),
            "</text>".to_string(),
            "<source>".to_string(),
            "</source>".to_string(),
        ];
        
        let requested_tokens = vec![];
        let min_merge_frequency = 1;
        
        bpe_train(&samples, &special_tokens, &requested_tokens, min_merge_frequency).unwrap()
    });
    
    #[test]
    fn test_create_bpe_cache() {
        clear_bpe_cache();
        let tokenizer = &TEST_TOKENIZER;
        
        // Log what's in the tokenizer's vocabulary
        println!("Tokenizer special token count: {}", tokenizer.special_token_count);
        println!("First 20 vocab entries: {:?}", &tokenizer.vocab[..20.min(tokenizer.vocab.len())]);
        
        // Check each special token individually
        let test_strings = vec![
            "<sample>", "</sample>", "<system>", "</system>",
            "<thought>", "</thought>", "<prompt>", "</prompt>",
            "<ai>", "</ai>", "<text>", "</text>", "<source>", "</source>",
            "<line-break />", "<line-break count=\"2\" />"
        ];
        
        for s in &test_strings {
            let is_in_vocab = tokenizer.token_to_id.contains_key(*s);
            println!("  '{}' in vocab: {}", s, is_in_vocab);
            if is_in_vocab {
                println!("    token_id: {:?}", tokenizer.token_to_id.get(*s));
            }
        }
        
        let bpe_cache = create_bpe_cache(tokenizer);
        
        let cache = BPE_CACHE.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        println!("Cache keys after create_bpe_cache: {:?}", cache.keys().collect::<Vec<_>>());
        
        // Verify all fields are populated
        assert!(!bpe_cache.sample_open.is_empty());
        assert!(!bpe_cache.sample_close.is_empty());
        assert!(!bpe_cache.system_open.is_empty());
        assert!(!bpe_cache.system_close.is_empty());
        assert!(!bpe_cache.thought_open.is_empty());
        assert!(!bpe_cache.thought_close.is_empty());
        assert!(!bpe_cache.prompt_open.is_empty());
        assert!(!bpe_cache.prompt_close.is_empty());
        assert!(!bpe_cache.ai_open.is_empty());
        assert!(!bpe_cache.ai_close.is_empty());
        assert!(!bpe_cache.text_open.is_empty());
        assert!(!bpe_cache.text_close.is_empty());
        assert!(!bpe_cache.source_open.is_empty());
        assert!(!bpe_cache.source_close.is_empty());
        assert!(!bpe_cache.line_break_single.is_empty());
        assert!(!bpe_cache.line_break_double.is_empty());
        
        // Verify the cache contains all these strings
        for s in &test_strings {
            assert!(cache.contains_key(*s), "Missing key: {}", s);
        }
    }
    
    #[test]
    fn test_get_bpe_cache_tokens_different_strings() {
        clear_bpe_cache();
        let tokenizer = &TEST_TOKENIZER;
        
        let string1 = "<prompt>";
        let string2 = "</prompt>";
        
        // Log token IDs for these strings
        println!("'{}' token_id: {:?}", string1, tokenizer.token_to_id.get(string1));
        println!("'{}' token_id: {:?}", string2, tokenizer.token_to_id.get(string2));
        
        let tokens1 = get_bpe_cache_tokens(tokenizer, string1);
        let tokens2 = get_bpe_cache_tokens(tokenizer, string2);
        
        println!("tokens1 length: {}, tokens2 length: {}", tokens1.len(), tokens2.len());
        
        assert_ne!(tokens1, tokens2);
        
        let cache = BPE_CACHE.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        assert!(cache.contains_key(string1));
        assert!(cache.contains_key(string2));
    }
    
    #[test]
    fn test_get_bpe_cache_tokens_empty_string() {
        clear_bpe_cache();
        let tokenizer = &TEST_TOKENIZER;
        let test_string = "";
        
        let tokens = get_bpe_cache_tokens(tokenizer, test_string);
        assert!(tokens.is_empty());
        
        let cache = BPE_CACHE.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        assert!(cache.contains_key(test_string));
        assert_eq!(cache.get(test_string).unwrap(), &tokens);
    }
}
