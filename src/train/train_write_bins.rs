// src/train/train_write_bins.rs

use bytemuck::cast_slice;
use std::{fs::File, path::Path};
use crate::sample::{Samples, Sample};
use std::io::{Seek, Write, BufWriter};
use byteorder::{LittleEndian, WriteBytesExt};
use crate::bpe::{create_bpe_cache, get_bpe_cache_tokens, BPECache, BPETokenizer};


/// Writes training and validation corpus and index binary files
/// 
/// This function creates two types of files for both training and validation sets:
/// - Corpus files: contain token IDs (u32) for all samples, concatenated
/// - Index files: allow O(1) random access to any sample and its AI response
///
/// # Arguments
/// * `output_dir` - Directory where files will be written
/// * `samples` - Samples struct containing both train and validation samples
/// * `tokenizer` - Trained BPE tokenizer for converting text to token IDs
///
/// # Files Created
/// * `train_corpus.bin` - Token IDs for training samples (little-endian u32)
/// * `train_index.bin` - Index entries for training samples
/// * `val_corpus.bin` - Token IDs for validation samples (little-endian u32)
/// * `val_index.bin` - Index entries for validation samples
///
/// # Index Format (24 bytes per sample)
/// | Field               | Type | Bytes | Description                          |
/// |---------------------|------|-------|--------------------------------------|
/// | offset              | u64  | 8     | Byte offset in corpus file           |
/// | token_count         | u64  | 8     | Total number of tokens in sample     |
/// | last_ai_token_index | u64  | 8     | Index of first token of last AI response |
pub fn train_write_bins(
    output_dir: &Path,
    samples: &Samples,
    tokenizer: &BPETokenizer,
) -> Result<(), Box<dyn std::error::Error>> {
    // Pre-tokenize all static XML parts
    let cache = create_bpe_cache(tokenizer);
    
    write_samples_to_bins(
        output_dir,
        "train",
        &samples.train_samples,
        tokenizer,
        &cache,
    )?;
    
    write_samples_to_bins(
        output_dir,
        "val",
        &samples.val_samples,
        tokenizer,
        &cache,
    )?;
    
    Ok(())
}


fn write_samples_to_bins(
    output_dir: &Path,
    prefix: &str,
    samples: &[Sample],
    tokenizer: &BPETokenizer,
    cache: &BPECache,
) -> Result<(), Box<dyn std::error::Error>> {
    let corpus_path = output_dir.join(format!("{}_corpus.bin", prefix));
    let index_path = output_dir.join(format!("{}_index.bin", prefix));
    
    let corpus_file = File::create(&corpus_path)?;
    let mut corpus_writer = BufWriter::with_capacity(1024 * 1024, corpus_file);

    let index_file = File::create(&index_path)?;
    let mut index_writer = BufWriter::new(index_file);
    
    for sample in samples.iter() {
        let sample_start = corpus_writer.stream_position()?;
        
        let mut token_count = 0;
        let mut tokens_before_last_ai = 0;
        let total_ai_items = sample.ai_section.len();
        
        // Write <sample>
        corpus_writer.write_all(cast_slice(&cache.sample_open))?;
        token_count += cache.sample_open.len();
        tokens_before_last_ai += cache.sample_open.len();
        
        // Write system if present
        if !sample.system.is_empty() {
            corpus_writer.write_all(cast_slice(&cache.system_open))?;
            token_count += cache.system_open.len();
            tokens_before_last_ai += cache.system_open.len();
            
            let system_tokens = get_bpe_cache_tokens(tokenizer, &sample.system);
            corpus_writer.write_all(cast_slice(&system_tokens))?;
            token_count += system_tokens.len();
            tokens_before_last_ai += system_tokens.len();
            
            corpus_writer.write_all(cast_slice(&cache.system_close))?;
            token_count += cache.system_close.len();
            tokens_before_last_ai += cache.system_close.len();
        }
        
        // Write prompt section
        corpus_writer.write_all(cast_slice(&cache.prompt_open))?;
        token_count += cache.prompt_open.len();
        tokens_before_last_ai += cache.prompt_open.len();
        
        for item in &sample.prompt_section {
            match item {
                crate::sample::SamplePromptEnum::Text(text) => {
                    let text_tokens = get_bpe_cache_tokens(tokenizer, text);
                    corpus_writer.write_all(cast_slice(&text_tokens))?;
                    token_count += text_tokens.len();
                    tokens_before_last_ai += text_tokens.len();
                }
                crate::sample::SamplePromptEnum::Code(code) => {
                    let code_tag = format!("<{}>", code.lang.as_str());
                    let code_open = get_bpe_cache_tokens(tokenizer, &code_tag);
                    let code_close = get_bpe_cache_tokens(tokenizer, &format!("</{}>", code.lang.as_str()));
                    
                    corpus_writer.write_all(cast_slice(&code_open))?;
                    token_count += code_open.len();
                    tokens_before_last_ai += code_open.len();
                    
                    let content_tokens = get_bpe_cache_tokens(tokenizer, &code.content);
                    corpus_writer.write_all(cast_slice(&content_tokens))?;
                    token_count += content_tokens.len();
                    tokens_before_last_ai += content_tokens.len();
                    
                    corpus_writer.write_all(cast_slice(&code_close))?;
                    token_count += code_close.len();
                    tokens_before_last_ai += code_close.len();
                }
                crate::sample::SamplePromptEnum::LineBreak(lb) => {
                    if lb.count == 1 {
                        corpus_writer.write_all(cast_slice(&cache.line_break_single))?;
                        token_count += cache.line_break_single.len();
                        tokens_before_last_ai += cache.line_break_single.len();
                    } else {
                        corpus_writer.write_all(cast_slice(&cache.line_break_double))?;
                        token_count += cache.line_break_double.len();
                        tokens_before_last_ai += cache.line_break_double.len();
                    }
                }
            }
        }

        corpus_writer.write_all(cast_slice(&cache.prompt_close))?;
        token_count += cache.prompt_close.len();
        tokens_before_last_ai += cache.prompt_close.len();
        
        // Write AI section - track when we reach the last AI item
        corpus_writer.write_all(cast_slice(&cache.ai_open))?;
        token_count += cache.ai_open.len();
        tokens_before_last_ai += cache.ai_open.len();
        
        for (item_idx, item) in sample.ai_section.iter().enumerate() {
            let is_last_item = item_idx == total_ai_items - 1;
            
            match item {
                crate::sample::SampleAiEnum::Text(text) => {
                    corpus_writer.write_all(cast_slice(&cache.text_open))?;
                    token_count += cache.text_open.len();
                    
                    let text_tokens = get_bpe_cache_tokens(tokenizer, text);
                    corpus_writer.write_all(cast_slice(&text_tokens))?;
                    token_count += text_tokens.len();
                    
                    corpus_writer.write_all(cast_slice(&cache.text_close))?;
                    token_count += cache.text_close.len();
                    
                    if !is_last_item {
                        tokens_before_last_ai += cache.text_open.len() + text_tokens.len() + cache.text_close.len();
                    }
                }
                crate::sample::SampleAiEnum::Source(source) => {
                    corpus_writer.write_all(cast_slice(&cache.source_open))?;
                    token_count += cache.source_open.len();
                    
                    let source_tokens = get_bpe_cache_tokens(tokenizer, source);
                    corpus_writer.write_all(cast_slice(&source_tokens))?;
                    token_count += source_tokens.len();
                    
                    corpus_writer.write_all(cast_slice(&cache.source_close))?;
                    token_count += cache.source_close.len();
                    
                    if !is_last_item {
                        tokens_before_last_ai += cache.source_open.len() + source_tokens.len() + cache.source_close.len();
                    }
                }
                crate::sample::SampleAiEnum::Code(code) => {
                    let code_tag = format!("<{}>", code.lang.as_str());
                    let code_open = get_bpe_cache_tokens(tokenizer, &code_tag);
                    let code_close = get_bpe_cache_tokens(tokenizer, &format!("</{}>", code.lang.as_str()));
                    
                    corpus_writer.write_all(cast_slice(&code_open))?;
                    token_count += code_open.len();
                    
                    let content_tokens = get_bpe_cache_tokens(tokenizer, &code.content);
                    corpus_writer.write_all(cast_slice(&content_tokens))?;
                    token_count += content_tokens.len();
                    
                    corpus_writer.write_all(cast_slice(&code_close))?;
                    token_count += code_close.len();
                    
                    if !is_last_item {
                        tokens_before_last_ai += code_open.len() + content_tokens.len() + code_close.len();
                    }
                }
                crate::sample::SampleAiEnum::LineBreak(lb) => {
                    if lb.count == 1 {
                        corpus_writer.write_all(cast_slice(&cache.line_break_single))?;
                        token_count += cache.line_break_single.len();
                        if !is_last_item {
                            tokens_before_last_ai += cache.line_break_single.len();
                        }
                    } else {
                        corpus_writer.write_all(cast_slice(&cache.line_break_double))?;
                        token_count += cache.line_break_double.len();
                        if !is_last_item {
                            tokens_before_last_ai += cache.line_break_double.len();
                        }
                    }
                }
            }
        }
        
        corpus_writer.write_all(cast_slice(&cache.ai_close))?;
        token_count += cache.ai_close.len();
        
        // Write </sample>
        corpus_writer.write_all(cast_slice(&cache.sample_close))?;
        token_count += cache.sample_close.len();
        
        // Write index entry
        index_writer.write_u64::<LittleEndian>(sample_start)?;
        index_writer.write_u64::<LittleEndian>(token_count as u64)?;
        index_writer.write_u64::<LittleEndian>(tokens_before_last_ai as u64)?;
    }
    
    index_writer.flush()?;
    corpus_writer.flush()?;

    println!("✅ Wrote {:?}", &index_path);
    println!("✅ Wrote {:?}", &corpus_path);
    
    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use tempfile::tempdir;
    use crate::sample::{
        Sample,
        SamplePromptEnum,
        SampleAiEnum,
        SampleCode,
        SampleLanguage,
        SampleIndent,
        SampleLineBreak,
    };
    use crate::bpe::{bpe_train, bpe_get_special_tokens};

    fn create_test_tokenizer() -> BPETokenizer {
        let samples = vec![
            Sample {
                system: "You are a helpful assistant.".to_string(),
                prompt_section: vec![
                    SamplePromptEnum::Text("Hello".to_string()),
                ],
                ai_section: vec![
                    SampleAiEnum::Text("Hi there".to_string()),
                ],
            },
        ];
        
        let special_tokens = bpe_get_special_tokens();
        let requested_tokens = vec![];
        let min_merge_frequency = 1;
        
        bpe_train(&samples, &special_tokens, &requested_tokens, min_merge_frequency).unwrap()
    }

    fn create_test_sample(id: &str) -> Sample {
        Sample {
            system: format!("System {}", id),
            prompt_section: vec![
                SamplePromptEnum::Text(format!("Prompt {}", id)),
            ],
            ai_section: vec![
                SampleAiEnum::Text(format!("Response {}", id)),
            ],
        }
    }

    fn create_test_sample_with_multiple_ai_items() -> Sample {
        Sample {
            system: "System".to_string(),
            prompt_section: vec![
                SamplePromptEnum::Text("What is AI?".to_string()),
            ],
            ai_section: vec![
                SampleAiEnum::Text("First response".to_string()),
                SampleAiEnum::Source("wikipedia".to_string()),
                SampleAiEnum::Text("Second response".to_string()),
            ],
        }
    }

    fn create_test_sample_with_code() -> Sample {
        Sample {
            system: "System".to_string(),
            prompt_section: vec![
                SamplePromptEnum::Text("Write code".to_string()),
                SamplePromptEnum::Code(SampleCode {
                    lang: SampleLanguage::Js,
                    inline: false,
                    indent: Some(SampleIndent::One),
                    content: "console.log('hello')".to_string(),
                }),
            ],
            ai_section: vec![
                SampleAiEnum::Text("Here's the code".to_string()),
                SampleAiEnum::Code(SampleCode {
                    lang: SampleLanguage::Js,
                    inline: false,
                    indent: None,
                    content: "console.log('world')".to_string(),
                }),
            ],
        }
    }

    fn create_test_sample_with_line_breaks() -> Sample {
        Sample {
            system: "System".to_string(),
            prompt_section: vec![
                SamplePromptEnum::Text("Line1".to_string()),
                SamplePromptEnum::LineBreak(SampleLineBreak { count: 1 }),
                SamplePromptEnum::Text("Line2".to_string()),
            ],
            ai_section: vec![
                SampleAiEnum::Text("Response1".to_string()),
                SampleAiEnum::LineBreak(SampleLineBreak { count: 2 }),
                SampleAiEnum::Text("Response2".to_string()),
            ],
        }
    }

    #[test]
    fn test_train_write_bins_basic() {
        let temp_dir = tempdir().unwrap();
        let tokenizer = create_test_tokenizer();
        
        let train_sample = create_test_sample("1");
        let val_sample = create_test_sample("2");
        
        let samples = Samples {
            train_samples: vec![train_sample],
            val_samples: vec![val_sample],
        };
        
        train_write_bins(temp_dir.path(), &samples, &tokenizer).unwrap();
        
        // Verify files exist
        assert!(temp_dir.path().join("train_corpus.bin").exists());
        assert!(temp_dir.path().join("train_index.bin").exists());
        assert!(temp_dir.path().join("val_corpus.bin").exists());
        assert!(temp_dir.path().join("val_index.bin").exists());
        
        // Verify file sizes (corpus files should be > 0, index files 24 bytes each)
        let train_corpus_size = std::fs::metadata(temp_dir.path().join("train_corpus.bin")).unwrap().len();
        let train_index_size = std::fs::metadata(temp_dir.path().join("train_index.bin")).unwrap().len();
        let val_corpus_size = std::fs::metadata(temp_dir.path().join("val_corpus.bin")).unwrap().len();
        let val_index_size = std::fs::metadata(temp_dir.path().join("val_index.bin")).unwrap().len();
        
        assert!(train_corpus_size > 0);
        assert_eq!(train_index_size, 24); // 1 sample = 24 bytes
        assert!(val_corpus_size > 0);
        assert_eq!(val_index_size, 24);
    }

    #[test]
    fn test_train_write_bins_multiple_samples() {
        let temp_dir = tempdir().unwrap();
        let tokenizer = create_test_tokenizer();
        
        let samples = Samples {
            train_samples: vec![
                create_test_sample("1"),
                create_test_sample("2"),
                create_test_sample("3"),
            ],
            val_samples: vec![
                create_test_sample("4"),
            ],
        };
        
        train_write_bins(temp_dir.path(), &samples, &tokenizer).unwrap();
        
        let train_index_size = std::fs::metadata(temp_dir.path().join("train_index.bin")).unwrap().len();
        let val_index_size = std::fs::metadata(temp_dir.path().join("val_index.bin")).unwrap().len();
        
        assert_eq!(train_index_size, 72); // 3 samples * 24 bytes = 72
        assert_eq!(val_index_size, 24);   // 1 sample * 24 bytes = 24
    }

    #[test]
    fn test_train_write_bins_with_multiple_ai_items() {
        let temp_dir = tempdir().unwrap();
        let tokenizer = create_test_tokenizer();
        let sample = create_test_sample_with_multiple_ai_items();
        
        let samples = Samples {
            train_samples: vec![sample],
            val_samples: vec![],
        };
        
        train_write_bins(temp_dir.path(), &samples, &tokenizer).unwrap();
        
        let corpus_path = temp_dir.path().join("train_corpus.bin");
        let index_path = temp_dir.path().join("train_index.bin");
        
        assert!(corpus_path.exists());
        assert!(index_path.exists());
        
        // Verify index entry (24 bytes)
        let index_size = std::fs::metadata(&index_path).unwrap().len();
        assert_eq!(index_size, 24);
        
        // Read index to verify values
        let mut index_file = File::open(&index_path).unwrap();
        let mut buffer = [0u8; 24];
        index_file.read_exact(&mut buffer).unwrap();
        
        let offset = u64::from_le_bytes(buffer[0..8].try_into().unwrap());
        let token_count = u64::from_le_bytes(buffer[8..16].try_into().unwrap());
        let last_ai_token = u64::from_le_bytes(buffer[16..24].try_into().unwrap());
        
        assert_eq!(offset, 0);
        assert!(token_count > 0);
        assert!(last_ai_token < token_count);
        assert!(last_ai_token > 0);
    }

    #[test]
    fn test_train_write_bins_with_code() {
        let temp_dir = tempdir().unwrap();
        let tokenizer = create_test_tokenizer();
        let sample = create_test_sample_with_code();
        
        let samples = Samples {
            train_samples: vec![sample],
            val_samples: vec![],
        };
        
        train_write_bins(temp_dir.path(), &samples, &tokenizer).unwrap();
        
        let corpus_path = temp_dir.path().join("train_corpus.bin");
        let index_path = temp_dir.path().join("train_index.bin");
        
        assert!(corpus_path.exists());
        assert!(index_path.exists());
        
        let index_size = std::fs::metadata(index_path).unwrap().len();
        assert_eq!(index_size, 24);
        
        // Verify we can read the corpus (just check it's not empty)
        let corpus_size = std::fs::metadata(corpus_path).unwrap().len();
        assert!(corpus_size > 0);
    }

    #[test]
    fn test_train_write_bins_with_line_breaks() {
        let temp_dir = tempdir().unwrap();
        let tokenizer = create_test_tokenizer();
        let sample = create_test_sample_with_line_breaks();
        
        let samples = Samples {
            train_samples: vec![sample],
            val_samples: vec![],
        };
        
        train_write_bins(temp_dir.path(), &samples, &tokenizer).unwrap();
        
        let corpus_path = temp_dir.path().join("train_corpus.bin");
        let index_path = temp_dir.path().join("train_index.bin");
        
        assert!(corpus_path.exists());
        assert!(index_path.exists());
        
        let corpus_size = std::fs::metadata(corpus_path).unwrap().len();
        let index_size = std::fs::metadata(index_path).unwrap().len();
        
        assert!(corpus_size > 0);
        assert_eq!(index_size, 24);
    }

    #[test]
    fn test_train_write_bins_empty_samples() {
        let temp_dir = tempdir().unwrap();
        let tokenizer = create_test_tokenizer();
        
        let samples = Samples {
            train_samples: vec![],
            val_samples: vec![],
        };
        
        train_write_bins(temp_dir.path(), &samples, &tokenizer).unwrap();
        
        // Files should still be created (empty corpus files)
        assert!(temp_dir.path().join("train_corpus.bin").exists());
        assert!(temp_dir.path().join("train_index.bin").exists());
        assert!(temp_dir.path().join("val_corpus.bin").exists());
        assert!(temp_dir.path().join("val_index.bin").exists());
        
        // Index files should be 0 bytes (no samples)
        let train_index_size = std::fs::metadata(temp_dir.path().join("train_index.bin")).unwrap().len();
        let val_index_size = std::fs::metadata(temp_dir.path().join("val_index.bin")).unwrap().len();
        
        assert_eq!(train_index_size, 0);
        assert_eq!(val_index_size, 0);
    }

    #[test]
    fn test_train_write_bins_no_system_prompt() {
        let temp_dir = tempdir().unwrap();
        let tokenizer = create_test_tokenizer();
        
        let sample = Sample {
            system: String::new(),
            prompt_section: vec![
                SamplePromptEnum::Text("Hello".to_string()),
            ],
            ai_section: vec![
                SampleAiEnum::Text("Hi".to_string()),
            ],
        };
        
        let samples = Samples {
            train_samples: vec![sample],
            val_samples: vec![],
        };
        
        train_write_bins(temp_dir.path(), &samples, &tokenizer).unwrap();
        
        let corpus_path = temp_dir.path().join("train_corpus.bin");
        let index_path = temp_dir.path().join("train_index.bin");
        
        assert!(corpus_path.exists());
        assert!(index_path.exists());
        
        let index_size = std::fs::metadata(index_path).unwrap().len();
        assert_eq!(index_size, 24);
    }

    #[test]
    fn test_train_write_bins_verify_index_values() {
        let temp_dir = tempdir().unwrap();
        let tokenizer = create_test_tokenizer();
        let sample = create_test_sample("test");
        
        let samples = Samples {
            train_samples: vec![sample],
            val_samples: vec![],
        };
        
        train_write_bins(temp_dir.path(), &samples, &tokenizer).unwrap();
        
        let mut index_file = File::open(temp_dir.path().join("train_index.bin")).unwrap();
        let mut buffer = [0u8; 24];
        index_file.read_exact(&mut buffer).unwrap();
        
        let offset = u64::from_le_bytes(buffer[0..8].try_into().unwrap());
        let token_count = u64::from_le_bytes(buffer[8..16].try_into().unwrap());
        let last_ai_token = u64::from_le_bytes(buffer[16..24].try_into().unwrap());
        
        // offset should be 0 (first sample)
        assert_eq!(offset, 0);
        // token_count should be > 0
        assert!(token_count > 0);
        // last_ai_token should be less than token_count
        assert!(last_ai_token < token_count);
        // last_ai_token should be > 0 (since there's at least some content before the AI response)
        assert!(last_ai_token > 0);
    }

    #[test]
    fn test_train_write_bins_multiple_samples_index_order() {
        let temp_dir = tempdir().unwrap();
        let tokenizer = create_test_tokenizer();
        
        let samples = Samples {
            train_samples: vec![
                create_test_sample("first"),
                create_test_sample("second"),
                create_test_sample("third"),
            ],
            val_samples: vec![],
        };
        
        train_write_bins(temp_dir.path(), &samples, &tokenizer).unwrap();
        
        let mut index_file = File::open(temp_dir.path().join("train_index.bin")).unwrap();
        let mut previous_end = 0;
        
        for _ in 0..3 {
            let mut buffer = [0u8; 24];
            index_file.read_exact(&mut buffer).unwrap();
            
            let offset = u64::from_le_bytes(buffer[0..8].try_into().unwrap());
            let token_count = u64::from_le_bytes(buffer[8..16].try_into().unwrap());
            
            // Offsets should be increasing and non-overlapping
            assert!(offset >= previous_end);
            previous_end = offset + token_count * 4; // each token is 4 bytes
        }
    }
}
