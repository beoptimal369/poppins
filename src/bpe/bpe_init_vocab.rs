// src/bpe/bpe_init_vocab.rs

use std::collections::HashSet;
use crate::bpe::BPETokenizer;
use crate::sample::{Sample, SamplePromptEnum, SampleAiEnum};


/// Build initial vocabulary from samples
///
/// This function:
/// 1. Adds all special tokens (including <unknown> as first)
/// 2. Adds all requested tokens
/// 3. Collects all characters from samples (thought, text and code content)
/// 4. Adds common programming characters and digits
/// 5. Builds token to ID mapping
///
/// # Arguments
/// * `tokenizer` - Mutable reference to the tokenizer to initialize
/// * `samples` - Slice of samples to scan for characters
/// * `special_tokens` - List of special tokens to add (must include <unknown> as first)
/// * `requested_tokens` - List of tokens to force-add to vocabulary
pub fn bpe_init_vocab(
    tokenizer: &mut BPETokenizer,
    samples: &[Sample],
    special_tokens: &[String],
    requested_tokens: &[String],
) {
    tokenizer.vocab.clear();
    tokenizer.token_to_id.clear();
    
    // Track added tokens to avoid duplicates
    let mut added_tokens = HashSet::new();
    
    // Add all special tokens (including <unknown>)
    for token in special_tokens {
        if !added_tokens.contains(token) {
            tokenizer.vocab.push(token.clone());
            added_tokens.insert(token.clone());
        }
    }
    
    // Set special token count
    tokenizer.special_token_count = tokenizer.vocab.len() as u32;
    
    // Add requested tokens
    for token in requested_tokens {
        if !added_tokens.contains(token) {
            tokenizer.vocab.push(token.clone());
            added_tokens.insert(token.clone());
        }
    }
    
    // Set initial token count
    tokenizer.initial_token_count = tokenizer.vocab.len() as u32;
    
    // Only collect characters if there are samples
    if !samples.is_empty() {
        // 3. Collect all characters from samples
        let mut all_chars = HashSet::new();
        
        for sample in samples {
            // Scan system prompt
            if let Some(system) = &sample.system {
                all_chars.extend(system.chars());
            }
            
            // Scan thought (if present)
            if let Some(thought) = &sample.thought {
                for c in thought.chars() {
                    all_chars.insert(c);
                }
            }
            
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
                        for c in text.chars() {
                            all_chars.insert(c);
                        }
                    }
                    SampleAiEnum::Code(code) => {
                        for c in code.content.chars() {
                            all_chars.insert(c);
                        }
                    }
                    SampleAiEnum::Source(source) => {
                        for c in source.chars() {
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
            if !added_tokens.contains(&token) {
                tokenizer.vocab.push(token.clone());
                added_tokens.insert(token);
            }
        }
        
        // Add common programming characters
        let common_chars = [
            '!', '@', '#', '$', '%', '^', '&', '*', '+', '=', '~', '`', '|', '\\', ';', ':'
        ];

        for c in common_chars {
            let token = c.to_string();
            if !added_tokens.contains(&token) {
                tokenizer.vocab.push(token.clone());
                added_tokens.insert(token);
            }
        }
        
        // Add digits 0-9 if missing
        for digit in '0'..='9' {
            let token = digit.to_string();
            if !added_tokens.contains(&token) {
                tokenizer.vocab.push(token.clone());
                added_tokens.insert(token);
            }
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
    use crate::bpe::{BPETokenizer, bpe_init_vocab, bpe_get_special_tokens};
    use crate::sample::{
        Sample,
        SampleAiEnum,
        SampleLanguage,
        SamplePromptEnum,
    };

    fn create_test_samples() -> Vec<Sample> {
        vec![
            Sample {
                system: Some(String::new()),
                thought: None,
                prompt_section: vec![
                    SamplePromptEnum::Text("Hi".to_string()),
                ],
                ai_section: vec![
                    SampleAiEnum::Text("World".to_string()),
                ],
            },
        ]
    }

    fn create_test_samples_with_system() -> Vec<Sample> {
        vec![
            Sample {
                system: Some("You are a helpful assistant.\n".to_string()),
                thought: None,
                prompt_section: vec![
                    SamplePromptEnum::Text("Hi".to_string()),
                ],
                ai_section: vec![
                    SampleAiEnum::Text("World".to_string()),
                ],
            },
        ]
    }

    fn create_test_samples_with_thought() -> Vec<Sample> {
        vec![
            Sample {
                system: Some("You are a helpful assistant.".to_string()),
                thought: Some("1. Understand the question\n2. Provide a clear answer\n3. Be concise".to_string()),
                prompt_section: vec![
                    SamplePromptEnum::Text("Explain programming".to_string()),
                ],
                ai_section: vec![
                    SampleAiEnum::Text("Programming is giving instructions to a computer.".to_string()),
                ],
            },
        ]
    }

    #[test]
    fn test_init_vocab_with_special_tokens() {
        let mut tokenizer = BPETokenizer {
            vocab: Vec::new(),
            token_to_id: HashMap::new(),
            merges: Vec::new(),
            special_token_count: 0,
            initial_token_count: 0,
        };
        
        let samples = create_test_samples();
        let special_tokens = bpe_get_special_tokens();
        let requested_tokens = vec!["custom".to_string()];
        
        bpe_init_vocab(&mut tokenizer, &samples, &special_tokens, &requested_tokens);
        
        // Verify <unknown> is always first
        assert_eq!(tokenizer.vocab[0], "<unknown>");
        assert_eq!(tokenizer.token_to_id.get("<unknown>"), Some(&0));
        
        // special_token_count should equal the number of special tokens
        assert_eq!(tokenizer.special_token_count, special_tokens.len() as u32);
        
        // Verify requested tokens are added after special tokens
        let expected_initial_count = special_tokens.len() + requested_tokens.len();
        assert_eq!(tokenizer.initial_token_count, expected_initial_count as u32);
        
        // Verify specific special tokens exist
        for token in &special_tokens {
            assert!(tokenizer.vocab.contains(token));
            assert!(tokenizer.token_to_id.contains_key(token));
        }
        
        // Verify requested token exists
        assert!(tokenizer.vocab.contains(&"custom".to_string()));
        assert!(tokenizer.token_to_id.contains_key("custom"));
        
        // Verify character tokens exist
        assert!(tokenizer.vocab.contains(&"H".to_string()));
        assert!(tokenizer.vocab.contains(&"i".to_string()));
        assert!(tokenizer.vocab.contains(&"W".to_string()));
        assert!(tokenizer.vocab.contains(&"o".to_string()));
        assert!(tokenizer.vocab.contains(&"r".to_string()));
        assert!(tokenizer.vocab.contains(&"l".to_string()));
        assert!(tokenizer.vocab.contains(&"d".to_string()));
    }

    #[test]
    fn test_init_vocab_with_system_prompts() {
        let mut tokenizer = BPETokenizer {
            vocab: Vec::new(),
            token_to_id: HashMap::new(),
            merges: Vec::new(),
            special_token_count: 0,
            initial_token_count: 0,
        };
        
        let samples = create_test_samples_with_system();
        let special_tokens = bpe_get_special_tokens();
        let requested_tokens: Vec<String> = vec![];
        
        bpe_init_vocab(&mut tokenizer, &samples, &special_tokens, &requested_tokens);
        
        // Verify <unknown> is first
        assert_eq!(tokenizer.vocab[0], "<unknown>");
        
        // Verify special tokens are present
        for token in &special_tokens {
            assert!(tokenizer.vocab.contains(token));
        }
        
        // Verify characters from system prompt are added
        let system_chars = [
            'Y', 'o', 'u', ' ', 'a', 'r', 'e', ' ', 'a', ' ', 'h', 'e', 'l', 'p', 'f', 'u', 'l', ' ',
            'a', 's', 's', 'i', 's', 't', 'a', 'n', 't', '.', '\n'
        ];
        for c in system_chars {
            assert!(tokenizer.vocab.contains(&c.to_string()), "Character '{}' not found in vocab", c);
        }
        
        // Verify characters from regular prompt are added
        assert!(tokenizer.vocab.contains(&"H".to_string()));
        assert!(tokenizer.vocab.contains(&"i".to_string()));
        
        // Verify characters from AI section are added
        assert!(tokenizer.vocab.contains(&"W".to_string()));
        assert!(tokenizer.vocab.contains(&"o".to_string()));
        assert!(tokenizer.vocab.contains(&"r".to_string()));
        assert!(tokenizer.vocab.contains(&"l".to_string()));
        assert!(tokenizer.vocab.contains(&"d".to_string()));
    }

    #[test]
    fn test_init_vocab_with_thought() {
        let mut tokenizer = BPETokenizer {
            vocab: Vec::new(),
            token_to_id: HashMap::new(),
            merges: Vec::new(),
            special_token_count: 0,
            initial_token_count: 0,
        };
        
        let samples = create_test_samples_with_thought();
        let special_tokens = bpe_get_special_tokens();
        let requested_tokens: Vec<String> = vec![];
        
        bpe_init_vocab(&mut tokenizer, &samples, &special_tokens, &requested_tokens);
        
        // Verify <unknown> is first
        assert_eq!(tokenizer.vocab[0], "<unknown>");
        
        // Verify characters from thought are added
        let thought_chars = [
            '1', '.', ' ', 'U', 'n', 'd', 'e', 'r', 's', 't', 'a', 'n', 'd', ' ', 't', 'h', 'e', ' ', 'q', 'u', 'e', 's', 't', 'i', 'o', 'n', '\n',
            '2', '.', ' ', 'P', 'r', 'o', 'v', 'i', 'd', 'e', ' ', 'a', ' ', 'c', 'l', 'e', 'a', 'r', ' ', 'a', 'n', 's', 'w', 'e', 'r', '\n',
            '3', '.', ' ', 'B', 'e', ' ', 'c', 'o', 'n', 'c', 'i', 's', 'e'
        ];
        for c in thought_chars {
            assert!(tokenizer.vocab.contains(&c.to_string()), "Character '{}' from thought not found in vocab", c);
        }
        
        // Verify regular prompt characters are added
        assert!(tokenizer.vocab.contains(&"E".to_string()));
        assert!(tokenizer.vocab.contains(&"x".to_string()));
        assert!(tokenizer.vocab.contains(&"p".to_string()));
        assert!(tokenizer.vocab.contains(&"l".to_string()));
        assert!(tokenizer.vocab.contains(&"a".to_string()));
        assert!(tokenizer.vocab.contains(&"i".to_string()));
        assert!(tokenizer.vocab.contains(&"n".to_string()));
        
        // Verify AI section characters are added
        assert!(tokenizer.vocab.contains(&"P".to_string()));
        assert!(tokenizer.vocab.contains(&"r".to_string()));
        assert!(tokenizer.vocab.contains(&"o".to_string()));
        assert!(tokenizer.vocab.contains(&"g".to_string()));
        assert!(tokenizer.vocab.contains(&"m".to_string()));
    }

    #[test]
    fn test_init_vocab_without_requested_tokens() {
        let mut tokenizer = BPETokenizer {
            vocab: Vec::new(),
            token_to_id: HashMap::new(),
            merges: Vec::new(),
            special_token_count: 0,
            initial_token_count: 0,
        };
        
        let samples = create_test_samples();
        let special_tokens = bpe_get_special_tokens();
        let requested_tokens: Vec<String> = vec![];
        
        bpe_init_vocab(&mut tokenizer, &samples, &special_tokens, &requested_tokens);
        
        // Verify <unknown> is first
        assert_eq!(tokenizer.vocab[0], "<unknown>");
        
        // special_token_count should equal the number of special tokens
        assert_eq!(tokenizer.special_token_count, special_tokens.len() as u32);
        
        // With no requested tokens, initial_token_count equals special_token_count
        assert_eq!(tokenizer.initial_token_count, tokenizer.special_token_count);
        
        // Verify character tokens exist
        assert!(tokenizer.vocab.contains(&"H".to_string()));
        assert!(tokenizer.vocab.contains(&"i".to_string()));
        assert!(tokenizer.vocab.contains(&"W".to_string()));
        assert!(tokenizer.vocab.contains(&"o".to_string()));
        assert!(tokenizer.vocab.contains(&"r".to_string()));
        assert!(tokenizer.vocab.contains(&"l".to_string()));
        assert!(tokenizer.vocab.contains(&"d".to_string()));
    }

    #[test]
    fn test_init_vocab_empty_samples() {
        let mut tokenizer = BPETokenizer {
            vocab: Vec::new(),
            token_to_id: HashMap::new(),
            merges: Vec::new(),
            special_token_count: 0,
            initial_token_count: 0,
        };
        
        let samples: Vec<Sample> = vec![];
        let special_tokens = vec!["<unknown>".to_string(), "<custom>".to_string()];
        let requested_tokens = vec!["test".to_string()];
        
        bpe_init_vocab(&mut tokenizer, &samples, &special_tokens, &requested_tokens);
        
        // Verify <unknown> is first
        assert_eq!(tokenizer.vocab[0], "<unknown>");
        assert_eq!(tokenizer.token_to_id.get("<unknown>"), Some(&0));
        
        // Verify special tokens are present
        assert!(tokenizer.vocab.contains(&"<custom>".to_string()));
        
        // special_token_count should be number of special tokens
        assert_eq!(tokenizer.special_token_count, special_tokens.len() as u32);
        
        // Verify requested token is present
        assert!(tokenizer.vocab.contains(&"test".to_string()));
        
        // initial_token_count = special_token_count + requested_tokens.len()
        let expected_initial_count = special_tokens.len() + requested_tokens.len();
        assert_eq!(tokenizer.initial_token_count, expected_initial_count as u32);
        
        // No character tokens from samples
        assert_eq!(tokenizer.vocab.len(), expected_initial_count as usize);
    }

    #[test]
    fn test_init_vocab_duplicate_tokens() {
        let mut tokenizer = BPETokenizer {
            vocab: Vec::new(),
            token_to_id: HashMap::new(),
            merges: Vec::new(),
            special_token_count: 0,
            initial_token_count: 0,
        };
        
        let samples = create_test_samples();
        let special_tokens = vec!["<unknown>".to_string(), "<test>".to_string(), "<test>".to_string()];
        let requested_tokens = vec!["duplicate".to_string(), "duplicate".to_string()];
        
        bpe_init_vocab(&mut tokenizer, &samples, &special_tokens, &requested_tokens);
        
        // Should not add duplicates
        assert_eq!(tokenizer.vocab[0], "<unknown>");
        
        // special_token_count = <unknown> + <test>
        assert_eq!(tokenizer.special_token_count, 2);
        
        // requested tokens should be added once
        assert!(tokenizer.vocab.contains(&"duplicate".to_string()));
        
        // initial_token_count = special_token_count + unique requested tokens
        assert_eq!(tokenizer.initial_token_count, 3);
        
        // Verify character tokens are added
        assert!(tokenizer.vocab.contains(&"H".to_string()));
    }

    #[test]
    fn test_init_vocab_with_code_sample() {
        let mut tokenizer = BPETokenizer {
            vocab: Vec::new(),
            token_to_id: HashMap::new(),
            merges: Vec::new(),
            special_token_count: 0,
            initial_token_count: 0,
        };
        
        let samples = vec![
            Sample {
                system: Some(String::new()),
                thought: None,
                prompt_section: vec![
                    SamplePromptEnum::Code(crate::sample::SampleCode {
                        lang: SampleLanguage::Js,
                        inline: false,
                        indent: None,
                        content: "console.log()".to_string(),
                    }),
                ],
                ai_section: vec![],
            },
        ];
        
        let special_tokens = bpe_get_special_tokens();
        let requested_tokens: Vec<String> = vec![];
        
        bpe_init_vocab(&mut tokenizer, &samples, &special_tokens, &requested_tokens);
        
        // Verify <unknown> is first
        assert_eq!(tokenizer.vocab[0], "<unknown>");
        
        // Verify special tokens are present
        assert!(tokenizer.vocab.contains(&"<js>".to_string()));
        assert!(tokenizer.vocab.contains(&"</js>".to_string()));
        
        // Verify characters from code are added
        let code_chars = ['c', 'o', 'n', 's', 'l', 'e', '.', 'g', '(', ')'];
        for c in code_chars {
            assert!(tokenizer.vocab.contains(&c.to_string()));
        }
    }
}
