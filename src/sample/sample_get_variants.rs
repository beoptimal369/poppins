// src/sample/sample_get_variants.rs

use crate::train_xml::TrainXML;
use crate::sample::sample_structs::{Sample, SamplePromptEnum, Samples};


/// Generates variant samples by applying regex patterns to the prompt
///
/// # Arguments
/// * `samples` - Mutable reference to Samples container (for ID assignment)
/// * `original` - The original Sample to check for patterns
/// * `train_xml` - The parsed train XML containing phrases with regex patterns
///
/// # Returns
/// * `Option<Vec<Sample>>` - Vector of variant samples, or None if no variants found
///
/// # Notes
/// * Each variant gets a unique ID from samples.next_id()
/// * The variants are NOT automatically added to train/val vectors - that's handled separately
/// 
/// # Examples
/// ```xml
/// <phrase pattern="What is (?:a |an |the )?(.*?)\?">
///   <variant value="Define $1." />
///   <variant value="Define: $1." />
/// </phrase>
/// <phrase pattern="ty">
///   <variant value="thanks" />
///   <variant value="thank you" />
/// </phrase>
/// ```
pub fn sample_get_variants(
    samples: &mut Samples,
    original: &Sample,
    train_xml: &TrainXML,
) -> Option<Vec<Sample>> {
    // Early return if no phrases
    let phrases = match &train_xml.phrases {
        Some(p) if !p.phrase.is_empty() => {
            &p.phrase
        },
        _ => {
            return None;
        },
    };
    
    let mut all_variants = Vec::new();
    
    // Process each phrase
    for (_phrase_idx, phrase) in phrases.iter().enumerate() {
        let pattern = &phrase.pattern;
        let variants = &phrase.variant;
        
        // Compile the regex pattern
        let regex = match phrase.compile_pattern() {
            Ok(r) => r,
            Err(e) => {
                println!("  ✗ Failed to compile regex '{}': {}", pattern, e);
                continue;
            }
        };
        
        // Check if this pattern matches ANYWHERE in the prompt
        let mut matches_found = false;
        let mut text_positions = Vec::new();
        
        for (item_idx, item) in original.prompt_section.iter().enumerate() {
            if let SamplePromptEnum::Text(text) = item {
                if regex.is_match(text) {
                    matches_found = true;
                    text_positions.push(item_idx);
                }
            }
        }
        
        if !matches_found {
            continue;
        }
        
        // Create variants for this pattern
        for (_var_idx, variant) in variants.iter().enumerate() {
            // Start with the original prompt section
            let mut new_prompt = original.prompt_section.clone();
            
            // Apply the replacement to ALL matching text elements using regex replacement
            for &item_idx in &text_positions {
                if let SamplePromptEnum::Text(text) = &new_prompt[item_idx] {
                    // Perform regex replacement with capture groups
                    let replaced = regex.replace_all(text, |caps: &regex::Captures| {
                        let mut result = variant.value.clone();
                        
                        // Replace $1, $2, etc. with captured groups
                        for i in 1..caps.len() {
                            if let Some(capture) = caps.get(i) {
                                let placeholder = format!("${}", i);
                                result = result.replace(&placeholder, capture.as_str());
                            }
                        }
                        result
                    }).to_string();
                    
                    new_prompt[item_idx] = SamplePromptEnum::Text(replaced);
                }
            }
            
            // Add the variant with a new ID from samples
            let variant_id = samples.next_id();
            
            all_variants.push(Sample {
                id: variant_id,
                prompt_section: new_prompt,
                ai_section: original.ai_section.clone(),
            });
        }
    }
    
    if all_variants.is_empty() {
        None
    } else {
        Some(all_variants)
    }
}



#[cfg(test)]
mod tests {
    use crate::sample::{
        sample_get_variants::sample_get_variants,
        sample_structs:: {
            Samples,
            Sample,
            SampleText,
            SampleAiEnum,
            SampleIndent,
            SampleLanguage,
            SamplePromptCode,
            SampleTokenStats,
            SamplePromptEnum,
        }
    };
    use crate::train_xml::{
        TrainXML,
        TrainXMLPhrases,
        TrainXMLPhrasesPhrase,
        TrainXMLPhrasesVariant,
    };
    
    fn create_test_train_xml() -> TrainXML {
        TrainXML {
            prompts: None,
            responses: None,
            sources: None,
            code_snippets: None,
            samples: None,
            constants: None,
            phrases: Some(TrainXMLPhrases {
                phrase: vec![
                    // Simple pattern - exact match
                    TrainXMLPhrasesPhrase {
                        pattern: "ty".to_string(),
                        variant: vec![
                            TrainXMLPhrasesVariant { value: "thanks".to_string() },
                            TrainXMLPhrasesVariant { value: "thank you".to_string() },
                        ],
                    },
                    // Complex pattern with capture group
                    TrainXMLPhrasesPhrase {
                        pattern: r"What is a (.*?)\?".to_string(),
                        variant: vec![
                            TrainXMLPhrasesVariant { value: "Define $1.".to_string() },
                            TrainXMLPhrasesVariant { value: "Define: $1.".to_string() },
                        ],
                    },
                ],
            }),
        }
    }
    
    fn create_test_sample() -> Sample {
        Sample {
            id: "1".to_string(),
            prompt_section: vec![
                SamplePromptEnum::Text("What is a computer? ty Ai".to_string()),
            ],
            ai_section: vec![
                SampleAiEnum::Text(SampleText {
                    content: "A computer is a computing device.".to_string(),
                    token_stats: SampleTokenStats {
                        weight_decay: 0.1,
                        dropout: 0.05,
                        loss_scale: 1.0,
                        gradient_scale: 1.0,
                        gradient_clip: 1.0,
                    },
                }),
            ],
        }
    }
    
    #[test]
    fn test_success_case_example() {
        let mut samples = Samples {
            train_samples: Vec::new(),
            val_samples: Vec::new(),
            total_sample_count: 1, // Starting with original sample having ID 1
        };
        
        let sample = create_test_sample();
        let train_xml = create_test_train_xml();

        let variants = sample_get_variants(&mut samples, &sample, &train_xml)
            .expect("Should find variants");
        
        // Expected variants:
        // - Simple "ty" pattern: 2 variants
        // - Complex pattern with capture group: 2 variants
        // Total: 4 variants
        assert_eq!(variants.len(), 4);
        assert_eq!(samples.total_sample_count, 5); // Started at 1, added 4 variants = 5
        
        // Collect all variant prompts
        let variant_prompts: Vec<String> = variants
            .iter()
            .map(|v| {
                let mut full_text = String::new();
                for item in &v.prompt_section {
                    if let SamplePromptEnum::Text(t) = item {
                        full_text.push_str(t);
                    }
                }
                full_text
            })
            .collect();
        
        // Expected variants
        assert!(variant_prompts.contains(&"What is a computer? thanks Ai".to_string()));
        assert!(variant_prompts.contains(&"What is a computer? thank you Ai".to_string()));
        assert!(variant_prompts.contains(&"Define computer. ty Ai".to_string()));
        assert!(variant_prompts.contains(&"Define: computer. ty Ai".to_string()));
        
        // Verify IDs are sequential and start from 2
        let ids: Vec<&String> = variants.iter().map(|v| &v.id).collect();
        for (i, id) in ids.iter().enumerate() {
            assert_eq!(id.parse::<usize>().unwrap(), i + 2);
        }
    }
    
    #[test]
    fn test_complex_pattern_with_capture() {
        let mut samples = Samples {
            train_samples: Vec::new(),
            val_samples: Vec::new(),
            total_sample_count: 1,
        };
        
        let sample = Sample {
            id: "1".to_string(),
            prompt_section: vec![
                SamplePromptEnum::Text("What is a modem?".to_string()),
            ],
            ai_section: vec![],
        };
        
        let train_xml = create_test_train_xml();
        
        let variants = sample_get_variants(&mut samples, &sample, &train_xml)
            .expect("Should find variants");
        
        // Only the complex pattern applies here
        // 2 variants = 2 variants
        assert_eq!(variants.len(), 2);
        assert_eq!(samples.total_sample_count, 3); // Started at 1, added 2 variants = 3
        
        // Collect all texts
        let texts: Vec<String> = variants
            .iter()
            .map(|v| {
                if let SamplePromptEnum::Text(t) = &v.prompt_section[0] {
                    t.clone()
                } else {
                    String::new()
                }
            })
            .collect();
        
        // Check that capture group was properly inserted
        assert!(texts.contains(&"Define modem.".to_string()));
        assert!(texts.contains(&"Define: modem.".to_string()));
        
        // Verify IDs are sequential and start from 2
        let ids: Vec<&String> = variants.iter().map(|v| &v.id).collect();
        assert_eq!(ids[0], "2");
        assert_eq!(ids[1], "3");
    }
    
    #[test]
    fn test_simple_pattern() {
        let mut samples = Samples {
            train_samples: Vec::new(),
            val_samples: Vec::new(),
            total_sample_count: 1,
        };
        
        let sample = Sample {
            id: "1".to_string(),
            prompt_section: vec![
                SamplePromptEnum::Text("Hello ty world".to_string()),
            ],
            ai_section: vec![],
        };
        
        let train_xml = create_test_train_xml();
        
        let variants = sample_get_variants(&mut samples, &sample, &train_xml)
            .expect("Should find variants");
        
        // Only the simple pattern applies here
        // 2 variants = 2 variants
        assert_eq!(variants.len(), 2);
        assert_eq!(samples.total_sample_count, 3); // Started at 1, added 2 variants = 3
        
        // Collect all texts
        let texts: Vec<String> = variants
            .iter()
            .map(|v| {
                if let SamplePromptEnum::Text(t) = &v.prompt_section[0] {
                    t.clone()
                } else {
                    String::new()
                }
            })
            .collect();
        
        // Check that simple replacement worked
        assert!(texts.contains(&"Hello thanks world".to_string()));
        assert!(texts.contains(&"Hello thank you world".to_string()));
    }
    
    #[test]
    fn test_multiple_matches_same_pattern() {
        let mut samples = Samples {
            train_samples: Vec::new(),
            val_samples: Vec::new(),
            total_sample_count: 1,
        };
        
        let sample = Sample {
            id: "1".to_string(),
            prompt_section: vec![
                SamplePromptEnum::Text("ty there, how are ty?".to_string()),
            ],
            ai_section: vec![],
        };
        
        let train_xml = create_test_train_xml();
        
        let variants = sample_get_variants(&mut samples, &sample, &train_xml)
            .expect("Should find variants");
        
        // Simple pattern only, but it appears twice in the text
        // 2 variants = 2 variants
        assert_eq!(variants.len(), 2);
        assert_eq!(samples.total_sample_count, 3); // Started at 1, added 2 variants = 3
        
        // Collect all texts
        let texts: Vec<String> = variants
            .iter()
            .map(|v| {
                if let SamplePromptEnum::Text(t) = &v.prompt_section[0] {
                    t.clone()
                } else {
                    String::new()
                }
            })
            .collect();
        
        // Check that all occurrences were replaced
        assert!(texts.contains(&"thanks there, how are thanks?".to_string()));
        assert!(texts.contains(&"thank you there, how are thank you?".to_string()));
    }
    
    #[test]
    fn test_with_mixed_prompt_elements() {
        let mut samples = Samples {
            train_samples: Vec::new(),
            val_samples: Vec::new(),
            total_sample_count: 1,
        };
        
        let sample = Sample {
            id: "1".to_string(),
            prompt_section: vec![
                SamplePromptEnum::Text("What is a computer? ".to_string()),
                SamplePromptEnum::Code(SamplePromptCode {
                    lang: SampleLanguage::Rust,
                    inline: false,
                    indent: SampleIndent::One,
                    content: "fn main() {}".to_string(),
                }),
                SamplePromptEnum::Text("ty".to_string()),
            ],
            ai_section: vec![],
        };
        
        let train_xml = create_test_train_xml();
        
        let variants = sample_get_variants(&mut samples, &sample, &train_xml)
            .expect("Should find variants");
        
        // 2 patterns × 2 variants each = 4 variants
        assert_eq!(variants.len(), 4);
        assert_eq!(samples.total_sample_count, 5); // Started at 1, added 4 variants = 5
        
        // Verify code element is preserved in all variants
        for variant in &variants {
            assert_eq!(variant.prompt_section.len(), 3);
            
            // Check that the code element is unchanged
            if let SamplePromptEnum::Code(code) = &variant.prompt_section[1] {
                assert_eq!(code.content, "fn main() {}");
            } else {
                panic!("Expected Code element at position 1");
            }
        }
        
        // Collect all prompts to verify the text transformations
        let prompts: Vec<String> = variants
            .iter()
            .map(|v| {
                let mut full = String::new();
                if let SamplePromptEnum::Text(t) = &v.prompt_section[0] {
                    full.push_str(t);
                }
                if let SamplePromptEnum::Text(t) = &v.prompt_section[2] {
                    full.push_str(t);
                }
                full
            })
            .collect();
        
        assert!(prompts.contains(&"What is a computer? thanks".to_string()));
        assert!(prompts.contains(&"What is a computer? thank you".to_string()));
        assert!(prompts.contains(&"Define computer. ty".to_string()));
        assert!(prompts.contains(&"Define: computer. ty".to_string()));
        
        // Verify IDs are sequential and start from 2
        let ids: Vec<&String> = variants.iter().map(|v| &v.id).collect();
        for (i, id) in ids.iter().enumerate() {
            assert_eq!(id.parse::<usize>().unwrap(), i + 2);
        }
    }
}
