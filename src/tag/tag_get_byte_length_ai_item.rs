// src/tag/tag_get_byte_length_ai_item.rs

use crate::sample::{SampleAiEnum, SampleLineBreak, SampleIndent};
use crate::tag::{tag_write_tag, tag_write_code_open, tag_write_code_close, tag_write_line_break};


/// Calculates the byte length of an AI item without writing it
///
/// This function provides byte-accurate length calculations for AI items,
/// matching exactly what would be written by the corresponding tag writing functions.
/// It's used for calculating offsets in index files without actually writing the content.
///
/// # Arguments
/// * `item` - The AI item to calculate byte length for
/// * `special_tokens` - Slice of special tokens for tag lookup
///
/// # Returns
/// * `usize` - The exact number of bytes that would be written
pub fn tag_get_byte_length_ai_item(
    item: &SampleAiEnum,
    special_tokens: &[String],
) -> usize {
    match item {
        SampleAiEnum::Text(text) => {
            let mut length = 0;
            
            // Opening <text> tag
            length += get_tag_byte_length("text", true, special_tokens);
            
            // Content
            length += text.len();
            
            // Closing </text> tag
            length += get_tag_byte_length("text", false, special_tokens);
            
            length
        }
        SampleAiEnum::Source(source) => {
            let mut length = 0;
            
            // Opening <source> tag
            length += get_tag_byte_length("source", true, special_tokens);
            
            // Content (source ID)
            length += source.len();
            
            // Closing </source> tag
            length += get_tag_byte_length("source", false, special_tokens);
            
            length
        }
        SampleAiEnum::Code(code) => {
            let mut length = 0;
            
            // Opening code tag
            length += get_code_open_byte_length(code.lang.as_str(), code.inline, code.indent, special_tokens);
            
            // Content
            length += code.content.len();
            
            // Closing code tag
            length += get_code_close_byte_length(code.lang.as_str(), special_tokens);
            
            length
        }
        SampleAiEnum::LineBreak(line_break) => {
            get_line_break_byte_length(line_break, special_tokens)
        }
    }
}


/// Gets the byte length of a simple tag using tag_write_tag
fn get_tag_byte_length(
    tag_name: &str,
    is_opening: bool,
    special_tokens: &[String],
) -> usize {
    let mut buffer = Vec::new();
    tag_write_tag(&mut buffer, tag_name, is_opening, special_tokens).unwrap();
    buffer.len()
}

/// Gets the byte length of a code opening tag using tag_write_code_open
fn get_code_open_byte_length(
    lang: &str,
    inline: bool,
    indent: SampleIndent,
    special_tokens: &[String],
) -> usize {
    let mut buffer = Vec::new();
    tag_write_code_open(&mut buffer, lang, inline, indent, special_tokens).unwrap();
    buffer.len()
}

/// Gets the byte length of a code closing tag using tag_write_code_close
fn get_code_close_byte_length(
    lang: &str,
    special_tokens: &[String],
) -> usize {
    let mut buffer = Vec::new();
    tag_write_code_close(&mut buffer, lang, special_tokens).unwrap();
    buffer.len()
}

/// Gets the byte length of a line break tag using tag_write_line_break
fn get_line_break_byte_length(
    line_break: &SampleLineBreak,
    special_tokens: &[String],
) -> usize {
    let mut buffer = Vec::new();
    tag_write_line_break(&mut buffer, line_break, special_tokens).unwrap();
    buffer.len()
}




#[cfg(test)]
mod tests {
    use super::tag_get_byte_length_ai_item;
    use crate::sample::{SampleLanguage, SampleIndent, SampleCode, SampleAiEnum};

    #[test]
    fn test_text_byte_length() {
        let special_tokens: Vec<String> = vec![
            "<text>".to_string(),
            "</text>".to_string(),
        ];
        
        let text = SampleAiEnum::Text("Hello".to_string());
        
        assert_eq!(tag_get_byte_length_ai_item(&text, &special_tokens), 18);
    }
    
    #[test]
    fn test_source_byte_length() {
        let special_tokens: Vec<String> = vec![
            "<source>".to_string(),
            "</source>".to_string(),
        ];
        
        let source = SampleAiEnum::Source("123".to_string());
        
        assert_eq!(tag_get_byte_length_ai_item(&source, &special_tokens), 20);
    }
    
    #[test]
    fn test_code_byte_length() {
        let special_tokens: Vec<String> = vec![
            "<js>".to_string(),
            "</js>".to_string(),
        ];
        
        let code = SampleAiEnum::Code(SampleCode {
            lang: SampleLanguage::Js,
            inline: false,
            indent: SampleIndent::Zero,
            content: "test".to_string(),
        });
        
        assert_eq!(tag_get_byte_length_ai_item(&code, &special_tokens), 13);
    }
}
