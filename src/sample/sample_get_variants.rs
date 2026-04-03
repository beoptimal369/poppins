// src/sample/sample_get_variants.rs

use rayon::prelude::*;
use crate::train_xml::TrainXMLPhrasePattern;
use crate::sample::sample_structs::{Sample, SamplePromptEnum};


/// Generates variants for multiple samples in batch with parallel processing
///
/// This function uses pre-compiled replacement functions for fast path,
/// and falls back to regex for complex patterns with multiple capture groups.
///
/// # Arguments
/// * `samples` - The samples to generate variants for
/// * `patterns` - The pre-compiled phrase patterns
///
/// # Returns
/// * `Vec<Vec<Sample>>` - A vector where each inner vector contains the variants
///   for the corresponding input sample, or empty vector if no variants for that sample
pub fn sample_get_variants(
    samples: &[Sample],
    patterns: &[TrainXMLPhrasePattern],
) -> Vec<Vec<Sample>> {
    if patterns.is_empty() || samples.is_empty() {
        return vec![Vec::new(); samples.len()];
    }
    
    // Process samples in parallel using rayon
    samples.par_iter()
        .map(|original| sample_get_variants_single(original, patterns))
        .collect()
}


/// Generate variants for a single sample (used by batch processing)
fn sample_get_variants_single(
    original: &Sample,
    patterns: &[TrainXMLPhrasePattern],
) -> Vec<Sample> {
    let mut all_variants = Vec::new();
    
    for pattern in patterns {
        // Find all text positions that match this pattern
        let mut text_positions = Vec::new();
        let mut text_contents = Vec::new();
        
        for (item_idx, item) in original.prompt_section.iter().enumerate() {
            if let SamplePromptEnum::Text(text) = item {
                if pattern.regex.is_match(text) {
                    text_positions.push(item_idx);
                    text_contents.push(text.as_str());
                }
            }
        }
        
        if text_positions.is_empty() {
            continue;
        }
        
        // Create variants for this pattern
        for variant_idx in 0..pattern.variants.len() {
            let mut new_prompt = original.prompt_section.clone();
            let mut all_replaced = true;
            
            // Try fast path first
            for (pos_idx, &item_idx) in text_positions.iter().enumerate() {
                let text = text_contents[pos_idx];
                
                match pattern.replace(text, variant_idx) {
                    Some(replaced) => {
                        new_prompt[item_idx] = SamplePromptEnum::Text(replaced);
                    }
                    None => {
                        // Fast path failed (multiple capture groups), fall back to slow path
                        all_replaced = false;
                        break;
                    }
                }
            }
            
            // If fast path failed for any text, use slow path for all
            if !all_replaced {
                for (pos_idx, &item_idx) in text_positions.iter().enumerate() {
                    let text = text_contents[pos_idx];
                    if let Some(replaced) = pattern.replace(text, variant_idx) {
                        new_prompt[item_idx] = SamplePromptEnum::Text(replaced);
                    }
                }
            }
            
            all_variants.push(Sample {
                system: original.system.clone(),
                prompt_section: new_prompt,
                ai_section: original.ai_section.clone(),
            });
        }
    }
    
    all_variants
}



#[cfg(test)]
mod tests {
    use regex::Regex;
    use std::sync::Arc;
    use super::sample_get_variants;
    use crate::train_xml::TrainXMLPhrasePattern;
    use crate::sample::sample_structs::{Sample, SamplePromptEnum, SampleAiEnum, SampleLanguage, SampleIndent, SampleCode, SampleLineBreak};

    fn create_test_patterns() -> Vec<TrainXMLPhrasePattern> {
        let variants = vec![
            "Define $1.".to_string(),
            "Define: $1.".to_string(),
            "Tell me about $1.".to_string(),
        ];
        
        // Use the original pattern with optional articles
        let pattern = r"What (?:is|are) (?:a |an |the )?(.*?)\?";
        let regex = Regex::new(pattern).unwrap();
        let has_captures = TrainXMLPhrasePattern::has_capture_groups(&regex);
        let has_multiple_captures = TrainXMLPhrasePattern::has_multiple_capture_groups(&regex);
        let variants_use_multiple_captures = TrainXMLPhrasePattern::variants_use_multiple_captures(&variants);
        let replacements = TrainXMLPhrasePattern::compile_replacements(&variants);
        
        vec![
            TrainXMLPhrasePattern {
                regex: Arc::new(regex),
                variants: Arc::new(variants),
                replacements: Arc::new(replacements),
                has_captures,
                has_multiple_captures,
                variants_use_multiple_captures,
            },
        ]
    }

    fn create_test_sample(text: &str) -> Sample {
        Sample {
            system: "You are a helpful assistant.".to_string(),
            prompt_section: vec![
                SamplePromptEnum::Text(text.to_string()),
            ],
            ai_section: vec![
                SampleAiEnum::Text("Test response.".to_string()),
            ],
        }
    }

    fn create_test_sample_with_multiple_text() -> Sample {
        Sample {
            system: "You are a helpful assistant.".to_string(),
            prompt_section: vec![
                SamplePromptEnum::Text("What is a computer?".to_string()),
                SamplePromptEnum::Code(SampleCode {
                    lang: SampleLanguage::Js,
                    inline: false,
                    indent: None,
                    content: "console.log('test')".to_string(),
                }),
                SamplePromptEnum::Text("What is a programming language?".to_string()),
            ],
            ai_section: vec![
                SampleAiEnum::Text("Test response.".to_string()),
            ],
        }
    }

    fn create_test_sample_with_code_and_line_break() -> Sample {
        Sample {
            system: "System".to_string(),
            prompt_section: vec![
                SamplePromptEnum::Text("What is a computer? ".to_string()),
                SamplePromptEnum::Code(SampleCode {
                    lang: SampleLanguage::Rust,
                    inline: false,
                    indent: Some(SampleIndent::One),
                    content: "fn main() {}".to_string(),
                }),
                SamplePromptEnum::LineBreak(SampleLineBreak { count: 1 }),
                SamplePromptEnum::Text("What is programming?".to_string()),  // Changed to match pattern
            ],
            ai_section: vec![
                SampleAiEnum::Text("Test response.".to_string()),
            ],
        }
    }

    #[test]
    fn test_sample_get_variants_empty_patterns() {
        let samples = vec![create_test_sample("What is a computer?")];
        let patterns = vec![];
        
        let results = sample_get_variants(&samples, &patterns);
        
        assert_eq!(results.len(), 1);
        assert!(results[0].is_empty());
    }

    #[test]
    fn test_sample_get_variants_empty_samples() {
        let samples: Vec<Sample> = vec![];
        let patterns = create_test_patterns();
        
        let results = sample_get_variants(&samples, &patterns);
        
        assert!(results.is_empty());
    }

    #[test]
    fn test_sample_get_variants_single_sample() {
        let samples = vec![create_test_sample("What is a computer?")];
        let patterns = create_test_patterns();
        
        let results = sample_get_variants(&samples, &patterns);
        
        assert_eq!(results.len(), 1);
        let variants = &results[0];
        assert_eq!(variants.len(), 3); // 3 variants from the pattern
        
        // Verify variant content
        let texts: Vec<String> = variants.iter()
            .map(|s| match &s.prompt_section[0] {
                SamplePromptEnum::Text(t) => t.clone(),
                _ => panic!("Expected text prompt"),
            })
            .collect();
        
        assert!(texts.contains(&"Define computer.".to_string()));
        assert!(texts.contains(&"Define: computer.".to_string()));
        assert!(texts.contains(&"Tell me about computer.".to_string()));
        
        // Verify system and AI sections are preserved
        for variant in variants {
            assert_eq!(variant.system, "You are a helpful assistant.");
            assert_eq!(variant.ai_section.len(), 1);
            match &variant.ai_section[0] {
                SampleAiEnum::Text(text) => assert_eq!(text, "Test response."),
                _ => panic!("Expected text response"),
            }
        }
    }

    #[test]
    fn test_sample_get_variants_multiple_samples() {
        let samples = vec![
            create_test_sample("What is a computer?"),
            create_test_sample("What is a programming language?"),
            create_test_sample("What is artificial intelligence?"),
        ];
        let patterns = create_test_patterns();
        
        let results = sample_get_variants(&samples, &patterns);
        
        assert_eq!(results.len(), 3);
        
        for variants in &results {
            assert_eq!(variants.len(), 3);
        }
        
        // Check first sample variants
        let first_variants: Vec<String> = results[0].iter()
            .map(|s| match &s.prompt_section[0] {
                SamplePromptEnum::Text(t) => t.clone(),
                _ => panic!("Expected text prompt"),
            })
            .collect();
        
        assert!(first_variants.contains(&"Define computer.".to_string()));
        
        // Check second sample variants
        let second_variants: Vec<String> = results[1].iter()
            .map(|s| match &s.prompt_section[0] {
                SamplePromptEnum::Text(t) => t.clone(),
                _ => panic!("Expected text prompt"),
            })
            .collect();
        
        assert!(second_variants.contains(&"Define programming language.".to_string()));
        
        // Check third sample variants
        let third_variants: Vec<String> = results[2].iter()
            .map(|s| match &s.prompt_section[0] {
                SamplePromptEnum::Text(t) => t.clone(),
                _ => panic!("Expected text prompt"),
            })
            .collect();
        
        assert!(third_variants.contains(&"Define artificial intelligence.".to_string()));
    }

    #[test]
    fn test_sample_get_variants_with_multiple_text_positions() {
        let sample = create_test_sample_with_multiple_text();
        let samples = vec![sample];
        let patterns = create_test_patterns();
        
        let results = sample_get_variants(&samples, &patterns);
        
        assert_eq!(results.len(), 1);
        let variants = &results[0];
        assert_eq!(variants.len(), 3);
        
        // Each variant should have modified both text positions
        for variant in variants {
            // First text position (index 0) should be transformed
            match &variant.prompt_section[0] {
                SamplePromptEnum::Text(text) => {
                    assert!(text.contains("Define") || text.contains("Tell me about"));
                    assert!(!text.contains("What is a computer?"));
                }
                _ => panic!("Expected text at position 0"),
            }
            
            // Code position (index 1) should remain unchanged
            match &variant.prompt_section[1] {
                SamplePromptEnum::Code(code) => {
                    assert_eq!(code.content, "console.log('test')");
                }
                _ => panic!("Expected code at position 1"),
            }
            
            // Third text position (index 2) should be transformed
            match &variant.prompt_section[2] {
                SamplePromptEnum::Text(text) => {
                    assert!(text.contains("Define") || text.contains("Tell me about"));
                    assert!(!text.contains("What is a programming language?"));
                }
                _ => panic!("Expected text at position 2"),
            }
        }
    }

    #[test]
    fn test_sample_get_variants_no_match() {
        let samples = vec![create_test_sample("Hello world")];
        let patterns = create_test_patterns();
        
        let results = sample_get_variants(&samples, &patterns);
        
        assert_eq!(results.len(), 1);
        assert!(results[0].is_empty());
    }

    #[test]
    fn test_sample_get_variants_with_code_and_line_break() {
        let sample = create_test_sample_with_code_and_line_break();
        let samples = vec![sample];
        let patterns = create_test_patterns();
        
        let results = sample_get_variants(&samples, &patterns);
        
        assert_eq!(results.len(), 1);
        let variants = &results[0];
        assert_eq!(variants.len(), 3);
        
        for variant in variants {
            // Verify structure preserved
            assert_eq!(variant.prompt_section.len(), 4);
            
            // Text at position 0: "What is a computer? " - should be transformed
            match &variant.prompt_section[0] {
                SamplePromptEnum::Text(text) => {
                    assert!(text.contains("Define") || text.contains("Tell me about"));
                    assert!(text.contains("computer"), "Should contain 'computer', got: {}", text);
                }
                _ => panic!("Expected text at position 0"),
            }
            
            // Code at position 1 should remain unchanged
            match &variant.prompt_section[1] {
                SamplePromptEnum::Code(code) => {
                    assert_eq!(code.content, "fn main() {}");
                    assert_eq!(code.indent, Some(SampleIndent::One));
                }
                _ => panic!("Expected code at position 1"),
            }
            
            // Line break at position 2 should remain unchanged
            match &variant.prompt_section[2] {
                SamplePromptEnum::LineBreak(lb) => {
                    assert_eq!(lb.count, 1);
                }
                _ => panic!("Expected line break at position 2"),
            }
            
            // Text at position 3: "What is programming?" - should be transformed
            match &variant.prompt_section[3] {
                SamplePromptEnum::Text(text) => {
                    assert!(text.contains("Define") || text.contains("Tell me about"));
                    assert!(text.contains("programming"), "Should contain 'programming', got: {}", text);
                }
                _ => panic!("Expected text at position 3"),
            }
        }
    }

    #[test]
    fn test_sample_get_variants_preserves_system_and_ai() {
        let samples = vec![create_test_sample("What is a computer?")];
        let patterns = create_test_patterns();
        
        let results = sample_get_variants(&samples, &patterns);
        let variants = &results[0];
        
        for variant in variants {
            assert_eq!(variant.system, "You are a helpful assistant.");
            assert_eq!(variant.ai_section.len(), 1);
            match &variant.ai_section[0] {
                SampleAiEnum::Text(text) => assert_eq!(text, "Test response."),
                _ => panic!("Expected text response"),
            }
        }
    }

    #[test]
    fn test_sample_get_variants_with_are_prefix() {
        let samples = vec![create_test_sample("What are movies?")];
        let patterns = create_test_patterns();
        
        let results = sample_get_variants(&samples, &patterns);
        let variants = &results[0];
        
        let texts: Vec<String> = variants.iter()
            .map(|s| match &s.prompt_section[0] {
                SamplePromptEnum::Text(t) => t.clone(),
                _ => panic!("Expected text prompt"),
            })
            .collect();
        
        assert!(texts.contains(&"Define movies.".to_string()));
        assert!(texts.contains(&"Define: movies.".to_string()));
        assert!(texts.contains(&"Tell me about movies.".to_string()));
    }

    #[test]
    fn test_sample_get_variants_with_the_prefix() {
        let samples = vec![create_test_sample("What is the olympics?")];
        let patterns = create_test_patterns();
        
        let results = sample_get_variants(&samples, &patterns);
        let variants = &results[0];
        
        let texts: Vec<String> = variants.iter()
            .map(|s| match &s.prompt_section[0] {
                SamplePromptEnum::Text(t) => t.clone(),
                _ => panic!("Expected text prompt"),
            })
            .collect();
        
        // The pattern captures "olympics" without the article
        assert!(texts.contains(&"Define olympics.".to_string()));
        assert!(texts.contains(&"Define: olympics.".to_string()));
        assert!(texts.contains(&"Tell me about olympics.".to_string()));
    }
}
