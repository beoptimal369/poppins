// src/sample/sample_create_samples.rs

use crate::beyond_scope::beyond_scope_create_samples;
use crate::train_xml::{TrainXML, TrainXMLIdMaps, TrainXMLPhrasePattern};
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
/// 6. Generates beyond-scope samples for topics the AI shouldn't answer
/// 7. Generates variants for beyond-scope samples as well
///
/// # Arguments
/// * `train_xml` - The parsed train XML document
/// * `train_xml_ids` - Validated ID maps containing all prompts, responses, sources, and code snippets
/// * `train_xml_patterns` - Pre-compiled phrase patterns for variant generation
///
/// # Returns
/// * `Samples` - Container with train_samples and val_samples vectors
pub fn sample_create_samples(
    train_xml: &TrainXML,
    train_xml_ids: &TrainXMLIdMaps,
    train_xml_patterns: &[TrainXMLPhrasePattern],
) -> Samples {
    let mut samples = Samples {
        train_samples: Vec::new(),
        val_samples: Vec::new(),
    };
    
    // Collect all regular samples first
    let mut regular_samples = Vec::new();
    
    // Process regular samples
    if let Some(samples_section) = &train_xml.samples {
        
        // Process sample-ids
        if let Some(sample_ids_list) = &samples_section.sample_ids {
            for sample_ids in sample_ids_list {
                if let Some(original) = sample_create_via_ids(sample_ids, train_xml_ids) {
                    regular_samples.push(original);
                }
            }
        }
        
        // Process sample tags
        if let Some(sample_tags_list) = &samples_section.sample {
            for sample_tags in sample_tags_list {
                if let Some(original) = sample_create_via_tags(sample_tags, train_xml_ids) {
                    regular_samples.push(original);
                }
            }
        }
        
        // Generate variants for all regular samples in batch
        let all_variants = sample_get_variants(&regular_samples, train_xml_patterns);
        
        // Place regular samples and their variants
        for (sample, variants) in regular_samples.into_iter().zip(all_variants.into_iter()) {
            if variants.is_empty() {
                sample_place_into_vecs(&mut samples, sample, None);
            } else {
                sample_place_into_vecs(&mut samples, sample, Some(variants));
            }
        }
    }
    
    // Generate beyond-scope samples
    let beyond_scope_samples = beyond_scope_create_samples(train_xml, train_xml_ids);
    
    // Generate variants for beyond-scope samples in batch
    let all_beyond_variants = sample_get_variants(&beyond_scope_samples, train_xml_patterns);
    
    // Place beyond-scope samples and their variants
    for (sample, variants) in beyond_scope_samples.into_iter().zip(all_beyond_variants.into_iter()) {
        if variants.is_empty() {
            sample_place_into_vecs(&mut samples, sample, None);
        } else {
            sample_place_into_vecs(&mut samples, sample, Some(variants));
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
        TrainXMLBeyondScope,
        TrainXMLCodeSnippets,
        TrainXMLSystemPrompts,
        TrainXMLSourcesSource,
        TrainXMLSamplesSystem,
        TrainXMLSamplesSample,
        TrainXMLPromptsPrompt,
        TrainXMLSamplesSource,
        TrainXMLSamplesPrompt,  
        TrainXMLPhrasesPhrase,
        TrainXMLPhrasesVariant,
        TrainXMLSamplesResponse,
        TrainXMLBeyondScopeTopic,
        TrainXMLSamplesSampleIds,
        TrainXMLCodeSnippetsCode,
        train_xml_phrase_patterns,
        TrainXMLResponsesResponse,
        TrainXMLBpeRequestedTokens,
        TrainXMLSamplesResponseIds,
        TrainXMLSystemPromptsSystem,
        TrainXMLSamplesSampleChildren,
    };
        
    /// Comprehensive test that covers:
    /// - sample-ids (compact) samples
    /// - sample w/ xml tags w/in it
    /// - Variants from regex patterns
    /// - Multiple responses, sources, and code snippets
    /// - System prompts
    /// - Beyond-scope validation
    /// - All component types with proper token stats
    #[test]
    fn test_sample_create_samples_comprehensive() {
        // Create a comprehensive train XML with all variations
        let train_xml = create_comprehensive_train_xml();
        
        // Create ID maps
        let ids = TrainXMLIdMaps::create(&train_xml).expect("Failed to create ID maps");
        
        // Create compiled phrase patterns
        let patterns = train_xml_phrase_patterns(&train_xml);
        
        // Create samples
        let samples = sample_create_samples(&train_xml, &ids, &patterns);
        
        let all_samples: Vec<&Sample> = samples.train_samples.iter()
            .chain(samples.val_samples.iter())
            .collect();
        
        // Verify train/val split (should have some in each)
        assert!(!samples.train_samples.is_empty(), "Train samples should not be empty");
        assert!(!samples.val_samples.is_empty(), "Val samples should not be empty");
        
        // Original samples: 3 base samples
        // Each base sample has 2 variants = 3 * 3 = 9 total original samples
        // Beyond-scope topics: sports (12) + movies (11) + science (10) + custom (2) = 35 topics
        // Each beyond-scope topic has 2 variants = 35 * 3 = 105 beyond-scope samples
        // Total = 9 + 105 = 114 samples
        let expected_total = 9 + (35 * 3);
        assert_eq!(
            samples.train_samples.len() + samples.val_samples.len(), 
            expected_total, 
            "Total samples should match (original variants + beyond-scope topics with variants)"
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
        
        let ai_samples = all_samples.iter().filter(|s| {
            s.prompt_section.iter().any(|p| {
                match p {
                    SamplePromptEnum::Text(t) => t.contains("artificial intelligence"),
                    _ => false,
                }
            })
        }).count();
        
        assert_eq!(computer_samples, 3, "Should have 3 computer-related samples (original + 2 variants)");
        assert_eq!(programming_samples, 3, "Should have 3 programming language-related samples (original + 2 variants)");
        assert_eq!(ai_samples, 3, "Should have 3 AI-related samples (original + 2 variants)");
        
        // Verify beyond-scope samples are present (including variants)
        let beyond_scope_samples = all_samples.iter().filter(|s| {
            s.ai_section.iter().any(|ai| {
                match ai {
                    SampleAiEnum::Text(text) => text.contains("I'm sorry, I don't know"),
                    _ => false,
                }
            })
        }).count();
        
        // Beyond-scope topics: 35 topics * 3 samples each (original + 2 variants) = 105
        assert_eq!(beyond_scope_samples, 105, "Should have 105 beyond-scope samples (35 topics * 3 samples each)");
        
        // Verify sample content includes system prompts (now in the system field)
        let has_system_prompt = all_samples.iter().any(|s| {
            !s.system.is_empty() && s.system.contains("You are a helpful computer programming assistant")
        });
        assert!(has_system_prompt, "Should have samples with system prompt");
        
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
        
        // Verify that beyond-scope topics are correctly formatted
        let beyond_scope_questions: Vec<String> = all_samples.iter()
            .filter(|s| {
                s.ai_section.iter().any(|ai| {
                    match ai {
                        SampleAiEnum::Text(text) => text.contains("I'm sorry, I don't know"),
                        _ => false,
                    }
                })
            })
            .flat_map(|s| s.prompt_section.iter())
            .filter_map(|p| match p {
                SamplePromptEnum::Text(t) => Some(t.clone()),
                _ => None,
            })
            .collect();
        
        // Check specific beyond-scope topics (original versions)
        assert!(beyond_scope_questions.contains(&"What is soccer?".to_string()));
        assert!(beyond_scope_questions.contains(&"What is the olympics?".to_string()));
        assert!(beyond_scope_questions.contains(&"What are movies?".to_string()));
        assert!(beyond_scope_questions.contains(&"What is netflix?".to_string()));
        assert!(beyond_scope_questions.contains(&"What is biology?".to_string()));
        assert!(beyond_scope_questions.contains(&"What is quantum computing?".to_string()));
        assert!(beyond_scope_questions.contains(&"What is blockchain?".to_string()));
        
        // Check variant versions of beyond-scope topics (should have "Define" variants)
        let has_define_variants = beyond_scope_questions.iter().any(|q| q.contains("Define"));
        assert!(has_define_variants, "Should have variant versions of beyond-scope topics with 'Define' format");
        
        // Verify that variants were created (should have different prompt text)
        let prompts: Vec<String> = all_samples.iter()
            .flat_map(|s| s.prompt_section.iter())
            .filter_map(|p| match p {
                SamplePromptEnum::Text(t) => Some(t.clone()),
                _ => None,
            })
            .collect();
        
        // Should have multiple variations of prompts
        assert!(prompts.len() > 100, "Should have many prompt variations");
        
        // Look for variant patterns in original prompts
        let has_define_format = prompts.iter().any(|p| p.contains("Define"));
        assert!(has_define_format, "Should have variants with 'Define' format");
    }

    #[test]
    fn test_sample_create_samples_empty_patterns() {
        let train_xml = create_comprehensive_train_xml();
        let ids = TrainXMLIdMaps::create(&train_xml).expect("Failed to create ID maps");
        let patterns = vec![]; // Empty patterns
        
        let samples = sample_create_samples(&train_xml, &ids, &patterns);
        
        // Without patterns, no variants should be generated
        // Original samples: 3 base samples
        // Beyond-scope topics: 35 topics (no variants since patterns empty)
        // Total = 3 + 35 = 38 samples
        let expected_total = 3 + 35;
        assert_eq!(
            samples.train_samples.len() + samples.val_samples.len(),
            expected_total,
            "Total samples should be original + beyond-scope (no variants)"
        );
        
        // Verify no variants exist (no "Define" in any prompt)
        let all_prompts: Vec<String> = samples.train_samples.iter()
            .chain(samples.val_samples.iter())
            .flat_map(|s| s.prompt_section.iter())
            .filter_map(|p| match p {
                SamplePromptEnum::Text(t) => Some(t.clone()),
                _ => None,
            })
            .collect();
        
        let has_define = all_prompts.iter().any(|p| p.contains("Define"));
        assert!(!has_define, "Should have no 'Define' variants when patterns empty");
    }

    #[test]
    fn test_sample_create_samples_no_beyond_scope() {
        let mut train_xml = create_comprehensive_train_xml();
        train_xml.beyond_scope = None;
        
        let ids = TrainXMLIdMaps::create(&train_xml).expect("Failed to create ID maps");
        let patterns = train_xml_phrase_patterns(&train_xml);
        
        let samples = sample_create_samples(&train_xml, &ids, &patterns);
        
        // Only original samples with variants (3 base * 3 = 9)
        let expected_total = 9;
        assert_eq!(
            samples.train_samples.len() + samples.val_samples.len(),
            expected_total,
            "Total samples should only be original samples with variants"
        );
        
        // Verify no beyond-scope samples exist
        let has_beyond_scope = samples.train_samples.iter()
            .chain(samples.val_samples.iter())
            .any(|s| {
                s.ai_section.iter().any(|ai| {
                    match ai {
                        SampleAiEnum::Text(text) => text.contains("I'm sorry, I don't know"),
                        _ => false,
                    }
                })
            });
        assert!(!has_beyond_scope, "Should have no beyond-scope samples");
    }

    #[test]
    fn test_sample_create_samples_no_samples() {
        let mut train_xml = create_comprehensive_train_xml();
        train_xml.samples = None;
        
        let ids = TrainXMLIdMaps::create(&train_xml).expect("Failed to create ID maps");
        let patterns = train_xml_phrase_patterns(&train_xml);
        
        let samples = sample_create_samples(&train_xml, &ids, &patterns);
        
        // Only beyond-scope samples with variants (35 topics * 3 = 105)
        let expected_total = 35 * 3;
        assert_eq!(
            samples.train_samples.len() + samples.val_samples.len(),
            expected_total,
            "Total samples should only be beyond-scope samples with variants"
        );
        
        // Verify no regular samples (no "computer" in prompts)
        let has_regular = samples.train_samples.iter()
            .chain(samples.val_samples.iter())
            .any(|s| {
                s.prompt_section.iter().any(|p| {
                    match p {
                        SamplePromptEnum::Text(t) => t.contains("computer") || t.contains("programming") || t.contains("artificial"),
                        _ => false,
                    }
                })
            });
        assert!(!has_regular, "Should have no regular samples");
    }

    /// Helper function to create a comprehensive train XML for testing
    fn create_comprehensive_train_xml() -> TrainXML {
        // System prompts
        let system_prompts = TrainXMLSystemPrompts {
            system: vec![
                TrainXMLSystemPromptsSystem {
                    id: "sy_default".to_string(),
                    content: "You are a helpful computer programming assistant.".to_string(),
                },
            ],
        };
        
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
                TrainXMLPromptsPrompt {
                    id: "3".to_string(),
                    content: "What is artificial intelligence?".to_string(),
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
                    content: "Artificial intelligence is the simulation of human intelligence in machines.".to_string(),
                },
                TrainXMLResponsesResponse {
                    id: "beyond_scope_response".to_string(),
                    content: "I'm sorry, I don't know, I'm a computer programming assistant".to_string(),
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
                    url: "https://en.wikipedia.org/wiki/Artificial_intelligence".to_string(),
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
                    system: Some("sy_default".to_string()),
                    prompt: "1".to_string(),
                    response: Some("1".to_string()),
                    source: Some("1".to_string()),
                    code: None,
                },
            ]),
            sample: Some(vec![
                TrainXMLSamplesSample {
                    children: vec![
                        // Prompt must be first or at least present
                        TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "2".to_string() }),
                        // System prompt
                        TrainXMLSamplesSampleChildren::System(TrainXMLSamplesSystem { 
                            id: "sy_default".to_string() 
                        }),
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
                TrainXMLSamplesSample {
                    children: vec![
                        // Prompt must be first or at least present
                        TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "3".to_string() }),
                        // System prompt
                        TrainXMLSamplesSampleChildren::System(TrainXMLSamplesSystem { 
                            id: "sy_default".to_string() 
                        }),
                        // Response
                        TrainXMLSamplesSampleChildren::Response(TrainXMLSamplesResponse { 
                            id: "3".to_string() 
                        }),
                        // Source
                        TrainXMLSamplesSampleChildren::Source(TrainXMLSamplesSource { 
                            id: "3".to_string() 
                        }),
                    ],
                },
            ]),
        };
        
        // Phrases with regex patterns for variants
        let phrases = TrainXMLPhrases {
            phrase: vec![
                TrainXMLPhrasesPhrase {
                    pattern: "What (?:is|are) (?:a |an |the )?(.*?)\\?".to_string(),
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
        
        // Constants - UPDATED to new element-based structure
        let constants = TrainXMLConstants {
            aim_train_gb: Some(3.0),
            aim_infer_gb: Some(0.9),
            learning_rate: Some(0.001),
            warmup_steps: Some(100),
            aim_loss: Some(0.45),
            val_interval: Some(10),
            batch_size: None,  // Will use device default
            mixed_precision: None,  // Will use device default
            gradient_accumulation_steps: None,  // Will use device default
            activation_precision: None,  // Will use device default
            kv_cache_precision: Some("int8".to_string()),
            rope_precision: None,  // Will use device default
            num_workers: None,  // Will use device default
            use_flash_attention: None,  // Will use device default
            use_tensor_cores: None,  // Will use device default
            
            bpe_min_merge_frequency: Some(3),
            bpe_requested_tokens: Some(TrainXMLBpeRequestedTokens {
                values: vec!["a".to_string(), "b".to_string()],
            }),
            
            weight_decay_response: Some(0.1),
            weight_decay_source: Some(0.01),
            weight_decay_code: Some(0.05),
            
            dropout_rate_response: None,  // Will use default
            dropout_rate_source: None,  // Will use default
            dropout_rate_code: None,  // Will use default
            
            loss_scale_response: Some(1.0),
            loss_scale_source: Some(0.2),
            loss_scale_code: Some(1.0),
            
            gradient_scale_response: Some(1.0),
            gradient_scale_source: Some(2.0),
            gradient_scale_code: Some(1.2),
            
            gradient_clip_response: Some(1.0),
            gradient_clip_source: Some(0.1),
            gradient_clip_code: Some(0.7),
        };
        
        // Beyond-scope configuration
        let beyond_scope = TrainXMLBeyondScope {
            system: "sy_default".to_string(),
            response: "beyond_scope_response".to_string(),
            sports: Some(true),
            food: Some(false),
            movies: Some(true),
            history: Some(false),
            geography: Some(false),
            politics: Some(false),
            science: Some(true),
            health: Some(false),
            art: Some(false),
            music: Some(false),
            fashion: Some(false),
            travel: Some(false),
            pets: Some(false),
            cars: Some(false),
            topics: vec![
                TrainXMLBeyondScopeTopic {
                    value: "quantum computing".to_string(),
                    prefix: "is".to_string(),
                },
                TrainXMLBeyondScopeTopic {
                    value: "blockchain".to_string(),
                    prefix: "is".to_string(),
                },
            ],
        };
        
        TrainXML {
            system_prompts: Some(system_prompts),
            prompts: Some(prompts),
            responses: Some(responses),
            sources: Some(sources),
            code_snippets: Some(code_snippets),
            samples: Some(samples),
            constants: Some(constants),
            phrases: Some(phrases),
            beyond_scope: Some(beyond_scope),
        }
    }
}