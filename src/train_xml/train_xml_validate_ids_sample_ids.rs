// src/train_xml/train_xml_validate_ids_sample_ids.rs

use crate::train_xml::{TrainXML, TrainXMLIdMaps, TrainXMLSamplesSampleIds};


pub fn train_xml_validate_ids_sample_ids(
    train_xml: &TrainXML,
    ids: &TrainXMLIdMaps,
    errors: &mut Vec<String>,
) {
    if let Some(samples) = &train_xml.samples {
        if let Some(sample_ids_list) = &samples.sample_ids {
            for sample_id in sample_ids_list {
                validate_sample_ids(sample_id, ids, errors);
            }
        }
    }
}

fn validate_sample_ids(
    sample_id: &TrainXMLSamplesSampleIds,
    ids: &TrainXMLIdMaps,
    errors: &mut Vec<String>,
) {
    // Check system ID if present
    if let Some(system_id) = &sample_id.system {
        if !ids.system_prompts.contains_key(system_id) {
            errors.push(format!(
                "Sample-ids references unknown system prompt ID '{}'",
                system_id
            ));
        }
    }
    
    // Check prompt ID
    if !ids.prompts.contains_key(&sample_id.prompt) {
        errors.push(format!(
            "Sample-ids references unknown prompt ID '{}'",
            sample_id.prompt
        ));
    }

    // Check thought ID if present
    if let Some(thought_id) = &sample_id.thought {
        if !ids.thoughts.contains_key(thought_id) {
            errors.push(format!(
                "Sample-ids references unknown thought ID '{}'",
                thought_id
            ));
        }
    }

    // Check response ID if present
    if let Some(response_id) = &sample_id.response {
        if !ids.responses.contains_key(response_id) {
            errors.push(format!(
                "Sample-ids references unknown response ID '{}'",
                response_id
            ));
        }
    }
    
    // Check source ID if present
    if let Some(source_id) = &sample_id.source {
        if !ids.sources.contains_key(source_id) {
            errors.push(format!(
                "Sample-ids references unknown source ID '{}'",
                source_id
            ));
        }
    }
    
    // Check code ID if present
    if let Some(code_id) = &sample_id.code {
        if !ids.code_snippets.contains_key(code_id) {
            errors.push(format!(
                "Sample-ids references unknown code ID '{}'",
                code_id
            ));
        }
    }
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::train_xml_validate_ids_sample_ids;
    use crate::train_xml::{
        TrainXML,
        TrainXMLIdMaps,
        TrainXMLSamples,
        TrainXMLSamplesSampleIds,
        TrainXMLSystemPromptsSystem,
        TrainXMLPromptsPrompt,
        TrainXMLThoughtsThought,
        TrainXMLResponsesResponse,
        TrainXMLSourcesSource,
        TrainXMLCodeSnippetsCode,
    };

    fn create_test_ids(
        system_ids: &[&str],
        prompt_ids: &[&str],
        thought_ids: &[&str],
        response_ids: &[&str],
        source_ids: &[&str],
        code_ids: &[&str],
    ) -> TrainXMLIdMaps<'static> {
        let mut system_prompts = HashMap::new();
        for &id in system_ids {
            let system = Box::new(TrainXMLSystemPromptsSystem {
                id: id.to_string(),
                content: "Test system".to_string(),
            });
            let system_ref: &'static TrainXMLSystemPromptsSystem = &*Box::leak(system);
            system_prompts.insert(id.to_string(), system_ref);
        }
        
        let mut prompts = HashMap::new();
        for &id in prompt_ids {
            let prompt = Box::new(TrainXMLPromptsPrompt {
                id: id.to_string(),
                content: "Test prompt".to_string(),
            });
            let prompt_ref: &'static TrainXMLPromptsPrompt = &*Box::leak(prompt);
            prompts.insert(id.to_string(), prompt_ref);
        }
        
        let mut thoughts = HashMap::new();
        for &id in thought_ids {
            let thought = Box::new(TrainXMLThoughtsThought {
                id: id.to_string(),
                content: "Test thought".to_string(),
            });
            let thought_ref: &'static TrainXMLThoughtsThought = &*Box::leak(thought);
            thoughts.insert(id.to_string(), thought_ref);
        }
        
        let mut responses = HashMap::new();
        for &id in response_ids {
            let response = Box::new(TrainXMLResponsesResponse {
                id: id.to_string(),
                content: "Test response".to_string(),
            });
            let response_ref: &'static TrainXMLResponsesResponse = &*Box::leak(response);
            responses.insert(id.to_string(), response_ref);
        }
        
        let mut sources = HashMap::new();
        for &id in source_ids {
            let source = Box::new(TrainXMLSourcesSource {
                id: id.to_string(),
                url: "https://example.com".to_string(),
                title: None,
            });
            let source_ref: &'static TrainXMLSourcesSource = &*Box::leak(source);
            sources.insert(id.to_string(), source_ref);
        }
        
        let mut code_snippets = HashMap::new();
        for &id in code_ids {
            let code = Box::new(TrainXMLCodeSnippetsCode {
                id: id.to_string(),
                lang: "rust".to_string(),
                content: "fn main() {}".to_string(),
            });
            let code_ref: &'static TrainXMLCodeSnippetsCode = &*Box::leak(code);
            code_snippets.insert(id.to_string(), code_ref);
        }
        
        TrainXMLIdMaps {
            system_prompts,
            prompts,
            thoughts,
            responses,
            sources,
            code_snippets,
        }
    }

    #[test]
    fn test_validate_ids_sample_ids_no_samples() {
        let train_xml = TrainXML {
            samples: None,
            ..Default::default()
        };
        
        let ids = create_test_ids(&[], &[], &[], &[], &[], &[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_sample_ids(&train_xml, &ids, &mut errors);
        
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_ids_sample_ids_empty_sample_ids() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: Some(vec![]),
                sample: None,
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids(&[], &[], &[], &[], &[], &[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_sample_ids(&train_xml, &ids, &mut errors);
        
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_ids_sample_ids_valid_all_ids() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: Some(vec![
                    TrainXMLSamplesSampleIds {
                        system: Some("system1".to_string()),
                        prompt: "prompt1".to_string(),
                        thought: Some("thought1".to_string()),
                        response: Some("response1".to_string()),
                        source: Some("source1".to_string()),
                        code: Some("code1".to_string()),
                    },
                ]),
                sample: None,
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids(
            &["system1"],
            &["prompt1"],
            &["thought1"],
            &["response1"],
            &["source1"],
            &["code1"],
        );
        let mut errors = Vec::new();
        
        train_xml_validate_ids_sample_ids(&train_xml, &ids, &mut errors);
        
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_ids_sample_ids_missing_prompt() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: Some(vec![
                    TrainXMLSamplesSampleIds {
                        system: None,
                        prompt: "nonexistent".to_string(),
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
        
        let ids = create_test_ids(&[], &["prompt1"], &[], &[], &[], &[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_sample_ids(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("unknown prompt ID 'nonexistent'"));
    }

    #[test]
    fn test_validate_ids_sample_ids_invalid_system_id() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: Some(vec![
                    TrainXMLSamplesSampleIds {
                        system: Some("bad_system".to_string()),
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
        
        let ids = create_test_ids(&["system1"], &["prompt1"], &[], &[], &[], &[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_sample_ids(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("unknown system prompt ID 'bad_system'"));
    }

    #[test]
    fn test_validate_ids_sample_ids_invalid_thought_id() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: Some(vec![
                    TrainXMLSamplesSampleIds {
                        system: None,
                        prompt: "prompt1".to_string(),
                        thought: Some("bad_thought".to_string()),
                        response: None,
                        source: None,
                        code: None,
                    },
                ]),
                sample: None,
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids(&[], &["prompt1"], &["thought1"], &[], &[], &[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_sample_ids(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("unknown thought ID 'bad_thought'"));
    }

    #[test]
    fn test_validate_ids_sample_ids_invalid_response_id() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: Some(vec![
                    TrainXMLSamplesSampleIds {
                        system: None,
                        prompt: "prompt1".to_string(),
                        thought: None,
                        response: Some("bad_response".to_string()),
                        source: None,
                        code: None,
                    },
                ]),
                sample: None,
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids(&[], &["prompt1"], &[], &["response1"], &[], &[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_sample_ids(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("unknown response ID 'bad_response'"));
    }

    #[test]
    fn test_validate_ids_sample_ids_invalid_source_id() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: Some(vec![
                    TrainXMLSamplesSampleIds {
                        system: None,
                        prompt: "prompt1".to_string(),
                        thought: None,
                        response: None,
                        source: Some("bad_source".to_string()),
                        code: None,
                    },
                ]),
                sample: None,
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids(&[], &["prompt1"], &[], &[], &["source1"], &[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_sample_ids(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("unknown source ID 'bad_source'"));
    }

    #[test]
    fn test_validate_ids_sample_ids_invalid_code_id() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: Some(vec![
                    TrainXMLSamplesSampleIds {
                        system: None,
                        prompt: "prompt1".to_string(),
                        thought: None,
                        response: None,
                        source: None,
                        code: Some("bad_code".to_string()),
                    },
                ]),
                sample: None,
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids(&[], &["prompt1"], &[], &[], &[], &["code1"]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_sample_ids(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("unknown code ID 'bad_code'"));
    }

    #[test]
    fn test_validate_ids_sample_ids_multiple_errors() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: Some(vec![
                    TrainXMLSamplesSampleIds {
                        system: Some("bad_system".to_string()),
                        prompt: "bad_prompt".to_string(),
                        thought: Some("bad_thought".to_string()),
                        response: Some("bad_response".to_string()),
                        source: Some("bad_source".to_string()),
                        code: Some("bad_code".to_string()),
                    },
                ]),
                sample: None,
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids(
            &["system1"],
            &["prompt1"],
            &["thought1"],
            &["response1"],
            &["source1"],
            &["code1"],
        );
        let mut errors = Vec::new();
        
        train_xml_validate_ids_sample_ids(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 6);
        assert!(errors[0].contains("unknown system prompt ID 'bad_system'"));
        assert!(errors[1].contains("unknown prompt ID 'bad_prompt'"));
        assert!(errors[2].contains("unknown thought ID 'bad_thought'"));
        assert!(errors[3].contains("unknown response ID 'bad_response'"));
        assert!(errors[4].contains("unknown source ID 'bad_source'"));
        assert!(errors[5].contains("unknown code ID 'bad_code'"));
    }

    #[test]
    fn test_validate_ids_sample_ids_multiple_sample_ids() {
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
                    TrainXMLSamplesSampleIds {
                        system: None,
                        prompt: "bad_prompt".to_string(),
                        thought: None,
                        response: None,
                        source: None,
                        code: None,
                    },
                    TrainXMLSamplesSampleIds {
                        system: None,
                        prompt: "prompt2".to_string(),
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
        
        let ids = create_test_ids(&[], &["prompt1", "prompt2"], &[], &[], &[], &[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_sample_ids(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("unknown prompt ID 'bad_prompt'"));
    }

    #[test]
    fn test_validate_ids_sample_ids_optional_fields_missing() {
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
        
        let ids = create_test_ids(&[], &["prompt1"], &[], &[], &[], &[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_sample_ids(&train_xml, &ids, &mut errors);
        
        assert!(errors.is_empty());
    }
}
