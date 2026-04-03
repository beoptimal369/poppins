// src/bpe/bpe_train.rs

use std::collections::{HashMap, BinaryHeap};
use crate::sample::Sample;
use crate::bpe::{
    BPETokenizer,
    bpe_init_vocab,
    bpe_train_tokenize,
    bpe_create_pair_counts_map,
};


/// Train a BPE tokenizer from structured samples
///
/// This function orchestrates the complete BPE training process:
/// 1. Initializes vocabulary with special tokens, requested tokens, and characters
/// 2. Converts samples to initial token sequence
/// 3. Iteratively merges the most frequent token pairs until no pair meets the frequency threshold
/// 4. Saves the final vocabulary and merges to vocab.json
///
/// # Arguments
/// * `samples` - Slice of samples to train on
/// * `special_tokens` - List of special tokens to protect from merging
/// * `requested_tokens` - List of tokens to force-add to vocabulary
/// * `min_merge_frequency` - Minimum frequency for a token pair to be merged. Higher values result in smaller vocabularies. Typical range: 2-100. Training stops automatically when the most frequent pair falls below this threshold.
///
/// # Returns
/// * `BPETokenizer` - The trained tokenizer
pub fn bpe_train(
    samples: &[Sample],
    special_tokens: &[String],
    requested_tokens: &[String],
    min_merge_frequency: usize,
) -> Result<BPETokenizer, Box<dyn std::error::Error>> {
    // Create tokenizer
    let mut tokenizer = BPETokenizer {
        vocab: Vec::new(),
        token_to_id: HashMap::new(),
        merges: Vec::new(),
        special_token_count: 0,
        initial_token_count: 0,
    };
    
    // Initialize vocabulary
    bpe_init_vocab(&mut tokenizer, samples, special_tokens, requested_tokens);
    
    // Convert samples to initial token sequence
    let mut token_sequence = bpe_train_tokenize(&tokenizer, samples)?;
    
    // Build initial pair counts and priority queue
    let mut pair_counts = bpe_create_pair_counts_map(&tokenizer, &token_sequence);
    let mut heap = BinaryHeap::new();
    
    for (&(a, b), &count) in &pair_counts {
        heap.push((count, a, b));
    }
    
    while let Some((count, a, b)) = heap.pop() {
        // Check if this pair is still valid and not outdated
        if count < min_merge_frequency {
            break;
        }
        
        // Verify the pair still exists in the current sequence
        // We need to check if this pair is still present with the same count
        let current_count = pair_counts.get(&(a, b)).copied().unwrap_or(0);
        if current_count != count {
            continue; // Outdated entry, skip
        }
        
        // Get token strings before modifying vocab
        let token_a = tokenizer.vocab[a as usize].clone();
        let token_b = tokenizer.vocab[b as usize].clone();
        let new_token = format!("{}{}", token_a, token_b);

        // Add new token to vocabulary
        let new_id = tokenizer.vocab.len() as u32;
        tokenizer.vocab.push(new_token.clone());
        tokenizer.token_to_id.insert(new_token, new_id);
        tokenizer.merges.push((token_a, token_b));
        
        // Update the token sequence and pair counts
        let mut i = 0;
        while i < token_sequence.len() - 1 {
            if token_sequence[i] == a && token_sequence[i + 1] == b {
                // Replace the pair with new token
                token_sequence[i] = new_id;
                token_sequence.remove(i + 1);
                
                // Update pair counts for affected positions
                // Check left neighbor
                if i > 0 {
                    let left_pair = (token_sequence[i - 1], a);
                    if let Some(old_count) = pair_counts.get_mut(&left_pair) {
                        *old_count -= 1;
                        if *old_count == 0 {
                            pair_counts.remove(&left_pair);
                        }
                    }
                    let new_left_pair = (token_sequence[i - 1], new_id);
                    *pair_counts.entry(new_left_pair).or_insert(0) += 1;
                }
                
                // Check right neighbor
                if i + 1 < token_sequence.len() {
                    let right_pair = (b, token_sequence[i + 1]);
                    if let Some(old_count) = pair_counts.get_mut(&right_pair) {
                        *old_count -= 1;
                        if *old_count == 0 {
                            pair_counts.remove(&right_pair);
                        }
                    }
                    let new_right_pair = (new_id, token_sequence[i + 1]);
                    *pair_counts.entry(new_right_pair).or_insert(0) += 1;
                }
                
                // Remove the old pair from counts
                let old_pair = (a, b);
                if let Some(old_count) = pair_counts.get_mut(&old_pair) {
                    *old_count -= 1;
                    if *old_count == 0 {
                        pair_counts.remove(&old_pair);
                    }
                }
            } else {
                i += 1;
            }
        }
        
        // Rebuild heap with current pair counts
        heap.clear();
        for (&(a, b), &count) in &pair_counts {
            heap.push((count, a, b));
        }
    }

    Ok(tokenizer)
}



#[cfg(test)]
mod tests {
    use crate::bpe::{bpe_train, bpe_get_special_tokens};
    use crate::sample::{
        Sample,
        SampleCode,
        SampleAiEnum,
        SampleLanguage,
        SamplePromptEnum,
    };

    fn create_test_samples() -> Vec<Sample> {
        vec![
            Sample {
                system: String::new(),
                prompt_section: vec![
                    SamplePromptEnum::Text("Define computer.".to_string()),
                ],
                ai_section: vec![
                    SampleAiEnum::Text("A computer is a computing device.".to_string()),
                    SampleAiEnum::Source("1".to_string()),
                ],
            },
            Sample {
                system: String::new(),
                prompt_section: vec![
                    SamplePromptEnum::Text("What is JavaScript?".to_string()),
                ],
                ai_section: vec![
                    SampleAiEnum::Text("JavaScript is a programming language.".to_string()),
                    SampleAiEnum::Code(SampleCode {
                        lang: SampleLanguage::Js,
                        inline: false,
                        indent: None,
                        content: "console.log('hello')".to_string(),
                    }),
                ],
            },
        ]
    }

    fn create_test_samples_with_system() -> Vec<Sample> {
        vec![
            Sample {
                system: "You are a helpful assistant.".to_string(),
                prompt_section: vec![
                    SamplePromptEnum::Text("Define computer.".to_string()),
                ],
                ai_section: vec![
                    SampleAiEnum::Text("A computer is a computing device.".to_string()),
                    SampleAiEnum::Source("1".to_string()),
                ],
            },
            Sample {
                system: "You are a programming expert.".to_string(),
                prompt_section: vec![
                    SamplePromptEnum::Text("What is JavaScript?".to_string()),
                ],
                ai_section: vec![
                    SampleAiEnum::Text("JavaScript is a programming language.".to_string()),
                    SampleAiEnum::Code(SampleCode {
                        lang: SampleLanguage::Js,
                        inline: false,
                        indent: None,
                        content: "console.log('hello')".to_string(),
                    }),
                ],
            },
        ]
    }

    #[test]
    fn test_bpe_train_basic() {
        let samples = create_test_samples();
        let special_tokens = bpe_get_special_tokens();
        let requested_tokens = vec!["console.log".to_string()];
        let min_merge_frequency = 3;
        
        let result = bpe_train(&samples, &special_tokens, &requested_tokens, min_merge_frequency);
        
        assert!(result.is_ok());
        let tokenizer = result.unwrap();
        
        // Verify tokenizer was trained
        assert!(tokenizer.vocab.len() > special_tokens.len());
        assert!(tokenizer.merges.len() > 0);
        assert_eq!(tokenizer.special_token_count, special_tokens.len() as u32);
        
        // Verify requested token was added
        assert!(tokenizer.vocab.contains(&"console.log".to_string()));
    }

    #[test]
    fn test_bpe_train_with_system_prompts() {
        let samples = create_test_samples_with_system();
        let special_tokens = bpe_get_special_tokens();
        let requested_tokens = vec!["console.log".to_string()];
        let min_merge_frequency = 3;
        
        let result = bpe_train(&samples, &special_tokens, &requested_tokens, min_merge_frequency);
        
        assert!(result.is_ok());
        let tokenizer = result.unwrap();
        
        // Verify tokenizer was trained
        assert!(tokenizer.vocab.len() > special_tokens.len());
        assert!(tokenizer.merges.len() > 0);
        assert_eq!(tokenizer.special_token_count, special_tokens.len() as u32);
        
        // Verify requested token was added
        assert!(tokenizer.vocab.contains(&"console.log".to_string()));
        
        // System prompts should contribute to vocabulary
        // Check if characters from system prompts appear in vocab
        let system_chars = ['Y', 'o', 'u', 'a', 'r', 'e', 'h', 'l', 'p', 'f', 'u', 'l', 'p', 'r', 'g', 'm', 'x', 't'];
        let has_system_chars = system_chars.iter().any(|&c| {
            tokenizer.vocab.contains(&c.to_string())
        });
        assert!(has_system_chars, "System prompt characters should be in vocabulary");
    }

    #[test]
    fn test_bpe_train_empty_samples() {
        let samples: Vec<Sample> = vec![];
        let special_tokens = bpe_get_special_tokens();
        let requested_tokens = vec!["test".to_string()];
        let min_merge_frequency = 3;
        
        let result = bpe_train(&samples, &special_tokens, &requested_tokens, min_merge_frequency);
        
        assert!(result.is_ok());
        let tokenizer = result.unwrap();
        
        // Should still have special tokens and requested token
        assert!(tokenizer.vocab.len() >= special_tokens.len());
        assert!(tokenizer.vocab.contains(&"test".to_string()));
        
        // No merges should be performed (no data)
        assert_eq!(tokenizer.merges.len(), 0);
    }

    #[test]
    fn test_bpe_train_without_requested_tokens() {
        let samples = create_test_samples();
        let special_tokens = bpe_get_special_tokens();
        let requested_tokens: Vec<String> = vec![];
        let min_merge_frequency = 3;
        
        let result = bpe_train(&samples, &special_tokens, &requested_tokens, min_merge_frequency);
        
        assert!(result.is_ok());
        let tokenizer = result.unwrap();
        
        // Verify tokenizer was trained
        assert!(tokenizer.vocab.len() > special_tokens.len());
        assert!(tokenizer.merges.len() > 0);
    }

    #[test]
    fn test_bpe_train_with_high_min_frequency() {
        let samples = create_test_samples();
        let special_tokens = bpe_get_special_tokens();
        let requested_tokens = vec![];
        let min_merge_frequency = 1000;
        
        let result = bpe_train(&samples, &special_tokens, &requested_tokens, min_merge_frequency);
        
        assert!(result.is_ok());
        let tokenizer = result.unwrap();
        
        // No merges should happen because frequency threshold is too high
        assert_eq!(tokenizer.merges.len(), 0);
        
        // Vocab size should be greater than just special tokens
        // (includes characters from the samples)
        assert!(tokenizer.vocab.len() > special_tokens.len());
        
        // With no merges, vocab size should be exactly the initial vocab size
        // Initial vocab includes: special_tokens + all unique chars from samples + requested_tokens
        // Let's verify it's not just special_tokens + 1
        assert!(tokenizer.vocab.len() > special_tokens.len() + 1);
    }

    #[test]
    fn test_bpe_train_with_low_min_frequency() {
        let samples = create_test_samples();
        let special_tokens = bpe_get_special_tokens();
        let requested_tokens = vec![];
        // Very low threshold - should merge until no pairs left
        let min_merge_frequency = 1;
        
        let result = bpe_train(&samples, &special_tokens, &requested_tokens, min_merge_frequency);
        
        assert!(result.is_ok());
        let tokenizer = result.unwrap();
        
        // Should have many merges
        assert!(tokenizer.merges.len() > 10);
        assert!(tokenizer.vocab.len() > 50);
    }

    #[test]
    fn test_bpe_train_preserves_special_tokens() {
        let samples = create_test_samples();
        let special_tokens = bpe_get_special_tokens();
        let requested_tokens = vec!["console.log".to_string()];
        let min_merge_frequency = 3;
        
        let result = bpe_train(&samples, &special_tokens, &requested_tokens, min_merge_frequency);
        
        assert!(result.is_ok());
        let tokenizer = result.unwrap();
        
        // Verify all special tokens are still in vocab
        for token in &special_tokens {
            assert!(tokenizer.vocab.contains(token), "Special token missing: {}", token);
        }
        
        // Verify special token count is correct
        assert_eq!(tokenizer.special_token_count, special_tokens.len() as u32);
        
        // Verify no merges created special tokens (they should be at the start)
        for (i, token) in tokenizer.vocab.iter().enumerate() {
            if i < tokenizer.special_token_count as usize {
                assert!(special_tokens.contains(token), "Non-special token in special range: {}", token);
            }
        }
    }
}
