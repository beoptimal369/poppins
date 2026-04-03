// src/train_xml/train_xml_validate.rs

use crate::train_xml::{
    TrainXML,
    TrainXMLIdMaps,
    TrainXMLPhrasePattern,
    TrainXMLConstantParsed,
    train_xml_validate_ids,
    train_xml_constants_parse,
    train_xml_phrase_patterns,
    TrainXMLSamplesSampleChildren,
};


pub fn train_xml_validate(train_xml: &TrainXML) -> Result<(TrainXMLIdMaps<'_>, TrainXMLConstantParsed, Vec<TrainXMLPhrasePattern>), Box<dyn std::error::Error>> {
    let train_xml_id_maps_map = TrainXMLIdMaps::create(train_xml)?;

    train_xml_validate_ids(train_xml, &train_xml_id_maps_map).map_err(|errors| {
        let error_string = errors.join("\n  ");
        format!("❌ Failed validating train xml ids:\n  {}", error_string)
    })?;
    
    validate_line_breaks(train_xml)?;
    
    validate_prompt_presence(train_xml)?;

    let train_xml_constants_parsed = train_xml_constants_parse(&train_xml.constants)?;

    let train_xml_patterns = train_xml_phrase_patterns(train_xml);

    Ok((train_xml_id_maps_map, train_xml_constants_parsed, train_xml_patterns))
}


/// Validates that all line break counts are either 1 or 2
fn validate_line_breaks(train_xml: &TrainXML) -> Result<(), String> {
    if let Some(samples) = &train_xml.samples {
        if let Some(sample_list) = &samples.sample {
            for (sample_idx, sample) in sample_list.iter().enumerate() {
                for (child_idx, child) in sample.children.iter().enumerate() {
                    if let TrainXMLSamplesSampleChildren::LineBreak(line_break) = child {
                        if line_break.count != 1 && line_break.count != 2 {
                            return Err(format!(
                                "Invalid line break count at sample {}, child {}: count = {} (must be 1 or 2)",
                                sample_idx + 1,
                                child_idx + 1,
                                line_break.count
                            ));
                        }
                    }
                }
            }
        }
    }
    Ok(())
}


/// Validates that every sample has at least one <prompt> element
fn validate_prompt_presence(train_xml: &TrainXML) -> Result<(), String> {
    let mut errors = Vec::new();
    
    if let Some(samples) = &train_xml.samples {
        if let Some(sample_list) = &samples.sample {
            for (sample_idx, sample) in sample_list.iter().enumerate() {
                let has_prompt = sample.children.iter().any(|child| {
                    matches!(child, TrainXMLSamplesSampleChildren::Prompt(_))
                });
                
                if !has_prompt {
                    errors.push(format!(
                        "Sample {} is missing a required <prompt> element",
                        sample_idx + 1
                    ));
                }
            }
        }
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}


#[cfg(test)]
mod tests {
    use super::validate_line_breaks;
    use crate::train_xml::{
        TrainXML,
        TrainXMLPhrases,
        TrainXMLPrompts,
        TrainXMLSamples,
        TrainXMLResponses,
        TrainXMLLineBreak,
        train_xml_validate,
        TrainXMLPhrasesPhrase,
        TrainXMLSamplesSample,
        TrainXMLSamplesPrompt,
        TrainXMLPromptsPrompt,
        TrainXMLPhrasesVariant,
        TrainXMLResponsesResponse,
        TrainXMLSamplesSampleChildren,
    };

    fn create_test_xml_with_line_breaks(line_break_counts: Vec<u8>) -> TrainXML {
        let children: Vec<TrainXMLSamplesSampleChildren> = line_break_counts
            .into_iter()
            .map(|count| TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count }))
            .collect();

        // Add a prompt to make the sample valid
        let mut full_children = vec![TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "1".to_string() })];
        full_children.extend(children);

        TrainXML {
            prompts: Some(TrainXMLPrompts {
                prompt: vec![TrainXMLPromptsPrompt {
                    id: "1".to_string(),
                    content: "Test prompt".to_string(),
                }],
            }),
            system_prompts: None,
            beyond_scope: None,
            responses: None,
            sources: None,
            code_snippets: None,
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![TrainXMLSamplesSample {
                    children: full_children,
                }]),
            }),
            constants: None,
            phrases: None,
        }
    }

    #[test]
    fn test_validate_line_breaks_valid_1() {
        let train_xml = create_test_xml_with_line_breaks(vec![1]);
        assert!(validate_line_breaks(&train_xml).is_ok());
    }

    #[test]
    fn test_validate_line_breaks_valid_2() {
        let train_xml = create_test_xml_with_line_breaks(vec![2]);
        assert!(validate_line_breaks(&train_xml).is_ok());
    }

    #[test]
    fn test_validate_line_breaks_valid_multiple() {
        let train_xml = create_test_xml_with_line_breaks(vec![1, 2, 1, 2]);
        assert!(validate_line_breaks(&train_xml).is_ok());
    }

    #[test]
    fn test_validate_line_breaks_invalid_0() {
        let train_xml = create_test_xml_with_line_breaks(vec![0]);
        let result = validate_line_breaks(&train_xml);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("count = 0"));
        assert!(err.contains("must be 1 or 2"));
    }

    #[test]
    fn test_validate_line_breaks_invalid_3() {
        let train_xml = create_test_xml_with_line_breaks(vec![3]);
        let result = validate_line_breaks(&train_xml);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("count = 3"));
    }

    #[test]
    fn test_validate_line_breaks_invalid_5() {
        let train_xml = create_test_xml_with_line_breaks(vec![5]);
        let result = validate_line_breaks(&train_xml);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_line_breaks_mixed_valid_invalid() {
        let train_xml = create_test_xml_with_line_breaks(vec![1, 2, 3, 1]);
        let result = validate_line_breaks(&train_xml);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("count = 3"));
        assert!(err.contains("child"), "Error should mention child number, got: {}", err);
    }

    #[test]
    fn test_validate_line_breaks_no_samples() {
        let train_xml = TrainXML {
            system_prompts: None,
            prompts: None,
            responses: None,
            sources: None,
            code_snippets: None,
            samples: None,
            constants: None,
            phrases: None,
            beyond_scope: None,
        };
        assert!(validate_line_breaks(&train_xml).is_ok());
    }

    #[test]
    fn test_validate_line_breaks_no_line_breaks() {
        let train_xml = TrainXML {
            prompts: Some(TrainXMLPrompts {
                prompt: vec![TrainXMLPromptsPrompt {
                    id: "1".to_string(),
                    content: "Test".to_string(),
                }],
            }),
            system_prompts: None,
            responses: None,
            sources: None,
            code_snippets: None,
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![TrainXMLSamplesSample {
                    children: vec![TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "1".to_string() })], // Only prompt, no line breaks
                }]),
            }),
            constants: None,
            phrases: None,
            beyond_scope: None,
        };
        assert!(validate_line_breaks(&train_xml).is_ok());
    }

    #[test]
    fn test_validate_prompt_presence_valid() {
        let train_xml = TrainXML {
            prompts: Some(TrainXMLPrompts {
                prompt: vec![TrainXMLPromptsPrompt {
                    id: "1".to_string(),
                    content: "Test".to_string(),
                }],
            }),
            system_prompts: None,
            responses: None,
            sources: None,
            code_snippets: None,
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "1".to_string() }),
                            TrainXMLSamplesSampleChildren::Response(crate::train_xml::train_xml_structs::TrainXMLSamplesResponse { id: "1".to_string() }),
                        ],
                    },
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "1".to_string() }),
                        ],
                    },
                ]),
            }),
            constants: None,
            phrases: None,
            beyond_scope: None,
        };
        
        let result = super::validate_prompt_presence(&train_xml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_prompt_presence_missing() {
        let train_xml = TrainXML {
            prompts: Some(TrainXMLPrompts {
                prompt: vec![TrainXMLPromptsPrompt {
                    id: "1".to_string(),
                    content: "Test".to_string(),
                }],
            }),
            system_prompts: None,
            responses: None,
            sources: None,
            code_snippets: None,
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Response(crate::train_xml::train_xml_structs::TrainXMLSamplesResponse { id: "1".to_string() }),
                        ],
                    },
                ]),
            }),
            constants: None,
            phrases: None,
            beyond_scope: None,
        };
        
        let result = super::validate_prompt_presence(&train_xml);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Sample 1 is missing a required <prompt> element"));
    }

    #[test]
    fn test_validate_prompt_presence_multiple_missing() {
        let train_xml = TrainXML {
            prompts: Some(TrainXMLPrompts {
                prompt: vec![TrainXMLPromptsPrompt {
                    id: "1".to_string(),
                    content: "Test".to_string(),
                }],
            }),
            system_prompts: None,
            responses: None,
            sources: None,
            code_snippets: None,
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Response(crate::train_xml::train_xml_structs::TrainXMLSamplesResponse { id: "1".to_string() }),
                        ],
                    },
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "1".to_string() }),
                        ],
                    },
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Source(crate::train_xml::train_xml_structs::TrainXMLSamplesSource { id: "1".to_string() }),
                        ],
                    },
                ]),
            }),
            constants: None,
            phrases: None,
            beyond_scope: None,
        };
        
        let result = super::validate_prompt_presence(&train_xml);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Sample 1"), "Error should mention Sample 1, got: {}", err);
        assert!(err.contains("Sample 3"), "Error should mention Sample 3, got: {}", err);
        assert!(!err.contains("Sample 2"), "Error should not mention Sample 2, got: {}", err);
    }

    #[test]
    fn test_train_xml_validate_success() {
        let train_xml = TrainXML {
            prompts: Some(TrainXMLPrompts {
                prompt: vec![TrainXMLPromptsPrompt {
                    id: "1".to_string(),
                    content: "Test prompt".to_string(),
                }],
            }),
            responses: Some(TrainXMLResponses {
                response: vec![TrainXMLResponsesResponse {
                    id: "1".to_string(),
                    content: "Test response".to_string(),
                }],
            }),
            system_prompts: None,
            beyond_scope: None,
            sources: None,
            code_snippets: None,
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![TrainXMLSamplesSample {
                    children: vec![
                        TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "1".to_string() }),
                        TrainXMLSamplesSampleChildren::Response(crate::train_xml::train_xml_structs::TrainXMLSamplesResponse { id: "1".to_string() }),
                    ],
                }]),
            }),
            constants: None,
            phrases: None,
        };
        
        let result = train_xml_validate(&train_xml);
        assert!(result.is_ok());
        
        let (id_maps, constants, patterns) = result.unwrap();
        
        // Verify ID maps
        assert_eq!(id_maps.prompts.len(), 1);
        assert_eq!(id_maps.responses.len(), 1);
        
        // Verify constants (default values)
        assert_eq!(constants.warmup_steps, 100);
        assert_eq!(constants.learning_rate, 1e-3);
        
        // Verify patterns (empty since no phrases)
        assert!(patterns.is_empty());
    }

    #[test]
    fn test_train_xml_validate_with_phrases() {
        let train_xml = TrainXML {
            prompts: Some(TrainXMLPrompts {
                prompt: vec![TrainXMLPromptsPrompt {
                    id: "1".to_string(),
                    content: "Test prompt".to_string(),
                }],
            }),
            responses: Some(TrainXMLResponses {
                response: vec![TrainXMLResponsesResponse {
                    id: "1".to_string(),
                    content: "Test response".to_string(),
                }],
            }),
            system_prompts: None,
            beyond_scope: None,
            sources: None,
            code_snippets: None,
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![TrainXMLSamplesSample {
                    children: vec![
                        TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "1".to_string() }),
                        TrainXMLSamplesSampleChildren::Response(crate::train_xml::train_xml_structs::TrainXMLSamplesResponse { id: "1".to_string() }),
                    ],
                }]),
            }),
            constants: None,
            phrases: Some(TrainXMLPhrases {
                phrase: vec![
                    TrainXMLPhrasesPhrase {
                        pattern: "test".to_string(),
                        variant: vec![
                            TrainXMLPhrasesVariant { value: "variant1".to_string() },
                        ],
                    },
                ],
            }),
        };
        
        let result = train_xml_validate(&train_xml);
        assert!(result.is_ok());
        
        let (id_maps, _constants, patterns) = result.unwrap();
        
        // Verify ID maps
        assert_eq!(id_maps.prompts.len(), 1);
        
        // Verify patterns were compiled
        assert_eq!(patterns.len(), 1);
        assert!(patterns[0].regex.is_match("test"));
        assert_eq!(patterns[0].variants.len(), 1);
        assert_eq!(patterns[0].variants[0], "variant1");
    }

    #[test]
    fn test_train_xml_validate_with_line_breaks_invalid() {
        // Create a train XML with an invalid line break
        let children = vec![
            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "1".to_string() }),
            TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 1 }),
            TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 2 }),
            TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 3 }), // Invalid
        ];

        let train_xml = TrainXML {
            prompts: Some(TrainXMLPrompts {
                prompt: vec![TrainXMLPromptsPrompt {
                    id: "1".to_string(),
                    content: "Test prompt".to_string(),
                }],
            }),
            system_prompts: None,
            beyond_scope: None,
            responses: None,
            sources: None,
            code_snippets: None,
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![TrainXMLSamplesSample {
                    children,
                }]),
            }),
            constants: None,
            phrases: None,
        };
        
        let result = train_xml_validate(&train_xml);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("line break"));
        assert!(err.contains("count = 3"));
    }

    #[test]
    fn test_train_xml_validate_with_missing_prompt() {
        let train_xml = TrainXML {
            prompts: Some(TrainXMLPrompts {
                prompt: vec![TrainXMLPromptsPrompt {
                    id: "1".to_string(),
                    content: "Test prompt".to_string(),
                }],
            }),
            system_prompts: None,
            beyond_scope: None,
            responses: None,
            sources: None,
            code_snippets: None,
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![TrainXMLSamplesSample {
                    children: vec![
                        // No prompt element!
                        TrainXMLSamplesSampleChildren::Response(crate::train_xml::train_xml_structs::TrainXMLSamplesResponse { id: "1".to_string() }),
                    ],
                }]),
            }),
            constants: None,
            phrases: None,
        };
        
        let result = train_xml_validate(&train_xml);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("missing a required <prompt> element"));
    }
}
