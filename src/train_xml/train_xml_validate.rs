// src/train_xml/train_xml_validate.rs

use crate::device::Device;
use crate::train_xml::{
    TrainXML,
    TrainXMLIdMaps,
    TrainXMLPhrasePattern,
    TrainXMLConstantParsed,
    train_xml_validate_ids,
    train_xml_phrase_patterns,
    train_xml_validate_precision,
    train_xml_validate_line_breaks,
    train_xml_validate_prompt_presence,
};


pub fn train_xml_validate<'a>(
    train_xml: &'a TrainXML, 
    device: &Device
) -> Result<(TrainXMLIdMaps<'a>, TrainXMLConstantParsed, Vec<TrainXMLPhrasePattern>), Box<dyn std::error::Error>> {
    let train_xml_id_maps_map = TrainXMLIdMaps::create(train_xml)?;

    train_xml_validate_ids(train_xml, &train_xml_id_maps_map).map_err(|errors| {
        let error_string = errors.join("\n  ");
        format!("❌ Failed validating train xml ids:\n  {}", error_string)
    })?;
    
    train_xml_validate_line_breaks(train_xml)?;
    
    train_xml_validate_prompt_presence(train_xml)?;

    train_xml_validate_precision(train_xml)?;

    // Updated: Use merge_with_defaults instead of train_xml_constants_parse
    let train_xml_constants_parsed = train_xml.constants
        .as_ref()
        .unwrap_or(&Default::default())
        .merge_with_defaults(device);

    let train_xml_patterns = train_xml_phrase_patterns(train_xml);

    Ok((train_xml_id_maps_map, train_xml_constants_parsed, train_xml_patterns))
}



#[cfg(test)]
mod tests {
    use crate::device::Device;
    use super::train_xml_validate;
    use crate::train_xml::{
        TrainXML,
        TrainXMLSamples,
        TrainXMLPrompts,
        TrainXMLConstants,
        TrainXMLLineBreak,
        TrainXMLSamplesSample,
        TrainXMLSamplesPrompt,
        TrainXMLPromptsPrompt,
        TrainXMLSamplesSampleChildren,
    };

    fn create_valid_train_xml() -> TrainXML {
        TrainXML {
            prompts: Some(TrainXMLPrompts {
                prompt: vec![
                    TrainXMLPromptsPrompt {
                        id: "1".to_string(),
                        content: "Test prompt".to_string(),
                    },
                ],
            }),
            constants: Some(TrainXMLConstants {
                activation_precision: Some("int8".to_string()),
                kv_cache_precision: Some("int8".to_string()),
                rope_precision: Some("fp16".to_string()),
                ..Default::default()
            }),
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
        }
    }

    #[test]
    fn test_train_xml_validate_success() {
        let train_xml = create_valid_train_xml();
        let device = Device::Cpu;
        
        let result = train_xml_validate(&train_xml, &device);
        assert!(result.is_ok(), "Expected Ok, got Err: {:?}", result.err());
    }

    #[test]
    fn test_train_xml_validate_fails_line_breaks() {
        let mut train_xml = create_valid_train_xml();
        if let Some(samples) = &mut train_xml.samples {
            if let Some(sample_list) = &mut samples.sample {
                sample_list[0].children.push(
                    TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 3 })
                );
            }
        }
        
        let device = Device::Cpu;
        let result = train_xml_validate(&train_xml, &device);
        assert!(result.is_err(), "Expected Err, got Ok");
        let err = result.unwrap_err().to_string();
        assert!(err.contains("count = 3") || err.contains("line break"), "Error: {}", err);
    }

    #[test]
    fn test_train_xml_validate_fails_prompt_presence() {
        let mut train_xml = create_valid_train_xml();
        if let Some(samples) = &mut train_xml.samples {
            if let Some(sample_list) = &mut samples.sample {
                sample_list[0].children.clear();
            }
        }
        
        let device = Device::Cpu;
        let result = train_xml_validate(&train_xml, &device);
        assert!(result.is_err(), "Expected Err, got Ok");
        let err = result.unwrap_err().to_string();
        assert!(err.contains("missing a required <prompt>"), "Error: {}", err);
    }

    #[test]
    fn test_train_xml_validate_fails_precision() {
        let mut train_xml = create_valid_train_xml();
        if let Some(constants) = &mut train_xml.constants {
            constants.activation_precision = Some("fp16".to_string()); // Invalid
        }
        
        let device = Device::Cpu;
        let result = train_xml_validate(&train_xml, &device);
        assert!(result.is_err(), "Expected Err, got Ok");
        let err = result.unwrap_err().to_string();
        assert!(err.contains("activation_precision"), "Error: {}", err);
    }

    #[test]
    fn test_train_xml_validate_merges_defaults() {
        let train_xml = TrainXML {
            prompts: Some(TrainXMLPrompts {
                prompt: vec![
                    TrainXMLPromptsPrompt {
                        id: "1".to_string(),
                        content: "Test prompt".to_string(),
                    },
                ],
            }),
            constants: Some(TrainXMLConstants {
                aim_train_gb: Some(10.0), // Custom value
                activation_precision: Some("int8".to_string()),
                kv_cache_precision: Some("int8".to_string()),
                rope_precision: Some("fp16".to_string()),
                ..Default::default()
            }),
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
        
        let device = Device::Cpu;
        let result = train_xml_validate(&train_xml, &device);
        assert!(result.is_ok(), "Expected Ok, got Err: {:?}", result.err());
        
        let (_, parsed, _) = result.unwrap();
        assert_eq!(parsed.aim_train_gb, 10.0, "Custom value not preserved");
        assert!(parsed.batch_size > 0, "Default value not applied");
    }

    #[test]
    fn test_train_xml_validate_no_constants() {
        let train_xml = TrainXML {
            prompts: Some(TrainXMLPrompts {
                prompt: vec![
                    TrainXMLPromptsPrompt {
                        id: "1".to_string(),
                        content: "Test prompt".to_string(),
                    },
                ],
            }),
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
        
        let device = Device::Cpu;
        let result = train_xml_validate(&train_xml, &device);
        assert!(result.is_ok(), "Expected Ok, got Err: {:?}", result.err());
        
        let (_, parsed, _) = result.unwrap();
        assert_eq!(parsed.aim_train_gb, 7.0, "Default value not applied");
    }
}
