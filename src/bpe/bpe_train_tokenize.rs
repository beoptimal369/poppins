// src/bpe/bpe_train_tokenize.rs

use crate::bpe::BPETokenizer;
use crate::bpe::bpe_token_writer::BPETokenWriter;
use crate::sample::Sample;
use crate::tag::{
    tag_write_tag,
    tag_write_prompt_content,
    tag_write_ai_content,
};


/// Convert structured samples to initial token sequence
///
/// This function walks through each sample and converts all content to token IDs:
/// - Special tags are looked up in the vocabulary using tag functions
/// - Text content is split into individual characters
/// - Code content preserves all whitespace and is split into individual characters
///
/// # Arguments
/// * `tokenizer` - Reference to the tokenizer with initialized vocabulary
/// * `samples` - Slice of samples to convert
///
/// # Returns
/// * `Result<Vec<u32>, std::io::Error>` - Sequence of token IDs or I/O error
pub fn bpe_train_tokenize(
    tokenizer: &BPETokenizer,
    samples: &[Sample],
) -> Result<Vec<u32>, std::io::Error> {
    let mut sequence = Vec::new();
    
    // Get special tokens for tag writing
    let special_tokens: Vec<String> = tokenizer.vocab[..tokenizer.special_token_count as usize].to_vec();
    
    for sample in samples {
        // Add <sample> tag
        add_tag_to_sequence(&mut sequence, "sample", true, &special_tokens, tokenizer)?;
        
        // Write system tag if present
        if !sample.system.is_empty() {
            add_tag_to_sequence(&mut sequence, "system", true, &special_tokens, tokenizer)?;
            add_text_to_sequence(&mut sequence, &sample.system, tokenizer);
            add_tag_to_sequence(&mut sequence, "system", false, &special_tokens, tokenizer)?;
        }
        
        // Write prompt section
        add_tag_to_sequence(&mut sequence, "prompt", true, &special_tokens, tokenizer)?;
        {
            let mut writer = BPETokenWriter::new(&mut sequence, tokenizer);
            tag_write_prompt_content(&mut writer, &sample.prompt_section, &special_tokens)?;
        }
        add_tag_to_sequence(&mut sequence, "prompt", false, &special_tokens, tokenizer)?;
        
        // Write AI section
        add_tag_to_sequence(&mut sequence, "ai", true, &special_tokens, tokenizer)?;
        {
            let mut writer = BPETokenWriter::new(&mut sequence, tokenizer);
            tag_write_ai_content(&mut writer, &sample.ai_section, &special_tokens)?;
        }
        add_tag_to_sequence(&mut sequence, "ai", false, &special_tokens, tokenizer)?;
        
        // Add </sample> tag
        add_tag_to_sequence(&mut sequence, "sample", false, &special_tokens, tokenizer)?;
    }
    
    Ok(sequence)
}


/// Adds a simple tag to the sequence as a single token
fn add_tag_to_sequence(
    sequence: &mut Vec<u32>,
    tag_name: &str,
    is_opening: bool,
    special_tokens: &[String],
    tokenizer: &BPETokenizer,
) -> Result<(), std::io::Error> {
    let mut buffer = Vec::new();
    tag_write_tag(&mut buffer, tag_name, is_opening, special_tokens)?;
    let tag_string = String::from_utf8(buffer).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, e)
    })?;
    
    if let Some(&id) = tokenizer.token_to_id.get(&tag_string) {
        sequence.push(id);
    } else {
        sequence.push(0); // <unknown>
    }
    Ok(())
}


/// Adds text content to the sequence by tokenizing each character
fn add_text_to_sequence(
    sequence: &mut Vec<u32>,
    text: &str,
    tokenizer: &BPETokenizer,
) {
    for c in text.chars() {
        let token = c.to_string();
        if let Some(&id) = tokenizer.token_to_id.get(&token) {
            sequence.push(id);
        } else {
            sequence.push(0); // <unknown>
        }
    }
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::bpe::{BPETokenizer, bpe_train_tokenize, bpe_get_special_tokens};
    use crate::sample::{
        Sample,
        SampleAiEnum,
        SampleCode,
        SampleLanguage,
        SampleLineBreak,
        SamplePromptEnum,
    };

    fn create_test_tokenizer() -> BPETokenizer {
        let special_tokens = bpe_get_special_tokens();
        
        // Start with all special tokens
        let mut vocab = special_tokens.clone();
        
        // Add all ASCII printable characters
        for c in ' '..='~' {
            let token = c.to_string();
            if !vocab.contains(&token) {
                vocab.push(token);
            }
        }
        
        // Add whitespace characters
        let whitespace = ['\n', '\t', '\r'];
        for c in whitespace {
            let token = c.to_string();
            if !vocab.contains(&token) {
                vocab.push(token);
            }
        }
        
        // Add Unicode characters that might appear
        let unicode_chars = ['é', 'á', 'í', 'ó', 'ú', 'ñ'];
        for c in unicode_chars {
            let token = c.to_string();
            if !vocab.contains(&token) {
                vocab.push(token);
            }
        }
        
        let initial_token_count = vocab.len() as u32;
        
        let mut token_to_id = HashMap::new();
        for (id, token) in vocab.iter().enumerate() {
            token_to_id.insert(token.clone(), id as u32);
        }
        
        BPETokenizer {
            vocab,
            token_to_id,
            merges: Vec::new(),
            special_token_count: special_tokens.len() as u32,
            initial_token_count,
        }
    }

    #[test]
    fn test_bpe_train_tokenize_basic() {
        let tokenizer = create_test_tokenizer();
        let samples = create_test_samples();
        
        let sequence = bpe_train_tokenize(&tokenizer, &samples).expect("Tokenizing failed");
        
        // Verify token IDs are valid
        for &id in &sequence {
            assert!(id < tokenizer.vocab.len() as u32, "Invalid token ID: {}", id);
        }
        
        // Verify the sequence contains the expected tokens
        let token_strings: Vec<String> = sequence.iter()
            .map(|&id| tokenizer.vocab[id as usize].clone())
            .collect();
        
        // Check for structural tags
        assert!(token_strings.contains(&"<sample>".to_string()));
        assert!(token_strings.contains(&"<prompt>".to_string()));
        assert!(token_strings.contains(&"</prompt>".to_string()));
        assert!(token_strings.contains(&"<ai>".to_string()));
        assert!(token_strings.contains(&"<text>".to_string()));
        assert!(token_strings.contains(&"</text>".to_string()));
        assert!(token_strings.contains(&"</ai>".to_string()));
        assert!(token_strings.contains(&"</sample>".to_string()));
        
        // Check for content characters
        assert!(token_strings.contains(&"H".to_string()));
        assert!(token_strings.contains(&"i".to_string()));
        assert!(token_strings.contains(&"W".to_string()));
        assert!(token_strings.contains(&"o".to_string()));
        assert!(token_strings.contains(&"r".to_string()));
        assert!(token_strings.contains(&"l".to_string()));
        assert!(token_strings.contains(&"d".to_string()));
    }

    #[test]
    fn test_bpe_train_tokenize_with_system_prompts() {
        let tokenizer = create_test_tokenizer();
        let samples = create_test_samples_with_system();
        
        let sequence = bpe_train_tokenize(&tokenizer, &samples).expect("Tokenizing failed");
        
        let token_strings: Vec<String> = sequence.iter()
            .map(|&id| tokenizer.vocab[id as usize].clone())
            .collect();
        
        // Build the full string from tokens to verify content
        let full_text: String = token_strings.concat();
        
        // Verify system prompt content appears
        assert!(full_text.contains("You are a helpful assistant.\n"), "System prompt content not found");
        
        // Verify regular prompt tokens are present
        assert!(full_text.contains("Hi"));
        assert!(full_text.contains("World"));
        
        // Verify the structure has system tag
        let system_open_count = token_strings.iter().filter(|&t| t == "<system>").count();
        let system_close_count = token_strings.iter().filter(|&t| t == "</system>").count();
        
        assert_eq!(system_open_count, 1, "Should have exactly one <system> tag");
        assert_eq!(system_close_count, 1, "Should have exactly one </system> tag");
    }

    #[test]
    fn test_bpe_train_tokenize_with_code() {
        let tokenizer = create_test_tokenizer();
        
        let samples = vec![
            Sample {
                system: String::new(),
                prompt_section: vec![
                    SamplePromptEnum::Code(SampleCode {
                        lang: SampleLanguage::Js,
                        inline: false,
                        indent: None,
                        content: "console.log()".to_string(),
                    }),
                ],
                ai_section: vec![],
            },
        ];
        
        let sequence = bpe_train_tokenize(&tokenizer, &samples).expect("Tokenizing failed");
        
        let token_strings: Vec<String> = sequence.iter()
            .map(|&id| tokenizer.vocab[id as usize].clone())
            .collect();
        
        // Should have code tags
        assert!(token_strings.contains(&"<js>".to_string()));
        assert!(token_strings.contains(&"</js>".to_string()));
        
        // Verify code characters are present
        let code_chars = ['c', 'o', 'n', 's', 'l', 'e', '.', 'g', '(', ')'];
        for c in code_chars {
            assert!(token_strings.contains(&c.to_string()));
        }
    }

    #[test]
    fn test_bpe_train_tokenize_with_line_break() {
        let tokenizer = create_test_tokenizer();
        
        let samples = vec![
            Sample {
                system: String::new(),
                prompt_section: vec![
                    SamplePromptEnum::Text("Line1".to_string()),
                    SamplePromptEnum::LineBreak(SampleLineBreak { count: 1 }),
                    SamplePromptEnum::Text("Line2".to_string()),
                ],
                ai_section: vec![],
            },
        ];
        
        let sequence = bpe_train_tokenize(&tokenizer, &samples).expect("Tokenizing failed");
        
        let token_strings: Vec<String> = sequence.iter()
            .map(|&id| tokenizer.vocab[id as usize].clone())
            .collect();
        
        assert!(token_strings.contains(&"<line-break />".to_string()));
    }

    #[test]
    fn test_bpe_train_tokenize_with_source() {
        let tokenizer = create_test_tokenizer();
        
        let samples = vec![
            Sample {
                system: String::new(),
                prompt_section: vec![
                    SamplePromptEnum::Text("Source test".to_string()),
                ],
                ai_section: vec![
                    SampleAiEnum::Source("wikipedia".to_string()),
                ],
            },
        ];
        
        let sequence = bpe_train_tokenize(&tokenizer, &samples).expect("Tokenizing failed");
        
        let token_strings: Vec<String> = sequence.iter()
            .map(|&id| tokenizer.vocab[id as usize].clone())
            .collect();
        
        assert!(token_strings.contains(&"<source>".to_string()));
        assert!(token_strings.contains(&"</source>".to_string()));
        
        // Verify source text characters are present
        let source_chars = ['w', 'i', 'k', 'p', 'e', 'd', 'a'];
        for c in source_chars {
            assert!(token_strings.contains(&c.to_string()), "Character '{}' not found", c);
        }
    }

    #[test]
    fn test_bpe_train_tokenize_multiple_samples() {
        let tokenizer = create_test_tokenizer();
        
        let samples = vec![
            Sample {
                system: String::new(),
                prompt_section: vec![SamplePromptEnum::Text("First".to_string())],
                ai_section: vec![SampleAiEnum::Text("Response1".to_string())],
            },
            Sample {
                system: String::new(),
                prompt_section: vec![SamplePromptEnum::Text("Second".to_string())],
                ai_section: vec![SampleAiEnum::Text("Response2".to_string())],
            },
        ];
        
        let sequence = bpe_train_tokenize(&tokenizer, &samples).expect("Tokenizing failed");
        
        let token_strings: Vec<String> = sequence.iter()
            .map(|&id| tokenizer.vocab[id as usize].clone())
            .collect();
        
        // Verify both samples are present (2 sample tags)
        let sample_open_count = token_strings.iter().filter(|&t| t == "<sample>").count();
        let sample_close_count = token_strings.iter().filter(|&t| t == "</sample>").count();
        assert_eq!(sample_open_count, 2);
        assert_eq!(sample_close_count, 2);
        
        // Check for characters from "First" and "Second"
        assert!(token_strings.contains(&"F".to_string()));
        assert!(token_strings.contains(&"i".to_string()));
        assert!(token_strings.contains(&"r".to_string()));
        assert!(token_strings.contains(&"s".to_string()));
        assert!(token_strings.contains(&"t".to_string()));
        
        assert!(token_strings.contains(&"S".to_string()));
        assert!(token_strings.contains(&"e".to_string()));
        assert!(token_strings.contains(&"c".to_string()));
        assert!(token_strings.contains(&"o".to_string()));
        assert!(token_strings.contains(&"n".to_string()));
        assert!(token_strings.contains(&"d".to_string()));
    }

    // Helper functions to create test samples
    fn create_test_samples() -> Vec<Sample> {
        vec![
            Sample {
                system: String::new(),
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
                system: "You are a helpful assistant.\n".to_string(),
                prompt_section: vec![
                    SamplePromptEnum::Text("Hi".to_string()),
                ],
                ai_section: vec![
                    SampleAiEnum::Text("World".to_string()),
                ],
            },
        ]
    }
}
