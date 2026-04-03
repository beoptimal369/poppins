// src/bpe/bpe_token_writer.rs

use crate::bpe::BPETokenizer;
use std::io::{Write, Result};
use std::collections::HashSet;


/// A writer that tokenizes XML content for BPE training initialization
///
/// This writer processes content written to it and:
/// - Treats known XML tags (ex: `<prompt>`, `<text>`, `<js indent="2">`) as single tokens
/// - Tokenizes text content character by character
/// - Handles nested tags and attributes correctly
/// - Unknown tags are tokenized as individual characters
pub struct BPETokenWriter<'a> {
    /// The sequence to push token IDs to
    sequence: &'a mut Vec<u32>,
    
    /// The tokenizer with vocabulary for token lookup
    tokenizer: &'a BPETokenizer,
    
    /// Set of known special tokens for quick lookup
    special_tokens_set: HashSet<String>,
    
    /// Whether we are currently inside an XML tag
    in_tag: bool,
    
    /// Buffer for accumulating the current tag
    tag_buffer: Vec<u8>,
}


impl<'a> BPETokenWriter<'a> {
    /// Creates a new BPETokenWriter
    ///
    /// # Arguments
    /// * `sequence` - Mutable reference to the token sequence to fill
    /// * `tokenizer` - Reference to the tokenizer with vocabulary
    ///
    /// # Returns
    /// * `BPETokenWriter` - The new token writer
    pub fn new(sequence: &'a mut Vec<u32>, tokenizer: &'a BPETokenizer) -> Self {
        let special_tokens_set: HashSet<String> = tokenizer.vocab[..tokenizer.special_token_count as usize]
            .iter()
            .cloned()
            .collect();
        
        Self {
            sequence,
            tokenizer,
            special_tokens_set,
            in_tag: false,
            tag_buffer: Vec::new(),
        }
    }
    
    /// Flushes the current tag buffer to the sequence
    /// If the tag is a known special token, treat as single token
    /// Otherwise, treat as text and tokenize character by character
    fn flush_tag(&mut self) -> Result<()> {
        if !self.tag_buffer.is_empty() {
            let tag_string = String::from_utf8_lossy(&self.tag_buffer);
            
            // Check if this is a known special token
            if self.special_tokens_set.contains(tag_string.as_ref()) {
                // Known special token - treat as single token
                if let Some(&id) = self.tokenizer.token_to_id.get(tag_string.as_ref()) {
                    self.sequence.push(id);
                } else {
                    self.sequence.push(0);
                }
            } else {
                // Unknown tag - treat as text and tokenize character by character
                for c in tag_string.chars() {
                    let token = c.to_string();
                    if let Some(&id) = self.tokenizer.token_to_id.get(&token) {
                        self.sequence.push(id);
                    } else {
                        self.sequence.push(0);
                    }
                }
            }
            self.tag_buffer.clear();
        }
        Ok(())
    }
    
    /// Processes a single character, handling tag detection and tokenization
    fn write_char(&mut self, c: char) -> Result<()> {
        if c == '<' {
            // Start of a tag - flush any pending text and start tag buffer
            self.flush_tag()?;
            self.in_tag = true;
            self.tag_buffer.push(c as u8);
        } else if c == '>' && self.in_tag {
            // End of a tag - add to buffer and flush
            self.tag_buffer.push(c as u8);
            self.in_tag = false;
            self.flush_tag()?;
        } else if self.in_tag {
            // Inside a tag - accumulate characters
            self.tag_buffer.push(c as u8);
        } else {
            // Outside a tag - tokenize character by character
            let token = c.to_string();
            if let Some(&id) = self.tokenizer.token_to_id.get(&token) {
                self.sequence.push(id);
            } else {
                self.sequence.push(0); // <unknown>
            }
        }
        Ok(())
    }
}


impl<'a> Write for BPETokenWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let s = String::from_utf8_lossy(buf);
        for c in s.chars() {
            self.write_char(c)?;
        }
        Ok(buf.len())
    }
    
    fn flush(&mut self) -> Result<()> {
        // Flush any remaining tag buffer
        if !self.tag_buffer.is_empty() {
            let tag_string = String::from_utf8_lossy(&self.tag_buffer);
            
            if self.special_tokens_set.contains(tag_string.as_ref()) {
                // Known special token - treat as single token
                if let Some(&id) = self.tokenizer.token_to_id.get(tag_string.as_ref()) {
                    self.sequence.push(id);
                } else {
                    self.sequence.push(0);
                }
            } else {
                // Unknown or incomplete tag - treat as text
                for c in tag_string.chars() {
                    let token = c.to_string();
                    if let Some(&id) = self.tokenizer.token_to_id.get(&token) {
                        self.sequence.push(id);
                    } else {
                        self.sequence.push(0);
                    }
                }
            }
            self.tag_buffer.clear();
        }
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::io::Write;
    use super::BPETokenWriter;
    use crate::bpe::{BPETokenizer, bpe_get_special_tokens};
    
    fn create_test_tokenizer() -> BPETokenizer {
        let special_tokens = bpe_get_special_tokens();
        
        // Start with all special tokens
        let mut vocab = special_tokens.clone();
        
        // Add all ASCII printable characters
        for c in ' '..='~' {
            let token = c.to_string();
            if !vocab.contains(&token) {
                vocab.push(token);
            }
        }
        
        // Add Unicode characters
        let unicode_chars = ['é', 'á', 'í', 'ó', 'ú', 'ñ'];
        for c in unicode_chars {
            let token = c.to_string();
            if !vocab.contains(&token) {
                vocab.push(token);
            }
        }
        
        let initial_token_count = vocab.len() as u32;
        
        let mut token_to_id = HashMap::new();
        for (id, token) in vocab.iter().enumerate() {
            token_to_id.insert(token.clone(), id as u32);
        }
        
        BPETokenizer {
            vocab,
            token_to_id,
            merges: Vec::new(),
            special_token_count: special_tokens.len() as u32,
            initial_token_count,
        }
    }
    
    #[test]
    fn test_bpe_token_writer_simple_text() {
        let mut sequence = Vec::new();
        let tokenizer = create_test_tokenizer();
        
        {
            let mut writer = BPETokenWriter::new(&mut sequence, &tokenizer);
            writer.write_all(b"Hello").unwrap();
        }
        
        assert_eq!(sequence.len(), 5);
        
        let token_strings: Vec<String> = sequence.iter()
            .map(|&id| tokenizer.vocab[id as usize].clone())
            .collect();
        
        assert_eq!(token_strings[0], "H");
        assert_eq!(token_strings[1], "e");
        assert_eq!(token_strings[2], "l");
        assert_eq!(token_strings[3], "l");
        assert_eq!(token_strings[4], "o");
    }
    
    #[test]
    fn test_bpe_token_writer_simple_tag() {
        let mut sequence = Vec::new();
        let tokenizer = create_test_tokenizer();
        
        {
            let mut writer = BPETokenWriter::new(&mut sequence, &tokenizer);
            writer.write_all(b"<prompt>").unwrap();
        }
        
        assert_eq!(sequence.len(), 1);
        
        let token_strings: Vec<String> = sequence.iter()
            .map(|&id| tokenizer.vocab[id as usize].clone())
            .collect();
        
        assert_eq!(token_strings[0], "<prompt>");
    }
    
    #[test]
    fn test_bpe_token_writer_tag_with_content() {
        let mut sequence = Vec::new();
        let tokenizer = create_test_tokenizer();
        
        {
            let mut writer = BPETokenWriter::new(&mut sequence, &tokenizer);
            writer.write_all(b"<prompt>Hello</prompt>").unwrap();
        }
        
        assert_eq!(sequence.len(), 7);
        
        let token_strings: Vec<String> = sequence.iter()
            .map(|&id| tokenizer.vocab[id as usize].clone())
            .collect();
        
        assert_eq!(token_strings[0], "<prompt>");
        assert_eq!(token_strings[1], "H");
        assert_eq!(token_strings[2], "e");
        assert_eq!(token_strings[3], "l");
        assert_eq!(token_strings[4], "l");
        assert_eq!(token_strings[5], "o");
        assert_eq!(token_strings[6], "</prompt>");
    }
    
    #[test]
    fn test_bpe_token_writer_nested_tags() {
        let mut sequence = Vec::new();
        let tokenizer = create_test_tokenizer();
        
        {
            let mut writer = BPETokenWriter::new(&mut sequence, &tokenizer);
            writer.write_all(b"<ai><text>Hello</text></ai>").unwrap();
        }
        
        assert_eq!(sequence.len(), 9);
        
        let token_strings: Vec<String> = sequence.iter()
            .map(|&id| tokenizer.vocab[id as usize].clone())
            .collect();
        
        assert_eq!(token_strings[0], "<ai>");
        assert_eq!(token_strings[1], "<text>");
        assert_eq!(token_strings[2], "H");
        assert_eq!(token_strings[3], "e");
        assert_eq!(token_strings[4], "l");
        assert_eq!(token_strings[5], "l");
        assert_eq!(token_strings[6], "o");
        assert_eq!(token_strings[7], "</text>");
        assert_eq!(token_strings[8], "</ai>");
    }
    
    #[test]
    fn test_bpe_token_writer_tag_with_attributes() {
        let mut sequence = Vec::new();
        let tokenizer = create_test_tokenizer();
        
        {
            let mut writer = BPETokenWriter::new(&mut sequence, &tokenizer);
            writer.write_all(b"<js indent=\"2\">console.log()</js>").unwrap();
        }
        
        let token_strings: Vec<String> = sequence.iter()
            .map(|&id| tokenizer.vocab[id as usize].clone())
            .collect();
        
        // Verify the opening tag exists (it should be in special tokens)
        assert!(token_strings.iter().any(|t| t.contains("<js") && t.contains("indent")));
        assert_eq!(token_strings[token_strings.len() - 1], "</js>");
    }
    
    #[test]
    fn test_bpe_token_writer_line_break() {
        let mut sequence = Vec::new();
        let tokenizer = create_test_tokenizer();
        
        {
            let mut writer = BPETokenWriter::new(&mut sequence, &tokenizer);
            writer.write_all(b"<line-break />").unwrap();
        }
        
        assert_eq!(sequence.len(), 1);
        
        let token_strings: Vec<String> = sequence.iter()
            .map(|&id| tokenizer.vocab[id as usize].clone())
            .collect();
        
        assert_eq!(token_strings[0], "<line-break />");
    }
    
    #[test]
    fn test_bpe_token_writer_mixed_content() {
        let mut sequence = Vec::new();
        let tokenizer = create_test_tokenizer();
        
        {
            let mut writer = BPETokenWriter::new(&mut sequence, &tokenizer);
            writer.write_all(b"<text>Hello <b>world</b>!</text>").unwrap();
        }
        
        let token_strings: Vec<String> = sequence.iter()
            .map(|&id| tokenizer.vocab[id as usize].clone())
            .collect();
        
        assert_eq!(token_strings[0], "<text>");
        assert_eq!(token_strings[1], "H");
        assert_eq!(token_strings[2], "e");
        assert_eq!(token_strings[3], "l");
        assert_eq!(token_strings[4], "l");
        assert_eq!(token_strings[5], "o");
        assert_eq!(token_strings[6], " ");
        // The <b> tag is not a special token, so it gets tokenized as characters
        assert_eq!(token_strings[7], "<");
        assert_eq!(token_strings[8], "b");
        assert_eq!(token_strings[9], ">");
        assert_eq!(token_strings[10], "w");
        assert_eq!(token_strings[11], "o");
        assert_eq!(token_strings[12], "r");
        assert_eq!(token_strings[13], "l");
        assert_eq!(token_strings[14], "d");
        assert_eq!(token_strings[15], "<");
        assert_eq!(token_strings[16], "/");
        assert_eq!(token_strings[17], "b");
        assert_eq!(token_strings[18], ">");
        assert_eq!(token_strings[19], "!");
        assert_eq!(token_strings[20], "</text>");
    }
    
    #[test]
    fn test_bpe_token_writer_empty() {
        let mut sequence = Vec::new();
        let tokenizer = create_test_tokenizer();
        
        {
            let mut writer = BPETokenWriter::new(&mut sequence, &tokenizer);
            writer.write_all(b"").unwrap();
        }
        
        assert_eq!(sequence.len(), 0);
    }
    
    #[test]
    fn test_bpe_token_writer_multiple_writes() {
        let mut sequence = Vec::new();
        let tokenizer = create_test_tokenizer();
        
        {
            let mut writer = BPETokenWriter::new(&mut sequence, &tokenizer);
            writer.write_all(b"<prompt>").unwrap();
            writer.write_all(b"Hello").unwrap();
            writer.write_all(b"</prompt>").unwrap();
        }
        
        assert_eq!(sequence.len(), 7);
        
        let token_strings: Vec<String> = sequence.iter()
            .map(|&id| tokenizer.vocab[id as usize].clone())
            .collect();
        
        assert_eq!(token_strings[0], "<prompt>");
        assert_eq!(token_strings[1], "H");
        assert_eq!(token_strings[2], "e");
        assert_eq!(token_strings[3], "l");
        assert_eq!(token_strings[4], "l");
        assert_eq!(token_strings[5], "o");
        assert_eq!(token_strings[6], "</prompt>");
    }
    
    #[test]
    fn test_bpe_token_writer_incomplete_tag() {
        let mut sequence = Vec::new();
        let tokenizer = create_test_tokenizer();
        
        {
            let mut writer = BPETokenWriter::new(&mut sequence, &tokenizer);
            writer.write_all(b"<prompt").unwrap();
            writer.flush().unwrap();
        }
        
        // Incomplete tag should be treated as text and tokenized character by character
        assert_eq!(sequence.len(), 7);
        
        let token_strings: Vec<String> = sequence.iter()
            .map(|&id| tokenizer.vocab[id as usize].clone())
            .collect();
        
        assert_eq!(token_strings[0], "<");
        assert_eq!(token_strings[1], "p");
        assert_eq!(token_strings[2], "r");
        assert_eq!(token_strings[3], "o");
        assert_eq!(token_strings[4], "m");
        assert_eq!(token_strings[5], "p");
        assert_eq!(token_strings[6], "t");
    }
    
    #[test]
    fn test_bpe_token_writer_unknown_tag() {
        let mut sequence = Vec::new();
        let tokenizer = create_test_tokenizer();
        
        {
            let mut writer = BPETokenWriter::new(&mut sequence, &tokenizer);
            writer.write_all(b"<xyz>").unwrap();
        }
        
        // Unknown tag should be tokenized character by character since it's not a special token
        assert_eq!(sequence.len(), 5); // < x y z >
        
        let token_strings: Vec<String> = sequence.iter()
            .map(|&id| tokenizer.vocab[id as usize].clone())
            .collect();
        
        assert_eq!(token_strings[0], "<");
        assert_eq!(token_strings[1], "x");
        assert_eq!(token_strings[2], "y");
        assert_eq!(token_strings[3], "z");
        assert_eq!(token_strings[4], ">");
    }
    
    #[test]
    fn test_bpe_token_writer_unicode() {
        let mut sequence = Vec::new();
        let tokenizer = create_test_tokenizer();
        
        {
            let mut writer = BPETokenWriter::new(&mut sequence, &tokenizer);
            writer.write_all("<text>café</text>".as_bytes()).unwrap();
        }
        
        let token_strings: Vec<String> = sequence.iter()
            .map(|&id| tokenizer.vocab[id as usize].clone())
            .collect();
        
        assert_eq!(token_strings[0], "<text>");
        assert_eq!(token_strings[1], "c");
        assert_eq!(token_strings[2], "a");
        assert_eq!(token_strings[3], "f");
        assert_eq!(token_strings[4], "é");
        assert_eq!(token_strings[5], "</text>");
    }
    
    #[test]
    fn test_bpe_token_writer_complex_scenario() {
        let mut sequence = Vec::new();
        let tokenizer = create_test_tokenizer();
        
        {
            let mut writer = BPETokenWriter::new(&mut sequence, &tokenizer);
            writer.write_all(b"<sample><prompt>What is AI?</prompt><ai><text>AI is artificial intelligence.</text></ai></sample>").unwrap();
        }
        
        let token_strings: Vec<String> = sequence.iter()
            .map(|&id| tokenizer.vocab[id as usize].clone())
            .collect();
        
        // Verify the structure
        let mut idx = 0;
        assert_eq!(token_strings[idx], "<sample>"); idx += 1;
        assert_eq!(token_strings[idx], "<prompt>"); idx += 1;
        assert_eq!(token_strings[idx], "W"); idx += 1;
        assert_eq!(token_strings[idx], "h"); idx += 1;
        assert_eq!(token_strings[idx], "a"); idx += 1;
        assert_eq!(token_strings[idx], "t"); idx += 1;
        assert_eq!(token_strings[idx], " "); idx += 1;
        assert_eq!(token_strings[idx], "i"); idx += 1;
        assert_eq!(token_strings[idx], "s"); idx += 1;
        assert_eq!(token_strings[idx], " "); idx += 1;
        assert_eq!(token_strings[idx], "A"); idx += 1;
        assert_eq!(token_strings[idx], "I"); idx += 1;
        assert_eq!(token_strings[idx], "?"); idx += 1;
        assert_eq!(token_strings[idx], "</prompt>"); idx += 1;
        assert_eq!(token_strings[idx], "<ai>"); idx += 1;
        assert_eq!(token_strings[idx], "<text>"); idx += 1;
        assert_eq!(token_strings[idx], "A"); idx += 1;
        assert_eq!(token_strings[idx], "I"); idx += 1;
        assert_eq!(token_strings[idx], " "); idx += 1;
        assert_eq!(token_strings[idx], "i"); idx += 1;
        assert_eq!(token_strings[idx], "s"); idx += 1;
        assert_eq!(token_strings[idx], " "); idx += 1;
        assert_eq!(token_strings[idx], "a"); idx += 1;
        assert_eq!(token_strings[idx], "r"); idx += 1;
        assert_eq!(token_strings[idx], "t"); idx += 1;
        assert_eq!(token_strings[idx], "i"); idx += 1;
        assert_eq!(token_strings[idx], "f"); idx += 1;
        assert_eq!(token_strings[idx], "i"); idx += 1;
        assert_eq!(token_strings[idx], "c"); idx += 1;
        assert_eq!(token_strings[idx], "i"); idx += 1;
        assert_eq!(token_strings[idx], "a"); idx += 1;
        assert_eq!(token_strings[idx], "l"); idx += 1;
        assert_eq!(token_strings[idx], " "); idx += 1;
        assert_eq!(token_strings[idx], "i"); idx += 1;
        assert_eq!(token_strings[idx], "n"); idx += 1;
        assert_eq!(token_strings[idx], "t"); idx += 1;
        assert_eq!(token_strings[idx], "e"); idx += 1;
        assert_eq!(token_strings[idx], "l"); idx += 1;
        assert_eq!(token_strings[idx], "l"); idx += 1;
        assert_eq!(token_strings[idx], "i"); idx += 1;
        assert_eq!(token_strings[idx], "g"); idx += 1;
        assert_eq!(token_strings[idx], "e"); idx += 1;
        assert_eq!(token_strings[idx], "n"); idx += 1;
        assert_eq!(token_strings[idx], "c"); idx += 1;
        assert_eq!(token_strings[idx], "e"); idx += 1;
        assert_eq!(token_strings[idx], "."); idx += 1;
        assert_eq!(token_strings[idx], "</text>"); idx += 1;
        assert_eq!(token_strings[idx], "</ai>"); idx += 1;
        assert_eq!(token_strings[idx], "</sample>"); idx += 1;
        
        assert_eq!(idx, token_strings.len());
    }
}
