// src/train_xml/train_xml_validate_prompt_presence.rs

use crate::train_xml::{TrainXML, TrainXMLSamplesSampleChildren};


/// Validates that every sample has at least one <prompt> element
/// and that every sample-ids has a prompt attribute
pub fn train_xml_validate_prompt_presence(train_xml: &TrainXML) -> Result<(), String> {
    let mut errors = Vec::new();
    
    if let Some(samples) = &train_xml.samples {
        // Validate sample-ids elements have prompt attribute
        if let Some(sample_ids_list) = &samples.sample_ids {
            for (sample_idx, sample_id) in sample_ids_list.iter().enumerate() {
                if sample_id.prompt.is_empty() {
                    errors.push(format!(
                        "Sample-ids {} is missing a required prompt attribute",
                        sample_idx + 1
                    ));
                }
            }
        }
        
        // Validate sample elements have at least one prompt child
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
        TrainXMLSamplesSampleIds,
        TrainXMLSamplesSampleChildren,
    };

    // ========== Tests for sample-ids ==========

    #[test]
    fn test_validate_prompt_presence_sample_ids_valid() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: Some(vec![
                    TrainXMLSamplesSampleIds {
                        system: None,
                        prompt: "prompt1".to_string(),
                        thought: None,
                        response: None,
                        source: None,
                        code: None,
                    },
                ]),
                sample: None,
            }),
            ..Default::default()
        };

        let result = train_xml_validate_prompt_presence(&train_xml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_prompt_presence_sample_ids_missing_prompt_attribute() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: Some(vec![
                    TrainXMLSamplesSampleIds {
                        system: None,
                        prompt: String::new(),  // Empty prompt attribute
                        thought: None,
                        response: None,
                        source: None,
                        code: None,
                    },
                ]),
                sample: None,
            }),
            ..Default::default()
        };

        let result = train_xml_validate_prompt_presence(&train_xml);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("missing a required prompt attribute"));
    }

    // ========== Tests for sample elements ==========

    #[test]
    fn test_validate_prompt_presence_sample_valid_single_prompt() {
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
    fn test_validate_prompt_presence_sample_valid_multiple_prompts() {
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
    fn test_validate_prompt_presence_sample_missing_prompt() {
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
    fn test_validate_prompt_presence_multiple_samples_missing_prompts() {
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

    // ========== Combined tests ==========

    #[test]
    fn test_validate_prompt_presence_both_sample_ids_and_samples_valid() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: Some(vec![
                    TrainXMLSamplesSampleIds {
                        system: None,
                        prompt: "prompt1".to_string(),
                        thought: None,
                        response: None,
                        source: None,
                        code: None,
                    },
                ]),
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { 
                                id: "prompt2".to_string() 
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
}
