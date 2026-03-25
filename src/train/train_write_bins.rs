// src/train/train_write_bins.rs

use std::{fs::File, path::Path};
use crate::sample::{Samples, Sample};
use std::io::{Seek, Write, BufWriter};
use byteorder::{LittleEndian, WriteBytesExt};
use crate::bpe::{BPETokenizer, bpe_infer_tokenize};
use crate::tag::{
    tag_write_tag,
    tag_write_ai_section,
    tag_write_prompt_section,
    tag_get_byte_offset_last_ai_start,
};


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
    // Extract special tokens for tag writing (first special_token_count entries in vocab)
    let special_tokens: Vec<String> = tokenizer.vocab[..tokenizer.special_token_count as usize].to_vec();
    
    // Write training files
    write_samples_to_bins(
        output_dir,
        "train",
        &samples.train_samples,
        tokenizer,
        &special_tokens,
    )?;
    
    // Write validation files
    write_samples_to_bins(
        output_dir,
        "val",
        &samples.val_samples,
        tokenizer,
        &special_tokens,
    )?;
    
    Ok(())
}

/// Writes a collection of samples to corpus and index files
fn write_samples_to_bins(
    output_dir: &Path,
    prefix: &str,
    samples: &[Sample],
    tokenizer: &BPETokenizer,
    special_tokens: &[String],
) -> Result<(), Box<dyn std::error::Error>> {
    let corpus_path = output_dir.join(format!("{}_corpus.bin", prefix));
    let index_path = output_dir.join(format!("{}_index.bin", prefix));
    
    let corpus_file = File::create(corpus_path)?;
    let mut corpus_writer = BufWriter::new(corpus_file);
    
    let index_file = File::create(index_path)?;
    let mut index_writer = BufWriter::new(index_file);
    
    for sample in samples {
        // Record byte offset where this sample starts in the corpus
        let sample_start = corpus_writer.stream_position()?;
        
        // Convert sample to XML and tokenize
        let xml_string = sample_to_xml_string(sample, special_tokens);
        let token_ids = bpe_infer_tokenize(tokenizer, &xml_string);
        
        // Write token IDs to corpus (4 bytes each, little-endian)
        for token_id in &token_ids {
            corpus_writer.write_u32::<LittleEndian>(*token_id)?;
        }
        
        // Calculate where the last AI response begins (as token index, not byte offset)
        let last_ai_token_index = calculate_last_ai_token_index(
            sample,
            special_tokens,
            tokenizer,
            &xml_string,
        );
        
        // Record sample metadata for index
        let sample_token_count = token_ids.len() as u64;
        
        // Write index entry (24 bytes total)
        index_writer.write_u64::<LittleEndian>(sample_start)?;           // offset in corpus
        index_writer.write_u64::<LittleEndian>(sample_token_count)?;     // total tokens
        index_writer.write_u64::<LittleEndian>(last_ai_token_index as u64)?; // AI response start
    }
    
    index_writer.flush()?;
    corpus_writer.flush()?;
    
    Ok(())
}

/// Converts a sample to its XML string representation (without formatting)
/// Uses tag functions to ensure consistency with tokenizer's special tokens
fn sample_to_xml_string(
    sample: &Sample,
    special_tokens: &[String],
) -> String {
    let mut buffer = Vec::new();
    
    // Write opening <sample> tag
    tag_write_tag(&mut buffer, "sample", true, special_tokens).unwrap();
    
    // Write prompt section (user input)
    tag_write_prompt_section(&mut buffer, &sample.prompt_section, special_tokens).unwrap();
    
    // Write AI section (model responses)
    tag_write_ai_section(&mut buffer, &sample.ai_section, special_tokens).unwrap();
    
    // Write closing </sample> tag
    tag_write_tag(&mut buffer, "sample", false, special_tokens).unwrap();
    
    String::from_utf8(buffer).unwrap()
}

/// Calculates the number of tokens that appear before the last AI response
///
/// This function determines where the target training tokens begin by:
/// 1. Getting the byte offset of the last AI response within the XML
/// 2. Extracting the XML prefix up to that offset
/// 3. Tokenizing the prefix to count tokens before the AI response
///
/// Returns the token index where the last AI response starts (0-indexed)
fn calculate_last_ai_token_index(
    sample: &Sample,
    special_tokens: &[String],
    tokenizer: &BPETokenizer,
    full_xml: &str,
) -> usize {
    // Get byte offset within XML where last AI response content begins
    let xml_byte_offset = tag_get_byte_offset_last_ai_start(&sample.ai_section, special_tokens);
    
    // Extract the XML prefix (everything before the last AI response)
    let prefix = &full_xml[..xml_byte_offset];
    
    // Tokenize prefix to count tokens before AI response
    bpe_infer_tokenize(tokenizer, prefix).len()
}


#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile::tempdir;
    use std::io::{Read, Seek};
    use super::train_write_bins;
    use crate::sample::{
        Sample,
        Samples,
        SampleCode,
        SampleIndent,
        SampleAiEnum,
        SampleLanguage,
        SampleLineBreak,
        SamplePromptEnum,
    };
    use crate::bpe::{
        bpe_train,
        BPETokenizer,
        bpe_get_special_tokens,
    };

    fn create_test_tokenizer(samples: &[Sample]) -> BPETokenizer {
        let special_tokens = bpe_get_special_tokens();
        bpe_train(samples, &special_tokens, &[], 2).unwrap()
    }

    fn create_basic_sample(prompt: &str, response: &str) -> Sample {
        Sample {
            prompt_section: vec![SamplePromptEnum::Text(prompt.to_string())],
            ai_section: vec![
                SampleAiEnum::Text(response.to_owned()),
            ],
        }
    }

    fn read_index_entry(file: &mut fs::File, entry_num: u64) -> (u64, u64, u64) {
        let offset = entry_num * 24;
        file.seek(std::io::SeekFrom::Start(offset)).unwrap();
        
        let mut buf = [0u8; 24];
        file.read_exact(&mut buf).unwrap();
        
        let start = u64::from_le_bytes(buf[0..8].try_into().unwrap());
        let length = u64::from_le_bytes(buf[8..16].try_into().unwrap());
        let last_ai = u64::from_le_bytes(buf[16..24].try_into().unwrap());
        
        (start, length, last_ai)
    }

    fn read_corpus_tokens(file: &mut fs::File, start: u64, count: u64) -> Vec<u32> {
        file.seek(std::io::SeekFrom::Start(start)).unwrap();
        
        let mut tokens = Vec::with_capacity(count as usize);
        let mut buf = [0u8; 4];
        
        for _ in 0..count {
            file.read_exact(&mut buf).unwrap();
            tokens.push(u32::from_le_bytes(buf));
        }
        
        tokens
    }

    #[test]
    fn test_train_write_bins_basic() {
        let temp_dir = tempdir().unwrap();
        
        let train_sample = create_basic_sample("Hello", "Hi");
        let val_sample = create_basic_sample("Question", "Answer");
        
        let samples = Samples {
            train_samples: vec![train_sample],
            val_samples: vec![val_sample],
        };
        
        let tokenizer = create_test_tokenizer(&samples.train_samples);
        train_write_bins(temp_dir.path(), &samples, &tokenizer).unwrap();
        
        // Verify files exist
        assert!(temp_dir.path().join("train_corpus.bin").exists());
        assert!(temp_dir.path().join("train_index.bin").exists());
        assert!(temp_dir.path().join("val_corpus.bin").exists());
        assert!(temp_dir.path().join("val_index.bin").exists());
        
        // Verify index size (24 bytes per sample)
        let train_index_metadata = fs::metadata(temp_dir.path().join("train_index.bin")).unwrap();
        assert_eq!(train_index_metadata.len(), 24);
        
        let val_index_metadata = fs::metadata(temp_dir.path().join("val_index.bin")).unwrap();
        assert_eq!(val_index_metadata.len(), 24);
        
        // Read and verify training index
        let mut index_file = fs::File::open(temp_dir.path().join("train_index.bin")).unwrap();
        let (start, length, last_ai_token) = read_index_entry(&mut index_file, 0);
        
        // Verify index values
        assert_eq!(start, 0);
        assert!(length > 0);
        assert!(last_ai_token < length);
        
        // Read and verify corpus tokens
        let mut corpus_file = fs::File::open(temp_dir.path().join("train_corpus.bin")).unwrap();
        let tokens = read_corpus_tokens(&mut corpus_file, start, length);
        
        // Verify last_ai_token_index points within the tokens
        assert!((last_ai_token as usize) <= tokens.len());
        
        // Verify AI response tokens exist (they should be after last_ai_token_index)
        let response_tokens = &tokens[last_ai_token as usize..];
        assert!(!response_tokens.is_empty());
    }
    
    #[test]
    fn test_train_write_bins_multiple_samples() {
        let temp_dir = tempdir().unwrap();
        
        let samples = Samples {
            train_samples: vec![
                create_basic_sample("First prompt", "First response"),
                create_basic_sample("Second prompt", "Second response"),
                create_basic_sample("Third prompt", "Third response"),
            ],
            val_samples: vec![],
        };
        
        let tokenizer = create_test_tokenizer(&samples.train_samples);
        train_write_bins(temp_dir.path(), &samples, &tokenizer).unwrap();
        
        // Verify index size (3 samples = 72 bytes)
        let index_metadata = fs::metadata(temp_dir.path().join("train_index.bin")).unwrap();
        assert_eq!(index_metadata.len(), 72);
        
        // Read all index entries and verify they point to valid data
        let mut index_file = fs::File::open(temp_dir.path().join("train_index.bin")).unwrap();
        let mut corpus_file = fs::File::open(temp_dir.path().join("train_corpus.bin")).unwrap();
        
        let mut prev_end = 0;
        for i in 0..3 {
            let (start, length, last_ai_token) = read_index_entry(&mut index_file, i);
            
            // Verify offsets are increasing and non-overlapping
            assert!(start >= prev_end);
            prev_end = start + length * 4; // Each token is 4 bytes
            
            // Verify last_ai_token_index is within bounds
            assert!(last_ai_token <= length);
            
            // Verify we can read tokens
            let tokens = read_corpus_tokens(&mut corpus_file, start, length);
            assert_eq!(tokens.len() as u64, length);
            
            // Verify AI response tokens exist
            let response_tokens = &tokens[last_ai_token as usize..];
            assert!(!response_tokens.is_empty());
        }
    }
    
    #[test]
    fn test_train_write_bins_with_code() {
        let temp_dir = tempdir().unwrap();
        
        let sample = Sample {
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
        };
        
        let samples = Samples {
            train_samples: vec![sample],
            val_samples: vec![],
        };
        
        let tokenizer = create_test_tokenizer(&samples.train_samples);
        train_write_bins(temp_dir.path(), &samples, &tokenizer).unwrap();
        
        // Verify files exist
        assert!(temp_dir.path().join("train_corpus.bin").exists());
        assert!(temp_dir.path().join("train_index.bin").exists());
        
        // Read and verify index
        let mut index_file = fs::File::open(temp_dir.path().join("train_index.bin")).unwrap();
        let (start, length, last_ai_token) = read_index_entry(&mut index_file, 0);
        
        assert!(length > 0);
        assert!(last_ai_token < length);
        
        // Read tokens and verify they exist
        let mut corpus_file = fs::File::open(temp_dir.path().join("train_corpus.bin")).unwrap();
        let tokens = read_corpus_tokens(&mut corpus_file, start, length);
        assert!(!tokens.is_empty());
    }
    
    #[test]
    fn test_train_write_bins_with_line_breaks() {
        let temp_dir = tempdir().unwrap();
        
        let sample = Sample {
            prompt_section: vec![
                SamplePromptEnum::Text("Line 1".to_string()),
                SamplePromptEnum::LineBreak(SampleLineBreak { count: 1 }),
                SamplePromptEnum::Text("Line 2".to_string()),
            ],
            ai_section: vec![
                SampleAiEnum::Text( "Response 1".to_string()),
                SampleAiEnum::LineBreak(SampleLineBreak { count: 2 }),
                SampleAiEnum::Text("Response 2".to_string()),
            ],
        };
        
        let samples = Samples {
            train_samples: vec![sample],
            val_samples: vec![],
        };
        
        let tokenizer = create_test_tokenizer(&samples.train_samples);
        train_write_bins(temp_dir.path(), &samples, &tokenizer).unwrap();
        
        assert!(temp_dir.path().join("train_corpus.bin").exists());
        assert!(temp_dir.path().join("train_index.bin").exists());
        
        let mut index_file = fs::File::open(temp_dir.path().join("train_index.bin")).unwrap();
        let (start, length, last_ai_token) = read_index_entry(&mut index_file, 0);
        
        assert!(length > 0);
        assert!(last_ai_token < length);
        
        // Verify we can read tokens
        let mut corpus_file = fs::File::open(temp_dir.path().join("train_corpus.bin")).unwrap();
        let tokens = read_corpus_tokens(&mut corpus_file, start, length);
        assert_eq!(tokens.len() as u64, length);
        
        // Verify there are tokens after last_ai_token (AI responses)
        let response_tokens = &tokens[last_ai_token as usize..];
        assert!(!response_tokens.is_empty());
    }
    
    #[test]
    fn test_train_write_bins_empty_sections() {
        let temp_dir = tempdir().unwrap();
        
        let sample = Sample {
            prompt_section: vec![],
            ai_section: vec![
                SampleAiEnum::Text("Just a response".to_string()),
            ],
        };
        
        let samples = Samples {
            train_samples: vec![sample],
            val_samples: vec![],
        };
        
        let tokenizer = create_test_tokenizer(&samples.train_samples);
        train_write_bins(temp_dir.path(), &samples, &tokenizer).unwrap();
        
        let mut index_file = fs::File::open(temp_dir.path().join("train_index.bin")).unwrap();
        let (start, length, last_ai_token) = read_index_entry(&mut index_file, 0);
        
        // Empty prompt section should still produce valid tokens
        assert!(length > 0);
        assert!(last_ai_token < length);
        
        // Verify AI response tokens exist
        let mut corpus_file = fs::File::open(temp_dir.path().join("train_corpus.bin")).unwrap();
        let tokens = read_corpus_tokens(&mut corpus_file, start, length);
        let response_tokens = &tokens[last_ai_token as usize..];
        assert!(!response_tokens.is_empty());
    }
    
    #[test]
    fn test_train_write_bins_validates_index_consistency() {
        let temp_dir = tempdir().unwrap();
        
        let samples = Samples {
            train_samples: vec![
                create_basic_sample("First", "Response A"),
                create_basic_sample("Second", "Response B"),
            ],
            val_samples: vec![
                create_basic_sample("Third", "Response C"),
            ],
        };
        
        let tokenizer = create_test_tokenizer(&samples.train_samples);
        train_write_bins(temp_dir.path(), &samples, &tokenizer).unwrap();
        
        // Read all index entries and verify they don't overlap
        let mut train_index = fs::File::open(temp_dir.path().join("train_index.bin")).unwrap();
        let mut train_corpus = fs::File::open(temp_dir.path().join("train_corpus.bin")).unwrap();
        
        let mut prev_end = 0;
        for i in 0..2 {
            let (start, length, last_ai_token) = read_index_entry(&mut train_index, i);
            
            // Verify start is after previous end
            assert!(start >= prev_end);
            prev_end = start + length * 4;
            
            // Verify last_ai_token is within bounds
            assert!(last_ai_token <= length);
            
            // Verify we can read tokens
            let tokens = read_corpus_tokens(&mut train_corpus, start, length);
            assert_eq!(tokens.len() as u64, length);
            assert!((last_ai_token as usize) <= tokens.len());
            
            // Verify AI response exists
            let response_tokens = &tokens[last_ai_token as usize..];
            assert!(!response_tokens.is_empty());
        }
        
        // Verify validation index
        let mut val_index = fs::File::open(temp_dir.path().join("val_index.bin")).unwrap();
        let mut val_corpus = fs::File::open(temp_dir.path().join("val_corpus.bin")).unwrap();
        
        let (start, length, last_ai_token) = read_index_entry(&mut val_index, 0);
        assert_eq!(start, 0);
        assert!(length > 0);
        assert!(last_ai_token < length);
        
        let tokens = read_corpus_tokens(&mut val_corpus, start, length);
        assert_eq!(tokens.len() as u64, length);
        let response_tokens = &tokens[last_ai_token as usize..];
        assert!(!response_tokens.is_empty());
    }
}
