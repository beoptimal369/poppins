// src/tag/tag_write_ai_content.rs

use std::io::{Write, Result};
use crate::sample::SampleAiEnum;
use crate::tag::tag_write_tag::tag_write_tag;
use crate::tag::tag_write_code_open::tag_write_code_open;
use crate::tag::tag_write_code_close::tag_write_code_close;
use crate::tag::tag_write_line_break::tag_write_line_break;


/// Writes the content of an AI section (without the outer <ai> tags)
///
/// This function writes the raw AI section bytes without any pretty-printing spaces or newlines.
/// It does NOT wrap the content in `<ai>` tags.
///
/// # Arguments
/// * `writer` - Mutable reference to a type that implements `Write`
/// * `ai_section` - Slice of AI items to write
/// * `special_tokens` - Slice of special tokens for tag lookup
///
/// # Returns
/// * `Result<()>` - Returns `Ok(())` on success, or an I/O error if writing fails
pub fn tag_write_ai_content<W: Write>(
    writer: &mut W,
    ai_section: &[SampleAiEnum],
    special_tokens: &[String],
) -> Result<()> {
    for item in ai_section {
        match item {
            SampleAiEnum::Text(text) => {
                tag_write_tag(writer, "text", true, special_tokens)?;
                writer.write_all(text.as_bytes())?;
                tag_write_tag(writer, "text", false, special_tokens)?;
            }
            SampleAiEnum::Source(source) => {
                tag_write_tag(writer, "source", true, special_tokens)?;
                writer.write_all(source.as_bytes())?;
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
    Ok(())
}



#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use crate::tag::tag_write_ai_content;
    use crate::sample::{SampleAiEnum, SampleLanguage, SampleIndent, SampleCode, SampleLineBreak};

    fn create_test_special_tokens() -> Vec<String> {
        vec![
            "<ai>".to_string(),
            "</ai>".to_string(),
            "<text>".to_string(),
            "</text>".to_string(),
            "<source>".to_string(),
            "</source>".to_string(),
            "<js>".to_string(),
            "</js>".to_string(),
            "<rust>".to_string(),
            "</rust>".to_string(),
            "<line-break />".to_string(),
            "<line-break count=\"2\" />".to_string(),
        ]
    }

    #[test]
    fn test_tag_write_ai_content_empty() {
        let special_tokens = create_test_special_tokens();
        let ai_section: Vec<SampleAiEnum> = vec![];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_ai_content(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        assert_eq!(buffer.get_ref(), b"");
    }

    #[test]
    fn test_tag_write_ai_content_single_text() {
        let special_tokens = create_test_special_tokens();
        let ai_section = vec![
            SampleAiEnum::Text("Hello, world!".to_string()),
        ];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_ai_content(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        assert_eq!(buffer.get_ref(), b"<text>Hello, world!</text>");
    }

    #[test]
    fn test_tag_write_ai_content_multiple_text() {
        let special_tokens = create_test_special_tokens();
        let ai_section = vec![
            SampleAiEnum::Text("First sentence. ".to_string()),
            SampleAiEnum::Text("Second sentence.".to_string()),
        ];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_ai_content(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        assert_eq!(buffer.get_ref(), b"<text>First sentence. </text><text>Second sentence.</text>");
    }

    #[test]
    fn test_tag_write_ai_content_single_source() {
        let special_tokens = create_test_special_tokens();
        let ai_section = vec![
            SampleAiEnum::Source("wikipedia".to_string()),
        ];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_ai_content(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        assert_eq!(buffer.get_ref(), b"<source>wikipedia</source>");
    }

    #[test]
    fn test_tag_write_ai_content_multiple_sources() {
        let special_tokens = create_test_special_tokens();
        let ai_section = vec![
            SampleAiEnum::Source("source1".to_string()),
            SampleAiEnum::Source("source2".to_string()),
        ];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_ai_content(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        assert_eq!(buffer.get_ref(), b"<source>source1</source><source>source2</source>");
    }

    #[test]
    fn test_tag_write_ai_content_with_code() {
        let special_tokens = create_test_special_tokens();
        let ai_section = vec![
            SampleAiEnum::Text("Here's the code:".to_string()),
            SampleAiEnum::Code(SampleCode {
                lang: SampleLanguage::Js,
                inline: false,
                indent: None,
                content: "console.log('test');".to_string(),
            }),
        ];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_ai_content(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        assert_eq!(
            buffer.get_ref(),
            b"<text>Here's the code:</text><js>console.log('test');</js>"
        );
    }

    #[test]
    fn test_tag_write_ai_content_with_inline_code() {
        let special_tokens = vec![
            "<ai>".to_string(),
            "</ai>".to_string(),
            "<text>".to_string(),
            "</text>".to_string(),
            "<js inline=\"true\">".to_string(),
            "</js>".to_string(),
        ];
        let ai_section = vec![
            SampleAiEnum::Text("Use ".to_string()),
            SampleAiEnum::Code(SampleCode {
                lang: SampleLanguage::Js,
                inline: true,
                indent: None,
                content: "const x = 42;".to_string(),
            }),
            SampleAiEnum::Text(" in your code.".to_string()),
        ];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_ai_content(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        assert_eq!(
            buffer.get_ref(),
            b"<text>Use </text><js inline=\"true\">const x = 42;</js><text> in your code.</text>"
        );
    }

    #[test]
    fn test_tag_write_ai_content_with_indented_code() {
        let special_tokens = vec![
            "<ai>".to_string(),
            "</ai>".to_string(),
            "<text>".to_string(),
            "</text>".to_string(),
            "<rust indent=\"2\">".to_string(),
            "</rust>".to_string(),
        ];
        let ai_section = vec![
            SampleAiEnum::Text("Rust code:\n".to_string()),
            SampleAiEnum::Code(SampleCode {
                lang: SampleLanguage::Rust,
                inline: false,
                indent: Some(SampleIndent::Two),
                content: "fn main() {\n  println!(\"hello\");\n}".to_string(),
            }),
        ];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_ai_content(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        assert_eq!(
            buffer.get_ref(),
            b"<text>Rust code:\n</text><rust indent=\"2\">fn main() {\n  println!(\"hello\");\n}</rust>"
        );
    }

    #[test]
    fn test_tag_write_ai_content_with_line_break() {
        let special_tokens = create_test_special_tokens();
        let ai_section = vec![
            SampleAiEnum::Text("Response 1".to_string()),
            SampleAiEnum::LineBreak(SampleLineBreak { count: 1 }),
            SampleAiEnum::Text("Response 2".to_string()),
        ];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_ai_content(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        assert_eq!(
            buffer.get_ref(),
            b"<text>Response 1</text><line-break /><text>Response 2</text>"
        );
    }

    #[test]
    fn test_tag_write_ai_content_with_double_line_break() {
        let special_tokens = create_test_special_tokens();
        let ai_section = vec![
            SampleAiEnum::Text("Paragraph 1".to_string()),
            SampleAiEnum::LineBreak(SampleLineBreak { count: 2 }),
            SampleAiEnum::Text("Paragraph 2".to_string()),
        ];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_ai_content(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        assert_eq!(
            buffer.get_ref(),
            b"<text>Paragraph 1</text><line-break count=\"2\" /><text>Paragraph 2</text>"
        );
    }

    #[test]
    fn test_tag_write_ai_content_mixed_items() {
        let special_tokens = create_test_special_tokens();
        let ai_section = vec![
            SampleAiEnum::Text("Answer:".to_string()),
            SampleAiEnum::Source("wikipedia".to_string()),
            SampleAiEnum::Code(SampleCode {
                lang: SampleLanguage::Js,
                inline: false,
                indent: None,
                content: "console.log('done');".to_string(),
            }),
            SampleAiEnum::LineBreak(SampleLineBreak { count: 1 }),
            SampleAiEnum::Text("End of response.".to_string()),
        ];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_ai_content(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        assert_eq!(
            buffer.get_ref(),
            b"<text>Answer:</text><source>wikipedia</source><js>console.log('done');</js><line-break /><text>End of response.</text>"
        );
    }

    #[test]
    fn test_tag_write_ai_content_fallback_to_generated_tags() {
        let special_tokens: Vec<String> = vec![]; // Empty special tokens - should fall back to generated tags
        let ai_section = vec![
            SampleAiEnum::Text("Test".to_string()),
            SampleAiEnum::Source("source".to_string()),
            SampleAiEnum::Code(SampleCode {
                lang: SampleLanguage::Html,
                inline: false,
                indent: None,
                content: "<div>test</div>".to_string(),
            }),
        ];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_ai_content(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        // Should fall back to generated tags
        assert_eq!(
            buffer.get_ref(),
            b"<text>Test</text><source>source</source><html><div>test</div></html>"
        );
    }

    #[test]
    fn test_tag_write_ai_content_with_all_language_variants() {
        let special_tokens: Vec<String> = vec![];
        let languages = ["html", "css", "js", "ts", "jsx", "tsx", "rust", "bash", "xml", "json", "txt", "md"];
        
        for lang in languages {
            let ai_section = vec![
                SampleAiEnum::Code(SampleCode {
                    lang: SampleLanguage::from_str(lang),
                    inline: false,
                    indent: None,
                    content: "test content".to_string(),
                }),
            ];
            
            let mut buffer = Cursor::new(Vec::<u8>::new());
            tag_write_ai_content(&mut buffer, &ai_section, &special_tokens).unwrap();
            
            let expected = format!("<{}>test content</{}>", lang, lang);
            assert_eq!(buffer.get_ref(), expected.as_bytes());
        }
    }

    #[test]
    fn test_tag_write_ai_content_text_and_code_alternating() {
        let special_tokens = create_test_special_tokens();
        let ai_section = vec![
            SampleAiEnum::Text("Step 1: ".to_string()),
            SampleAiEnum::Code(SampleCode {
                lang: SampleLanguage::Js,
                inline: true,
                indent: None,
                content: "const x = 1;".to_string(),
            }),
            SampleAiEnum::Text(" Step 2: ".to_string()),
            SampleAiEnum::Code(SampleCode {
                lang: SampleLanguage::Js,
                inline: true,
                indent: None,
                content: "const y = 2;".to_string(),
            }),
            SampleAiEnum::Text(" Done.".to_string()),
        ];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_ai_content(&mut buffer, &ai_section, &special_tokens).unwrap();
        
        assert_eq!(
            buffer.get_ref(),
            b"<text>Step 1: </text><js inline=\"true\">const x = 1;</js><text> Step 2: </text><js inline=\"true\">const y = 2;</js><text> Done.</text>"
        );
    }
}
