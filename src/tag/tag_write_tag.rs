// src/tag/tag_write_tag.rs

use std::io::{Write, Result};


/// Writes an opening or closing XML tag to the provided writer without formatting
///
/// This function writes the raw tag bytes without any pretty-printing spaces or newlines.
/// The tag is looked up from special_tokens to ensure consistent byte counts for binary
/// file generation, falling back to the generated tag string if not found in special_tokens.
///
/// # Arguments
/// * `writer` - Mutable reference to a type that implements `Write` (e.g., `BufWriter<File>`, `Vec<u8>`)
/// * `tag_name` - The name of the XML tag (e.g., "sample", "prompt", "ai")
/// * `is_opening` - If true, writes opening tag `<tag_name>`; if false, writes closing tag `</tag_name>`
/// * `special_tokens` - Slice of special tokens for tag lookup
///
/// # Returns
/// * `Result<()>` - Returns `Ok(())` on success, or an I/O error if writing fails
pub fn tag_write_tag<W: Write>(
    writer: &mut W,
    tag_name: &str,
    is_opening: bool,
    special_tokens: &[String],
) -> Result<()> {
    let tag = if is_opening {
        format!("<{}>", tag_name)
    } else {
        format!("</{}>", tag_name)
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
    fn test_tag_write_tag_opening() {
        let special_tokens = vec![
            "<sample>".to_string(),
            "</sample>".to_string(),
            "<prompt>".to_string(),
            "</prompt>".to_string(),
        ];
        
        let mut buffer = Cursor::new(Vec::new());
        
        // Test opening tag
        tag_write_tag(&mut buffer, "sample", true, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<sample>");
        
        buffer = Cursor::new(Vec::new());
        tag_write_tag(&mut buffer, "prompt", true, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<prompt>");
    }
    
    #[test]
    fn test_tag_write_tag_closing() {
        let special_tokens = vec![
            "<sample>".to_string(),
            "</sample>".to_string(),
            "<prompt>".to_string(),
            "</prompt>".to_string(),
        ];
        
        let mut buffer = Cursor::new(Vec::new());
        
        // Test closing tag
        tag_write_tag(&mut buffer, "sample", false, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"</sample>");
        
        buffer = Cursor::new(Vec::new());
        tag_write_tag(&mut buffer, "prompt", false, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"</prompt>");
    }
    
    #[test]
    fn test_tag_write_tag_fallback_to_generated() {
        let special_tokens = vec!["<sample>".to_string()]; // Missing closing tag
        let mut buffer = Cursor::new(Vec::new());
        
        // Should fall back to generated tag since </sample> isn't in special_tokens
        tag_write_tag(&mut buffer, "sample", false, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"</sample>");
    }
    
    #[test]
    fn test_tag_write_tag_with_empty_special_tokens() {
        let special_tokens: Vec<String> = vec![];
        let mut buffer = Cursor::new(Vec::new());
        
        tag_write_tag(&mut buffer, "sample", true, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<sample>");
        
        let mut buffer = Cursor::new(Vec::new());
        tag_write_tag(&mut buffer, "prompt", false, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"</prompt>");
    }
    
    #[test]
    fn test_tag_write_tag_multiple_writes() {
        let special_tokens = vec![
            "<sample>".to_string(),
            "</sample>".to_string(),
            "<prompt>".to_string(),
            "</prompt>".to_string(),
        ];
        
        let mut buffer = Cursor::new(Vec::new());
        
        tag_write_tag(&mut buffer, "sample", true, &special_tokens).unwrap();
        tag_write_tag(&mut buffer, "prompt", true, &special_tokens).unwrap();
        tag_write_tag(&mut buffer, "prompt", false, &special_tokens).unwrap();
        tag_write_tag(&mut buffer, "sample", false, &special_tokens).unwrap();
        
        assert_eq!(buffer.get_ref(), b"<sample><prompt></prompt></sample>");
    }
    
    #[test]
    fn test_tag_write_tag_with_custom_writer() {
        // Test with Vec<u8> directly (implements Write)
        let special_tokens = vec!["<test>".to_string()];
        let mut buffer = Vec::new();
        
        tag_write_tag(&mut buffer, "test", true, &special_tokens).unwrap();
        assert_eq!(buffer, b"<test>");
    }
}
