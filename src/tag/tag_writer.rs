// src/tag/tag_writer.rs

use std::io::{Write, Result};
use crate::sample::{SampleLineBreak, SampleIndent};
use crate::tag::{
    tag_write_tag,
    tag_write_code_open,
    tag_write_code_close,
    tag_write_line_break,
};

/// A flexible XML tag writer that can operate in both binary and pretty modes
///
/// In binary mode (pretty=false), writes tags without any formatting (no spaces, no newlines).
/// In pretty mode (pretty=true), writes tags with proper indentation and newlines.
pub struct TagWriter<W: Write> {
    writer: W,
    pub pretty: bool,
    indent_level: usize,
    indent_string: String,
    at_line_start: bool,
}

impl<W: Write> TagWriter<W> {
    /// Create a new TagWriter
    ///
    /// # Arguments
    /// * `writer` - The underlying writer
    /// * `pretty` - If true, enables pretty formatting with indentation
    /// * `indent_spaces` - Number of spaces per indent level (only used when pretty=true)
    pub fn new(writer: W, pretty: bool, indent_spaces: usize) -> Self {
        Self {
            writer,
            pretty,
            indent_level: 0,
            indent_string: " ".repeat(indent_spaces),
            at_line_start: true,
        }
    }
    
    /// Increase indentation level (only effective in pretty mode)
    pub fn indent(&mut self) {
        if self.pretty {
            self.indent_level += 1;
        }
    }
    
    /// Decrease indentation level (only effective in pretty mode)
    pub fn outdent(&mut self) {
        if self.pretty && self.indent_level > 0 {
            self.indent_level -= 1;
        }
    }
    
    /// Write a simple opening or closing tag (e.g., <prompt>, </prompt>)
    ///
    /// # Arguments
    /// * `tag_name` - The tag name (e.g., "sample", "prompt", "ai")
    /// * `is_opening` - If true, writes opening tag; if false, writes closing tag
    /// * `special_tokens` - Slice of special tokens for binary mode lookup
    pub fn write_simple_tag(
        &mut self,
        tag_name: &str,
        is_opening: bool,
        special_tokens: &[String],
    ) -> Result<()> {
        if self.pretty {
            self.write_indent()?;
            // Use a temporary buffer to get the tag from the underlying function
            let mut temp = Vec::new();
            tag_write_tag(&mut temp, tag_name, is_opening, special_tokens)?;
            self.writer.write_all(&temp)?;
            self.at_line_start = false;
            Ok(())
        } else {
            tag_write_tag(&mut self.writer, tag_name, is_opening, special_tokens)
        }
    }
    
    /// Write a simple opening tag, optionally with a newline
    pub fn write_simple_tag_open(
        &mut self,
        tag_name: &str,
        special_tokens: &[String],
        newline_after: bool,
    ) -> Result<()> {
        self.write_simple_tag(tag_name, true, special_tokens)?;
        if self.pretty && newline_after {
            self.write_newline()?;
        }
        Ok(())
    }
    
    /// Write a simple closing tag, optionally with a newline
    pub fn write_simple_tag_close(
        &mut self,
        tag_name: &str,
        special_tokens: &[String],
        newline_after: bool,
    ) -> Result<()> {
        self.write_simple_tag(tag_name, false, special_tokens)?;
        if self.pretty && newline_after {
            self.write_newline()?;
        }
        Ok(())
    }
    
    /// Write a tag pair with content (e.g., <text>content</text>)
    ///
    /// # Arguments
    /// * `tag_name` - The tag name
    /// * `content` - The content to wrap
    /// * `special_tokens` - Slice of special tokens for binary mode lookup
    /// * `newline_after` - If true and in pretty mode, adds a newline after the closing tag
    pub fn write_tag_pair(
        &mut self,
        tag_name: &str,
        content: &str,
        special_tokens: &[String],
        newline_after: bool,
    ) -> Result<()> {
        if self.pretty {
            self.write_indent()?;
            // Write opening tag using the tag function
            let mut temp = Vec::new();
            tag_write_tag(&mut temp, tag_name, true, special_tokens)?;
            self.writer.write_all(&temp)?;
            
            // Write content
            self.writer.write_all(content.as_bytes())?;
            
            // Write closing tag
            let mut temp = Vec::new();
            tag_write_tag(&mut temp, tag_name, false, special_tokens)?;
            self.writer.write_all(&temp)?;
            
            if newline_after {
                self.write_newline()?;
            } else {
                self.at_line_start = false;
            }
            Ok(())
        } else {
            // Binary mode: just write sequentially
            self.write_simple_tag(tag_name, true, special_tokens)?;
            self.writer.write_all(content.as_bytes())?;
            self.write_simple_tag(tag_name, false, special_tokens)
        }
    }
    
    /// Write a code opening tag with attributes
    ///
    /// # Arguments
    /// * `lang` - Programming language
    /// * `inline` - Whether this is inline code
    /// * `indent` - Indentation level
    /// * `special_tokens` - Slice of special tokens for binary mode lookup
    pub fn write_code_open(
        &mut self,
        lang: &str,
        inline: bool,
        indent: SampleIndent,
        special_tokens: &[String],
    ) -> Result<()> {
        if self.pretty {
            self.write_indent()?;
            let mut temp = Vec::new();
            tag_write_code_open(&mut temp, lang, inline, indent, special_tokens)?;
            self.writer.write_all(&temp)?;
            self.at_line_start = false;
            Ok(())
        } else {
            tag_write_code_open(&mut self.writer, lang, inline, indent, special_tokens)
        }
    }
    
    /// Write a code closing tag
    ///
    /// # Arguments
    /// * `lang` - Programming language
    /// * `special_tokens` - Slice of special tokens for binary mode lookup
    /// * `newline_after` - If true and in pretty mode, adds a newline after the closing tag
    pub fn write_code_close(
        &mut self,
        lang: &str,
        special_tokens: &[String],
        newline_after: bool,
    ) -> Result<()> {
        if self.pretty {
            let mut temp = Vec::new();
            tag_write_code_close(&mut temp, lang, special_tokens)?;
            self.writer.write_all(&temp)?;
            if newline_after {
                self.write_newline()?;
            } else {
                self.at_line_start = false;
            }
            Ok(())
        } else {
            tag_write_code_close(&mut self.writer, lang, special_tokens)
        }
    }
    
    /// Write a line break tag
    ///
    /// # Arguments
    /// * `line_break` - The line break configuration
    /// * `special_tokens` - Slice of special tokens for binary mode lookup
    /// * `newline_after` - If true and in pretty mode, adds a newline after the tag
    pub fn write_line_break(
        &mut self,
        line_break: &SampleLineBreak,
        special_tokens: &[String],
        newline_after: bool,
    ) -> Result<()> {
        if self.pretty {
            self.write_indent()?;
            let mut temp = Vec::new();
            tag_write_line_break(&mut temp, line_break, special_tokens)?;
            self.writer.write_all(&temp)?;
            if newline_after {
                self.write_newline()?;
            } else {
                self.at_line_start = false;
            }
            Ok(())
        } else {
            tag_write_line_break(&mut self.writer, line_break, special_tokens)
        }
    }
    
    /// Write raw text content
    ///
    /// # Arguments
    /// * `text` - The text to write
    pub fn write_text(&mut self, text: &str) -> Result<()> {
        if self.pretty && self.at_line_start {
            self.write_indent()?;
        }
        self.writer.write_all(text.as_bytes())?;
        self.at_line_start = false;
        Ok(())
    }
    
    /// Write a newline (only effective in pretty mode)
    pub fn write_newline(&mut self) -> Result<()> {
        if self.pretty {
            self.writer.write_all(b"\n")?;
            self.at_line_start = true;
        }
        Ok(())
    }
    
    /// Write indentation based on current level (only in pretty mode)
    pub fn write_indent(&mut self) -> Result<()> {
        if self.pretty && self.at_line_start {
            for _ in 0..self.indent_level {
                self.writer.write_all(self.indent_string.as_bytes())?;
            }
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_binary_mode_no_formatting() {
        let special_tokens: Vec<String> = vec![];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        let mut writer = TagWriter::new(&mut buffer, false, 2);
        
        writer.write_simple_tag("sample", true, &special_tokens).unwrap();
        writer.write_simple_tag("prompt", true, &special_tokens).unwrap();
        writer.write_text("Hello").unwrap();
        writer.write_simple_tag("prompt", false, &special_tokens).unwrap();
        writer.write_simple_tag("sample", false, &special_tokens).unwrap();
        
        assert_eq!(buffer.get_ref(), b"<sample><prompt>Hello</prompt></sample>");
    }
    
    #[test]
    fn test_pretty_mode_with_indentation() {
        let special_tokens: Vec<String> = vec![];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        let mut writer = TagWriter::new(&mut buffer, true, 2);
        
        writer.write_simple_tag_open("sample", &special_tokens, true).unwrap();
        writer.indent();
        
        writer.write_simple_tag_open("prompt", &special_tokens, false).unwrap();
        writer.write_text("Hello").unwrap();
        writer.write_simple_tag_close("prompt", &special_tokens, true).unwrap();
        
        writer.outdent();
        writer.write_simple_tag_close("sample", &special_tokens, true).unwrap();
        
        let expected = "<sample>\n  <prompt>Hello</prompt>\n</sample>\n";
        assert_eq!(String::from_utf8(buffer.get_ref().to_vec()).unwrap(), expected);
    }
    
    #[test]
    fn test_tag_pair_binary() {
        let special_tokens: Vec<String> = vec![];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        let mut writer = TagWriter::new(&mut buffer, false, 2);
        
        writer.write_tag_pair("text", "Hello", &special_tokens, false).unwrap();
        assert_eq!(buffer.get_ref(), b"<text>Hello</text>");
    }
    
    #[test]
    fn test_tag_pair_pretty() {
        let special_tokens: Vec<String> = vec![];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        let mut writer = TagWriter::new(&mut buffer, true, 2);
        
        writer.indent(); // Add indentation to match the expected output
        writer.write_tag_pair("text", "Hello", &special_tokens, true).unwrap();
        
        let expected = "  <text>Hello</text>\n";
        assert_eq!(String::from_utf8(buffer.get_ref().to_vec()).unwrap(), expected);
    }
    
    #[test]
    fn test_tag_pair_pretty_no_indent() {
        let special_tokens: Vec<String> = vec![];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        let mut writer = TagWriter::new(&mut buffer, true, 2);
        
        // No indent() called, so no leading spaces
        writer.write_tag_pair("text", "Hello", &special_tokens, true).unwrap();
        
        let expected = "<text>Hello</text>\n";
        assert_eq!(String::from_utf8(buffer.get_ref().to_vec()).unwrap(), expected);
    }
    
    #[test]
    fn test_code_tags_binary() {
        let special_tokens: Vec<String> = vec![];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        let mut writer = TagWriter::new(&mut buffer, false, 2);
        
        writer.write_code_open("js", false, SampleIndent::Zero, &special_tokens).unwrap();
        writer.write_text("console.log('test');").unwrap();
        writer.write_code_close("js", &special_tokens, false).unwrap();
        
        assert_eq!(buffer.get_ref(), b"<js>console.log('test');</js>");
    }
    
    #[test]
    fn test_code_tags_pretty() {
        let special_tokens: Vec<String> = vec![];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        let mut writer = TagWriter::new(&mut buffer, true, 2);
        
        writer.indent();
        writer.write_code_open("js", false, SampleIndent::Zero, &special_tokens).unwrap();
        writer.write_text("console.log('test');").unwrap();
        writer.write_code_close("js", &special_tokens, true).unwrap();
        
        let expected = "  <js>console.log('test');</js>\n";
        assert_eq!(String::from_utf8(buffer.get_ref().to_vec()).unwrap(), expected);
    }
    
    #[test]
    fn test_indented_code_tag_pretty() {
        let special_tokens: Vec<String> = vec![];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        let mut writer = TagWriter::new(&mut buffer, true, 2);
        
        writer.indent();
        writer.write_code_open("rust", false, SampleIndent::Two, &special_tokens).unwrap();
        writer.write_text("fn main() {}").unwrap();
        writer.write_code_close("rust", &special_tokens, true).unwrap();
        
        let expected = "  <rust indent=\"2\">fn main() {}</rust>\n";
        assert_eq!(String::from_utf8(buffer.get_ref().to_vec()).unwrap(), expected);
    }
    
    #[test]
    fn test_inline_code_tag_pretty() {
        let special_tokens: Vec<String> = vec![];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        let mut writer = TagWriter::new(&mut buffer, true, 2);
        
        writer.write_code_open("css", true, SampleIndent::Zero, &special_tokens).unwrap();
        writer.write_text("color: red;").unwrap();
        writer.write_code_close("css", &special_tokens, false).unwrap();
        
        let expected = "<css inline=\"true\">color: red;</css>";
        assert_eq!(String::from_utf8(buffer.get_ref().to_vec()).unwrap(), expected);
    }
    
    #[test]
    fn test_line_break_binary() {
        let special_tokens: Vec<String> = vec![];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        let mut writer = TagWriter::new(&mut buffer, false, 2);
        
        let lb = SampleLineBreak { count: 1 };
        writer.write_line_break(&lb, &special_tokens, false).unwrap();
        assert_eq!(buffer.get_ref(), b"<line-break />");
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        let mut writer = TagWriter::new(&mut buffer, false, 2);
        let lb = SampleLineBreak { count: 2 };
        writer.write_line_break(&lb, &special_tokens, false).unwrap();
        assert_eq!(buffer.get_ref(), b"<line-break count=\"2\" />");
    }
    
    #[test]
    fn test_line_break_pretty() {
        let special_tokens: Vec<String> = vec![];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        let mut writer = TagWriter::new(&mut buffer, true, 2);
        
        writer.indent();
        let lb = SampleLineBreak { count: 1 };
        writer.write_line_break(&lb, &special_tokens, true).unwrap();
        
        let expected = "  <line-break />\n";
        assert_eq!(String::from_utf8(buffer.get_ref().to_vec()).unwrap(), expected);
    }
    
    #[test]
    fn test_nested_structure_pretty() {
        let special_tokens: Vec<String> = vec![];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        let mut writer = TagWriter::new(&mut buffer, true, 2);
        
        writer.write_simple_tag_open("samples", &special_tokens, true).unwrap();
        writer.indent();
        
        writer.write_simple_tag_open("sample", &special_tokens, true).unwrap();
        writer.indent();
        
        writer.write_tag_pair("prompt", "Question?", &special_tokens, true).unwrap();
        
        writer.write_simple_tag_open("ai", &special_tokens, true).unwrap();
        writer.indent();
        writer.write_tag_pair("text", "Answer!", &special_tokens, true).unwrap();
        writer.outdent();
        writer.write_simple_tag_close("ai", &special_tokens, true).unwrap();
        
        writer.outdent();
        writer.write_simple_tag_close("sample", &special_tokens, true).unwrap();
        
        writer.outdent();
        writer.write_simple_tag_close("samples", &special_tokens, true).unwrap();
        
        let result = String::from_utf8(buffer.get_ref().to_vec()).unwrap();
        
        let expected = r#"<samples>
  <sample>
    <prompt>Question?</prompt>
    <ai>
      <text>Answer!</text>
    </ai>
  </sample>
</samples>
"#;
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_mixed_content_pretty() {
        let special_tokens: Vec<String> = vec![];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        let mut writer = TagWriter::new(&mut buffer, true, 2);
        
        writer.write_simple_tag_open("prompt", &special_tokens, false).unwrap();
        writer.write_text("Here is ").unwrap();
        writer.write_code_open("js", true, SampleIndent::Zero, &special_tokens).unwrap();
        writer.write_text("code").unwrap();
        writer.write_code_close("js", &special_tokens, false).unwrap();
        writer.write_text(" in a sentence.").unwrap();
        writer.write_simple_tag_close("prompt", &special_tokens, true).unwrap();
        
        let expected = r#"<prompt>Here is <js inline="true">code</js> in a sentence.</prompt>
"#;
        assert_eq!(String::from_utf8(buffer.get_ref().to_vec()).unwrap(), expected);
    }
    
    #[test]
    fn test_with_special_tokens_binary() {
        let special_tokens: Vec<String> = vec![
            "<custom>".to_string(),
            "</custom>".to_string(),
        ];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        let mut writer = TagWriter::new(&mut buffer, false, 2);
        
        writer.write_simple_tag("custom", true, &special_tokens).unwrap();
        writer.write_text("test").unwrap();
        writer.write_simple_tag("custom", false, &special_tokens).unwrap();
        
        // Should use the special token strings
        assert_eq!(buffer.get_ref(), b"<custom>test</custom>");
    }
}
