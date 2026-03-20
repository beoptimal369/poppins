// src/train_xml/train_xml_validate_ids.rs

use crate::train_xml::{
    TrainXML,
    TrainXMLIdMaps,
    TrainXMLSamplesSampleChildren,
};


/// Validates that every ID referenced in the train XML exists in TrainXMLIdMaps
pub fn train_xml_validate_ids(train_xml: &TrainXML, ids: &TrainXMLIdMaps) -> Result<(), Vec<String>> {
    let mut errors: Vec<String> = Vec::new();
    
    // Check prompt IDs in samples
    if let Some(samples) = &train_xml.samples {
        // Check sample-ids elements
        if let Some(sample_with_ids) = &samples.sample_ids {
            for sample_id in sample_with_ids {
                // Check prompt ID
                if !ids.prompts.contains_key(&sample_id.prompt) {
                    errors.push(format!(
                        "Sample-ids references unknown prompt ID '{}'",
                        sample_id.prompt
                    ));
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
        }
        
        // Process children xml tags of <sample> parent
        if let Some(sample_with_children_tags) = &samples.sample {
            for sample in sample_with_children_tags {
                // Check prompt ID
                if !ids.prompts.contains_key(&sample.prompt.id) {
                    errors.push(format!(
                        "Sample references unknown prompt ID '{}'",
                        sample.prompt.id
                    ));
                }
                
                // Check all children for ID references
                for child in &sample.children {
                    match child {
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
                        
                        TrainXMLSamplesSampleChildren::ResponseIds(response_id) => {
                            // Check response ID
                            if !ids.responses.contains_key(&response_id.response) {
                                errors.push(format!(
                                    "Sample response-ids references unknown response ID '{}'",
                                    response_id.response
                                ));
                            }
                            
                            // Check source ID if present
                            if let Some(source_id) = &response_id.source {
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
    use std::collections::HashMap;
    use crate::sample::SampleIndent;
    use crate::train_xml::{
        TrainXML,
        TrainXMLIdMaps,
        train_xml_validate_ids::train_xml_validate_ids,
        train_xml_structs::{
            TrainXMLSamples,
            TrainXMLPrompts,
            TrainXMLSources,
            TrainXMLResponses,
            TrainXMLCodeSnippets,
            TrainXMLPromptsPrompt,
            TrainXMLSourcesSource,
            TrainXMLSamplesSample,
            TrainXMLSamplesPrompt,
            TrainXMLCodeSnippetsCode,
            TrainXMLResponsesResponse,
            TrainXMLSamplesSampleIds,
            TrainXMLSamplesSampleChildren,
            TrainXMLSamplesResponse,
            TrainXMLSamplesSource,
            TrainXMLSamplesCode,
            TrainXMLSamplesResponseIds,
            TrainXMLLineBreak,
        }
    };

    fn create_test_prompt(id: &str, content: &str) -> TrainXMLPromptsPrompt {
        TrainXMLPromptsPrompt {
            id: id.to_string(),
            content: content.to_string(),
        }
    }

    fn create_test_response(id: &str, content: &str) -> TrainXMLResponsesResponse {
        TrainXMLResponsesResponse {
            id: id.to_string(),
            content: content.to_string(),
        }
    }

    fn create_test_source(id: &str, url: &str) -> TrainXMLSourcesSource {
        TrainXMLSourcesSource {
            id: id.to_string(),
            url: url.to_string(),
            title: None,
        }
    }

    fn create_test_code(id: &str, lang: &str, content: &str) -> TrainXMLCodeSnippetsCode {
        TrainXMLCodeSnippetsCode {
            id: id.to_string(),
            lang: lang.to_string(),
            content: content.to_string(),
        }
    }

    #[test]
    fn test_validate_ids_success() {
        // Create prompt, response, source, code data
        let prompt1 = create_test_prompt("p1", "Test prompt");
        let prompt2 = create_test_prompt("p2", "Another prompt");
        let response1 = create_test_response("r1", "Test response");
        let source1 = create_test_source("s1", "https://example.com");
        let code1 = create_test_code("c1", "rust", "fn main() {}");

        // Create train XML with children
        let train_xml = TrainXML {
            prompts: Some(TrainXMLPrompts {
                prompt: vec![prompt1.clone(), prompt2.clone()],
            }),
            responses: Some(TrainXMLResponses {
                response: vec![response1.clone()],
            }),
            sources: Some(TrainXMLSources {
                source: vec![source1.clone()],
            }),
            code_snippets: Some(TrainXMLCodeSnippets {
                code: vec![code1.clone()],
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
                        children: vec![
                            TrainXMLSamplesSampleChildren::Response(TrainXMLSamplesResponse { id: "r1".to_string() }),
                            TrainXMLSamplesSampleChildren::Source(TrainXMLSamplesSource { id: "s1".to_string() }),
                            TrainXMLSamplesSampleChildren::Code(TrainXMLSamplesCode { 
                                id: "c1".to_string(), 
                                indent: Some(SampleIndent::One), 
                                inline: Some(false) 
                            }),
                            TrainXMLSamplesSampleChildren::ResponseIds(TrainXMLSamplesResponseIds {
                                response: "r1".to_string(),
                                source: Some("s1".to_string()),
                            }),
                            TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 1 }),
                        ],
                    },
                ]),
            }),
            constants: None,
            phrases: None,
        };

        // Create IDs with references to the actual data
        let mut prompts = HashMap::new();
        prompts.insert("p1".to_string(), &prompt1);
        prompts.insert("p2".to_string(), &prompt2);
        
        let mut responses = HashMap::new();
        responses.insert("r1".to_string(), &response1);
        
        let mut sources = HashMap::new();
        sources.insert("s1".to_string(), &source1);
        
        let mut code_snippets = HashMap::new();
        code_snippets.insert("c1".to_string(), &code1);

        let ids = TrainXMLIdMaps {
            prompts,
            responses,
            sources,
            code_snippets,
        };

        let result = train_xml_validate_ids(&train_xml, &ids);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_ids_errors() {
        // Create valid data
        let prompt1 = create_test_prompt("p1", "Test prompt");
        let response1 = create_test_response("r1", "Test response");
        let source1 = create_test_source("s1", "https://example.com");
        let code1 = create_test_code("c1", "rust", "fn main() {}");

        // Create train XML with multiple ID errors using children
        let train_xml = TrainXML {
            prompts: Some(TrainXMLPrompts {
                prompt: vec![prompt1.clone()],
            }),
            responses: Some(TrainXMLResponses {
                response: vec![response1.clone()],
            }),
            sources: Some(TrainXMLSources {
                source: vec![source1.clone()],
            }),
            code_snippets: Some(TrainXMLCodeSnippets {
                code: vec![code1.clone()],
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
                        children: vec![
                            TrainXMLSamplesSampleChildren::Response(TrainXMLSamplesResponse { id: "r99".to_string() }),  // Invalid response ID
                            TrainXMLSamplesSampleChildren::Source(TrainXMLSamplesSource { id: "s99".to_string() }),  // Invalid source ID
                            TrainXMLSamplesSampleChildren::Code(TrainXMLSamplesCode { 
                                id: "c99".to_string(), 
                                indent: None, 
                                inline: None 
                            }),  // Invalid code ID
                        ],
                    },
                    TrainXMLSamplesSample {
                        prompt: TrainXMLSamplesPrompt { id: "p1".to_string() },
                        children: vec![
                            TrainXMLSamplesSampleChildren::ResponseIds(TrainXMLSamplesResponseIds { 
                                response: "r99".to_string(),  // Invalid response ID
                                source: Some("s1".to_string()) 
                            }),
                            TrainXMLSamplesSampleChildren::ResponseIds(TrainXMLSamplesResponseIds { 
                                response: "r1".to_string(), 
                                source: Some("s99".to_string())  // Invalid source ID
                            }),
                            TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 1 }), // Valid, no IDs
                        ],
                    },
                ]),
            }),
            constants: None,
            phrases: None,
        };

        // Create IDs with only valid references
        let mut prompts = HashMap::new();
        prompts.insert("p1".to_string(), &prompt1);
        
        let mut responses = HashMap::new();
        responses.insert("r1".to_string(), &response1);
        
        let mut sources = HashMap::new();
        sources.insert("s1".to_string(), &source1);
        
        let mut code_snippets = HashMap::new();
        code_snippets.insert("c1".to_string(), &code1);

        let ids = TrainXMLIdMaps {
            prompts,
            responses,
            sources,
            code_snippets,
        };

        let result = train_xml_validate_ids(&train_xml, &ids);
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        
        // Print errors for debugging
        println!("Errors: {:#?}", errors);
        
        // We expect at least 9 errors:
        // - sample-ids: 4 errors (r99, s99, c99, p99)
        // - first sample with children: 4 errors (p99, r99, s99, c99)
        // - second sample with response-ids: 2 errors (r99, s99)
        // Total: 10 errors
        assert!(errors.len() >= 9, "Expected at least 9 errors, got {}", errors.len());
        
        // Verify some specific errors are present
        let error_string = errors.join("\n");
        assert!(error_string.contains("prompt ID 'p99'"));
        assert!(error_string.contains("response ID 'r99'"));
        assert!(error_string.contains("source ID 's99'"));
        assert!(error_string.contains("code ID 'c99'"));
    }
}
