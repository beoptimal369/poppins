// src/tag/tag_write_prompt_section.rs

use std::io::{Write, Result};
use crate::sample::SamplePromptEnum;
use crate::tag::tag_write_tag::tag_write_tag;
use crate::tag::tag_write_code_open::tag_write_code_open;
use crate::tag::tag_write_code_close::tag_write_code_close;
use crate::tag::tag_write_line_break::tag_write_line_break;


/// Writes a complete prompt section to the provided writer without formatting
///
/// This function writes the raw prompt section bytes without any pretty-printing spaces or newlines.
/// It wraps the entire section in `<prompt>` and `</prompt>` tags and handles all prompt items
/// (text, code, line breaks) using their respective tag functions.
///
/// # Arguments
/// * `writer` - Mutable reference to a type that implements `Write`
/// * `prompt_section` - Slice of prompt items to write
/// * `special_tokens` - Slice of special tokens for tag lookup
///
/// # Returns
/// * `Result<()>` - Returns `Ok(())` on success, or an I/O error if writing fails
pub fn tag_write_prompt_section<W: Write>(
    writer: &mut W,
    prompt_section: &[SamplePromptEnum],
    special_tokens: &[String],
) -> Result<()> {
    // Write opening <prompt> tag
    tag_write_tag(writer, "prompt", true, special_tokens)?;
    
    // Write all prompt items
    for item in prompt_section {
        match item {
            SamplePromptEnum::Text(content) => {
                writer.write_all(content.as_bytes())?;
            }
            SamplePromptEnum::Code(code) => {
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
            SamplePromptEnum::LineBreak(line_break) => {
                tag_write_line_break(writer, line_break, special_tokens)?;
            }
        }
    }
    
    // Write closing </prompt> tag
    tag_write_tag(writer, "prompt", false, special_tokens)
}



#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use super::tag_write_prompt_section;
    use crate::sample::{SampleLanguage, SampleIndent, SampleCode, SampleLineBreak, SamplePromptEnum};

    #[test]
    fn test_tag_write_prompt_section_empty() {
        let special_tokens: Vec<String> = vec![
            "<prompt>".to_string(),
            "</prompt>".to_string(),
        ];
        
        let prompt_section: Vec<SamplePromptEnum> = vec![];
        let mut buffer = Cursor::new(Vec::<u8>::new());
        
        tag_write_prompt_section(&mut buffer, &prompt_section, &special_tokens).unwrap();
        assert_eq!(buffer.get_ref(), b"<prompt></prompt>");
    }
    
    #[test]
    fn test_tag_write_prompt_section_single_text() {
        let special_tokens: Vec<String> = vec![
            "<prompt>".to_string(),
            "</prompt>".to_string(),
        ];
        
        let prompt_section = vec![
            SamplePromptEnum::Text("Hello, world!".to_string()),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_prompt_section(&mut buffer, &prompt_section, &special_tokens).unwrap();
        
        assert_eq!(buffer.get_ref(), b"<prompt>Hello, world!</prompt>");
    }
    
    #[test]
    fn test_tag_write_prompt_section_multiple_text() {
        let special_tokens: Vec<String> = vec![
            "<prompt>".to_string(),
            "</prompt>".to_string(),
        ];
        
        let prompt_section = vec![
            SamplePromptEnum::Text("First sentence. ".to_string()),
            SamplePromptEnum::Text("Second sentence.".to_string()),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_prompt_section(&mut buffer, &prompt_section, &special_tokens).unwrap();
        
        assert_eq!(buffer.get_ref(), b"<prompt>First sentence. Second sentence.</prompt>");
    }
    
    #[test]
    fn test_tag_write_prompt_section_with_code() {
        let special_tokens: Vec<String> = vec![
            "<prompt>".to_string(),
            "</prompt>".to_string(),
            "<js>".to_string(),
            "</js>".to_string(),
        ];
        
        let prompt_section = vec![
            SamplePromptEnum::Text("Write code: ".to_string()),
            SamplePromptEnum::Code(SampleCode {
                lang: SampleLanguage::Js,
                inline: false,
                indent: SampleIndent::Zero,
                content: "console.log('test');".to_string(),
            }),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_prompt_section(&mut buffer, &prompt_section, &special_tokens).unwrap();
        
        assert_eq!(
            buffer.get_ref(),
            b"<prompt>Write code: <js>console.log('test');</js></prompt>"
        );
    }
    
    #[test]
    fn test_tag_write_prompt_section_with_inline_code() {
        let special_tokens: Vec<String> = vec![
            "<prompt>".to_string(),
            "</prompt>".to_string(),
            "<js inline=\"true\">".to_string(),
            "</js>".to_string(),
        ];
        
        let prompt_section = vec![
            SamplePromptEnum::Text("Use ".to_string()),
            SamplePromptEnum::Code(SampleCode {
                lang: SampleLanguage::Js,
                inline: true,
                indent: SampleIndent::Zero,
                content: "const x = 42;".to_string(),
            }),
            SamplePromptEnum::Text(" in your code.".to_string()),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_prompt_section(&mut buffer, &prompt_section, &special_tokens).unwrap();
        
        assert_eq!(
            buffer.get_ref(),
            b"<prompt>Use <js inline=\"true\">const x = 42;</js> in your code.</prompt>"
        );
    }
    
    #[test]
    fn test_tag_write_prompt_section_with_indented_code() {
        let special_tokens: Vec<String> = vec![
            "<prompt>".to_string(),
            "</prompt>".to_string(),
            "<rust indent=\"2\">".to_string(),
            "</rust>".to_string(),
        ];
        
        let prompt_section = vec![
            SamplePromptEnum::Text("Rust code:\n".to_string()),
            SamplePromptEnum::Code(SampleCode {
                lang: SampleLanguage::Rust,
                inline: false,
                indent: SampleIndent::Two,
                content: "fn main() {\n  println!(\"hello\");\n}".to_string(),
            }),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_prompt_section(&mut buffer, &prompt_section, &special_tokens).unwrap();
        
        assert_eq!(
            buffer.get_ref(),
            b"<prompt>Rust code:\n<rust indent=\"2\">fn main() {\n  println!(\"hello\");\n}</rust></prompt>"
        );
    }
    
    #[test]
    fn test_tag_write_prompt_section_with_line_break() {
        let special_tokens: Vec<String> = vec![
            "<prompt>".to_string(),
            "</prompt>".to_string(),
            "<line-break />".to_string(),
        ];
        
        let prompt_section = vec![
            SamplePromptEnum::Text("First line".to_string()),
            SamplePromptEnum::LineBreak(SampleLineBreak { count: 1 }),
            SamplePromptEnum::Text("Second line".to_string()),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_prompt_section(&mut buffer, &prompt_section, &special_tokens).unwrap();
        
        assert_eq!(
            buffer.get_ref(),
            b"<prompt>First line<line-break />Second line</prompt>"
        );
    }
    
    #[test]
    fn test_tag_write_prompt_section_with_double_line_break() {
        let special_tokens: Vec<String> = vec![
            "<prompt>".to_string(),
            "</prompt>".to_string(),
            "<line-break count=\"2\" />".to_string(),
        ];
        
        let prompt_section = vec![
            SamplePromptEnum::Text("Paragraph 1".to_string()),
            SamplePromptEnum::LineBreak(SampleLineBreak { count: 2 }),
            SamplePromptEnum::Text("Paragraph 2".to_string()),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_prompt_section(&mut buffer, &prompt_section, &special_tokens).unwrap();
        
        assert_eq!(
            buffer.get_ref(),
            b"<prompt>Paragraph 1<line-break count=\"2\" />Paragraph 2</prompt>"
        );
    }
    
    #[test]
    fn test_tag_write_prompt_section_mixed_items() {
        let special_tokens: Vec<String> = vec![
            "<prompt>".to_string(),
            "</prompt>".to_string(),
            "<ts>".to_string(),
            "</ts>".to_string(),
            "<line-break />".to_string(),
        ];
        
        let prompt_section = vec![
            SamplePromptEnum::Text("Question: ".to_string()),
            SamplePromptEnum::Code(SampleCode {
                lang: SampleLanguage::Ts,
                inline: false,
                indent: SampleIndent::Zero,
                content: "type Answer = string;".to_string(),
            }),
            SamplePromptEnum::LineBreak(SampleLineBreak { count: 1 }),
            SamplePromptEnum::Text("Explain your answer.".to_string()),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_prompt_section(&mut buffer, &prompt_section, &special_tokens).unwrap();
        
        assert_eq!(
            buffer.get_ref(),
            b"<prompt>Question: <ts>type Answer = string;</ts><line-break />Explain your answer.</prompt>"
        );
    }
    
    #[test]
    fn test_tag_write_prompt_section_fallback_to_generated_tags() {
        let special_tokens: Vec<String> = vec![
            "<prompt>".to_string(),
            // Missing closing prompt tag and other tags
        ];
        
        let prompt_section = vec![
            SamplePromptEnum::Text("Test".to_string()),
            SamplePromptEnum::Code(SampleCode {
                lang: SampleLanguage::Html,
                inline: false,
                indent: SampleIndent::Zero,
                content: "<div>test</div>".to_string(),
            }),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_prompt_section(&mut buffer, &prompt_section, &special_tokens).unwrap();
        
        // Should fall back to generated tags for missing special tokens
        assert_eq!(
            buffer.get_ref(),
            b"<prompt>Test<html><div>test</div></html></prompt>"
        );
    }
    
    #[test]
    fn test_tag_write_prompt_section_with_empty_special_tokens() {
        let special_tokens: Vec<String> = vec![];
        
        let prompt_section = vec![
            SamplePromptEnum::Text("Hello ".to_string()),
            SamplePromptEnum::Code(SampleCode {
                lang: SampleLanguage::Rust,
                inline: false,
                indent: SampleIndent::One,
                content: "println!(\"world\");".to_string(),
            }),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_prompt_section(&mut buffer, &prompt_section, &special_tokens).unwrap();
        
        assert_eq!(
            buffer.get_ref(),
            b"<prompt>Hello <rust indent=\"1\">println!(\"world\");</rust></prompt>"
        );
    }
    
    #[test]
    fn test_tag_write_prompt_section_with_all_language_variants() {
        let special_tokens: Vec<String> = vec![];
        let languages = ["html", "css", "js", "ts", "jsx", "tsx", "rust", "bash", "xml", "json", "txt", "md"];
        
        for lang in languages {
            let prompt_section = vec![
                SamplePromptEnum::Text(format!("{} code: ", lang)),
                SamplePromptEnum::Code(SampleCode {
                    lang: SampleLanguage::from_str(lang),
                    inline: false,
                    indent: SampleIndent::Zero,
                    content: "test content".to_string(),
                }),
            ];
            
            let mut buffer = Cursor::new(Vec::<u8>::new());
            tag_write_prompt_section(&mut buffer, &prompt_section, &special_tokens).unwrap();
            
            let expected = format!("<prompt>{} code: <{}>test content</{}></prompt>", lang, lang, lang);
            assert_eq!(buffer.get_ref(), expected.as_bytes());
        }
    }
    
    #[test]
    fn test_tag_write_prompt_section_complex_scenario() {
        let special_tokens: Vec<String> = vec![
            "<prompt>".to_string(),
            "</prompt>".to_string(),
            "<js>".to_string(),
            "</js>".to_string(),
            "<css>".to_string(),
            "</css>".to_string(),
            "<line-break />".to_string(),
            "<line-break count=\"2\" />".to_string(),
        ];
        
        let prompt_section = vec![
            SamplePromptEnum::Text("Create a component with:\n".to_string()),
            SamplePromptEnum::Code(SampleCode {
                lang: SampleLanguage::Js,
                inline: false,
                indent: SampleIndent::Zero,
                content: "function App() {\n  return <div>Hello</div>;\n}".to_string(),
            }),
            SamplePromptEnum::LineBreak(SampleLineBreak { count: 1 }),
            SamplePromptEnum::Text("And style it:\n".to_string()),
            SamplePromptEnum::Code(SampleCode {
                lang: SampleLanguage::Css,
                inline: false,
                indent: SampleIndent::Two,
                content: ".app {\n  color: blue;\n}".to_string(),
            }),
            SamplePromptEnum::LineBreak(SampleLineBreak { count: 2 }),
            SamplePromptEnum::Text("Make it responsive.".to_string()),
        ];
        
        let mut buffer = Cursor::new(Vec::<u8>::new());
        tag_write_prompt_section(&mut buffer, &prompt_section, &special_tokens).unwrap();
        
        let expected = b"<prompt>Create a component with:\n<js>function App() {\n  return <div>Hello</div>;\n}</js><line-break />And style it:\n<css indent=\"2\">.app {\n  color: blue;\n}</css><line-break count=\"2\" />Make it responsive.</prompt>";
        assert_eq!(buffer.get_ref(), expected);
    }
    
    #[test]
    fn test_tag_write_prompt_section_with_custom_writer() {
        let special_tokens: Vec<String> = vec![
            "<prompt>".to_string(),
            "</prompt>".to_string(),
        ];
        
        let prompt_section = vec![
            SamplePromptEnum::Text("Test".to_string()),
        ];
        
        let mut buffer: Vec<u8> = Vec::new();
        tag_write_prompt_section(&mut buffer, &prompt_section, &special_tokens).unwrap();
        
        assert_eq!(buffer, b"<prompt>Test</prompt>");
    }
}
