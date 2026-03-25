// src/tag/tag_write_line_break.rs

use std::io::{Write, Result};
use crate::sample::SampleLineBreak;


/// Writes a line break tag to the provided writer without formatting
///
/// This function writes the raw line break tag bytes without any pretty-printing spaces or newlines.
/// The tag format depends on the count:
/// - For count == 1: `<line-break />`
/// - For count >= 2: `<line-break count="{count}" />`
///
/// The tag is looked up from special_tokens to ensure consistent byte counts for binary file generation.
///
/// # Arguments
/// * `writer` - Mutable reference to a type that implements `Write`
/// * `line_break` - The line break configuration
/// * `special_tokens` - Slice of special tokens for tag lookup
///
/// # Returns
/// * `Result<()>` - Returns `Ok(())` on success, or an I/O error if writing fails
pub fn tag_write_line_break<W: Write>(
    writer: &mut W,
    line_break: &SampleLineBreak,
    special_tokens: &[String],
) -> Result<()> {
    let tag = if line_break.count >= 2 {
        format!("<line-break count=\"{}\" />", line_break.count)
    } else {
        "<line-break />".to_string()
    };
    
    // Try to use the special token version first for consistency
    if let Some(tag_token) = special_tokens.iter().find(|t| **t == tag) {
        writer.write_all(tag_token.as_bytes())
    } else {
        writer.write_all(tag.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_tag_write_line_break_single() {
        let special_tokens: Vec<String> = vec![
            "<line-break />".to_string(),
        ];
        
        let line_break = SampleLineBreak { count: 1 };
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_line_break(&mut buffer, &line_break, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<line-break />");
    }
    
    #[test]
    fn test_tag_write_line_break_double() {
        let special_tokens: Vec<String> = vec![
            "<line-break count=\"2\" />".to_string(),
        ];
        
        let line_break = SampleLineBreak { count: 2 };
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_line_break(&mut buffer, &line_break, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<line-break count=\"2\" />");
    }
    
    #[test]
    fn test_tag_write_line_break_triple() {
        let special_tokens: Vec<String> = vec![
            "<line-break count=\"3\" />".to_string(),
        ];
        
        let line_break = SampleLineBreak { count: 3 };
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_line_break(&mut buffer, &line_break, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<line-break count=\"3\" />");
    }
    
    #[test]
    fn test_tag_write_line_break_count_4() {
        let special_tokens: Vec<String> = vec![
            "<line-break count=\"4\" />".to_string(),
        ];
        
        let line_break = SampleLineBreak { count: 4 };
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_line_break(&mut buffer, &line_break, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<line-break count=\"4\" />");
    }
    
    #[test]
    fn test_tag_write_line_break_all_counts() {
        let special_tokens: Vec<String> = vec![];
        
        // Test counts from 1 to 10
        for count in 1..=10 {
            let line_break = SampleLineBreak { count };
            let mut buffer = Cursor::new(Vec::<u8>::new());
            
            tag_write_line_break(&mut buffer, &line_break, &special_tokens).unwrap();
            
            let expected = if count >= 2 {
                format!("<line-break count=\"{}\" />", count)
            } else {
                "<line-break />".to_string()
            };
            assert_eq!(buffer.get_ref(), expected.as_bytes());
        }
    }
    
    #[test]
    fn test_tag_write_line_break_fallback_to_generated() {
        let special_tokens: Vec<String> = vec!["<line-break />".to_string()]; // Missing count variants
        let line_break = SampleLineBreak { count: 2 };
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        // Should fall back to generated tag since "<line-break count=\"2\" />" isn't in special_tokens
        tag_write_line_break(&mut buffer, &line_break, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<line-break count=\"2\" />");
    }
    
    #[test]
    fn test_tag_write_line_break_with_empty_special_tokens() {
        let special_tokens: Vec<String> = vec![];
        
        let line_break = SampleLineBreak { count: 1 };
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_line_break(&mut buffer, &line_break, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<line-break />");
        
        let line_break = SampleLineBreak { count: 2 };
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_line_break(&mut buffer, &line_break, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<line-break count=\"2\" />");
        
        let line_break = SampleLineBreak { count: 5 };
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_line_break(&mut buffer, &line_break, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<line-break count=\"5\" />");
    }
    
    #[test]
    fn test_tag_write_line_break_multiple_writes() {
        let special_tokens: Vec<String> = vec![
            "<line-break />".to_string(),
            "<line-break count=\"2\" />".to_string(),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        let line_break1 = SampleLineBreak { count: 1 };
        let line_break2 = SampleLineBreak { count: 2 };
        let line_break3 = SampleLineBreak { count: 1 };
        
        tag_write_line_break(&mut buffer, &line_break1, &special_tokens).unwrap();
        tag_write_line_break(&mut buffer, &line_break2, &special_tokens).unwrap();
        tag_write_line_break(&mut buffer, &line_break3, &special_tokens).unwrap();
        
        assert_eq!(buffer.get_ref(), b"<line-break /><line-break count=\"2\" /><line-break />");
    }
    
    #[test]
    fn test_tag_write_line_break_with_custom_writer() {
        // Test with Vec<u8> directly (implements Write)
        let special_tokens: Vec<String> = vec!["<line-break />".to_string()];
        let line_break = SampleLineBreak { count: 1 };
        let mut buffer: Vec<u8> = Vec::new();
        
        tag_write_line_break(&mut buffer, &line_break, &special_tokens).unwrap();
        assert_eq!(buffer, b"<line-break />");
    }
    
    #[test]
    fn test_tag_write_line_break_count_zero_behavior() {
        let special_tokens: Vec<String> = vec![];
        let line_break = SampleLineBreak { count: 0 };
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        // Count 0 should be treated as <line-break /> (since count >= 2 is false)
        tag_write_line_break(&mut buffer, &line_break, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<line-break />");
    }
    
    #[test]
    fn test_tag_write_line_break_count_255() {
        let special_tokens: Vec<String> = vec![];
        let line_break = SampleLineBreak { count: 255 };
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_line_break(&mut buffer, &line_break, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<line-break count=\"255\" />");
    }
    
    #[test]
    fn test_tag_write_line_break_preserves_whitespace() {
        let special_tokens: Vec<String> = vec![];
        let line_break = SampleLineBreak { count: 2 };
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_line_break(&mut buffer, &line_break, &special_tokens).unwrap();
        
        // Verify the exact string (no extra spaces)
        let result = String::from_utf8(buffer.get_ref().to_vec()).unwrap();
        assert_eq!(result, "<line-break count=\"2\" />");
        assert!(!result.contains("  ")); // No double spaces
        assert!(!result.contains("\n")); // No newlines
    }
    
    #[test]
    fn test_tag_write_line_break_consistent_with_special_tokens_generation() {
        // This test ensures that line break tags generated match the ones in bpe_get_special_tokens
        let expected_tokens = vec![
            "<line-break />".to_string(),
            "<line-break count=\"2\" />".to_string(),
        ];
        
        let special_tokens: Vec<String> = expected_tokens.clone();
        
        // Test count 1
        let line_break = SampleLineBreak { count: 1 };
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_line_break(&mut buffer, &line_break, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), expected_tokens[0].as_bytes());
        
        // Test count 2
        let line_break = SampleLineBreak { count: 2 };
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_line_break(&mut buffer, &line_break, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), expected_tokens[1].as_bytes());
    }
}
