// src/train_xml/train_xml_validate_prompt_presence.rs

use crate::train_xml::{TrainXML, TrainXMLSamplesSampleChildren};


/// Validates that every sample has at least one <prompt> element
pub fn train_xml_validate_prompt_presence(train_xml: &TrainXML) -> Result<(), String> {
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
    use super::train_xml_validate_prompt_presence;
    use crate::train_xml::{
        TrainXML,
        TrainXMLSamples,
        TrainXMLSamplesSample,
        TrainXMLSamplesPrompt,
        TrainXMLSamplesSampleChildren,
    };

    #[test]
    fn test_validate_prompt_presence_valid_single_prompt() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { 
                                id: "1".to_string() 
                            }),
                        ],
                    },
                ]),
            }),
            ..Default::default()
        };

        let result = train_xml_validate_prompt_presence(&train_xml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_prompt_presence_valid_multiple_prompts() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { 
                                id: "1".to_string() 
                            }),
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { 
                                id: "2".to_string() 
                            }),
                        ],
                    },
                ]),
            }),
            ..Default::default()
        };

        let result = train_xml_validate_prompt_presence(&train_xml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_prompt_presence_valid_prompt_with_other_elements() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { 
                                id: "1".to_string() 
                            }),
                            // Other non-prompt elements (using placeholder variants)
                        ],
                    },
                ]),
            }),
            ..Default::default()
        };

        let result = train_xml_validate_prompt_presence(&train_xml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_prompt_presence_invalid_missing_prompt() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![],  // No prompt element
                    },
                ]),
            }),
            ..Default::default()
        };

        let result = train_xml_validate_prompt_presence(&train_xml);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Sample 1 is missing a required <prompt> element"));
    }

    #[test]
    fn test_validate_prompt_presence_invalid_multiple_samples_missing_prompts() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![],  // Missing prompt
                    },
                    TrainXMLSamplesSample {
                        children: vec![],  // Missing prompt
                    },
                ]),
            }),
            ..Default::default()
        };

        let result = train_xml_validate_prompt_presence(&train_xml);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Sample 1 is missing a required <prompt> element"));
        assert!(err.contains("Sample 2 is missing a required <prompt> element"));
    }

    #[test]
    fn test_validate_prompt_presence_invalid_some_samples_missing_prompts() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { 
                                id: "1".to_string() 
                            }),
                        ],
                    },
                    TrainXMLSamplesSample {
                        children: vec![],  // Missing prompt
                    },
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { 
                                id: "3".to_string() 
                            }),
                        ],
                    },
                ]),
            }),
            ..Default::default()
        };

        let result = train_xml_validate_prompt_presence(&train_xml);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Sample 2 is missing a required <prompt> element"));
        assert!(!err.contains("Sample 1"));
        assert!(!err.contains("Sample 3"));
    }

    #[test]
    fn test_validate_prompt_presence_no_samples() {
        let train_xml = TrainXML {
            samples: None,
            ..Default::default()
        };

        let result = train_xml_validate_prompt_presence(&train_xml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_prompt_presence_empty_samples() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![]),
            }),
            ..Default::default()
        };

        let result = train_xml_validate_prompt_presence(&train_xml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_prompt_presence_samples_with_no_children() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![],
                    },
                ]),
            }),
            ..Default::default()
        };

        let result = train_xml_validate_prompt_presence(&train_xml);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Sample 1 is missing a required <prompt> element"));
    }

    #[test]
    fn test_validate_prompt_presence_large_sample_count() {
        let mut samples = Vec::new();
        
        // Create 10 samples, 2 missing prompts
        for i in 1..=10 {
            let children = if i == 3 || i == 7 {
                vec![]  // Missing prompt
            } else {
                vec![
                    TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { 
                        id: i.to_string() 
                    }),
                ]
            };
            
            samples.push(TrainXMLSamplesSample { children });
        }
        
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(samples),
            }),
            ..Default::default()
        };

        let result = train_xml_validate_prompt_presence(&train_xml);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Sample 3 is missing a required <prompt> element"));
        assert!(err.contains("Sample 7 is missing a required <prompt> element"));
        assert!(!err.contains("Sample 1"));
        assert!(!err.contains("Sample 10"));
    }

    #[test]
    fn test_validate_prompt_preserves_error_order() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![],  // Missing - sample 1
                    },
                    TrainXMLSamplesSample {
                        children: vec![],  // Missing - sample 2
                    },
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { 
                                id: "3".to_string() 
                            }),
                        ],
                    },
                    TrainXMLSamplesSample {
                        children: vec![],  // Missing - sample 4
                    },
                ]),
            }),
            ..Default::default()
        };

        let result = train_xml_validate_prompt_presence(&train_xml);
        assert!(result.is_err());
        let err = result.unwrap_err();
        
        // Check order (should match sample indices)
        let lines: Vec<&str> = err.split('\n').collect();
        assert_eq!(lines.len(), 3); // 3 errors
        assert!(lines[0].contains("Sample 1"));
        assert!(lines[1].contains("Sample 2"));
        assert!(lines[2].contains("Sample 4"));
    }
}
