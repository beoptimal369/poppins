// src/bpe/bpe_train_tokenize.rs

use crate::bpe::BPETokenizer;
use crate::sample::{Sample, SamplePromptEnum, SampleAiEnum};
use crate::tag::{
    tag_write_tag,
    tag_write_code_open,
    tag_write_code_close,
    tag_write_line_break,
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
/// * `Vec<u32>` - Sequence of token IDs
pub fn bpe_train_tokenize(tokenizer: &BPETokenizer, samples: &[Sample]) -> Vec<u32> {
    let mut sequence = Vec::new();
    
    // Get special tokens for tag writing
    let special_tokens: Vec<String> = tokenizer.vocab[..tokenizer.special_token_count as usize].to_vec();
    
    for sample in samples {
        // Add <sample> tag
        add_tag_to_sequence(&mut sequence, "sample", true, &special_tokens, tokenizer);
        
        // Process prompt section
        for item in &sample.prompt_section {
            match item {
                SamplePromptEnum::Text(text) => {
                    add_tag_to_sequence(&mut sequence, "prompt", true, &special_tokens, tokenizer);
                    add_text_to_sequence(&mut sequence, text, tokenizer);
                    add_tag_to_sequence(&mut sequence, "prompt", false, &special_tokens, tokenizer);
                }
                SamplePromptEnum::Code(code) => {
                    // Add opening code tag
                    add_code_open_to_sequence(&mut sequence, code.lang.as_str(), code.inline, code.indent, &special_tokens, tokenizer);
                    // Add code content character by character
                    add_text_to_sequence(&mut sequence, &code.content, tokenizer);
                    // Add closing code tag
                    add_code_close_to_sequence(&mut sequence, code.lang.as_str(), &special_tokens, tokenizer);
                }
                SamplePromptEnum::LineBreak(lb) => {
                    add_line_break_to_sequence(&mut sequence, lb, &special_tokens, tokenizer);
                }
            }
        }
        
        // Add <ai> tag
        add_tag_to_sequence(&mut sequence, "ai", true, &special_tokens, tokenizer);
        
        // Process AI section
        for item in &sample.ai_section {
            match item {
                SampleAiEnum::Text(text) => {
                    add_tag_to_sequence(&mut sequence, "text", true, &special_tokens, tokenizer);
                    add_text_to_sequence(&mut sequence, &text, tokenizer);
                    add_tag_to_sequence(&mut sequence, "text", false, &special_tokens, tokenizer);
                }
                SampleAiEnum::Source(source) => {
                    add_tag_to_sequence(&mut sequence, "source", true, &special_tokens, tokenizer);
                    add_text_to_sequence(&mut sequence, &source, tokenizer);
                    add_tag_to_sequence(&mut sequence, "source", false, &special_tokens, tokenizer);
                }
                SampleAiEnum::Code(code) => {
                    add_code_open_to_sequence(&mut sequence, code.lang.as_str(), code.inline, code.indent, &special_tokens, tokenizer);
                    add_text_to_sequence(&mut sequence, &code.content, tokenizer);
                    add_code_close_to_sequence(&mut sequence, code.lang.as_str(), &special_tokens, tokenizer);
                }
                SampleAiEnum::LineBreak(lb) => {
                    add_line_break_to_sequence(&mut sequence, lb, &special_tokens, tokenizer);
                }
            }
        }
        
        // Add </ai> tag
        add_tag_to_sequence(&mut sequence, "ai", false, &special_tokens, tokenizer);
        
        // Add </sample> tag
        add_tag_to_sequence(&mut sequence, "sample", false, &special_tokens, tokenizer);
    }
    
    sequence
}

/// Adds a simple tag to the sequence
fn add_tag_to_sequence(
    sequence: &mut Vec<u32>,
    tag_name: &str,
    is_opening: bool,
    special_tokens: &[String],
    tokenizer: &BPETokenizer,
) {
    let mut buffer = Vec::new();
    tag_write_tag(&mut buffer, tag_name, is_opening, special_tokens).unwrap();
    let tag_string = String::from_utf8(buffer).unwrap();
    
    if let Some(&id) = tokenizer.token_to_id.get(&tag_string) {
        sequence.push(id);
    } else {
        sequence.push(0); // <unknown>
    }
}

/// Adds a code opening tag to the sequence
fn add_code_open_to_sequence(
    sequence: &mut Vec<u32>,
    lang: &str,
    inline: bool,
    indent: crate::sample::SampleIndent,
    special_tokens: &[String],
    tokenizer: &BPETokenizer,
) {
    let mut buffer = Vec::new();
    tag_write_code_open(&mut buffer, lang, inline, indent, special_tokens).unwrap();
    let tag_string = String::from_utf8(buffer).unwrap();
    
    if let Some(&id) = tokenizer.token_to_id.get(&tag_string) {
        sequence.push(id);
    } else {
        sequence.push(0); // <unknown>
    }
}

/// Adds a code closing tag to the sequence
fn add_code_close_to_sequence(
    sequence: &mut Vec<u32>,
    lang: &str,
    special_tokens: &[String],
    tokenizer: &BPETokenizer,
) {
    let mut buffer = Vec::new();
    tag_write_code_close(&mut buffer, lang, special_tokens).unwrap();
    let tag_string = String::from_utf8(buffer).unwrap();
    
    if let Some(&id) = tokenizer.token_to_id.get(&tag_string) {
        sequence.push(id);
    } else {
        sequence.push(0); // <unknown>
    }
}

/// Adds a line break tag to the sequence
fn add_line_break_to_sequence(
    sequence: &mut Vec<u32>,
    line_break: &crate::sample::SampleLineBreak,
    special_tokens: &[String],
    tokenizer: &BPETokenizer,
) {
    let mut buffer = Vec::new();
    tag_write_line_break(&mut buffer, line_break, special_tokens).unwrap();
    let tag_string = String::from_utf8(buffer).unwrap();
    
    if let Some(&id) = tokenizer.token_to_id.get(&tag_string) {
        sequence.push(id);
    } else {
        sequence.push(0); // <unknown>
    }
}

/// Adds text content to the sequence, splitting into individual characters
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
