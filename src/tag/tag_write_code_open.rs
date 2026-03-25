// src/tag/tag_write_code_open.rs

use std::io::{Write, Result};
use crate::sample::SampleIndent;


/// Writes an opening code tag with optional attributes to the provided writer without formatting
///
/// This function writes the raw code tag bytes without any pretty-printing spaces or newlines.
/// The tag is constructed based on the language, inline status, and indent level, then looked up
/// from special_tokens to ensure consistent byte counts for binary file generation.
///
/// # Arguments
/// * `writer` - Mutable reference to a type that implements `Write`
/// * `lang` - The programming language name (e.g., "ts", "rust", "js")
/// * `inline` - If true, adds `inline="true"` attribute
/// * `indent` - Indentation level (0-6), only included if > 0
/// * `special_tokens` - Slice of special tokens for tag lookup
///
/// # Returns
/// * `Result<()>` - Returns `Ok(())` on success, or an I/O error if writing fails
pub fn tag_write_code_open<W: Write>(
    writer: &mut W,
    lang: &str,
    inline: bool,
    indent: SampleIndent,
    special_tokens: &[String],
) -> Result<()> {
    let tag = if inline {
        format!("<{} inline=\"true\">", lang)
    } else if indent.as_u8() > 0 {
        format!("<{} indent=\"{}\">", lang, indent.as_u8())
    } else {
        format!("<{}>", lang)
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
    fn test_tag_write_code_open_standard() {
        let special_tokens: Vec<String> = vec![
            "<js>".to_string(),
            "<ts>".to_string(),
            "<rust>".to_string(),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_code_open(&mut buffer, "js", false, SampleIndent::Zero, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<js>");
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_code_open(&mut buffer, "ts", false, SampleIndent::Zero, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<ts>");
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_code_open(&mut buffer, "rust", false, SampleIndent::Zero, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<rust>");
    }
    
    #[test]
    fn test_tag_write_code_open_indented() {
        let special_tokens: Vec<String> = vec![
            "<js indent=\"2\">".to_string(),
            "<ts indent=\"4\">".to_string(),
            "<rust indent=\"6\">".to_string(),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_code_open(&mut buffer, "js", false, SampleIndent::Two, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<js indent=\"2\">");
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_code_open(&mut buffer, "ts", false, SampleIndent::Four, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<ts indent=\"4\">");
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_code_open(&mut buffer, "rust", false, SampleIndent::Six, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<rust indent=\"6\">");
    }
    
    #[test]
    fn test_tag_write_code_open_inline() {
        let special_tokens: Vec<String> = vec![
            "<js inline=\"true\">".to_string(),
            "<ts inline=\"true\">".to_string(),
            "<rust inline=\"true\">".to_string(),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_code_open(&mut buffer, "js", true, SampleIndent::Zero, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<js inline=\"true\">");
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_code_open(&mut buffer, "ts", true, SampleIndent::Zero, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<ts inline=\"true\">");
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_code_open(&mut buffer, "rust", true, SampleIndent::Zero, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<rust inline=\"true\">");
    }
    
    #[test]
    fn test_tag_write_code_open_inline_with_indent_ignores_indent() {
        let special_tokens: Vec<String> = vec![
            "<js inline=\"true\">".to_string(),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        // Even though indent is Two, inline should take precedence and ignore indent
        tag_write_code_open(&mut buffer, "js", true, SampleIndent::Two, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<js inline=\"true\">");
    }
    
    #[test]
    fn test_tag_write_code_open_all_indent_levels() {
        let special_tokens: Vec<String> = vec![];

        // Test all indent levels
        let indent_levels = [
            (SampleIndent::Zero, "<html>"),
            (SampleIndent::One, "<html indent=\"1\">"),
            (SampleIndent::Two, "<html indent=\"2\">"),
            (SampleIndent::Three, "<html indent=\"3\">"),
            (SampleIndent::Four, "<html indent=\"4\">"),
            (SampleIndent::Five, "<html indent=\"5\">"),
            (SampleIndent::Six, "<html indent=\"6\">"),
        ];
        
        for (indent, expected) in indent_levels {
            let mut buffer = Cursor::new(Vec::<u8>::new());
            tag_write_code_open(&mut buffer, "html", false, indent, &special_tokens).unwrap();
            assert_eq!(buffer.get_ref(), expected.as_bytes());
        }
    }
    
    #[test]
    fn test_tag_write_code_open_fallback_to_generated() {
        let special_tokens: Vec<String> = vec!["<js>".to_string()]; // Missing other variants
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        // Should fall back to generated tag since <js indent=\"2\"> isn't in special_tokens
        tag_write_code_open(&mut buffer, "js", false, SampleIndent::Two, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<js indent=\"2\">");
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_code_open(&mut buffer, "rust", true, SampleIndent::Zero, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<rust inline=\"true\">");
    }
    
    #[test]
    fn test_tag_write_code_open_with_empty_special_tokens() {
        let special_tokens: Vec<String> = vec![];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_code_open(&mut buffer, "xml", false, SampleIndent::Zero, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<xml>");
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_code_open(&mut buffer, "json", false, SampleIndent::Three, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<json indent=\"3\">");
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_code_open(&mut buffer, "css", true, SampleIndent::Zero, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<css inline=\"true\">");
    }
    
    #[test]
    fn test_tag_write_code_open_priority_order() {
        let special_tokens: Vec<String> = vec![];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        // Test priority: inline > indent > standard
        tag_write_code_open(&mut buffer, "test", true, SampleIndent::Four, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<test inline=\"true\">");
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_code_open(&mut buffer, "test", false, SampleIndent::Four, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<test indent=\"4\">");
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_code_open(&mut buffer, "test", false, SampleIndent::Zero, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<test>");
    }
    
    #[test]
    fn test_tag_write_code_open_all_languages_standard() {
        let languages = ["html", "css", "js", "ts", "jsx", "tsx", "rust", "bash", "xml", "json", "txt", "md"];
        let special_tokens: Vec<String> = vec![];
        
        for lang in languages {
            let mut buffer = Cursor::new(Vec::<u8>::new());
            tag_write_code_open(&mut buffer, lang, false, SampleIndent::Zero, &special_tokens).unwrap();
            let expected = format!("<{}>", lang);
            assert_eq!(buffer.get_ref(), expected.as_bytes());
        }
    }
    
    #[test]
    fn test_tag_write_code_open_with_custom_writer() {
        // Test with Vec<u8> directly (implements Write)
        let special_tokens: Vec<String> = vec!["<custom>".to_string()];
        let mut buffer: Vec<u8> = Vec::new();
        
        tag_write_code_open(&mut buffer, "custom", false, SampleIndent::Zero, &special_tokens).unwrap();
        assert_eq!(buffer, b"<custom>");
    }
    
    #[test]
    fn test_tag_write_code_open_multiple_attributes_never_combined() {
        let special_tokens: Vec<String> = vec![];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        // The function should never generate a tag with both inline and indent
        // (inline takes precedence and indent is ignored)
        tag_write_code_open(&mut buffer, "test", true, SampleIndent::Three, &special_tokens).unwrap();
        let result = String::from_utf8(buffer.get_ref().to_vec()).unwrap();
        
        assert!(!result.contains("indent"));
        assert!(result.contains("inline=\"true\""));
        assert_eq!(result, "<test inline=\"true\">");
    }
}
