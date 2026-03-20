// src/train_xml/train_xml_validate.rs

use std::collections::HashMap;
pub use crate::train_xml::{TrainXML, TrainXMLIdMaps};
use crate::train_xml::{
    TrainXMLConstantParsed,
    train_xml_validate_ids,
    train_xml_constants_parse,
    TrainXMLSamplesSampleChildren,
};


pub fn train_xml_validate(train_xml: &TrainXML) -> (TrainXMLIdMaps<'_>, TrainXMLConstantParsed, HashMap<&str, Vec<&str>>) {
    let train_xml_id_maps_map = TrainXMLIdMaps::create(train_xml).expect("❌ Should have valid id's");
    let train_xml_constants_parsed = train_xml_constants_parse(&train_xml.constants).expect("❌ Should have valid constants:");

    train_xml_validate_ids(train_xml, &train_xml_id_maps_map).expect("❌ Should have valid id's");
    
    validate_line_breaks(train_xml).expect("❌ Line break count must be 1 or 2");

    let train_xml_phrase_map = create_phrase_map(&train_xml);

    (train_xml_id_maps_map, train_xml_constants_parsed, train_xml_phrase_map)
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


/// Create phrase map using pattern instead of key
fn create_phrase_map(train_xml: &TrainXML) -> HashMap<&str, Vec<&str>> {
    train_xml.phrases
        .as_ref()  // Handle Option
        .iter()
        .flat_map(|p| p.phrase.iter())
        .map(|phrase| {
            let pattern = phrase.pattern.as_str();
            let variants: Vec<&str> = phrase.variant.iter().map(|v| v.value.as_str()).collect();
            (pattern, variants)
        })
        .collect()
}



#[cfg(test)]
mod tests {
    use super::{create_phrase_map, validate_line_breaks};
    use crate::train_xml::{
        TrainXML,
        TrainXMLPrompts,
        TrainXMLPromptsPrompt,
        TrainXMLSamples,
        TrainXMLSamplesSample,
        TrainXMLSamplesPrompt,
        TrainXMLSamplesSampleChildren,
        TrainXMLLineBreak,
        TrainXMLPhrases,
        TrainXMLPhrasesPhrase,
        TrainXMLPhrasesVariant,
        train_xml_validate,
    };

    fn create_test_xml_with_line_breaks(line_break_counts: Vec<u8>) -> TrainXML {
        let children: Vec<TrainXMLSamplesSampleChildren> = line_break_counts
            .into_iter()
            .map(|count| TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count }))
            .collect();

        TrainXML {
            prompts: Some(TrainXMLPrompts {
                prompt: vec![TrainXMLPromptsPrompt {
                    id: "1".to_string(),
                    content: "Test prompt".to_string(),
                }],
            }),
            responses: None,
            sources: None,
            code_snippets: None,
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![TrainXMLSamplesSample {
                    prompt: TrainXMLSamplesPrompt { id: "1".to_string() },
                    children,
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
        // Should fail on the first invalid one
        assert!(err.contains("child 3")); // The 3rd child (index 2 + 1) is the invalid one
    }

    #[test]
    fn test_validate_line_breaks_no_samples() {
        let train_xml = TrainXML {
            prompts: None,
            responses: None,
            sources: None,
            code_snippets: None,
            samples: None,
            constants: None,
            phrases: None,
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
            responses: None,
            sources: None,
            code_snippets: None,
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![TrainXMLSamplesSample {
                    prompt: TrainXMLSamplesPrompt { id: "1".to_string() },
                    children: vec![], // No line breaks
                }]),
            }),
            constants: None,
            phrases: None,
        };
        assert!(validate_line_breaks(&train_xml).is_ok());
    }

    #[test]
    fn test_create_phrase_map() {
        let train_xml = TrainXML {
            prompts: None,
            responses: None,
            sources: None,
            code_snippets: None,
            samples: None,
            constants: None,
            phrases: Some(TrainXMLPhrases {
                phrase: vec![
                    TrainXMLPhrasesPhrase {
                        pattern: "What is a (.*?)\\?".to_string(),
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
            }),
        };
        
        let phrase_map = create_phrase_map(&train_xml);
        assert_eq!(phrase_map.len(), 2);
        assert!(phrase_map.contains_key("What is a (.*?)\\?"));
        assert!(phrase_map.contains_key("ty"));
        assert_eq!(phrase_map["What is a (.*?)\\?"].len(), 2);
        assert_eq!(phrase_map["ty"].len(), 2);
    }

    #[test]
    fn test_create_phrase_map_no_phrases() {
        let train_xml = TrainXML {
            prompts: None,
            responses: None,
            sources: None,
            code_snippets: None,
            samples: None,
            constants: None,
            phrases: None,
        };
        
        let phrase_map = create_phrase_map(&train_xml);
        assert!(phrase_map.is_empty());
    }
    
    #[test]
    fn test_train_xml_validate_with_line_breaks() {
        // Test that the main validate function properly calls validate_line_breaks
        let train_xml = create_test_xml_with_line_breaks(vec![1, 2, 3]); // Invalid (3)
        
        // This should panic because of the invalid line break
        let result = std::panic::catch_unwind(|| {
            train_xml_validate(&train_xml);
        });
        
        assert!(result.is_err());
    }
}
