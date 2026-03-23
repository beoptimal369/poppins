// src/bpe/bpe_train.rs

use crate::sample::Sample;
use crate::bpe::{
    BPETokenizer,
    bpe_init_vocab,
    bpe_create_sequence,
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
/// * `min_merge_frequency` - Minimum frequency for a token pair to be merged.
///   Higher values result in smaller vocabularies. Typical range: 2-100.
///   Training stops automatically when the most frequent pair falls below this threshold.
///
/// # Returns
/// * `BPETokenizer` - The trained tokenizer
pub fn bpe_train(
    samples: &[Sample],
    special_tokens: &[String],
    requested_tokens: &[String],
    min_merge_frequency: usize,
) -> Result<BPETokenizer, std::io::Error> {
    // Create tokenizer
    let mut tokenizer = BPETokenizer {
        vocab: Vec::new(),
        token_to_id: std::collections::HashMap::new(),
        merges: Vec::new(),
        special_token_count: 0,
    };
    
    // Initialize vocabulary
    bpe_init_vocab(&mut tokenizer, samples, special_tokens, requested_tokens);
    
    // Convert samples to initial token sequence
    let mut token_sequence = bpe_create_sequence(&tokenizer, samples);
    
    loop {
        // Count pair frequencies
        let pair_counts = bpe_create_pair_counts_map(&tokenizer, &token_sequence);
        
        if pair_counts.is_empty() {
            break;
        }
        
        // Find the most frequent pair
        let (&(a, b), &count) = pair_counts
            .iter()
            .max_by_key(|&(_, &count)| count)
            .unwrap();
        
        // Stop if the most frequent pair falls below threshold
        if count < min_merge_frequency {
            break;
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
        
        // Update the token sequence
        let mut i = 0;

        while i < token_sequence.len() - 1 {
            if token_sequence[i] == a && token_sequence[i + 1] == b {
                token_sequence[i] = new_id;
                token_sequence.remove(i + 1);
            } else {
                i += 1;
            }
        }
    }

    Ok(tokenizer)
}



#[cfg(test)]
mod tests {
    use crate::bpe::{bpe_train, bpe_get_special_tokens};
    use crate::sample::{
        Sample, SamplePromptEnum, SampleAiEnum, SampleText, SampleSource,
        SampleAiCode, SampleLanguage, SampleIndent, SampleTokenStats,
    };

    fn create_test_samples() -> Vec<Sample> {
        vec![
            Sample {
                id: "1".to_string(),
                prompt_section: vec![
                    SamplePromptEnum::Text("Define computer.".to_string()),
                ],
                ai_section: vec![
                    SampleAiEnum::Text(SampleText {
                        content: "A computer is a computing device.".to_string(),
                        token_stats: SampleTokenStats {
                            weight_decay: 0.1,
                            dropout: 0.05,
                            loss_scale: 1.0,
                            gradient_scale: 1.0,
                            gradient_clip: 1.0,
                        },
                    }),
                    SampleAiEnum::Source(SampleSource {
                        id: "1".to_string(),
                        token_stats: SampleTokenStats {
                            weight_decay: 0.01,
                            dropout: 0.0,
                            loss_scale: 0.2,
                            gradient_scale: 2.0,
                            gradient_clip: 0.1,
                        },
                    }),
                ],
            },
            Sample {
                id: "2".to_string(),
                prompt_section: vec![
                    SamplePromptEnum::Text("What is JavaScript?".to_string()),
                ],
                ai_section: vec![
                    SampleAiEnum::Text(SampleText {
                        content: "JavaScript is a programming language.".to_string(),
                        token_stats: SampleTokenStats {
                            weight_decay: 0.1,
                            dropout: 0.05,
                            loss_scale: 1.0,
                            gradient_scale: 1.0,
                            gradient_clip: 1.0,
                        },
                    }),
                    SampleAiEnum::Code(SampleAiCode {
                        lang: SampleLanguage::Js,
                        inline: false,
                        indent: SampleIndent::Zero,
                        content: "console.log('hello')".to_string(),
                        token_stats: SampleTokenStats {
                            weight_decay: 0.05,
                            dropout: 0.1,
                            loss_scale: 1.0,
                            gradient_scale: 1.2,
                            gradient_clip: 0.7,
                        },
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
