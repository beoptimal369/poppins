// src/train_xml/train_xml_id_maps.rs

use std::collections::HashMap;
use crate::train_xml::{
    TrainXML,
    TrainXMLSourcesSource,
    TrainXMLPromptsPrompt,
    TrainXMLCodeSnippetsCode,
    TrainXMLResponsesResponse,
};


/// Validates and collects all IDs from a TrainXML document with references to original data
#[derive(Debug)]
pub struct TrainXMLIdMaps<'a> {
    /// Map of prompt IDs to their corresponding prompt data
    pub prompts: HashMap<String, &'a TrainXMLPromptsPrompt>,
    
    /// Map of response IDs to their corresponding response data
    pub responses: HashMap<String, &'a TrainXMLResponsesResponse>,
    
    /// Map of source IDs to their corresponding source data
    pub sources: HashMap<String, &'a TrainXMLSourcesSource>,
    
    /// Map of code snippet IDs to their corresponding code data
    pub code_snippets: HashMap<String, &'a TrainXMLCodeSnippetsCode>,
}


impl<'a> TrainXMLIdMaps<'a> {
    /// Create a new TrainXMLIdMaps by validating and collecting all IDs from TrainXML
    ///
    /// # Arguments
    /// * `train_xml` - Reference to a parsed TrainXML document
    ///
    /// # Returns
    /// * `Result<Self, String>` - TrainXMLIdMaps if all IDs are unique, error message otherwise
    ///
    /// # Errors
    /// Returns an error if any duplicate IDs are found within the same category
    /// (prompts, responses, sources, or code-snippets)
    pub fn create(train_xml: &'a TrainXML) -> Result<Self, String> {
        let mut prompts = HashMap::new();
        let mut responses = HashMap::new();
        let mut sources = HashMap::new();
        let mut code_snippets = HashMap::new();
        
        // Validate prompts
        if let Some(prompts_section) = &train_xml.prompts {
            for prompt in &prompts_section.prompt {
                if prompts.insert(prompt.id.clone(), prompt).is_some() {
                    return Err(format!("Duplicate prompt ID: '{}'", prompt.id));
                }
            }
        }
        
        // Validate responses
        if let Some(responses_section) = &train_xml.responses {
            for response in &responses_section.response {
                if responses.insert(response.id.clone(), response).is_some() {
                    return Err(format!("Duplicate response ID: '{}'", response.id));
                }
            }
        }
        
        // Validate sources
        if let Some(sources_section) = &train_xml.sources {
            for source in &sources_section.source {
                if sources.insert(source.id.clone(), source).is_some() {
                    return Err(format!("Duplicate source ID: '{}'", source.id));
                }
            }
        }
        
        // Validate code snippets
        if let Some(code_snippets_section) = &train_xml.code_snippets {
            for code in &code_snippets_section.code {
                if code_snippets.insert(code.id.clone(), code).is_some() {
                    return Err(format!("Duplicate code snippet ID: '{}'", code.id));
                }
            }
        }
        
        Ok(Self {
            prompts,
            responses,
            sources,
            code_snippets,
        })
    }
}



#[cfg(test)]
mod tests {
    use crate::train_xml::{
        TrainXML,
        TrainXMLIdMaps,
        TrainXMLSourcesSource,
        TrainXMLPromptsPrompt,
        TrainXMLCodeSnippetsCode,
        TrainXMLResponsesResponse,
        train_xml_structs::{
            TrainXMLPrompts,
            TrainXMLSources,
            TrainXMLResponses,
            TrainXMLCodeSnippets,
        },
    };

    #[test]
    fn test_create_with_valid_ids() {
        let train_xml = TrainXML {
            prompts: Some(TrainXMLPrompts {
                prompt: vec![
                    TrainXMLPromptsPrompt {
                        id: "prompt1".to_string(),
                        content: "Content 1".to_string(),
                    },
                    TrainXMLPromptsPrompt {
                        id: "prompt2".to_string(),
                        content: "Content 2".to_string(),
                    },
                ],
            }),
            responses: Some(TrainXMLResponses {
                response: vec![
                    TrainXMLResponsesResponse {
                        id: "response1".to_string(),
                        content: "Response 1".to_string(),
                    },
                ],
            }),
            sources: Some(TrainXMLSources {
                source: vec![
                    TrainXMLSourcesSource {
                        title: None,
                        id: "source1".to_string(),
                        url: "https://example.com".to_string(),
                    },
                ],
            }),
            code_snippets: Some(TrainXMLCodeSnippets {
                code: vec![
                    TrainXMLCodeSnippetsCode {
                        id: "code1".to_string(),
                        lang: "rust".to_string(),
                        content: "fn main() {}".to_string(),
                    },
                ],
            }),
            samples: None,
            constants: None,
            phrases: None,
        };

        let ids = TrainXMLIdMaps::create(&train_xml).unwrap();
        
        assert_eq!(ids.prompts.len(), 2);
        assert!(ids.prompts.contains_key("prompt1"));
        assert!(ids.prompts.contains_key("prompt2"));
        assert_eq!(ids.prompts.get("prompt1").unwrap().content, "Content 1");
        assert_eq!(ids.prompts.get("prompt2").unwrap().content, "Content 2");
        
        assert_eq!(ids.responses.len(), 1);
        assert!(ids.responses.contains_key("response1"));
        assert_eq!(ids.responses.get("response1").unwrap().content, "Response 1");
        
        assert_eq!(ids.sources.len(), 1);
        assert!(ids.sources.contains_key("source1"));
        assert_eq!(ids.sources.get("source1").unwrap().url, "https://example.com");
        
        assert_eq!(ids.code_snippets.len(), 1);
        assert!(ids.code_snippets.contains_key("code1"));
        assert_eq!(ids.code_snippets.get("code1").unwrap().lang, "rust");
    }

    #[test]
    fn test_create_with_mixed_duplicates() {
        let train_xml = TrainXML {
            prompts: Some(TrainXMLPrompts {
                prompt: vec![
                    TrainXMLPromptsPrompt {
                        id: "id123".to_string(),
                        content: "Prompt".to_string(),
                    },
                ],
            }),
            responses: Some(TrainXMLResponses {
                response: vec![
                    TrainXMLResponsesResponse {
                        id: "id123".to_string(),
                        content: "Response".to_string(),
                    },
                ],
            }),
            sources: Some(TrainXMLSources {
                source: vec![
                    TrainXMLSourcesSource {
                        title: None,
                        id: "id123".to_string(),
                        url: "https://example.com".to_string(),
                    },
                ],
            }),
            code_snippets: Some(TrainXMLCodeSnippets {
                code: vec![
                    TrainXMLCodeSnippetsCode {
                        id: "id123".to_string(),
                        lang: "rust".to_string(),
                        content: "fn main() {}".to_string(),
                    },
                ],
            }),
            samples: None,
            constants: None,
            phrases: None,
        };

        let ids = TrainXMLIdMaps::create(&train_xml).unwrap();
        
        assert_eq!(ids.prompts.len(), 1);
        assert!(ids.prompts.contains_key("id123"));
        
        assert_eq!(ids.responses.len(), 1);
        assert!(ids.responses.contains_key("id123"));
        
        assert_eq!(ids.sources.len(), 1);
        assert!(ids.sources.contains_key("id123"));
        
        assert_eq!(ids.code_snippets.len(), 1);
        assert!(ids.code_snippets.contains_key("id123"));
        
        // Verify we can access the actual data
        assert_eq!(ids.prompts.get("id123").unwrap().content, "Prompt");
        assert_eq!(ids.responses.get("id123").unwrap().content, "Response");
        assert_eq!(ids.sources.get("id123").unwrap().url, "https://example.com");
        assert_eq!(ids.code_snippets.get("id123").unwrap().lang, "rust");
    }

    #[test]
    fn test_create_with_optional_sections_none() {
        let train_xml = TrainXML {
            prompts: None,
            responses: None,
            sources: None,
            code_snippets: None,
            samples: None,
            constants: None,
            phrases: None,
        };

        let ids = TrainXMLIdMaps::create(&train_xml).unwrap();
        
        assert_eq!(ids.prompts.len(), 0);
        assert_eq!(ids.responses.len(), 0);
        assert_eq!(ids.sources.len(), 0);
        assert_eq!(ids.code_snippets.len(), 0);
    }
}
