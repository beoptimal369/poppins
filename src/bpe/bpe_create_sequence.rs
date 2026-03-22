// src/bpe/bpe_create_sequence.rs

use crate::bpe::BPETokenizer;
use crate::sample::{Sample, SamplePromptEnum, SampleAiEnum};


/// Convert structured samples to initial token sequence
///
/// This function walks through each sample and converts all content to token IDs:
/// - Special tags are looked up in the vocabulary
/// - Text content is split into individual characters
/// - Code content preserves all whitespace and is split into individual characters
///
/// # Arguments
/// * `tokenizer` - Reference to the tokenizer with initialized vocabulary
/// * `samples` - Slice of samples to convert
///
/// # Returns
/// * `Vec<u32>` - Sequence of token IDs
pub fn bpe_create_sequence(tokenizer: &BPETokenizer, samples: &[Sample]) -> Vec<u32> {
    let mut sequence = Vec::new();
    
    for sample in samples {
        // Add <sample> tag
        if let Some(&id) = tokenizer.token_to_id.get("<sample>") {
            sequence.push(id);
        }
        
        // Process prompt section
        for item in &sample.prompt_section {
            match item {
                SamplePromptEnum::Text(text) => {
                    if let Some(&id) = tokenizer.token_to_id.get("<prompt>") {
                        sequence.push(id);
                    }
                    // Add each character of the prompt text
                    for c in text.chars() {
                        let token = c.to_string();
                        if let Some(&id) = tokenizer.token_to_id.get(&token) {
                            sequence.push(id);
                        } else {
                            sequence.push(0); // <unknown>
                        }
                    }
                    if let Some(&id) = tokenizer.token_to_id.get("</prompt>") {
                        sequence.push(id);
                    }
                }
                SamplePromptEnum::Code(code) => {
                    let lang_tag = code.lang.as_str();
                    
                    // Build opening tag
                    let open_tag = if code.inline {
                        format!("<{} inline=\"true\">", lang_tag)
                    } else if code.indent.as_u8() > 0 {
                        format!("<{} indent=\"{}\">", lang_tag, code.indent.as_u8())
                    } else {
                        format!("<{}>", lang_tag)
                    };
                    
                    if let Some(&id) = tokenizer.token_to_id.get(&open_tag) {
                        sequence.push(id);
                    }
                    
                    // Add code content character by character (preserving all whitespace)
                    for c in code.content.chars() {
                        let token = c.to_string();
                        if let Some(&id) = tokenizer.token_to_id.get(&token) {
                            sequence.push(id);
                        } else {
                            sequence.push(0);
                        }
                    }
                    
                    let close_tag = format!("</{}>", lang_tag);
                    if let Some(&id) = tokenizer.token_to_id.get(&close_tag) {
                        sequence.push(id);
                    }
                }
                SamplePromptEnum::LineBreak(lb) => {
                    if lb.count == 1 {
                        if let Some(&id) = tokenizer.token_to_id.get("<line-break />") {
                            sequence.push(id);
                        }
                    } else {
                        let tag = format!("<line-break count=\"{}\" />", lb.count);
                        if let Some(&id) = tokenizer.token_to_id.get(&tag) {
                            sequence.push(id);
                        }
                    }
                }
            }
        }
        
        // Add <ai> tag
        if let Some(&id) = tokenizer.token_to_id.get("<ai>") {
            sequence.push(id);
        }
        
        // Process AI section
        for item in &sample.ai_section {
            match item {
                SampleAiEnum::Text(text) => {
                    if let Some(&id) = tokenizer.token_to_id.get("<text>") {
                        sequence.push(id);
                    }
                    for c in text.content.chars() {
                        let token = c.to_string();
                        if let Some(&id) = tokenizer.token_to_id.get(&token) {
                            sequence.push(id);
                        } else {
                            sequence.push(0);
                        }
                    }
                    if let Some(&id) = tokenizer.token_to_id.get("</text>") {
                        sequence.push(id);
                    }
                }
                SampleAiEnum::Source(source) => {
                    if let Some(&id) = tokenizer.token_to_id.get("<source>") {
                        sequence.push(id);
                    }
                    for c in source.id.chars() {
                        let token = c.to_string();
                        if let Some(&id) = tokenizer.token_to_id.get(&token) {
                            sequence.push(id);
                        } else {
                            sequence.push(0);
                        }
                    }
                    if let Some(&id) = tokenizer.token_to_id.get("</source>") {
                        sequence.push(id);
                    }
                }
                SampleAiEnum::LineBreak(lb) => {
                    if lb.count == 1 {
                        if let Some(&id) = tokenizer.token_to_id.get("<line-break />") {
                            sequence.push(id);
                        }
                    } else {
                        let tag = format!("<line-break count=\"{}\" />", lb.count);
                        if let Some(&id) = tokenizer.token_to_id.get(&tag) {
                            sequence.push(id);
                        }
                    }
                }
                SampleAiEnum::Code(code) => {
                    let lang_tag = code.lang.as_str();
                    
                    // Build opening tag
                    let open_tag = if code.inline {
                        format!("<{} inline=\"true\">", lang_tag)
                    } else if code.indent.as_u8() > 0 {
                        format!("<{} indent=\"{}\">", lang_tag, code.indent.as_u8())
                    } else {
                        format!("<{}>", lang_tag)
                    };
                    
                    if let Some(&id) = tokenizer.token_to_id.get(&open_tag) {
                        sequence.push(id);
                    }
                    
                    // Add code content character by character
                    for c in code.content.chars() {
                        let token = c.to_string();
                        if let Some(&id) = tokenizer.token_to_id.get(&token) {
                            sequence.push(id);
                        } else {
                            sequence.push(0);
                        }
                    }
                    
                    let close_tag = format!("</{}>", lang_tag);
                    if let Some(&id) = tokenizer.token_to_id.get(&close_tag) {
                        sequence.push(id);
                    }
                }
            }
        }
        
        // Add </ai> tag
        if let Some(&id) = tokenizer.token_to_id.get("</ai>") {
            sequence.push(id);
        }
        
        // Add </sample> tag
        if let Some(&id) = tokenizer.token_to_id.get("</sample>") {
            sequence.push(id);
        }
    }
    
    sequence
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::bpe::{BPETokenizer, bpe_create_sequence, bpe_get_special_tokens};
    use crate::sample::{
        Sample,
        SamplePromptEnum,
        SampleAiEnum,
        SampleText,
        SampleSource,
        SampleLineBreak,
        SampleAiCode,
        SampleLanguage,
        SampleIndent,
        SampleTokenStats,
    };

    fn create_test_tokenizer() -> BPETokenizer {
        let mut token_to_id = HashMap::new();
        
        // Get all special tokens
        let special_tokens = bpe_get_special_tokens();
        
        // Add all special tokens
        for (id, token) in special_tokens.iter().enumerate() {
            token_to_id.insert(token.clone(), id as u32);
        }
        
        // Add characters that appear in the test
        let chars = vec![
            'W', 'h', 'a', 't', ' ', 'i', 's', '?', 'D', 'e', 'f', 'n', 'c', 'o', 'm', 'p', 'u', 't', 'r',
            '.', ':', '/', 'J', 'v', 'S', 'k', 'l', 'g', 'h', 'y', '(', ')', '{', '}', '\'', '\n', '1', '2', '3',
        ];
        let base_id = special_tokens.len() as u32;
        for (offset, c) in chars.iter().enumerate() {
            token_to_id.insert(c.to_string(), base_id + offset as u32);
        }
        
        let mut vocab = special_tokens;
        let vocab_len = vocab.len() + chars.len(); // Calculate length before extending
        vocab.extend(chars.iter().map(|c| c.to_string()));
        
        BPETokenizer {
            vocab,
            token_to_id,
            merges: Vec::new(),
            special_token_count: vocab_len as u32,
        }
    }

    fn create_test_sample() -> Sample {
        Sample {
            id: "1".to_string(),
            prompt_section: vec![
                SamplePromptEnum::Text("What is?".to_string()),
            ],
            ai_section: vec![
                SampleAiEnum::Text(SampleText {
                    content: "A computer is a computing / information processing device.".to_string(),
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
                SampleAiEnum::LineBreak(SampleLineBreak { count: 1 }),
                SampleAiEnum::Text(SampleText {
                    content: "Example Code:".to_string(),
                    token_stats: SampleTokenStats {
                        weight_decay: 0.1,
                        dropout: 0.05,
                        loss_scale: 1.0,
                        gradient_scale: 1.0,
                        gradient_clip: 1.0,
                    },
                }),
                SampleAiEnum::Source(SampleSource {
                    id: "3".to_string(),
                    token_stats: SampleTokenStats {
                        weight_decay: 0.01,
                        dropout: 0.0,
                        loss_scale: 0.2,
                        gradient_scale: 2.0,
                        gradient_clip: 0.1,
                    },
                }),
                SampleAiEnum::Code(SampleAiCode {
                    lang: SampleLanguage::Ts,
                    inline: false,
                    indent: SampleIndent::Zero,
                    content: "function example() {\n  console.log('hi world')\n}".to_string(),
                    token_stats: SampleTokenStats {
                        weight_decay: 0.05,
                        dropout: 0.1,
                        loss_scale: 1.0,
                        gradient_scale: 1.2,
                        gradient_clip: 0.7,
                    },
                }),
            ],
        }
    }

    #[test]
    fn test_create_sequence_comprehensive() {
        let tokenizer = create_test_tokenizer();
        let sample = create_test_sample();
        let samples = vec![sample];
        
        let sequence = bpe_create_sequence(&tokenizer, &samples);
        
        // Verify all tag types are present
        assert!(sequence.contains(tokenizer.token_to_id.get("<sample>").unwrap()));
        assert!(sequence.contains(tokenizer.token_to_id.get("</sample>").unwrap()));
        assert!(sequence.contains(tokenizer.token_to_id.get("<prompt>").unwrap()));
        assert!(sequence.contains(tokenizer.token_to_id.get("</prompt>").unwrap()));
        assert!(sequence.contains(tokenizer.token_to_id.get("<ai>").unwrap()));
        assert!(sequence.contains(tokenizer.token_to_id.get("</ai>").unwrap()));
        assert!(sequence.contains(tokenizer.token_to_id.get("<text>").unwrap()));
        assert!(sequence.contains(tokenizer.token_to_id.get("</text>").unwrap()));
        assert!(sequence.contains(tokenizer.token_to_id.get("<source>").unwrap()));
        assert!(sequence.contains(tokenizer.token_to_id.get("</source>").unwrap()));
        assert!(sequence.contains(tokenizer.token_to_id.get("<line-break />").unwrap()));
        assert!(sequence.contains(tokenizer.token_to_id.get("<ts>").unwrap()));
        assert!(sequence.contains(tokenizer.token_to_id.get("</ts>").unwrap()));
        
        // Verify text content characters
        let text_chars = ['W', 'h', 'a', 't', ' ', 'i', 's', '?', 'A', 'c', 'o', 'm', 'p', 'u', 't', 'r', 'd', 'v', 'e', '.', '/', 'E', 'x', 'l', ':'];
        for c in text_chars {
            let id = tokenizer.token_to_id.get(&c.to_string());
            if let Some(id) = id {
                assert!(sequence.contains(id), "Character '{}' not found in sequence", c);
            }
        }
        
        // Verify code content characters (including newlines)
        let code_chars = ['f', 'u', 'n', 'c', 't', 'i', 'o', 'n', '(', ')', ' ', '{', '\n', ' ', 'c', 'o', 'n', 's', 'o', 'l', 'e', '.', 'l', 'o', 'g', '\'', 'h', 'i', 'w', 'r', 'l', 'd', '\'', ')', '}'];
        for c in code_chars {
            let id = tokenizer.token_to_id.get(&c.to_string());
            if let Some(id) = id {
                assert!(sequence.contains(id), "Character '{}' not found in sequence", c);
            }
        }
        
        // Verify source IDs
        let one_id = tokenizer.token_to_id.get("1").unwrap();
        let three_id = tokenizer.token_to_id.get("3").unwrap();
        assert!(sequence.contains(one_id));
        assert!(sequence.contains(three_id));
        
        // Verify order: <sample> before <prompt> before <ai> before </ai> before </sample>
        let sample_open_pos = sequence.iter().position(|&id| id == *tokenizer.token_to_id.get("<sample>").unwrap()).unwrap();
        let prompt_open_pos = sequence.iter().position(|&id| id == *tokenizer.token_to_id.get("<prompt>").unwrap()).unwrap();
        let ai_open_pos = sequence.iter().position(|&id| id == *tokenizer.token_to_id.get("<ai>").unwrap()).unwrap();
        let ai_close_pos = sequence.iter().position(|&id| id == *tokenizer.token_to_id.get("</ai>").unwrap()).unwrap();
        let sample_close_pos = sequence.iter().position(|&id| id == *tokenizer.token_to_id.get("</sample>").unwrap()).unwrap();
        
        assert!(sample_open_pos < prompt_open_pos);
        assert!(prompt_open_pos < ai_open_pos);
        assert!(ai_open_pos < ai_close_pos);
        assert!(ai_close_pos < sample_close_pos);
        
        // Verify we have multiple text sections
        let text_open_id = *tokenizer.token_to_id.get("<text>").unwrap();
        let text_open_positions: Vec<_> = sequence.iter().enumerate()
            .filter(|(_, id)| **id == text_open_id)
            .map(|(pos, _)| pos)
            .collect();
        assert!(text_open_positions.len() >= 2, "Expected at least 2 <text> tags");
        
        // Verify we have multiple source sections
        let source_open_id = *tokenizer.token_to_id.get("<source>").unwrap();
        let source_open_positions: Vec<_> = sequence.iter().enumerate()
            .filter(|(_, id)| **id == source_open_id)
            .map(|(pos, _)| pos)
            .collect();
        assert!(source_open_positions.len() >= 2, "Expected at least 2 <source> tags");
        
        // Verify line break is present
        let lb_id = tokenizer.token_to_id.get("<line-break />").unwrap();
        assert!(sequence.contains(lb_id));
        
        // Verify code block is present with content
        let ts_open_id = *tokenizer.token_to_id.get("<ts>").unwrap();
        let ts_close_id = *tokenizer.token_to_id.get("</ts>").unwrap();
        let ts_open_pos = sequence.iter().position(|&id| id == ts_open_id).unwrap();
        let ts_close_pos = sequence.iter().position(|&id| id == ts_close_id).unwrap();
        assert!(ts_open_pos < ts_close_pos);
        
        // Verify sequence length is reasonable
        assert!(sequence.len() > 100, "Sequence length should be > 100, got {}", sequence.len());
    }
}
