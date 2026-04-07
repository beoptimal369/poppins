// src/train/train_write_txts.rs

use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::tag::TagWriter;
use crate::sample::Samples;


/// Writes training and validation text files from samples
///
/// This function creates human-readable text files for both training and validation samples.
/// The files are formatted with proper indentation for easy viewing and debugging.
///
/// # Arguments
/// * `output_dir` - Directory where files will be written
/// * `samples` - Samples struct containing both train and validation samples
///
/// # Files Created
/// * `train_corpus.txt` - Human-readable text for training samples
/// * `val_corpus.txt` - Human-readable text for validation samples
pub fn train_write_txts(
    output_dir: &Path,
    samples: &Samples,
) -> Result<(), Box<dyn std::error::Error>> {
    // Write training text
    let train_corpus = create_corpus_string(&samples.train_samples);
    let train_path = output_dir.join("train_corpus.txt");
    let mut train_file = File::create(&train_path)?;
    train_file.write_all(train_corpus.as_bytes())?;
    println!("✅ Wrote {:?}", &train_path);

    // Write validation text
    let val_corpus = create_corpus_string(&samples.val_samples);
    let val_path = output_dir.join("val_corpus.txt");
    let mut val_file = File::create(&val_path)?;
    val_file.write_all(val_corpus.as_bytes())?;
    println!("✅ Wrote {:?}", &val_path);
    
    Ok(())
}

/// Creates a corpus string from training samples in the specified XML format
fn create_corpus_string(samples: &[crate::sample::Sample]) -> String {
    let mut buffer = Vec::new();
    let mut writer = TagWriter::new(&mut buffer, true, 2); // Pretty mode with 2 spaces
    
    // We don't need special tokens for pretty XML generation
    let special_tokens: Vec<String> = vec![];
    
    for (idx, sample) in samples.iter().enumerate() {
        // Add a newline before each sample (except the first one)
        if idx > 0 {
            writer.write_newline().unwrap();
        }
        
        // Sample wrapper
        writer.write_simple_tag_open("sample", &special_tokens, true).unwrap();
        writer.indent();
        
        // <system> - Now using Option
        if let Some(system) = &sample.system {
            if !system.is_empty() {
                writer.write_simple_tag_open("system", &special_tokens, false).unwrap();
                writer.write_text(system).unwrap();
                writer.write_simple_tag_close("system", &special_tokens, true).unwrap();
            }
        }
        
        // <prompt> - Now with wrapped content like ai_section
        writer.write_simple_tag_open("prompt", &special_tokens, true).unwrap();
        writer.indent();
        
        for prompt_item in &sample.prompt_section {
            match prompt_item {
                crate::sample::SamplePromptEnum::Text(text) => {
                    writer.write_tag_pair("text", text, &special_tokens, true).unwrap();
                }
                crate::sample::SamplePromptEnum::Code(code) => {
                    writer.write_code_open(code.lang.as_str(), code.inline, code.indent, &special_tokens).unwrap();
                    writer.write_text(&code.content).unwrap();
                    writer.write_code_close(code.lang.as_str(), &special_tokens, true).unwrap();
                }
                crate::sample::SamplePromptEnum::LineBreak(lb) => {
                    writer.write_line_break(lb, &special_tokens, true).unwrap();
                }
            }
        }
        
        writer.outdent();
        writer.write_simple_tag_close("prompt", &special_tokens, true).unwrap();
        
        // <thought>
        if let Some(thought) = &sample.thought {
            if !thought.is_empty() {
                writer.write_simple_tag_open("thought", &special_tokens, false).unwrap();
                writer.write_text(thought).unwrap();
                writer.write_simple_tag_close("thought", &special_tokens, true).unwrap();
            }
        }
        
        // <ai>
        writer.write_simple_tag_open("ai", &special_tokens, true).unwrap();
        writer.indent();
        
        for ai_item in &sample.ai_section {
            match ai_item {
                crate::sample::SampleAiEnum::Text(text) => {
                    writer.write_tag_pair("text", text, &special_tokens, true).unwrap();
                }
                crate::sample::SampleAiEnum::Source(source) => {
                    writer.write_tag_pair("source", source, &special_tokens, true).unwrap();
                }
                crate::sample::SampleAiEnum::Code(code) => {
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
        
        // Add an extra newline after each sample
        writer.write_newline().unwrap();
    }
    
    String::from_utf8(buffer).unwrap()
}



#[cfg(test)]
mod tests {
    use tempfile::tempdir;
    use super::train_write_txts;
    use crate::sample::{
        Sample,
        Samples,
        SampleCode,
        SampleAiEnum,
        SampleIndent,
        SampleLanguage,
        SamplePromptEnum,
        SampleLineBreak,
    };

    #[test]
    fn test_train_write_txts_basic() {
        let temp_dir = tempdir().unwrap();
        
        let samples = Samples {
            train_samples: vec![
                Sample {
                    system: None,
                    thought: None,
                    prompt_section: vec![SamplePromptEnum::Text("Hello".to_string())],
                    ai_section: vec![
                        SampleAiEnum::Text("Hi".to_string()),
                    ],
                },
            ],
            val_samples: vec![
                Sample {
                    system: None,
                    thought: None,
                    prompt_section: vec![SamplePromptEnum::Text("Question".to_string())],
                    ai_section: vec![
                        SampleAiEnum::Text("Answer".to_string()),
                    ],
                },
            ],
        };
        
        train_write_txts(temp_dir.path(), &samples).unwrap();
        
        // Check that files were created
        assert!(temp_dir.path().join("train_corpus.txt").exists());
        assert!(temp_dir.path().join("val_corpus.txt").exists());
        
        // Read and verify content - no wrapper samples tag
        let train_content = std::fs::read_to_string(temp_dir.path().join("train_corpus.txt")).unwrap();
        assert!(train_content.contains("<sample>"));
        assert!(train_content.contains("  <prompt>"));
        assert!(train_content.contains("    <text>Hello</text>"));
        assert!(train_content.contains("  </prompt>"));
        assert!(train_content.contains("  <ai>"));
        assert!(train_content.contains("    <text>Hi</text>"));
        assert!(train_content.contains("  </ai>"));
        assert!(train_content.contains("</sample>"));
        
        let val_content = std::fs::read_to_string(temp_dir.path().join("val_corpus.txt")).unwrap();
        assert!(val_content.contains("<sample>"));
        assert!(val_content.contains("    <text>Question</text>"));
        assert!(val_content.contains("    <text>Answer</text>"));
    }

    #[test]
    fn test_train_write_txts_with_system() {
        let temp_dir = tempdir().unwrap();
        
        let samples = Samples {
            train_samples: vec![
                Sample {
                    system: Some("You are a helpful assistant.".to_string()),
                    thought: None,
                    prompt_section: vec![SamplePromptEnum::Text("Hello".to_string())],
                    ai_section: vec![
                        SampleAiEnum::Text("Hi".to_string()),
                    ],
                },
            ],
            val_samples: vec![],
        };
        
        train_write_txts(temp_dir.path(), &samples).unwrap();
        
        let content = std::fs::read_to_string(temp_dir.path().join("train_corpus.txt")).unwrap();
        
        // Verify system and prompt are separate tags with correct order
        let system_pos = content.find("  <system>").unwrap();
        let prompt_pos = content.find("  <prompt>").unwrap();
        assert!(system_pos < prompt_pos, "System should come before prompt");
        
        assert!(content.contains("  <system>You are a helpful assistant.</system>"));
        assert!(content.contains("    <text>Hello</text>"));
    }

    #[test]
    fn test_train_write_txts_with_thought() {
        let temp_dir = tempdir().unwrap();
        
        let samples = Samples {
            train_samples: vec![
                Sample {
                    system: Some("You are a helpful assistant.".to_string()),
                    thought: Some("1. Understand the question\n2. Provide a clear answer".to_string()),
                    prompt_section: vec![SamplePromptEnum::Text("Hello".to_string())],
                    ai_section: vec![
                        SampleAiEnum::Text("Hi".to_string()),
                    ],
                },
            ],
            val_samples: vec![],
        };
        
        train_write_txts(temp_dir.path(), &samples).unwrap();
        
        let content = std::fs::read_to_string(temp_dir.path().join("train_corpus.txt")).unwrap();
        
        // Verify correct order: system > prompt > thought > ai
        let system_pos = content.find("  <system>").unwrap();
        let prompt_pos = content.find("  <prompt>").unwrap();
        let thought_pos = content.find("  <thought>").unwrap();
        let ai_pos = content.find("  <ai>").unwrap();
        
        assert!(system_pos < prompt_pos, "System should come before prompt");
        assert!(prompt_pos < thought_pos, "Prompt should come before thought");
        assert!(thought_pos < ai_pos, "Thought should come before AI");
        
        // Verify thought tag and content
        assert!(content.contains("  <thought>1. Understand the question\n2. Provide a clear answer</thought>"));
    }

    #[test]
    fn test_train_write_txts_with_multiline_system() {
        let temp_dir = tempdir().unwrap();
        
        let samples = Samples {
            train_samples: vec![
                Sample {
                    system: Some("System instruction 1.\nSystem instruction 2.\n".to_string()),
                    thought: None,
                    prompt_section: vec![SamplePromptEnum::Text("User prompt".to_string())],
                    ai_section: vec![
                        SampleAiEnum::Text("Response".to_string()),
                    ],
                },
            ],
            val_samples: vec![],
        };
        
        train_write_txts(temp_dir.path(), &samples).unwrap();
        
        let content = std::fs::read_to_string(temp_dir.path().join("train_corpus.txt")).unwrap();
        
        // System content should be in its own tag
        assert!(content.contains("  <system>System instruction 1.\nSystem instruction 2.\n</system>"));
        assert!(content.contains("    <text>User prompt</text>"));
    }

    #[test]
    fn test_train_write_txts_formatting() {
        let temp_dir = tempdir().unwrap();

        let samples = Samples {
            train_samples: vec![Sample {
                system: None,
                thought: None,
                prompt_section: vec![SamplePromptEnum::Text("Hello".to_string())],
                ai_section: vec![
                    SampleAiEnum::Text("Hi".to_string()),
                    SampleAiEnum::Code(SampleCode {
                        lang: SampleLanguage::Ts,
                        inline: false,
                        indent: None,
                        content: "function() {\n  console.log('test')\n}".to_string(),
                    }),
                ],
            }],
            val_samples: vec![],
        };

        train_write_txts(temp_dir.path(), &samples).unwrap();
        
        let content = std::fs::read_to_string(temp_dir.path().join("train_corpus.txt")).unwrap();

        assert!(content.contains("<sample>"));
        assert!(content.contains("  <prompt>"));
        assert!(content.contains("    <text>Hello</text>"));
        assert!(content.contains("  </prompt>"));
        assert!(content.contains("  <ai>"));
        
        // Check that text and code have proper indentation
        let lines: Vec<&str> = content.lines().collect();
        for line in lines {
            if line.contains("<text>Hi</text>") {
                assert!(line.starts_with("    <text>Hi</text>"), "Text should have 4 spaces, got: {}", line);
            }
            if line.contains("<ts>") && line.contains("function") {
                assert!(line.starts_with("    <ts>"), "Code block should have 4 spaces, got: {}", line);
            }
        }
        
        assert!(content.contains("  </ai>"));
        assert!(content.contains("</sample>"));
    }
    
    #[test]
    fn test_train_write_txts_with_code() {
        let temp_dir = tempdir().unwrap();
        
        let samples = Samples {
            train_samples: vec![Sample {
                system: None,
                thought: None,
                prompt_section: vec![
                    SamplePromptEnum::Text("Write code: ".to_string()),
                    SamplePromptEnum::Code(SampleCode {
                        lang: SampleLanguage::Js,
                        inline: false,
                        indent: None,
                        content: "console.log('hello');".to_string(),
                    }),
                ],
                ai_section: vec![
                    SampleAiEnum::Text("Here's the code:".to_string()),
                    SampleAiEnum::Code(SampleCode {
                        lang: SampleLanguage::Js,
                        inline: false,
                        indent: None,
                        content: "console.log('world');".to_string(),
                    }),
                ],
            }],
            val_samples: vec![],
        };

        train_write_txts(temp_dir.path(), &samples).unwrap();
        
        let content = std::fs::read_to_string(temp_dir.path().join("train_corpus.txt")).unwrap();
        
        assert!(content.contains("    <text>Write code: </text>"));
        assert!(content.contains("    <js>console.log('hello');</js>"));
        assert!(content.contains("    <text>Here's the code:</text>"));
        assert!(content.contains("    <js>console.log('world');</js>"));
    }
    
    #[test]
    fn test_train_write_txts_with_indented_code() {
        let temp_dir = tempdir().unwrap();
        let samples = Samples {
            train_samples: vec![Sample {
                system: None,
                thought: None,
                prompt_section: vec![
                    SamplePromptEnum::Text("Indented code:\n".to_string()),
                    SamplePromptEnum::Code(SampleCode {
                        lang: SampleLanguage::Rust,
                        inline: false,
                        indent: Some(SampleIndent::Two),
                        content: "fn main() {\n  println!(\"hello\");\n}".to_string(),
                    }),
                ],
                ai_section: vec![
                    SampleAiEnum::Code(SampleCode {
                        lang: SampleLanguage::Rust,
                        inline: false,
                        indent: Some(SampleIndent::Four),
                        content: "fn example() {\n    println!(\"world\");\n}".to_string(),
                    }),
                ],
            }],
            val_samples: vec![],
        };

        train_write_txts(temp_dir.path(), &samples).unwrap();
        
        let content = std::fs::read_to_string(temp_dir.path().join("train_corpus.txt")).unwrap();
        
        assert!(content.contains("<rust indent=\"2\">"));
        assert!(content.contains("<rust indent=\"4\">"));
    }
    
    #[test]
    fn test_train_write_txts_with_source() {
        let temp_dir = tempdir().unwrap();
        
        let samples = Samples {
            train_samples: vec![Sample {
                system: None,
                thought: None,
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

        train_write_txts(temp_dir.path(), &samples).unwrap();
        
        let content = std::fs::read_to_string(temp_dir.path().join("train_corpus.txt")).unwrap();
        
        assert!(content.contains("    <source>wikipedia</source>"));
    }
    
    #[test]
    fn test_train_write_txts_multiple_samples() {
        let temp_dir = tempdir().unwrap();
        
        let samples = Samples {
            train_samples: vec![
                Sample {
                    system: None,
                    thought: None,
                    prompt_section: vec![SamplePromptEnum::Text("First prompt".to_string())],
                    ai_section: vec![SampleAiEnum::Text("First response".to_string())],
                },
                Sample {
                    system: None,
                    thought: None,
                    prompt_section: vec![SamplePromptEnum::Text("Second prompt".to_string())],
                    ai_section: vec![SampleAiEnum::Text("Second response".to_string())],
                },
            ],
            val_samples: vec![],
        };

        train_write_txts(temp_dir.path(), &samples).unwrap();
        
        let content = std::fs::read_to_string(temp_dir.path().join("train_corpus.txt")).unwrap();
        
        assert!(content.contains("    <text>First response</text>"));
        assert!(content.contains("    <text>Second response</text>"));
        
        // Ensure both samples are present (no wrapper tag)
        let sample_count = content.matches("<sample>").count();
        assert_eq!(sample_count, 2);
    }
    
    #[test]
    fn test_train_write_txts_with_prompt_code_and_line_break() {
        let temp_dir = tempdir().unwrap();
        
        let samples = Samples {
            train_samples: vec![Sample {
                system: Some("You are a coding assistant.".to_string()),
                thought: None,
                prompt_section: vec![
                    SamplePromptEnum::Text("Write a function:\n".to_string()),
                    SamplePromptEnum::Code(SampleCode {
                        lang: SampleLanguage::Rust,
                        inline: false,
                        indent: Some(SampleIndent::One),
                        content: "fn hello() {\n  println!(\"world\");\n}".to_string(),
                    }),
                    SamplePromptEnum::LineBreak(SampleLineBreak { count: 2 }),
                    SamplePromptEnum::Text("Now explain it.".to_string()),
                ],
                ai_section: vec![
                    SampleAiEnum::Text("This function prints 'world'.".to_string()),
                ],
            }],
            val_samples: vec![],
        };

        train_write_txts(temp_dir.path(), &samples).unwrap();
        
        let content = std::fs::read_to_string(temp_dir.path().join("train_corpus.txt")).unwrap();
        
        assert!(content.contains("  <system>You are a coding assistant.</system>"));
        assert!(content.contains("    <text>Write a function:\n</text>"));
        assert!(content.contains("    <rust indent=\"1\">fn hello() {\n  println!(\"world\");\n}</rust>"));
        assert!(content.contains("    <line-break count=\"2\" />"));
        assert!(content.contains("    <text>Now explain it.</text>"));
    }
}
