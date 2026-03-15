// src/train_xml/train_xml_validate_ids.rs

use crate::train_xml::{TrainXML, TrainXMLIds};


/// Validates that every ID referenced in the train XML exists in TrainXMLIds
pub fn train_xml_validate_ids(train_xml: &TrainXML, ids: &TrainXMLIds) -> Result<(), Vec<String>> {
    let mut errors: Vec<String> = Vec::new();
    
    // Check prompt IDs in samples
    if let Some(samples) = &train_xml.samples {
        // Check sample-ids elements
        if let Some(sample_ids) = &samples.sample_ids {
            for sample_id in sample_ids {
                // Check prompt ID
                if !ids.prompt_ids.contains(&sample_id.prompt) {
                    errors.push(format!(
                        "Sample-ids references unknown prompt ID '{}'",
                        sample_id.prompt
                    ));
                }
                
                // Check response ID if present
                if let Some(response_id) = &sample_id.response {
                    if !ids.response_ids.contains(response_id) {
                        errors.push(format!(
                            "Sample-ids references unknown response ID '{}'",
                            response_id
                        ));
                    }
                }
                
                // Check source ID if present
                if let Some(source_id) = &sample_id.source {
                    if !ids.source_ids.contains(source_id) {
                        errors.push(format!(
                            "Sample-ids references unknown source ID '{}'",
                            source_id
                        ));
                    }
                }
                
                // Check code ID if present
                if let Some(code_id) = &sample_id.code {
                    if !ids.code_ids.contains(code_id) {
                        errors.push(format!(
                            "Sample-ids references unknown code ID '{}'",
                            code_id
                        ));
                    }
                }
            }
        }
        
        // Check verbose sample elements
        if let Some(samples_verbose) = &samples.sample {
            for sample in samples_verbose {
                // Check prompt ID
                if !ids.prompt_ids.contains(&sample.prompt.id) {
                    errors.push(format!(
                        "Sample references unknown prompt ID '{}'",
                        sample.prompt.id
                    ));
                }
                
                // Check response IDs if present
                if let Some(responses) = &sample.response {
                    for response in responses {
                        if !ids.response_ids.contains(&response.id) {
                            errors.push(format!(
                                "Sample references unknown response ID '{}'",
                                response.id
                            ));
                        }
                    }
                }
                
                // Check response-ids if present
                if let Some(response_ids) = &sample.response_ids {
                    for response_id in response_ids {
                        if !ids.response_ids.contains(&response_id.response) {
                            errors.push(format!(
                                "Sample response-ids references unknown response ID '{}'",
                                response_id.response
                            ));
                        }
                        
                        // Check source ID in response-ids if present
                        if let Some(source_id) = &response_id.source {
                            if !ids.source_ids.contains(source_id) {
                                errors.push(format!(
                                    "Sample response-ids references unknown source ID '{}'",
                                    source_id
                                ));
                            }
                        }
                    }
                }
                
                // Check source IDs if present
                if let Some(sources) = &sample.source {
                    for source in sources {
                        if !ids.source_ids.contains(&source.id) {
                            errors.push(format!(
                                "Sample references unknown source ID '{}'",
                                source.id
                            ));
                        }
                    }
                }
                
                // Check code IDs if present
                if let Some(codes) = &sample.code {
                    for code in codes {
                        if !ids.code_ids.contains(&code.id) {
                            errors.push(format!(
                                "Sample references unknown code ID '{}'",
                                code.id
                            ));
                        }
                    }
                }
            }
        }
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}



#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::train_xml::{
        TrainXML,
        TrainXMLIds,
        train_xml_validate_ids::train_xml_validate_ids,
        train_xml_prompts::{TrainXMLPrompts, TrainXMLPromptsPrompt},
        train_xml_sources::{TrainXMLSources, TrainXMLSourcesSource},
        train_xml_responses::{TrainXMLResponses, TrainXMLResponsesResponse},
        train_xml_code_snippets::{TrainXMLCodeSnippets, TrainXMLCodeSnippetsCode},
        train_xml_samples::{TrainXMLSamples, TrainXMLSamplesCode, TrainXMLSamplesPrompt, TrainXMLSamplesResponse, TrainXMLSamplesResponseIds, TrainXMLSamplesSample, TrainXMLSamplesSampleIds, TrainXMLSamplesSource},
    };

    #[test]
    fn test_validate_ids_success() {
        // Create minimal valid train XML
        let train_xml = TrainXML {
            prompts: Some(TrainXMLPrompts {
                prompt: vec![
                    TrainXMLPromptsPrompt { id: "p1".to_string(), content: "Test prompt".to_string() },
                    TrainXMLPromptsPrompt { id: "p2".to_string(), content: "Another prompt".to_string() },
                ],
            }),
            responses: Some(TrainXMLResponses {
                response: vec![
                    TrainXMLResponsesResponse { id: "r1".to_string(), content: "Test response".to_string() },
                ],
            }),
            sources: Some(TrainXMLSources {
                source: vec![
                    TrainXMLSourcesSource { id: "s1".to_string(), url: "https://example.com".to_string(), title: None },
                ],
            }),
            code_snippets: Some(TrainXMLCodeSnippets {
                code: vec![
                    TrainXMLCodeSnippetsCode { id: "c1".to_string(), lang: "rust".to_string(), content: "fn main() {}".to_string() },
                ],
            }),
            samples: Some(TrainXMLSamples {
                sample_ids: Some(vec![
                    TrainXMLSamplesSampleIds { 
                        prompt: "p1".to_string(), 
                        response: Some("r1".to_string()), 
                        source: Some("s1".to_string()), 
                        code: Some("c1".to_string()) 
                    },
                ]),
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        prompt: TrainXMLSamplesPrompt { id: "p2".to_string() },
                        response_ids: None,
                        response: Some(vec![TrainXMLSamplesResponse { id: "r1".to_string() }]),
                        source: Some(vec![TrainXMLSamplesSource { id: "s1".to_string() }]),
                        code: Some(vec![TrainXMLSamplesCode { id: "c1".to_string() }]),
                    },
                ]),
            }),
            constants: None,
            phrases: None,
        };

        let ids = TrainXMLIds {
            prompt_ids: HashSet::from(["p1".to_string(), "p2".to_string()]),
            response_ids: HashSet::from(["r1".to_string()]),
            source_ids: HashSet::from(["s1".to_string()]),
            code_ids: HashSet::from(["c1".to_string()]),
        };

        let result = train_xml_validate_ids(&train_xml, &ids);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_ids_errors() {
        // Create train XML with multiple ID errors
        let train_xml = TrainXML {
            prompts: Some(TrainXMLPrompts {
                prompt: vec![
                    TrainXMLPromptsPrompt { id: "p1".to_string(), content: "Test prompt".to_string() },
                ],
            }),
            responses: Some(TrainXMLResponses {
                response: vec![
                    TrainXMLResponsesResponse { id: "r1".to_string(), content: "Test response".to_string() },
                ],
            }),
            sources: Some(TrainXMLSources {
                source: vec![
                    TrainXMLSourcesSource { id: "s1".to_string(), url: "https://example.com".to_string(), title: None },
                ],
            }),
            code_snippets: Some(TrainXMLCodeSnippets {
                code: vec![
                    TrainXMLCodeSnippetsCode { id: "c1".to_string(), lang: "rust".to_string(), content: "fn main() {}".to_string() },
                ],
            }),
            samples: Some(TrainXMLSamples {
                sample_ids: Some(vec![
                    TrainXMLSamplesSampleIds { 
                        prompt: "p1".to_string(), 
                        response: Some("r99".to_string()),  // Invalid response ID
                        source: Some("s99".to_string()),    // Invalid source ID
                        code: Some("c99".to_string()),      // Invalid code ID
                    },
                    TrainXMLSamplesSampleIds { 
                        prompt: "p99".to_string(),          // Invalid prompt ID
                        response: Some("r1".to_string()), 
                        source: Some("s1".to_string()), 
                        code: Some("c1".to_string()) 
                    },
                ]),
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        prompt: TrainXMLSamplesPrompt { id: "p99".to_string() },  // Invalid prompt ID
                        response_ids: None,
                        response: Some(vec![TrainXMLSamplesResponse { id: "r99".to_string() }]),  // Invalid response ID
                        source: Some(vec![
                            TrainXMLSamplesSource { id: "s1".to_string() },
                            TrainXMLSamplesSource { id: "s99".to_string() },  // Invalid source ID
                        ]),
                        code: Some(vec![TrainXMLSamplesCode { id: "c99".to_string() }]),  // Invalid code ID
                    },
                    TrainXMLSamplesSample {
                        prompt: TrainXMLSamplesPrompt { id: "p1".to_string() },
                        response_ids: Some(vec![
                            TrainXMLSamplesResponseIds { 
                                response: "r99".to_string(),  // Invalid response ID
                                source: Some("s1".to_string()) 
                            },
                            TrainXMLSamplesResponseIds { 
                                response: "r1".to_string(), 
                                source: Some("s99".to_string())  // Invalid source ID
                            },
                        ]),
                        response: None,
                        source: None,
                        code: None,
                    },
                ]),
            }),
            constants: None,
            phrases: None,
        };

        let ids = TrainXMLIds {
            prompt_ids: HashSet::from(["p1".to_string()]),
            response_ids: HashSet::from(["r1".to_string()]),
            source_ids: HashSet::from(["s1".to_string()]),
            code_ids: HashSet::from(["c1".to_string()]),
        };

        let result = train_xml_validate_ids(&train_xml, &ids);
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        assert!(errors.len() >= 7);  // We expect multiple errors
        
        // Verify some specific errors are present
        let error_string = errors.join("\n");
        assert!(error_string.contains("prompt ID 'p99'"));
        assert!(error_string.contains("response ID 'r99'"));
        assert!(error_string.contains("source ID 's99'"));
        assert!(error_string.contains("code ID 'c99'"));
    }
}
