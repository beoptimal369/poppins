// src/tag/tag_write_ai_section.rs

use std::io::{Write, Result};
use crate::sample::SampleAiEnum;
use crate::tag::tag_write_tag::tag_write_tag;
use crate::tag::tag_write_code_open::tag_write_code_open;
use crate::tag::tag_write_code_close::tag_write_code_close;
use crate::tag::tag_write_line_break::tag_write_line_break;


/// Writes a complete AI section to the provided writer without formatting
///
/// This function writes the raw AI section bytes without any pretty-printing spaces or newlines.
/// It wraps the entire section in `<ai>` and `</ai>` tags and handles all AI items
/// (text, source, code, line breaks) using their respective tag functions.
///
/// # Arguments
/// * `writer` - Mutable reference to a type that implements `Write`
/// * `ai_section` - Slice of AI items to write
/// * `special_tokens` - Slice of special tokens for tag lookup
///
/// # Returns
/// * `Result<()>` - Returns `Ok(())` on success, or an I/O error if writing fails
pub fn tag_write_ai_section<W: Write>(
    writer: &mut W,
    ai_section: &[SampleAiEnum],
    special_tokens: &[String],
) -> Result<()> {
    // Write opening <ai> tag
    tag_write_tag(writer, "ai", true, special_tokens)?;
    
    // Write all AI items
    for item in ai_section {
        match item {
            SampleAiEnum::Text(text) => {
                // Write opening <text> tag
                tag_write_tag(writer, "text", true, special_tokens)?;
                writer.write_all(text.as_bytes())?;
                // Write closing </text> tag
                tag_write_tag(writer, "text", false, special_tokens)?;
            }
            SampleAiEnum::Source(source) => {
                // Write opening <source> tag
                tag_write_tag(writer, "source", true, special_tokens)?;
                writer.write_all(source.as_bytes())?;
                // Write closing </source> tag
                tag_write_tag(writer, "source", false, special_tokens)?;
            }
            SampleAiEnum::Code(code) => {
                tag_write_code_open(
                    writer,
                    code.lang.as_str(),
                    code.inline,
                    code.indent,
                    special_tokens,
                )?;
                writer.write_all(code.content.as_bytes())?;
                tag_write_code_close(writer, code.lang.as_str(), special_tokens)?;
            }
            SampleAiEnum::LineBreak(line_break) => {
                tag_write_line_break(writer, line_break, special_tokens)?;
            }
        }
    }
    
    // Write closing </ai> tag
    tag_write_tag(writer, "ai", false, special_tokens)
}



#[cfg(test)]
mod tests {
    use super::tag_write_ai_section;
    use std::io::Cursor;
    use crate::sample::{
        SampleLanguage,
        SampleIndent,
        SampleAiEnum,
        SampleCode,
        SampleLineBreak
    };

    #[test]
    fn test_tag_write_ai_section_empty() {
        let special_tokens: Vec<String> = vec![
            "<ai>".to_string(),
            "</ai>".to_string(),
        ];
        
        let ai_section: Vec<SampleAiEnum> = vec![];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_ai_section(&mut buffer, &ai_section, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<ai></ai>");
    }
    
    #[test]
    fn test_tag_write_ai_section_single_text() {
        let special_tokens: Vec<String> = vec![
            "<ai>".to_string(),
            "</ai>".to_string(),
            "<text>".to_string(),
            "</text>".to_string(),
        ];
        
        let ai_section = vec![
            SampleAiEnum::Text("Hi world!".to_string()),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_ai_section(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        assert_eq!(buffer.get_ref(), b"<ai><text>Hi world!</text></ai>");
    }
    
    #[test]
    fn test_tag_write_ai_section_multiple_text() {
        let special_tokens: Vec<String> = vec![
            "<ai>".to_string(),
            "</ai>".to_string(),
            "<text>".to_string(),
            "</text>".to_string(),
        ];
        
        let ai_section = vec![
            SampleAiEnum::Text("First sentence. ".to_string()),
            SampleAiEnum::Text("Second sentence.".to_string()),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_ai_section(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        assert_eq!(
            buffer.get_ref(),
            b"<ai><text>First sentence. </text><text>Second sentence.</text></ai>"
        );
    }
    
    #[test]
    fn test_tag_write_ai_section_single_source() {
        let special_tokens: Vec<String> = vec![
            "<ai>".to_string(),
            "</ai>".to_string(),
            "<source>".to_string(),
            "</source>".to_string(),
        ];
        
        let ai_section = vec![
            SampleAiEnum::Source("42".to_string()),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_ai_section(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        assert_eq!(buffer.get_ref(), b"<ai><source>42</source></ai>");
    }
    
    #[test]
    fn test_tag_write_ai_section_multiple_sources() {
        let special_tokens: Vec<String> = vec![
            "<ai>".to_string(),
            "</ai>".to_string(),
            "<source>".to_string(),
            "</source>".to_string(),
        ];
        
        let ai_section = vec![
            SampleAiEnum::Source("1".to_string()),
            SampleAiEnum::Source("2".to_string()),
            SampleAiEnum::Source("3".to_string()),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_ai_section(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        assert_eq!(
            buffer.get_ref(),
            b"<ai><source>1</source><source>2</source><source>3</source></ai>"
        );
    }
    
    #[test]
    fn test_tag_write_ai_section_with_code() {
        let special_tokens: Vec<String> = vec![
            "<ai>".to_string(),
            "</ai>".to_string(),
            "<js>".to_string(),
            "</js>".to_string(),
        ];
        
        let ai_section = vec![
            SampleAiEnum::Code(SampleCode {
                lang: SampleLanguage::Js,
                inline: false,
                indent: SampleIndent::Zero,
                content: "console.log('test');".to_string(),
            }),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_ai_section(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        assert_eq!(
            buffer.get_ref(),
            b"<ai><js>console.log('test');</js></ai>"
        );
    }
    
    #[test]
    fn test_tag_write_ai_section_with_inline_code() {
        let special_tokens: Vec<String> = vec![
            "<ai>".to_string(),
            "</ai>".to_string(),
            "<rust inline=\"true\">".to_string(),
            "</rust>".to_string(),
        ];
        
        let ai_section = vec![
            SampleAiEnum::Text("Use ".to_string()),
            SampleAiEnum::Code(SampleCode {
                lang: SampleLanguage::Rust,
                inline: true,
                indent: SampleIndent::Zero,
                content: "let x = 42;".to_string(),
            }),
            SampleAiEnum::Text(" in your code.".to_string()),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_ai_section(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        assert_eq!(
            buffer.get_ref(),
            b"<ai><text>Use </text><rust inline=\"true\">let x = 42;</rust><text> in your code.</text></ai>"
        );
    }
    
    #[test]
    fn test_tag_write_ai_section_with_indented_code() {
        let special_tokens: Vec<String> = vec![
            "<ai>".to_string(),
            "</ai>".to_string(),
            "<ts indent=\"4\">".to_string(),
            "</ts>".to_string(),
        ];
        
        let ai_section = vec![
            SampleAiEnum::Text("TypeScript code:\n".to_string()),
            SampleAiEnum::Code(SampleCode {
                lang: SampleLanguage::Ts,
                inline: false,
                indent: SampleIndent::Four,
                content: "interface User {\n  name: string;\n  age: number;\n}".to_string(),
            }),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_ai_section(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        assert_eq!(
            buffer.get_ref(),
            b"<ai><text>TypeScript code:\n</text><ts indent=\"4\">interface User {\n  name: string;\n  age: number;\n}</ts></ai>"
        );
    }
    
    #[test]
    fn test_tag_write_ai_section_with_line_break() {
        let special_tokens: Vec<String> = vec![
            "<ai>".to_string(),
            "</ai>".to_string(),
            "<text>".to_string(),
            "</text>".to_string(),
            "<line-break />".to_string(),
        ];
        
        let ai_section = vec![
            SampleAiEnum::Text("First line".to_string()),
            SampleAiEnum::LineBreak(SampleLineBreak { count: 1 }),
            SampleAiEnum::Text("Second line".to_string()),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_ai_section(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        assert_eq!(
            buffer.get_ref(),
            b"<ai><text>First line</text><line-break /><text>Second line</text></ai>"
        );
    }
    
    #[test]
    fn test_tag_write_ai_section_with_double_line_break() {
        let special_tokens: Vec<String> = vec![
            "<ai>".to_string(),
            "</ai>".to_string(),
            "<text>".to_string(),
            "</text>".to_string(),
            "<line-break count=\"2\" />".to_string(),
        ];
        
        let ai_section = vec![
            SampleAiEnum::Text("Paragraph 1".to_string()),
            SampleAiEnum::LineBreak(SampleLineBreak { count: 2 }),
            SampleAiEnum::Text("Paragraph 2".to_string()),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_ai_section(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        assert_eq!(
            buffer.get_ref(),
            b"<ai><text>Paragraph 1</text><line-break count=\"2\" /><text>Paragraph 2</text></ai>"
        );
    }
    
    #[test]
    fn test_tag_write_ai_section_mixed_items() {
        let special_tokens: Vec<String> = vec![
            "<ai>".to_string(),
            "</ai>".to_string(),
            "<text>".to_string(),
            "</text>".to_string(),
            "<source>".to_string(),
            "</source>".to_string(),
            "<rust>".to_string(),
            "</rust>".to_string(),
            "<line-break />".to_string(),
        ];
        
        let ai_section = vec![
            SampleAiEnum::Text("Here's a solution: ".to_string()),
            SampleAiEnum::Code(SampleCode {
                lang: SampleLanguage::Rust,
                inline: false,
                indent: SampleIndent::Zero,
                content: "fn solve() -> i32 {\n  42\n}".to_string(),
            }),
            SampleAiEnum::LineBreak(SampleLineBreak { count: 1 }),
            SampleAiEnum::Text("Reference: ".to_string()),
            SampleAiEnum::Source("rust-book".to_string()),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_ai_section(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        assert_eq!(
            buffer.get_ref(),
            b"<ai><text>Here's a solution: </text><rust>fn solve() -> i32 {\n  42\n}</rust><line-break /><text>Reference: </text><source>rust-book</source></ai>"
        );
    }
    
    #[test]
    fn test_tag_write_ai_section_complex_scenario() {
        let special_tokens: Vec<String> = vec![
            "<ai>".to_string(),
            "</ai>".to_string(),
            "<text>".to_string(),
            "</text>".to_string(),
            "<source>".to_string(),
            "</source>".to_string(),
            "<js>".to_string(),
            "</js>".to_string(),
            "<css>".to_string(),
            "</css>".to_string(),
            "<line-break />".to_string(),
            "<line-break count=\"2\" />".to_string(),
        ];
        
        let ai_section = vec![
            SampleAiEnum::Text("Build an Ace component:\n".to_string()),
            SampleAiEnum::Code(SampleCode {
                lang: SampleLanguage::Jsx,
                inline: false,
                indent: SampleIndent::Zero,
                content: "function App() {\n  return <div>Hello</div>;\n}".to_string(),
            }),
            SampleAiEnum::LineBreak(SampleLineBreak { count: 1 }),
            SampleAiEnum::Text("Style it:\n".to_string()),
            SampleAiEnum::Code(SampleCode {
                lang: SampleLanguage::Css,
                inline: false,
                indent: SampleIndent::Two,
                content: ".app {\n  color: blue;\n}".to_string(),
            }),
            SampleAiEnum::LineBreak(SampleLineBreak { count: 2 }),
            SampleAiEnum::Text("Based on ".to_string()),
            SampleAiEnum::Source("ace-docs".to_string()),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_ai_section(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        let expected = b"<ai><text>Build an Ace component:\n</text><jsx>function App() {\n  return <div>Hello</div>;\n}</jsx><line-break /><text>Style it:\n</text><css indent=\"2\">.app {\n  color: blue;\n}</css><line-break count=\"2\" /><text>Based on </text><source>ace-docs</source></ai>";
        assert_eq!(buffer.get_ref(), expected);
    }
    
    #[test]
    fn test_tag_write_ai_section_fallback_to_generated_tags() {
        let special_tokens: Vec<String> = vec![
            "<ai>".to_string(),
            // Missing many tags
        ];
        
        let ai_section = vec![
            SampleAiEnum::Text("Test".to_string()),
            SampleAiEnum::Code(SampleCode {
                lang: SampleLanguage::Html,
                inline: false,
                indent: SampleIndent::Zero,
                content: "<div>test</div>".to_string(),
            }),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_ai_section(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        // Should fall back to generated tags for missing special tokens
        assert_eq!(
            buffer.get_ref(),
            b"<ai><text>Test</text><html><div>test</div></html></ai>"
        );
    }
    
    #[test]
    fn test_tag_write_ai_section_with_empty_special_tokens() {
        let special_tokens: Vec<String> = vec![];
        
        let ai_section = vec![
            SampleAiEnum::Text("Hi ".to_string()),
            SampleAiEnum::Source("world".to_string()),
            SampleAiEnum::Code(SampleCode {
                lang: SampleLanguage::Rust,
                inline: false,
                indent: SampleIndent::One,
                content: "println!(\"world\");".to_string(),
            }),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_ai_section(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        assert_eq!(
            buffer.get_ref(),
            b"<ai><text>Hi </text><source>world</source><rust indent=\"1\">println!(\"world\");</rust></ai>"
        );
    }
    
    #[test]
    fn test_tag_write_ai_section_all_language_variants() {
        let special_tokens: Vec<String> = vec![];
        let languages = ["html", "css", "js", "ts", "jsx", "tsx", "rust", "bash", "xml", "json", "txt", "md"];
        
        for lang in languages {
            let ai_section = vec![
                SampleAiEnum::Text(format!("{} code: ", lang)),
                SampleAiEnum::Code(SampleCode {
                    lang: SampleLanguage::from_str(lang),
                    inline: false,
                    indent: SampleIndent::Zero,
                    content: "test content".to_string(),
                }),
            ];
            
            let mut buffer = Cursor::new(Vec::<u8>::new());
            tag_write_ai_section(&mut buffer, &ai_section, &special_tokens).unwrap();
            
            let expected = format!("<ai><text>{} code: </text><{}>test content</{}></ai>", lang, lang, lang);
            assert_eq!(buffer.get_ref(), expected.as_bytes());
        }
    }
    
    #[test]
    fn test_tag_write_ai_section_with_custom_writer() {
        let special_tokens: Vec<String> = vec![
            "<ai>".to_string(),
            "</ai>".to_string(),
            "<text>".to_string(),
            "</text>".to_string(),
        ];
        
        let ai_section = vec![
            SampleAiEnum::Text("Test".to_string()),
        ];
        
        let mut buffer: Vec<u8> = Vec::new();
        tag_write_ai_section(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        assert_eq!(buffer, b"<ai><text>Test</text></ai>");
    }
    
    #[test]
    fn test_tag_write_ai_section_preserves_content_formatting() {
        let special_tokens: Vec<String> = vec![
            "<ai>".to_string(),
            "</ai>".to_string(),
            "<text>".to_string(),
            "</text>".to_string(),
        ];
        
        let multiline_content = "Line 1\n  Line 2 with spaces\nLine 3";
        
        let ai_section = vec![
            SampleAiEnum::Text(multiline_content.to_string()),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_ai_section(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        let expected = format!("<ai><text>{}</text></ai>", multiline_content);
        assert_eq!(buffer.get_ref(), expected.as_bytes());
    }
}
