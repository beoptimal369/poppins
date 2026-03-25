// src/tag/tag_get_byte_offset_last_ai_start.rs

use crate::sample::SampleAiEnum;
use crate::tag::{
    tag_write_tag,
    tag_write_code_open,
    tag_write_line_break,
    tag_get_byte_length_ai_item,
};


/// Calculates the byte offset where the last AI response begins within a sample
///
/// This function calculates the exact byte offset from the start of the sample to the
/// beginning of the last AI response content (the first byte after the opening tags).
/// It's used when building the index file to know where the target training tokens begin.
///
/// # Arguments
/// * `ai_section` - Slice of AI items (the complete AI section)
/// * `special_tokens` - Slice of special tokens for tag lookup
///
/// # Returns
/// * `usize` - Byte offset from the start of the sample to the beginning of the last AI response
pub fn tag_get_byte_offset_last_ai_start(
    ai_section: &[SampleAiEnum],
    special_tokens: &[String],
) -> usize {
    let mut offset = 0;
    
    // Add the opening <ai> tag
    offset += get_tag_byte_length("ai", true, special_tokens);
    
    // Add all items before the last one
    if ai_section.len() > 1 {
        for item in &ai_section[..ai_section.len() - 1] {
            offset += tag_get_byte_length_ai_item(item, special_tokens);
        }
    }
    
    // Add the opening tag(s) of the last item
    if let Some(last_item) = ai_section.last() {
        offset += get_ai_item_opening_byte_length(last_item, special_tokens);
    }
    
    offset
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

/// Gets the byte length of the opening part of an AI item
fn get_ai_item_opening_byte_length(
    item: &SampleAiEnum,
    special_tokens: &[String],
) -> usize {
    let mut buffer = Vec::new();
    
    match item {
        SampleAiEnum::Text(_) => {
            tag_write_tag(&mut buffer, "text", true, special_tokens).unwrap();
        }
        SampleAiEnum::Source(_) => {
            tag_write_tag(&mut buffer, "source", true, special_tokens).unwrap();
        }
        SampleAiEnum::Code(code) => {
            tag_write_code_open(
                &mut buffer,
                code.lang.as_str(),
                code.inline,
                code.indent,
                special_tokens,
            ).unwrap();
        }
        SampleAiEnum::LineBreak(line_break) => {
            tag_write_line_break(&mut buffer, line_break, special_tokens).unwrap();
        }
    }
    
    buffer.len()
}



#[cfg(test)]
mod tests {
    use super::tag_get_byte_offset_last_ai_start;
    use crate::bpe::bpe_get_special_tokens;
    use crate::sample::{
        SampleCode,
        SampleAiEnum,
        SampleIndent,
        SampleLanguage,
        SampleLineBreak,
    };

    #[test]
    fn test_last_ai_start_single_text() {
        let special_tokens = bpe_get_special_tokens();
        
        let ai_section = vec![
            SampleAiEnum::Text("Response".to_string()),
        ];
        
        let offset = tag_get_byte_offset_last_ai_start(&ai_section, &special_tokens);
        
        // <ai> + <text> = 4 + 6 = 10
        assert_eq!(offset, 10);
    }
    
    #[test]
    fn test_last_ai_start_single_source() {
        let special_tokens = bpe_get_special_tokens();
        
        let ai_section = vec![
            SampleAiEnum::Source("1".to_string()),
        ];
        
        let offset = tag_get_byte_offset_last_ai_start(&ai_section, &special_tokens);
        
        // <ai> + <source> = 4 + 8 = 12
        assert_eq!(offset, 12);
    }
    
    #[test]
    fn test_last_ai_start_single_code() {
        let special_tokens = bpe_get_special_tokens();
        
        let ai_section = vec![
            SampleAiEnum::Code(SampleCode {
                lang: SampleLanguage::Js,
                inline: false,
                indent: SampleIndent::Zero,
                content: "code".to_string(),
            }),
        ];
        
        let offset = tag_get_byte_offset_last_ai_start(&ai_section, &special_tokens);
        
        // <ai> + <js> = 4 + 4 = 8
        assert_eq!(offset, 8);
    }
    
    #[test]
    fn test_last_ai_start_multiple_items() {
        let special_tokens = bpe_get_special_tokens();
        
        let ai_section = vec![
            SampleAiEnum::Text("First".to_string()),
            SampleAiEnum::Source("ref".to_string()),
        ];
        
        let offset = tag_get_byte_offset_last_ai_start(&ai_section, &special_tokens);
        
        // <ai> (4) + <text>First</text> (6+5+7=18) + <source> (8) = 30
        assert_eq!(offset, 30);
    }
    
    #[test]
    fn test_last_ai_start_with_line_break() {
        let special_tokens = bpe_get_special_tokens();
        
        let ai_section = vec![
            SampleAiEnum::Text("First".to_string()),
            SampleAiEnum::LineBreak(SampleLineBreak { count: 1 }),
        ];
        
        let offset = tag_get_byte_offset_last_ai_start(&ai_section, &special_tokens);
        
        // <ai> (4) + <text>First</text> (6+5+7=18) + <line-break /> (14) = 36
        assert_eq!(offset, 36);
    }
}
