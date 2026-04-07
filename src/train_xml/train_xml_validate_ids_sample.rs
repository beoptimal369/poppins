// src/train_xml/train_xml_validate_ids_sample.rs

use crate::train_xml::{TrainXML, TrainXMLIdMaps, TrainXMLSamplesSampleChildren};


pub fn train_xml_validate_ids_sample(
    train_xml: &TrainXML,
    ids: &TrainXMLIdMaps,
    errors: &mut Vec<String>,
) {
    if let Some(samples) = &train_xml.samples {
        if let Some(samples_list) = &samples.sample {
            for sample in samples_list {
                validate_sample_children(&sample.children, ids, errors);
            }
        }
    }
}

fn validate_sample_children(
    children: &[TrainXMLSamplesSampleChildren],
    ids: &TrainXMLIdMaps,
    errors: &mut Vec<String>,
) {
    for child in children {
        match child {
            TrainXMLSamplesSampleChildren::Prompt(prompt) => {
                if !ids.prompts.contains_key(&prompt.id) {
                    errors.push(format!(
                        "Sample references unknown prompt ID '{}'",
                        prompt.id
                    ));
                }
            },
            
            TrainXMLSamplesSampleChildren::System(system) => {
                if !ids.system_prompts.contains_key(&system.id) {
                    errors.push(format!(
                        "Sample references unknown system prompt ID '{}'",
                        system.id
                    ));
                }
            },
            
            TrainXMLSamplesSampleChildren::Thought(thought) => {
                if !ids.thoughts.contains_key(&thought.id) {
                    errors.push(format!(
                        "Sample references unknown thought ID '{}'",
                        thought.id
                    ));
                }
            },
            
            TrainXMLSamplesSampleChildren::Response(response) => {
                if !ids.responses.contains_key(&response.id) {
                    errors.push(format!(
                        "Sample references unknown response ID '{}'",
                        response.id
                    ));
                }
            },
            
            TrainXMLSamplesSampleChildren::Source(source) => {
                if !ids.sources.contains_key(&source.id) {
                    errors.push(format!(
                        "Sample references unknown source ID '{}'",
                        source.id
                    ));
                }
            },
            
            TrainXMLSamplesSampleChildren::Code(code) => {
                if !ids.code_snippets.contains_key(&code.id) {
                    errors.push(format!(
                        "Sample references unknown code ID '{}'",
                        code.id
                    ));
                }
            },
            
            TrainXMLSamplesSampleChildren::ResponseIds(response_ids) => {
                if !ids.responses.contains_key(&response_ids.response) {
                    errors.push(format!(
                        "Sample response-ids references unknown response ID '{}'",
                        response_ids.response
                    ));
                }
                
                if let Some(source_id) = &response_ids.source {
                    if !ids.sources.contains_key(source_id) {
                        errors.push(format!(
                            "Sample response-ids references unknown source ID '{}'",
                            source_id
                        ));
                    }
                }
            },
            
            TrainXMLSamplesSampleChildren::LineBreak(_) => {
                // Line breaks don't have IDs, nothing to validate
            },
        }
    }
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::train_xml_validate_ids_sample;
    use crate::train_xml::{
        TrainXML,
        TrainXMLIdMaps,
        TrainXMLSamples,
        TrainXMLSamplesSample,
        TrainXMLSamplesPrompt,
        TrainXMLSamplesSystem,
        TrainXMLSamplesThought,
        TrainXMLSamplesResponse,
        TrainXMLSamplesSource,
        TrainXMLSamplesCode,
        TrainXMLSamplesResponseIds,
        TrainXMLSamplesSampleChildren,
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
    fn test_validate_ids_sample_no_samples() {
        let train_xml = TrainXML {
            samples: None,
            ..Default::default()
        };
        
        let ids = create_test_ids(&[], &[], &[], &[], &[], &[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_sample(&train_xml, &ids, &mut errors);
        
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_ids_sample_empty_samples() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![]),
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids(&[], &[], &[], &[], &[], &[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_sample(&train_xml, &ids, &mut errors);
        
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_ids_sample_valid_all_ids() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "prompt1".to_string() }),
                            TrainXMLSamplesSampleChildren::System(TrainXMLSamplesSystem { id: "system1".to_string() }),
                            TrainXMLSamplesSampleChildren::Thought(TrainXMLSamplesThought { id: "thought1".to_string() }),
                            TrainXMLSamplesSampleChildren::Response(TrainXMLSamplesResponse { id: "response1".to_string() }),
                            TrainXMLSamplesSampleChildren::Source(TrainXMLSamplesSource { id: "source1".to_string() }),
                            TrainXMLSamplesSampleChildren::Code(TrainXMLSamplesCode { 
                                id: "code1".to_string(),
                                indent: None,
                                inline: None,
                            }),
                            TrainXMLSamplesSampleChildren::ResponseIds(TrainXMLSamplesResponseIds {
                                response: "response1".to_string(),
                                source: Some("source1".to_string()),
                            }),
                        ],
                    },
                ]),
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
        
        train_xml_validate_ids_sample(&train_xml, &ids, &mut errors);
        
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_ids_sample_invalid_prompt_id() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "bad_prompt".to_string() }),
                        ],
                    },
                ]),
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids(&[], &["prompt1"], &[], &[], &[], &[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_sample(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("unknown prompt ID 'bad_prompt'"));
    }

    #[test]
    fn test_validate_ids_sample_invalid_system_id() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "prompt1".to_string() }),
                            TrainXMLSamplesSampleChildren::System(TrainXMLSamplesSystem { id: "bad_system".to_string() }),
                        ],
                    },
                ]),
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids(&["system1"], &["prompt1"], &[], &[], &[], &[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_sample(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("unknown system prompt ID 'bad_system'"));
    }

    #[test]
    fn test_validate_ids_sample_invalid_thought_id() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "prompt1".to_string() }),
                            TrainXMLSamplesSampleChildren::Thought(TrainXMLSamplesThought { id: "bad_thought".to_string() }),
                        ],
                    },
                ]),
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids(&[], &["prompt1"], &["thought1"], &[], &[], &[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_sample(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("unknown thought ID 'bad_thought'"));
    }

    #[test]
    fn test_validate_ids_sample_invalid_response_id() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "prompt1".to_string() }),
                            TrainXMLSamplesSampleChildren::Response(TrainXMLSamplesResponse { id: "bad_response".to_string() }),
                        ],
                    },
                ]),
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids(&[], &["prompt1"], &[], &["response1"], &[], &[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_sample(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("unknown response ID 'bad_response'"));
    }

    #[test]
    fn test_validate_ids_sample_invalid_source_id() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "prompt1".to_string() }),
                            TrainXMLSamplesSampleChildren::Source(TrainXMLSamplesSource { id: "bad_source".to_string() }),
                        ],
                    },
                ]),
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids(&[], &["prompt1"], &[], &[], &["source1"], &[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_sample(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("unknown source ID 'bad_source'"));
    }

    #[test]
    fn test_validate_ids_sample_invalid_code_id() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "prompt1".to_string() }),
                            TrainXMLSamplesSampleChildren::Code(TrainXMLSamplesCode { 
                                id: "bad_code".to_string(),
                                indent: None,
                                inline: None,
                            }),
                        ],
                    },
                ]),
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids(&[], &["prompt1"], &[], &[], &[], &["code1"]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_sample(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("unknown code ID 'bad_code'"));
    }

    #[test]
    fn test_validate_ids_sample_invalid_response_ids_response() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "prompt1".to_string() }),
                            TrainXMLSamplesSampleChildren::ResponseIds(TrainXMLSamplesResponseIds {
                                response: "bad_response".to_string(),
                                source: None,
                            }),
                        ],
                    },
                ]),
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids(&[], &["prompt1"], &[], &["response1"], &[], &[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_sample(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("unknown response ID 'bad_response'"));
    }

    #[test]
    fn test_validate_ids_sample_invalid_response_ids_source() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "prompt1".to_string() }),
                            TrainXMLSamplesSampleChildren::ResponseIds(TrainXMLSamplesResponseIds {
                                response: "response1".to_string(),
                                source: Some("bad_source".to_string()),
                            }),
                        ],
                    },
                ]),
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids(&[], &["prompt1"], &[], &["response1"], &["source1"], &[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_sample(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("unknown source ID 'bad_source'"));
    }

    #[test]
    fn test_validate_ids_sample_multiple_errors() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "bad_prompt".to_string() }),
                            TrainXMLSamplesSampleChildren::System(TrainXMLSamplesSystem { id: "bad_system".to_string() }),
                            TrainXMLSamplesSampleChildren::Thought(TrainXMLSamplesThought { id: "bad_thought".to_string() }),
                            TrainXMLSamplesSampleChildren::Response(TrainXMLSamplesResponse { id: "bad_response".to_string() }),
                            TrainXMLSamplesSampleChildren::Source(TrainXMLSamplesSource { id: "bad_source".to_string() }),
                            TrainXMLSamplesSampleChildren::Code(TrainXMLSamplesCode { 
                                id: "bad_code".to_string(),
                                indent: None,
                                inline: None,
                            }),
                        ],
                    },
                ]),
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
        
        train_xml_validate_ids_sample(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 6);
        assert!(errors[0].contains("unknown prompt ID 'bad_prompt'"));
        assert!(errors[1].contains("unknown system prompt ID 'bad_system'"));
        assert!(errors[2].contains("unknown thought ID 'bad_thought'"));
        assert!(errors[3].contains("unknown response ID 'bad_response'"));
        assert!(errors[4].contains("unknown source ID 'bad_source'"));
        assert!(errors[5].contains("unknown code ID 'bad_code'"));
    }

    #[test]
    fn test_validate_ids_sample_multiple_samples() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "prompt1".to_string() }),
                        ],
                    },
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "bad_prompt".to_string() }),
                        ],
                    },
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "prompt2".to_string() }),
                        ],
                    },
                ]),
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids(&[], &["prompt1", "prompt2"], &[], &[], &[], &[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_sample(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("unknown prompt ID 'bad_prompt'"));
    }

    #[test]
    fn test_validate_ids_sample_line_break_no_error() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "prompt1".to_string() }),
                            TrainXMLSamplesSampleChildren::LineBreak(crate::train_xml::TrainXMLLineBreak { count: 1 }),
                        ],
                    },
                ]),
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids(&[], &["prompt1"], &[], &[], &[], &[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_sample(&train_xml, &ids, &mut errors);
        
        assert!(errors.is_empty());
    }
}
