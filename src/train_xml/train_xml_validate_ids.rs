// src/train_xml/train_xml_validate_ids.rs

use crate::train_xml::{
    TrainXML,
    TrainXMLIdMaps,
    TrainXMLSamplesSampleChildren,
};


/// Validates that every ID referenced in the train XML exists in TrainXMLIdMaps
pub fn train_xml_validate_ids(train_xml: &TrainXML, ids: &TrainXMLIdMaps) -> Result<(), Vec<String>> {
    let mut errors: Vec<String> = Vec::new();
    
    // Validate beyond-scope references if present
    if let Some(beyond_scope) = &train_xml.beyond_scope {
        if !ids.system_prompts.contains_key(&beyond_scope.system) {
            errors.push(format!(
                "Beyond-scope references unknown system prompt ID '{}'",
                beyond_scope.system
            ));
        }
        if !ids.responses.contains_key(&beyond_scope.response) {
            errors.push(format!(
                "Beyond-scope references unknown response ID '{}'",
                beyond_scope.response
            ));
        }
    }
    
    // Check prompt IDs in samples
    if let Some(samples) = &train_xml.samples {
        // Check sample-ids elements
        if let Some(sample_ids_list) = &samples.sample_ids {
            for sample_id in sample_ids_list {
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
        if let Some(samples_list) = &samples.sample {
            for sample in samples_list {
                // Track if we found a prompt
                let mut found_prompt = false;
                
                // Check all children for ID references
                for child in &sample.children {
                    match child {
                        TrainXMLSamplesSampleChildren::Prompt(prompt) => {
                            found_prompt = true;
                            // Check prompt ID
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
                            // Check response ID
                            if !ids.responses.contains_key(&response_ids.response) {
                                errors.push(format!(
                                    "Sample response-ids references unknown response ID '{}'",
                                    response_ids.response
                                ));
                            }
                            
                            // Check source ID if present
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
                
                // Validate that a prompt was found
                if !found_prompt {
                    errors.push("Sample is missing a required <prompt> element".to_string());
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
            TrainXMLSystemPrompts,
            TrainXMLPromptsPrompt,
            TrainXMLSourcesSource,
            TrainXMLSamplesSample,
            TrainXMLSamplesPrompt,
            TrainXMLSamplesSystem,
            TrainXMLCodeSnippetsCode,
            TrainXMLResponsesResponse,
            TrainXMLSamplesSampleIds,
            TrainXMLSamplesSampleChildren,
            TrainXMLSamplesResponse,
            TrainXMLSamplesSource,
            TrainXMLSamplesCode,
            TrainXMLSamplesResponseIds,
            TrainXMLSystemPromptsSystem,
            TrainXMLLineBreak,
            TrainXMLBeyondScope,
            TrainXMLBeyondScopeTopic,
        }
    };

    fn create_test_system_prompt(id: &str, content: &str) -> TrainXMLSystemPromptsSystem {
        TrainXMLSystemPromptsSystem {
            id: id.to_string(),
            content: content.to_string(),
        }
    }

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
        // Create system prompt, prompt, response, source, code data
        let system1 = create_test_system_prompt("sy1", "You are a helpful assistant");
        let response_beyond = create_test_response("r_beyond", "I'm sorry, I don't know");
        let prompt1 = create_test_prompt("p1", "Test prompt");
        let prompt2 = create_test_prompt("p2", "Another prompt");
        let response1 = create_test_response("r1", "Test response");
        let source1 = create_test_source("s1", "https://example.com");
        let code1 = create_test_code("c1", "rust", "fn main() {}");

        // Create train XML with children and beyond-scope
        let train_xml = TrainXML {
            system_prompts: Some(TrainXMLSystemPrompts {
                system: vec![system1.clone()],
            }),
            prompts: Some(TrainXMLPrompts {
                prompt: vec![prompt1.clone(), prompt2.clone()],
            }),
            responses: Some(TrainXMLResponses {
                response: vec![response1.clone(), response_beyond.clone()],
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
                        system: Some("sy1".to_string()),
                        prompt: "p1".to_string(), 
                        response: Some("r1".to_string()), 
                        source: Some("s1".to_string()), 
                        code: Some("c1".to_string()) 
                    },
                ]),
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "p2".to_string() }),
                            TrainXMLSamplesSampleChildren::System(TrainXMLSamplesSystem { id: "sy1".to_string() }),
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
            beyond_scope: Some(TrainXMLBeyondScope {
                system: "sy1".to_string(),
                response: "r_beyond".to_string(),
                sports: Some(true),
                food: None,
                movies: None,
                history: None,
                geography: None,
                politics: None,
                science: None,
                health: None,
                art: None,
                music: None,
                fashion: None,
                travel: None,
                pets: None,
                cars: None,
                topics: vec![
                    TrainXMLBeyondScopeTopic {
                        value: "soccer".to_string(),
                        prefix: "is".to_string(),
                    },
                ],
            }),
        };

        // Create IDs with references to the actual data
        let mut system_prompts = HashMap::new();
        system_prompts.insert("sy1".to_string(), &system1);
        
        let mut prompts = HashMap::new();
        prompts.insert("p1".to_string(), &prompt1);
        prompts.insert("p2".to_string(), &prompt2);
        
        let mut responses = HashMap::new();
        responses.insert("r1".to_string(), &response1);
        responses.insert("r_beyond".to_string(), &response_beyond);
        
        let mut sources = HashMap::new();
        sources.insert("s1".to_string(), &source1);
        
        let mut code_snippets = HashMap::new();
        code_snippets.insert("c1".to_string(), &code1);

        let ids = TrainXMLIdMaps {
            system_prompts,
            prompts,
            responses,
            sources,
            code_snippets,
        };

        let result = train_xml_validate_ids(&train_xml, &ids);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_ids_beyond_scope_missing_system() {
        // Create valid data
        let system1 = create_test_system_prompt("sy1", "Valid system");
        let response_beyond = create_test_response("r_beyond", "I'm sorry, I don't know");
        let prompt1 = create_test_prompt("p1", "Test prompt");
        let response1 = create_test_response("r1", "Test response");

        let train_xml = TrainXML {
            system_prompts: Some(TrainXMLSystemPrompts {
                system: vec![system1.clone()],
            }),
            prompts: Some(TrainXMLPrompts {
                prompt: vec![prompt1.clone()],
            }),
            responses: Some(TrainXMLResponses {
                response: vec![response1.clone(), response_beyond.clone()],
            }),
            sources: None,
            code_snippets: None,
            samples: None,
            constants: None,
            phrases: None,
            beyond_scope: Some(TrainXMLBeyondScope {
                system: "missing_system".to_string(),  // Invalid system ID
                response: "r_beyond".to_string(),
                sports: Some(true),
                food: None,
                movies: None,
                history: None,
                geography: None,
                politics: None,
                science: None,
                health: None,
                art: None,
                music: None,
                fashion: None,
                travel: None,
                pets: None,
                cars: None,
                topics: vec![],
            }),
        };

        let mut system_prompts = HashMap::new();
        system_prompts.insert("sy1".to_string(), &system1);
        
        let mut prompts = HashMap::new();
        prompts.insert("p1".to_string(), &prompt1);
        
        let mut responses = HashMap::new();
        responses.insert("r1".to_string(), &response1);
        responses.insert("r_beyond".to_string(), &response_beyond);

        let ids = TrainXMLIdMaps {
            system_prompts,
            prompts,
            responses,
            sources: HashMap::new(),
            code_snippets: HashMap::new(),
        };

        let result = train_xml_validate_ids(&train_xml, &ids);
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        let error_string = errors.join("\n");
        assert!(error_string.contains("Beyond-scope references unknown system prompt ID 'missing_system'"));
    }

    #[test]
    fn test_validate_ids_beyond_scope_missing_response() {
        // Create valid data
        let system1 = create_test_system_prompt("sy1", "Valid system");
        let prompt1 = create_test_prompt("p1", "Test prompt");
        let response1 = create_test_response("r1", "Test response");

        let train_xml = TrainXML {
            system_prompts: Some(TrainXMLSystemPrompts {
                system: vec![system1.clone()],
            }),
            prompts: Some(TrainXMLPrompts {
                prompt: vec![prompt1.clone()],
            }),
            responses: Some(TrainXMLResponses {
                response: vec![response1.clone()],
            }),
            sources: None,
            code_snippets: None,
            samples: None,
            constants: None,
            phrases: None,
            beyond_scope: Some(TrainXMLBeyondScope {
                system: "sy1".to_string(),
                response: "missing_response".to_string(),  // Invalid response ID
                sports: Some(true),
                food: None,
                movies: None,
                history: None,
                geography: None,
                politics: None,
                science: None,
                health: None,
                art: None,
                music: None,
                fashion: None,
                travel: None,
                pets: None,
                cars: None,
                topics: vec![],
            }),
        };

        let mut system_prompts = HashMap::new();
        system_prompts.insert("sy1".to_string(), &system1);
        
        let mut prompts = HashMap::new();
        prompts.insert("p1".to_string(), &prompt1);
        
        let mut responses = HashMap::new();
        responses.insert("r1".to_string(), &response1);

        let ids = TrainXMLIdMaps {
            system_prompts,
            prompts,
            responses,
            sources: HashMap::new(),
            code_snippets: HashMap::new(),
        };

        let result = train_xml_validate_ids(&train_xml, &ids);
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        let error_string = errors.join("\n");
        assert!(error_string.contains("Beyond-scope references unknown response ID 'missing_response'"));
    }

    #[test]
    fn test_validate_ids_errors() {
        // Create valid data
        let system1 = create_test_system_prompt("sy1", "Valid system");
        let prompt1 = create_test_prompt("p1", "Test prompt");
        let response1 = create_test_response("r1", "Test response");
        let source1 = create_test_source("s1", "https://example.com");
        let code1 = create_test_code("c1", "rust", "fn main() {}");

        // Create train XML with multiple ID errors
        let train_xml = TrainXML {
            system_prompts: Some(TrainXMLSystemPrompts {
                system: vec![system1.clone()],
            }),
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
                        system: Some("sys99".to_string()),  // Invalid system ID
                        prompt: "p1".to_string(), 
                        response: Some("r99".to_string()),  // Invalid response ID
                        source: Some("s99".to_string()),    // Invalid source ID
                        code: Some("c99".to_string()),      // Invalid code ID
                    },
                    TrainXMLSamplesSampleIds { 
                        system: Some("sy1".to_string()),
                        prompt: "p99".to_string(),          // Invalid prompt ID
                        response: Some("r1".to_string()), 
                        source: Some("s1".to_string()), 
                        code: Some("c1".to_string()) 
                    },
                ]),
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "p99".to_string() }),  // Invalid prompt ID
                            TrainXMLSamplesSampleChildren::System(TrainXMLSamplesSystem { id: "sys99".to_string() }),  // Invalid system ID
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
                        children: vec![
                            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt { id: "p1".to_string() }),
                            TrainXMLSamplesSampleChildren::ResponseIds(TrainXMLSamplesResponseIds { 
                                response: "r99".to_string(),  // Invalid response ID
                                source: Some("s1".to_string()) 
                            }),
                            TrainXMLSamplesSampleChildren::ResponseIds(TrainXMLSamplesResponseIds { 
                                response: "r1".to_string(), 
                                source: Some("s99".to_string())  // Invalid source ID
                            }),
                            TrainXMLSamplesSampleChildren::System(TrainXMLSamplesSystem { id: "sys99".to_string() }),  // Invalid system ID
                            TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 1 }), // Valid, no IDs
                        ],
                    },
                ]),
            }),
            constants: None,
            phrases: None,
            beyond_scope: Some(TrainXMLBeyondScope {
                system: "sys99".to_string(),  // Invalid system ID
                response: "r99".to_string(),  // Invalid response ID
                sports: Some(true),
                food: None,
                movies: None,
                history: None,
                geography: None,
                politics: None,
                science: None,
                health: None,
                art: None,
                music: None,
                fashion: None,
                travel: None,
                pets: None,
                cars: None,
                topics: vec![],
            }),
        };

        // Create IDs with only valid references
        let mut system_prompts = HashMap::new();
        system_prompts.insert("sy1".to_string(), &system1);
        
        let mut prompts = HashMap::new();
        prompts.insert("p1".to_string(), &prompt1);
        
        let mut responses = HashMap::new();
        responses.insert("r1".to_string(), &response1);
        
        let mut sources = HashMap::new();
        sources.insert("s1".to_string(), &source1);
        
        let mut code_snippets = HashMap::new();
        code_snippets.insert("c1".to_string(), &code1);

        let ids = TrainXMLIdMaps {
            system_prompts,
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
        
        // Verify specific errors are present
        let error_string = errors.join("\n");
        assert!(error_string.contains("system prompt ID 'sys99'"));
        assert!(error_string.contains("prompt ID 'p99'"));
        assert!(error_string.contains("response ID 'r99'"));
        assert!(error_string.contains("source ID 's99'"));
        assert!(error_string.contains("code ID 'c99'"));
        assert!(error_string.contains("Beyond-scope references unknown system prompt ID 'sys99'"));
        assert!(error_string.contains("Beyond-scope references unknown response ID 'r99'"));
    }

    #[test]
    fn test_validate_ids_without_system_prompts() {
        // Create prompt, response, source, code data (no system prompts)
        let prompt1 = create_test_prompt("p1", "Test prompt");
        let response1 = create_test_response("r1", "Test response");

        let train_xml = TrainXML {
            system_prompts: None,
            prompts: Some(TrainXMLPrompts {
                prompt: vec![prompt1.clone()],
            }),
            responses: Some(TrainXMLResponses {
                response: vec![response1.clone()],
            }),
            sources: None,
            code_snippets: None,
            samples: Some(TrainXMLSamples {
                sample_ids: Some(vec![
                    TrainXMLSamplesSampleIds { 
                        system: None,
                        prompt: "p1".to_string(), 
                        response: Some("r1".to_string()), 
                        source: None, 
                        code: None 
                    },
                ]),
                sample: None,
            }),
            constants: None,
            phrases: None,
            beyond_scope: None,  // No beyond-scope reference
        };

        let mut prompts = HashMap::new();
        prompts.insert("p1".to_string(), &prompt1);
        
        let mut responses = HashMap::new();
        responses.insert("r1".to_string(), &response1);

        let ids = TrainXMLIdMaps {
            system_prompts: HashMap::new(),
            prompts,
            responses,
            sources: HashMap::new(),
            code_snippets: HashMap::new(),
        };

        let result = train_xml_validate_ids(&train_xml, &ids);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_ids_missing_prompt() {
        // Create valid data
        let prompt1 = create_test_prompt("p1", "Test prompt");
        let response1 = create_test_response("r1", "Test response");

        let train_xml = TrainXML {
            system_prompts: None,
            prompts: Some(TrainXMLPrompts {
                prompt: vec![prompt1.clone()],
            }),
            responses: Some(TrainXMLResponses {
                response: vec![response1.clone()],
            }),
            sources: None,
            code_snippets: None,
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            // No Prompt element!
                            TrainXMLSamplesSampleChildren::Response(TrainXMLSamplesResponse { id: "r1".to_string() }),
                        ],
                    },
                ]),
            }),
            constants: None,
            phrases: None,
            beyond_scope: None,
        };

        let mut prompts = HashMap::new();
        prompts.insert("p1".to_string(), &prompt1);
        
        let mut responses = HashMap::new();
        responses.insert("r1".to_string(), &response1);

        let ids = TrainXMLIdMaps {
            system_prompts: HashMap::new(),
            prompts,
            responses,
            sources: HashMap::new(),
            code_snippets: HashMap::new(),
        };

        let result = train_xml_validate_ids(&train_xml, &ids);
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        assert!(errors.contains(&"Sample is missing a required <prompt> element".to_string()));
    }
}
