// src/tag/tag_write_code_close.rs

use std::io::{Write, Result};


/// Writes a closing code tag to the provided writer without formatting
///
/// This function writes the raw closing code tag bytes without any pretty-printing spaces or newlines.
/// The tag is constructed as `</{lang}>` and looked up from special_tokens to ensure consistent
/// byte counts for binary file generation.
///
/// # Arguments
/// * `writer` - Mutable reference to a type that implements `Write`
/// * `lang` - The programming language name (e.g., "ts", "rust", "js")
/// * `special_tokens` - Slice of special tokens for tag lookup
///
/// # Returns
/// * `Result<()>` - Returns `Ok(())` on success, or an I/O error if writing fails
pub fn tag_write_code_close<W: Write>(
    writer: &mut W,
    lang: &str,
    special_tokens: &[String],
) -> Result<()> {
    let tag = format!("</{}>", lang);
    
    // Try to use the special token version first for consistency
    if let Some(tag_token) = special_tokens.iter().find(|t| **t == tag) {
        writer.write_all(tag_token.as_bytes())
    } else {
        writer.write_all(tag.as_bytes())
    }
}



#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::io::Cursor;
    use crate::tag::tag_write_code_close;

    #[test]
    fn test_tag_write_code_close_standard() {
        let special_tokens: Vec<String> = vec![
            "</js>".to_string(),
            "</ts>".to_string(),
            "</rust>".to_string(),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_code_close(&mut buffer, "js", &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"</js>");
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_code_close(&mut buffer, "ts", &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"</ts>");
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_code_close(&mut buffer, "rust", &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"</rust>");
    }
    
    #[test]
    fn test_tag_write_code_close_all_languages() {
        let languages = ["html", "css", "js", "ts", "jsx", "tsx", "rust", "bash", "xml", "json", "txt", "md"];
        let special_tokens: Vec<String> = vec![];
        
        for lang in languages {
            let mut buffer = Cursor::new(Vec::<u8>::new());
            tag_write_code_close(&mut buffer, lang, &special_tokens).unwrap();
            let expected = format!("</{}>", lang);
            assert_eq!(buffer.get_ref(), expected.as_bytes());
        }
    }
    
    #[test]
    fn test_tag_write_code_close_fallback_to_generated() {
        let special_tokens: Vec<String> = vec!["</js>".to_string()]; // Missing other languages
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        // Should fall back to generated tag since </ts> isn't in special_tokens
        tag_write_code_close(&mut buffer, "ts", &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"</ts>");
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_code_close(&mut buffer, "rust", &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"</rust>");
    }
    
    #[test]
    fn test_tag_write_code_close_with_empty_special_tokens() {
        let special_tokens: Vec<String> = vec![];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_code_close(&mut buffer, "xml", &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"</xml>");
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_code_close(&mut buffer, "json", &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"</json>");
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_code_close(&mut buffer, "css", &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"</css>");
    }
    
    #[test]
    fn test_tag_write_code_close_multiple_writes() {
        let special_tokens: Vec<String> = vec![
            "</html>".to_string(),
            "</body>".to_string(),
            "</div>".to_string(),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_code_close(&mut buffer, "html", &special_tokens).unwrap();
        tag_write_code_close(&mut buffer, "body", &special_tokens).unwrap();
        tag_write_code_close(&mut buffer, "div", &special_tokens).unwrap();
        
        assert_eq!(buffer.get_ref(), b"</html></body></div>");
    }
    
    #[test]
    fn test_tag_write_code_close_with_custom_writer() {
        // Test with Vec<u8> directly (implements Write)
        let special_tokens: Vec<String> = vec!["</custom>".to_string()];
        let mut buffer: Vec<u8> = Vec::new();
        
        tag_write_code_close(&mut buffer, "custom", &special_tokens).unwrap();
        assert_eq!(buffer, b"</custom>");
    }
    
    #[test]
    fn test_tag_write_code_close_case_sensitivity() {
        let special_tokens: Vec<String> = vec!["</HTML>".to_string()]; // Uppercase variant
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        // Should not match the uppercase token since lang is lowercase
        tag_write_code_close(&mut buffer, "html", &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"</html>");
    }
    
    #[test]
    fn test_tag_write_code_close_with_special_characters() {
        let special_tokens: Vec<String> = vec![];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        // Test with language names that might have special characters
        tag_write_code_close(&mut buffer, "c++", &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"</c++>");
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_code_close(&mut buffer, "c#", &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"</c#>");
    }
    
    #[test]
    fn test_tag_write_code_close_paired_with_open() {
        let special_tokens: Vec<String> = vec![
            "<ts>".to_string(),
            "</ts>".to_string(),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        // Write opening tag
        if let Some(tag_token) = special_tokens.iter().find(|t| **t == "<ts>") {
            buffer.write_all(tag_token.as_bytes()).unwrap();
        }
        
        // Write content
        buffer.write_all(b"function test() {}").unwrap();
        
        // Write closing tag
        tag_write_code_close(&mut buffer, "ts", &special_tokens).unwrap();
        
        assert_eq!(buffer.get_ref(), b"<ts>function test() {}</ts>");
    }
    
    #[test]
    fn test_tag_write_code_close_consistent_with_tag_write_tag() {
        // This test ensures that tag_write_code_close produces the same output
        // as tag_write_tag would for a closing code tag
        use crate::tag::tag_write_tag::tag_write_tag;
        
        let special_tokens: Vec<String> = vec![
            "</div>".to_string(),
        ];
        
        let mut buffer1 = Cursor::new(Vec::<u8>::new());
        let mut buffer2 = Cursor::new(Vec::<u8>::new());
        
        tag_write_code_close(&mut buffer1, "div", &special_tokens).unwrap();
        tag_write_tag(&mut buffer2, "div", false, &special_tokens).unwrap();
        
        assert_eq!(buffer1.get_ref(), buffer2.get_ref());
    }
}
