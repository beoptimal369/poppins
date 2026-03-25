// src/train/train_write_xmls.rs

use crate::sample::Samples;
use crate::tag::TagWriter;
use std::fs::File;
use std::io::Write;
use std::path::Path;


/// Writes training and validation XML files from samples
///
/// This function creates human-readable XML files for both training and validation samples.
/// The files are formatted with proper indentation for easy viewing and debugging.
///
/// # Arguments
/// * `output_dir` - Directory where files will be written
/// * `samples` - Samples struct containing both train and validation samples
///
/// # Files Created
/// * `train_corpus.xml` - Human-readable XML for training samples
/// * `val_corpus.xml` - Human-readable XML for validation samples
pub fn train_write_xmls(
    output_dir: &Path,
    samples: &Samples,
) -> Result<(), Box<dyn std::error::Error>> {
    // Write training XML
    let train_corpus = create_corpus_string(&samples.train_samples);
    let train_path = output_dir.join("train_corpus.xml");
    let mut train_file = File::create(train_path)?;
    train_file.write_all(train_corpus.as_bytes())?;
    
    // Write validation XML
    let val_corpus = create_corpus_string(&samples.val_samples);
    let val_path = output_dir.join("val_corpus.xml");
    let mut val_file = File::create(val_path)?;
    val_file.write_all(val_corpus.as_bytes())?;
    
    Ok(())
}

/// Creates a corpus string from training samples in the specified XML format
fn create_corpus_string(samples: &[crate::sample::Sample]) -> String {
    let mut buffer = Vec::new();
    let mut writer = TagWriter::new(&mut buffer, true, 2); // Pretty mode with 2 spaces
    
    // We don't need special tokens for pretty XML generation
    let special_tokens: Vec<String> = vec![];
    
    // Write XML header
    writer.write_text("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n").unwrap();
    
    // Root samples tag
    writer.write_simple_tag_open("samples", &special_tokens, true).unwrap();
    writer.indent();
    
    for sample in samples {
        // Sample wrapper
        writer.write_simple_tag_open("sample", &special_tokens, true).unwrap();
        writer.indent();
        
        // --- Prompt Section ---
        writer.write_simple_tag_open("prompt", &special_tokens, false).unwrap();
        
        for prompt_item in &sample.prompt_section {
            match prompt_item {
                crate::sample::SamplePromptEnum::Text(text) => {
                    writer.write_text(text).unwrap();
                }
                crate::sample::SamplePromptEnum::Code(code) => {
                    writer.write_code_open(code.lang.as_str(), code.inline, code.indent, &special_tokens).unwrap();
                    writer.write_text(&code.content).unwrap();
                    writer.write_code_close(code.lang.as_str(), &special_tokens, false).unwrap();
                }
                crate::sample::SamplePromptEnum::LineBreak(lb) => {
                    writer.write_line_break(lb, &special_tokens, false).unwrap();
                }
            }
        }
        
        writer.write_simple_tag_close("prompt", &special_tokens, true).unwrap();
        
        // --- AI Section ---
        writer.write_simple_tag_open("ai", &special_tokens, true).unwrap();
        writer.indent();
        
        for ai_item in &sample.ai_section {
            match ai_item {
                crate::sample::SampleAiEnum::Text(text) => {
                    // Use write_tag_pair which handles indentation internally
                    writer.write_tag_pair("text", &text, &special_tokens, true).unwrap();
                }
                crate::sample::SampleAiEnum::Source(source) => {
                    writer.write_tag_pair("source", &source, &special_tokens, true).unwrap();
                }
                crate::sample::SampleAiEnum::Code(code) => {
                    // Write code - tag functions handle indentation
                    writer.write_code_open(code.lang.as_str(), code.inline, code.indent, &special_tokens).unwrap();
                    writer.write_text(&code.content).unwrap();
                    writer.write_code_close(code.lang.as_str(), &special_tokens, true).unwrap();
                }
                crate::sample::SampleAiEnum::LineBreak(lb) => {
                    writer.write_line_break(lb, &special_tokens, true).unwrap();
                }
            }
        }
        
        writer.outdent();
        writer.write_simple_tag_close("ai", &special_tokens, true).unwrap();
        
        writer.outdent();
        writer.write_simple_tag_close("sample", &special_tokens, true).unwrap();
    }
    
    writer.outdent();
    writer.write_simple_tag_close("samples", &special_tokens, true).unwrap();
    
    String::from_utf8(buffer).unwrap()
}



#[cfg(test)]
mod tests {
    use tempfile::tempdir;
    use super::train_write_xmls;
    use crate::sample::{
        Sample,
        Samples,
        SampleCode,
        SampleAiEnum,
        SampleIndent,
        SampleLanguage,
        SamplePromptEnum,
    };

    #[test]
    fn test_train_write_xmls_basic() {
        let temp_dir = tempdir().unwrap();
        
        let samples = Samples {
            train_samples: vec![
                Sample {
                    prompt_section: vec![SamplePromptEnum::Text("Hello".to_string())],
                    ai_section: vec![
                        SampleAiEnum::Text("Hi".to_string()),
                    ],
                },
            ],
            val_samples: vec![
                Sample {
                    prompt_section: vec![SamplePromptEnum::Text("Question".to_string())],
                    ai_section: vec![
                        SampleAiEnum::Text("Answer".to_string()),
                    ],
                },
            ],
        };
        
        train_write_xmls(temp_dir.path(), &samples).unwrap();
        
        // Check that files were created
        assert!(temp_dir.path().join("train_corpus.xml").exists());
        assert!(temp_dir.path().join("val_corpus.xml").exists());
        
        // Read and verify content
        let train_content = std::fs::read_to_string(temp_dir.path().join("train_corpus.xml")).unwrap();
        assert!(train_content.contains("<?xml version=\"1.0\" encoding=\"UTF-8\"?>"));
        assert!(train_content.contains("<samples>"));
        assert!(train_content.contains("  <sample>"));
        assert!(train_content.contains("    <prompt>Hello</prompt>"));
        assert!(train_content.contains("    <ai>"));
        assert!(train_content.contains("      <text>Hi</text>"));
        assert!(train_content.contains("    </ai>"));
        assert!(train_content.contains("  </sample>"));
        assert!(train_content.contains("</samples>"));
        
        let val_content = std::fs::read_to_string(temp_dir.path().join("val_corpus.xml")).unwrap();
        assert!(val_content.contains("    <prompt>Question</prompt>"));
        assert!(val_content.contains("      <text>Answer</text>"));
    }

    #[test]
    fn test_train_write_xmls_formatting() {
        let temp_dir = tempdir().unwrap();

        let samples = Samples {
            train_samples: vec![Sample {
                prompt_section: vec![SamplePromptEnum::Text("Hello".to_string())],
                ai_section: vec![
                    SampleAiEnum::Text("Hi".to_string()),
                    SampleAiEnum::Code(SampleCode {
                        lang: SampleLanguage::Ts,
                        inline: false,
                        indent: SampleIndent::Zero,
                        content: "function() {\n  console.log('test')\n}".to_string(),
                    }),
                ],
            }],
            val_samples: vec![],
        };

        train_write_xmls(temp_dir.path(), &samples).unwrap();
        
        let content = std::fs::read_to_string(temp_dir.path().join("train_corpus.xml")).unwrap();

        assert!(content.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<samples>"));
        assert!(content.ends_with("</samples>\n"));
        assert!(content.contains("  <sample>"));
        assert!(content.contains("    <prompt>Hello</prompt>"));
        assert!(content.contains("    <ai>"));
        
        // Check that text and code have 6 spaces indentation
        let lines: Vec<&str> = content.lines().collect();
        for line in lines {
            if line.contains("<text>Hi</text>") {
                assert!(line.starts_with("      <text>Hi</text>"), "Text should have 6 spaces, got: {}", line);
            }
            if line.contains("<ts>") && line.contains("function") {
                assert!(line.starts_with("      <ts>"), "Code block should have 6 spaces, got: {}", line);
            }
        }
        
        assert!(content.contains("    </ai>"));
        assert!(content.contains("  </sample>"));
    }
    
    #[test]
    fn test_train_write_xmls_with_code() {
        let temp_dir = tempdir().unwrap();
        
        let samples = Samples {
            train_samples: vec![Sample {
                prompt_section: vec![
                    SamplePromptEnum::Text("Write code: ".to_string()),
                    SamplePromptEnum::Code(SampleCode {
                        lang: SampleLanguage::Js,
                        inline: false,
                        indent: SampleIndent::Zero,
                        content: "console.log('hello');".to_string(),
                    }),
                ],
                ai_section: vec![
                    SampleAiEnum::Text("Here's the code:".to_string()),
                    SampleAiEnum::Code(SampleCode {
                        lang: SampleLanguage::Js,
                        inline: false,
                        indent: SampleIndent::Zero,
                        content: "console.log('world');".to_string(),
                    }),
                ],
            }],
            val_samples: vec![],
        };

        train_write_xmls(temp_dir.path(), &samples).unwrap();
        
        let content = std::fs::read_to_string(temp_dir.path().join("train_corpus.xml")).unwrap();
        
        assert!(content.contains("      <text>Here's the code:</text>"));
        assert!(content.contains("      <js>console.log('world');</js>"));
    }
    
    #[test]
    fn test_train_write_xmls_with_indented_code() {
        let temp_dir = tempdir().unwrap();
        let samples = Samples {
            train_samples: vec![Sample {
                prompt_section: vec![
                    SamplePromptEnum::Text("Indented code:\n".to_string()),
                    SamplePromptEnum::Code(SampleCode {
                        lang: SampleLanguage::Rust,
                        inline: false,
                        indent: SampleIndent::Two,
                        content: "fn main() {\n  println!(\"hello\");\n}".to_string(),
                    }),
                ],
                ai_section: vec![
                    SampleAiEnum::Code(SampleCode {
                        lang: SampleLanguage::Rust,
                        inline: false,
                        indent: SampleIndent::Four,
                        content: "fn example() {\n    println!(\"world\");\n}".to_string(),
                    }),
                ],
            }],
            val_samples: vec![],
        };

        train_write_xmls(temp_dir.path(), &samples).unwrap();
        
        let content = std::fs::read_to_string(temp_dir.path().join("train_corpus.xml")).unwrap();
        
        assert!(content.contains("<rust indent=\"2\">"));
        assert!(content.contains("<rust indent=\"4\">"));
    }
    
    #[test]
    fn test_train_write_xmls_with_source() {
        let temp_dir = tempdir().unwrap();
        
        let samples = Samples {
            train_samples: vec![Sample {
                prompt_section: vec![
                    SamplePromptEnum::Text("What is AI?".to_string()),
                ],
                ai_section: vec![
                    SampleAiEnum::Text("Artificial Intelligence is...".to_string()),
                    SampleAiEnum::Source("wikipedia".to_string()),
                ],
            }],
            val_samples: vec![],
        };

        train_write_xmls(temp_dir.path(), &samples).unwrap();
        
        let content = std::fs::read_to_string(temp_dir.path().join("train_corpus.xml")).unwrap();
        
        assert!(content.contains("      <source>wikipedia</source>"));
    }
    
    #[test]
    fn test_train_write_xmls_multiple_samples() {
        let temp_dir = tempdir().unwrap();
        
        let samples = Samples {
            train_samples: vec![
                Sample {
                    prompt_section: vec![SamplePromptEnum::Text("First prompt".to_string())],
                    ai_section: vec![SampleAiEnum::Text("First response".to_string())],
                },
                Sample {
                    prompt_section: vec![SamplePromptEnum::Text("Second prompt".to_string())],
                    ai_section: vec![SampleAiEnum::Text("Second response".to_string())],
                },
            ],
            val_samples: vec![],
        };

        train_write_xmls(temp_dir.path(), &samples).unwrap();
        
        let content = std::fs::read_to_string(temp_dir.path().join("train_corpus.xml")).unwrap();
        
        assert!(content.contains("      <text>First response</text>"));
        assert!(content.contains("      <text>Second response</text>"));
        
        // Ensure both samples are present
        let sample_count = content.matches("<sample>").count();
        assert_eq!(sample_count, 2);
    }
}
