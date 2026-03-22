// src/bpe/bpe_init_vocab.rs

use std::collections::HashSet;
use crate::bpe::BPETokenizer;
use crate::sample::{Sample, SamplePromptEnum, SampleAiEnum};


/// Build initial vocabulary from samples
///
/// This function:
/// 1. Adds all special tokens
/// 2. Adds all requested tokens
/// 3. Collects all characters from samples (text and code content)
/// 4. Adds common programming characters and digits
/// 5. Builds token to ID mapping
///
/// # Arguments
/// * `tokenizer` - Mutable reference to the tokenizer to initialize
/// * `samples` - Slice of samples to scan for characters
/// * `special_tokens` - List of special tokens to add
/// * `requested_tokens` - List of tokens to force-add to vocabulary
pub fn bpe_init_vocab(
    tokenizer: &mut BPETokenizer,
    samples: &[Sample],
    special_tokens: &[String],
    requested_tokens: &[String],
) {
    tokenizer.vocab.clear();
    tokenizer.token_to_id.clear();
    
    // Add all special tokens
    for token in special_tokens {
        tokenizer.vocab.push(token.clone());
    }
    tokenizer.special_token_count = special_tokens.len() as u32;
    
    // Add requested tokens
    for token in requested_tokens {
        if !tokenizer.vocab.contains(token) {
            tokenizer.vocab.push(token.clone());
        }
    }
    
    // Collect all characters from samples
    let mut all_chars = HashSet::new();
    
    for sample in samples {
        // Scan prompt section
        for item in &sample.prompt_section {
            match item {
                SamplePromptEnum::Text(text) => {
                    for c in text.chars() {
                        all_chars.insert(c);
                    }
                }
                SamplePromptEnum::Code(code) => {
                    for c in code.content.chars() {
                        all_chars.insert(c);
                    }
                }
                SamplePromptEnum::LineBreak(_) => {
                    // LineBreak doesn't contain text content
                }
            }
        }
        
        // Scan AI section
        for item in &sample.ai_section {
            match item {
                SampleAiEnum::Text(text) => {
                    for c in text.content.chars() {
                        all_chars.insert(c);
                    }
                }
                SampleAiEnum::Code(code) => {
                    for c in code.content.chars() {
                        all_chars.insert(c);
                    }
                }
                SampleAiEnum::Source(source) => {
                    for c in source.id.chars() {
                        all_chars.insert(c);
                    }
                }
                SampleAiEnum::LineBreak(_) => {
                    // LineBreak doesn't contain text content
                }
            }
        }
    }
    
    // Add all characters to vocab
    let mut chars: Vec<char> = all_chars.into_iter().collect();
    chars.sort();
    for c in chars {
        let token = c.to_string();
        if !tokenizer.vocab.contains(&token) {
            tokenizer.vocab.push(token);
        }
    }
    
    // Add common programming characters
    let common_chars = [
        '!', '@', '#', '$', '%', '^', '&', '*', '+', '=', '~', '`', '|', '\\', ';', ':'
    ];
    for c in common_chars {
        let token = c.to_string();
        if !tokenizer.vocab.contains(&token) {
            tokenizer.vocab.push(token);
        }
    }
    
    // Add digits 0-9 if missing
    for digit in '0'..='9' {
        let token = digit.to_string();
        if !tokenizer.vocab.contains(&token) {
            tokenizer.vocab.push(token);
        }
    }
    
    // Build token to ID mapping
    for (id, token) in tokenizer.vocab.iter().enumerate() {
        tokenizer.token_to_id.insert(token.clone(), id as u32);
    }
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::bpe::{BPETokenizer, bpe_init_vocab};
    use crate::sample::{
        Sample, SamplePromptEnum, SampleAiEnum, SampleText, 
        SampleTokenStats, SampleAiCode, SampleLanguage, SampleIndent
    };

    fn create_test_sample() -> Sample {
        Sample {
            id: "1".to_string(),
            prompt_section: vec![
                SamplePromptEnum::Text("What is a computer?".to_string()),
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
                SampleAiEnum::Code(SampleAiCode {
                    lang: SampleLanguage::Ts,
                    inline: false,
                    indent: SampleIndent::Zero,
                    content: "function example() {\n  console.log('hi')\n}".to_string(),
                    token_stats: SampleTokenStats {
                        weight_decay: 0.1,
                        dropout: 0.05,
                        loss_scale: 1.0,
                        gradient_scale: 1.0,
                        gradient_clip: 1.0,
                    },
                }),
            ],
        }
    }

    #[test]
    fn test_init_vocab_with_special_tokens() {
        let mut tokenizer = BPETokenizer {
            vocab: Vec::new(),
            token_to_id: HashMap::new(),
            merges: Vec::new(),
            special_token_count: 0,
        };
        
        let samples = vec![create_test_sample()];
        let special_tokens = vec!["<unknown>".to_string(), "<sample>".to_string(), "</sample>".to_string()];
        let requested_tokens: Vec<String> = vec![];
        
        bpe_init_vocab(&mut tokenizer, &samples, &special_tokens, &requested_tokens);
        
        // Check special tokens are added
        assert_eq!(tokenizer.vocab[0], "<unknown>");
        assert_eq!(tokenizer.vocab[1], "<sample>");
        assert_eq!(tokenizer.vocab[2], "</sample>");
        assert_eq!(tokenizer.special_token_count, 3);
    }

    #[test]
    fn test_init_vocab_with_requested_tokens() {
        let mut tokenizer = BPETokenizer {
            vocab: Vec::new(),
            token_to_id: HashMap::new(),
            merges: Vec::new(),
            special_token_count: 0,
        };
        
        let samples = vec![create_test_sample()];
        let special_tokens = vec!["<unknown>".to_string()];
        let requested_tokens = vec!["console.log".to_string(), "Array".to_string()];
        
        bpe_init_vocab(&mut tokenizer, &samples, &special_tokens, &requested_tokens);
        
        // Check requested tokens are added
        assert!(tokenizer.vocab.contains(&"console.log".to_string()));
        assert!(tokenizer.vocab.contains(&"Array".to_string()));
    }

    #[test]
    fn test_init_vocab_collects_characters() {
        let mut tokenizer = BPETokenizer {
            vocab: Vec::new(),
            token_to_id: HashMap::new(),
            merges: Vec::new(),
            special_token_count: 0,
        };
        
        let samples = vec![create_test_sample()];
        let special_tokens = vec!["<unknown>".to_string()];
        let requested_tokens: Vec<String> = vec![];
        
        bpe_init_vocab(&mut tokenizer, &samples, &special_tokens, &requested_tokens);
        
        // Check characters from text are collected
        assert!(tokenizer.vocab.contains(&"W".to_string()));
        assert!(tokenizer.vocab.contains(&"h".to_string()));
        assert!(tokenizer.vocab.contains(&"a".to_string()));
        assert!(tokenizer.vocab.contains(&"t".to_string()));
        
        // Check characters from code are collected (including newline)
        assert!(tokenizer.vocab.contains(&"\n".to_string()));
        assert!(tokenizer.vocab.contains(&"{".to_string()));
        assert!(tokenizer.vocab.contains(&"}".to_string()));
        assert!(tokenizer.vocab.contains(&"'".to_string()));
    }

    #[test]
    fn test_init_vocab_adds_common_chars() {
        let mut tokenizer = BPETokenizer {
            vocab: Vec::new(),
            token_to_id: HashMap::new(),
            merges: Vec::new(),
            special_token_count: 0,
        };
        
        let samples: Vec<Sample> = vec![];
        let special_tokens = vec!["<unknown>".to_string()];
        let requested_tokens: Vec<String> = vec![];
        
        bpe_init_vocab(&mut tokenizer, &samples, &special_tokens, &requested_tokens);
        
        // Check common programming characters are added
        assert!(tokenizer.vocab.contains(&"!".to_string()));
        assert!(tokenizer.vocab.contains(&"@".to_string()));
        assert!(tokenizer.vocab.contains(&"#".to_string()));
        assert!(tokenizer.vocab.contains(&"$".to_string()));
        assert!(tokenizer.vocab.contains(&"%".to_string()));
        assert!(tokenizer.vocab.contains(&"^".to_string()));
        assert!(tokenizer.vocab.contains(&"&".to_string()));
        assert!(tokenizer.vocab.contains(&"*".to_string()));
        assert!(tokenizer.vocab.contains(&"+".to_string()));
        assert!(tokenizer.vocab.contains(&"=".to_string()));
        assert!(tokenizer.vocab.contains(&"~".to_string()));
        assert!(tokenizer.vocab.contains(&"`".to_string()));
        assert!(tokenizer.vocab.contains(&"|".to_string()));
        assert!(tokenizer.vocab.contains(&"\\".to_string()));
        assert!(tokenizer.vocab.contains(&";".to_string()));
        assert!(tokenizer.vocab.contains(&":".to_string()));
    }

    #[test]
    fn test_init_vocab_adds_digits() {
        let mut tokenizer = BPETokenizer {
            vocab: Vec::new(),
            token_to_id: HashMap::new(),
            merges: Vec::new(),
            special_token_count: 0,
        };
        
        let samples: Vec<Sample> = vec![];
        let special_tokens = vec!["<unknown>".to_string()];
        let requested_tokens: Vec<String> = vec![];
        
        bpe_init_vocab(&mut tokenizer, &samples, &special_tokens, &requested_tokens);
        
        // Check all digits are added
        for digit in '0'..='9' {
            assert!(tokenizer.vocab.contains(&digit.to_string()));
        }
    }

    #[test]
    fn test_init_vocab_no_duplicates() {
        let mut tokenizer = BPETokenizer {
            vocab: Vec::new(),
            token_to_id: HashMap::new(),
            merges: Vec::new(),
            special_token_count: 0,
        };
        
        let samples = vec![create_test_sample()];
        let special_tokens = vec!["<unknown>".to_string(), "<sample>".to_string()];
        let requested_tokens = vec!["a".to_string()]; // 'a' is also in text
        
        bpe_init_vocab(&mut tokenizer, &samples, &special_tokens, &requested_tokens);
        
        // Check no duplicates
        let unique_tokens: std::collections::HashSet<_> = tokenizer.vocab.iter().collect();
        assert_eq!(unique_tokens.len(), tokenizer.vocab.len());
    }

    #[test]
    fn test_init_vocab_builds_token_to_id() {
        let mut tokenizer = BPETokenizer {
            vocab: Vec::new(),
            token_to_id: HashMap::new(),
            merges: Vec::new(),
            special_token_count: 0,
        };
        
        let samples = vec![create_test_sample()];
        let special_tokens = vec!["<unknown>".to_string(), "<sample>".to_string()];
        let requested_tokens: Vec<String> = vec![];
        
        bpe_init_vocab(&mut tokenizer, &samples, &special_tokens, &requested_tokens);
        
        // Check each token has correct ID
        for (id, token) in tokenizer.vocab.iter().enumerate() {
            assert_eq!(tokenizer.token_to_id.get(token), Some(&(id as u32)));
        }
        
        // Check ID count matches vocab length
        assert_eq!(tokenizer.token_to_id.len(), tokenizer.vocab.len());
    }

    #[test]
    fn test_init_vocab_empty_samples() {
        let mut tokenizer = BPETokenizer {
            vocab: Vec::new(),
            token_to_id: HashMap::new(),
            merges: Vec::new(),
            special_token_count: 0,
        };
        
        let samples: Vec<Sample> = vec![];
        let special_tokens = vec!["<unknown>".to_string()];
        let requested_tokens = vec!["console.log".to_string()];
        
        bpe_init_vocab(&mut tokenizer, &samples, &special_tokens, &requested_tokens);
        
        // Should still have special tokens and requested tokens
        assert_eq!(tokenizer.vocab[0], "<unknown>");
        assert!(tokenizer.vocab.contains(&"console.log".to_string()));
        
        // Should still have common chars and digits
        assert!(tokenizer.vocab.contains(&"!".to_string()));
        assert!(tokenizer.vocab.contains(&"0".to_string()));
    }
}
