// src/sample/sample_create_samples.rs

use crate::train_xml::{TrainXML, TrainXMLIdMaps};
use crate::sample::{
    Samples,
    sample_get_variants,
    sample_create_via_ids,
    sample_create_via_tags,
    sample_place_into_vecs,
};


/// Creates all samples from a train XML document
///
/// This function orchestrates the entire sample creation process:
/// 1. Creates a token stats map from constants
/// 2. Iterates through all samples in the train XML
/// 3. For each sample (both IDs and tags), creates the base sample
/// 4. Generates variants using regex patterns from phrases
/// 5. Places original + variants into train/validation vectors
///
/// # Arguments
/// * `train_xml` - The parsed train XML document
/// * `id_map` - Validated ID maps containing all prompts, responses, sources, and code snippets
/// * `constants` - Parsed constants from train.xml containing per-component weights
///
/// # Returns
/// * `Samples` - Container with train_samples and val_samples vectors
pub fn sample_create_samples(
    train_xml: &TrainXML,
    id_map: &TrainXMLIdMaps,
) -> Samples {
    // Initialize samples container
    let mut samples = Samples {
        train_samples: Vec::new(),
        val_samples: Vec::new(),
    };
    
    // Process samples if they exist
    if let Some(samples_section) = &train_xml.samples {
        // Process compact ID-based samples (sample-ids)
        if let Some(sample_ids_list) = &samples_section.sample_ids {
            for sample_ids in sample_ids_list {
                
                // Create sample via IDs
                if let Some(original) = sample_create_via_ids(
                    sample_ids,
                    id_map,
                ) {
                    // Get variants for this sample
                    let variants = sample_get_variants( &original, train_xml);
                    
                    // Place original and variants into train/val vectors
                    sample_place_into_vecs(&mut samples, original, variants);
                }
            }
        }
        
        // Process sample w/ xml tags w/in it
        if let Some(sample_tags_list) = &samples_section.sample {
            for sample_tags in sample_tags_list {
                
                // Create sample via tags
                if let Some(original) = sample_create_via_tags(
                    sample_tags,
                    id_map,
                ) {
                    // Get variants for this sample
                    let variants = sample_get_variants(&original, train_xml);
                    
                    // Place original and variants into train/val vectors
                    sample_place_into_vecs(&mut samples, original, variants);
                }
            }
        }
    }

    samples
}



#[cfg(test)]
mod tests {
    use crate::sample:: {
        Sample,
        SampleAiEnum,
        SamplePromptEnum,
        sample_create_samples
    };
    use crate::train_xml::{
        TrainXML,
        TrainXMLIdMaps, 
        TrainXMLPrompts,
        TrainXMLPhrases, 
        TrainXMLSamples,
        TrainXMLSources,
        TrainXMLResponses,
        TrainXMLLineBreak,
        TrainXMLConstants, 
        TrainXMLSamplesCode,
        TrainXMLCodeSnippets,
        TrainXMLConstantsKey,
        TrainXMLSourcesSource,
        TrainXMLSamplesSample,
        TrainXMLPromptsPrompt,
        TrainXMLSamplesSource,
        TrainXMLSamplesPrompt,  
        TrainXMLPhrasesPhrase,
        TrainXMLPhrasesVariant,
        TrainXMLSamplesResponse,
        TrainXMLSamplesSampleIds,
        TrainXMLCodeSnippetsCode,
        TrainXMLConstantsConstant,
        TrainXMLResponsesResponse,
        TrainXMLSamplesResponseIds,
        TrainXMLSamplesSampleChildren,
    };
    
    /// Comprehensive test that covers:
    /// - sample-ids (compact) samples
    /// - sample w/ xml tags w/in it
    /// - Variants from regex patterns
    /// - Multiple responses, sources, and code snippets
    /// - All component types with proper token stats
    #[test]
    fn test_sample_create_samples_comprehensive() {
        // Create a comprehensive train XML with all variations
        let train_xml = create_comprehensive_train_xml();
        
        // Create ID maps
        let ids = TrainXMLIdMaps::create(&train_xml).expect("Failed to create ID maps");
        
        // Create samples
        let samples = sample_create_samples(&train_xml, &ids);
        
        let all_samples: Vec<&Sample> = samples.train_samples.iter()
            .chain(samples.val_samples.iter())
            .collect();
        
        // Verify train/val split (should have some in each)
        assert!(!samples.train_samples.is_empty(), "Train samples should not be empty");
        assert!(!samples.val_samples.is_empty(), "Val samples should not be empty");
        assert_eq!(
            samples.train_samples.len() + samples.val_samples.len(), 
            6, 
            "Total samples should match"
        );
        
        // Verify that we have samples from both original prompts
        let computer_samples = all_samples.iter().filter(|s| {
            s.prompt_section.iter().any(|p| {
                match p {
                    SamplePromptEnum::Text(t) => t.contains("computer"),
                    _ => false,
                }
            })
        }).count();
        
        let programming_samples = all_samples.iter().filter(|s| {
            s.prompt_section.iter().any(|p| {
                match p {
                    SamplePromptEnum::Text(t) => t.contains("programming language"),
                    _ => false,
                }
            })
        }).count();
        
        assert_eq!(computer_samples, 3, "Should have 3 computer-related samples (original + 2 variants)");
        assert_eq!(programming_samples, 3, "Should have 3 programming language-related samples (original + 2 variants)");
        
        // Verify sample content
        for sample in &all_samples {
            // Verify token stats on AI section items
            for ai_item in &sample.ai_section {
                match ai_item {
                    SampleAiEnum::Text(text) => {
                        assert!(!text.is_empty());
                    },
                    SampleAiEnum::Source(source) => {
                        assert!(!source.is_empty());
                    },
                    SampleAiEnum::Code(code) => {
                        assert!(!code.content.is_empty());
                    },
                    SampleAiEnum::LineBreak(line_break) => {
                        assert!(line_break.count == 1 || line_break.count == 2, "Line break count must be 1 or 2, got {}", line_break.count);
                    },
                }
            }
        }
        
        // Verify that the original prompt content appears in some samples
        let has_computer_prompt = all_samples.iter().any(|s| {
            s.prompt_section.iter().any(|p| {
                match p {
                    SamplePromptEnum::Text(t) => t.contains("computer"),
                    _ => false,
                }
            })
        });
        assert!(has_computer_prompt, "Should have samples with 'computer' prompt");
        
        let has_programming_prompt = all_samples.iter().any(|s| {
            s.prompt_section.iter().any(|p| {
                match p {
                    SamplePromptEnum::Text(t) => t.contains("programming language"),
                    _ => false,
                }
            })
        });
        assert!(has_programming_prompt, "Should have samples with 'programming language' prompt");
        
        // Verify that variants were created (should have different prompt text)
        let prompts: Vec<String> = all_samples.iter()
            .flat_map(|s| s.prompt_section.iter())
            .filter_map(|p| {
                match p {
                    SamplePromptEnum::Text(t) => Some(t.clone()),
                    _ => None,
                }
            })
            .collect();
        
        // Should have multiple variations of prompts
        assert!(prompts.len() > 2, "Should have multiple prompt variations from variants");
        
        // Look for variant patterns
        let has_define_format = prompts.iter().any(|p| p.contains("Define"));
        assert!(has_define_format, "Should have variants with 'Define' format");
    }


    /// Helper function to create a comprehensive train XML for testing
    fn create_comprehensive_train_xml() -> TrainXML {
        // Prompts
        let prompts = TrainXMLPrompts {
            prompt: vec![
                TrainXMLPromptsPrompt {
                    id: "1".to_string(),
                    content: "What is a computer?".to_string(),
                },
                TrainXMLPromptsPrompt {
                    id: "2".to_string(),
                    content: "What is a programming language?".to_string(),
                },
            ],
        };
        
        // Responses
        let responses = TrainXMLResponses {
            response: vec![
                TrainXMLResponsesResponse {
                    id: "1".to_string(),
                    content: "A computer is a computing / information processing device.".to_string(),
                },
                TrainXMLResponsesResponse {
                    id: "2".to_string(),
                    content: "A programming language is a formal language for writing instructions.".to_string(),
                },
                TrainXMLResponsesResponse {
                    id: "3".to_string(),
                    content: "Programming languages have syntax and semantics.".to_string(),
                },
            ],
        };
        
        // Sources
        let sources = TrainXMLSources {
            source: vec![
                TrainXMLSourcesSource {
                    id: "1".to_string(),
                    url: "https://en-word.net/lemma/computer".to_string(),
                    title: None,
                },
                TrainXMLSourcesSource {
                    id: "2".to_string(),
                    url: "https://en.wikipedia.org/wiki/Programming_language".to_string(),
                    title: None,
                },
                TrainXMLSourcesSource {
                    id: "3".to_string(),
                    url: "https://en.wikipedia.org/wiki/Syntax_(programming_languages)".to_string(),
                    title: None,
                },
            ],
        };
        
        // Code snippets
        let code_snippets = TrainXMLCodeSnippets {
            code: vec![
                TrainXMLCodeSnippetsCode {
                    id: "1".to_string(),
                    lang: "ts".to_string(),
                    content: "\nfunction example() {\n  console.log('hi world')\n}\n    ".to_string(),
                },
            ],
        };
        
        // Samples w/ <sample/> & <sample-ids/>
        let samples = TrainXMLSamples {
            sample_ids: Some(vec![
                TrainXMLSamplesSampleIds {
                    prompt: "1".to_string(),
                    response: Some("1".to_string()),
                    source: Some("1".to_string()),
                    code: None,
                },
            ]),
            sample: Some(vec![
                TrainXMLSamplesSample {
                    prompt: TrainXMLSamplesPrompt {
                        id: "2".to_string(),
                    },
                    children: vec![
                        // First response
                        TrainXMLSamplesSampleChildren::Response(TrainXMLSamplesResponse { 
                            id: "2".to_string() 
                        }),
                        // Its source
                        TrainXMLSamplesSampleChildren::Source(TrainXMLSamplesSource { 
                            id: "2".to_string() 
                        }),
                        // Second response
                        TrainXMLSamplesSampleChildren::Response(TrainXMLSamplesResponse { 
                            id: "3".to_string() 
                        }),
                        // Its source
                        TrainXMLSamplesSampleChildren::Source(TrainXMLSamplesSource { 
                            id: "3".to_string() 
                        }),
                        // Code example
                        TrainXMLSamplesSampleChildren::Code(TrainXMLSamplesCode { 
                            id: "1".to_string(),
                            indent: None,
                            inline: None,
                        }),
                        // Add a response-id for testing
                        TrainXMLSamplesSampleChildren::ResponseIds(TrainXMLSamplesResponseIds { 
                            response: "1".to_string(),
                            source: Some("1".to_string()),
                        }),
                        // Add some line breaks for testing
                        TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 1 }),
                        TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 2 }),
                    ],
                },
            ]),
        };
        
        // Phrases with regex patterns for variants
        let phrases = TrainXMLPhrases {
            phrase: vec![
                TrainXMLPhrasesPhrase {
                    pattern: "What is (?:a |an |the )?(.*?)\\?".to_string(),
                    variant: vec![
                        TrainXMLPhrasesVariant { value: "Define $1.".to_string() },
                        TrainXMLPhrasesVariant { value: "Define: $1.".to_string() },
                    ],
                },
                TrainXMLPhrasesPhrase {
                    pattern: "ty".to_string(),
                    variant: vec![
                        TrainXMLPhrasesVariant { value: "thanks".to_string() },
                        TrainXMLPhrasesVariant { value: "thank you".to_string() },
                    ],
                },
            ],
        };
        
        // Constants
        let constants = TrainXMLConstants {
            constant: vec![
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::WeightDecayResponse, value: "0.1".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::WeightDecaySource, value: "0.01".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::WeightDecayCode, value: "0.05".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::DropoutRateResponse, value: "0.05".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::DropoutRateSource, value: "0.0".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::DropoutRateCode, value: "0.1".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::LossScaleResponse, value: "1.0".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::LossScaleSource, value: "0.2".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::LossScaleCode, value: "1.0".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientScaleResponse, value: "1.0".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientScaleSource, value: "2.0".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientScaleCode, value: "1.2".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientClipResponse, value: "1.0".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientClipSource, value: "0.1".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientClipCode, value: "0.7".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::AimTrainGb, value: "3.0".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::AimInferF16Gb, value: "0.9".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::LearningRate, value: "1e-3".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::WarmupSteps, value: "100".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::AimLoss, value: "0.45".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::ValInterval, value: "10".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::BpeMinMergeFrequency, value: "3".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::BpeRequestedTokens, value: "a,b".to_string(), delimiter: Some(",".to_owned()) },
            ],
        };
        
        TrainXML {
            prompts: Some(prompts),
            responses: Some(responses),
            sources: Some(sources),
            code_snippets: Some(code_snippets),
            samples: Some(samples),
            constants: Some(constants),
            phrases: Some(phrases),
        }
    }
}
