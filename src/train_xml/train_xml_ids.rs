// src/train_xml/train_xml_ids.rs

use crate::train_xml::TrainXML;
use std::collections::HashSet;


/// Validates and collects all IDs from a TrainXML document
#[derive(Debug)]
pub struct TrainXMLIds {
    /// Set of all prompt IDs
    pub prompt_ids: HashSet<String>,
    
    /// Set of all response IDs
    pub response_ids: HashSet<String>,
    
    /// Set of all source IDs
    pub source_ids: HashSet<String>,
    
    /// Set of all code snippet IDs
    pub code_ids: HashSet<String>,
}


impl TrainXMLIds {
    /// Create a new TrainXMLIds by validating and collecting all IDs from TrainXML
    ///
    /// # Arguments
    /// * `train_xml` - Reference to a parsed TrainXML document
    ///
    /// # Returns
    /// * `Result<Self, String>` - TrainXMLIds if all IDs are unique, error message otherwise
    ///
    /// # Errors
    /// Returns an error if any duplicate IDs are found within the same category
    /// (prompts, responses, sources, or code-snippets)
    pub fn create(train_xml: &TrainXML) -> Result<Self, String> {
        let mut prompt_ids = HashSet::new();
        let mut response_ids = HashSet::new();
        let mut source_ids = HashSet::new();
        let mut code_ids = HashSet::new();
        
        // Validate prompts
        if let Some(prompts) = &train_xml.prompts {
            for prompt in &prompts.prompt {
                if !prompt_ids.insert(prompt.id.clone()) {
                    return Err(format!("Duplicate prompt ID: '{}'", prompt.id));
                }
            }
        }
        
        // Validate responses
        if let Some(responses) = &train_xml.responses {
            for response in &responses.response {
                if !response_ids.insert(response.id.clone()) {
                    return Err(format!("Duplicate response ID: '{}'", response.id));
                }
            }
        }
        
        // Validate sources
        if let Some(sources) = &train_xml.sources {
            for source in &sources.source {
                if !source_ids.insert(source.id.clone()) {
                    return Err(format!("Duplicate source ID: '{}'", source.id));
                }
            }
        }
        
        // Validate code snippets
        if let Some(code_snippets) = &train_xml.code_snippets {
            for code in &code_snippets.code {
                if !code_ids.insert(code.id.clone()) {
                    return Err(format!("Duplicate code snippet ID: '{}'", code.id));
                }
            }
        }
        
        Ok(Self {
            prompt_ids,
            response_ids,
            source_ids,
            code_ids,
        })
    }
}



#[cfg(test)]
mod tests {
    use crate::train_xml::{
        TrainXML,
        TrainXMLIds,
        train_xml_prompts::{TrainXMLPrompts, TrainXMLPromptsPrompt},
        train_xml_sources::{TrainXMLSources, TrainXMLSourcesSource},
        train_xml_responses::{TrainXMLResponses, TrainXMLResponsesResponse},
        train_xml_code_snippets::{TrainXMLCodeSnippets, TrainXMLCodeSnippetsCode},
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

        let ids = TrainXMLIds::create(&train_xml).unwrap();
        
        assert_eq!(ids.prompt_ids.len(), 2);
        assert!(ids.prompt_ids.contains("prompt1"));
        assert!(ids.prompt_ids.contains("prompt2"));
        
        assert_eq!(ids.response_ids.len(), 1);
        assert!(ids.response_ids.contains("response1"));
        
        assert_eq!(ids.source_ids.len(), 1);
        assert!(ids.source_ids.contains("source1"));
        
        assert_eq!(ids.code_ids.len(), 1);
        assert!(ids.code_ids.contains("code1"));
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

        let ids = TrainXMLIds::create(&train_xml).unwrap();
        
        assert_eq!(ids.prompt_ids.len(), 1);
        assert!(ids.prompt_ids.contains("id123"));
        
        assert_eq!(ids.response_ids.len(), 1);
        assert!(ids.response_ids.contains("id123"));
        
        assert_eq!(ids.source_ids.len(), 1);
        assert!(ids.source_ids.contains("id123"));
        
        assert_eq!(ids.code_ids.len(), 1);
        assert!(ids.code_ids.contains("id123"));
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

        let ids = TrainXMLIds::create(&train_xml).unwrap();
        
        assert_eq!(ids.prompt_ids.len(), 0);
        assert_eq!(ids.response_ids.len(), 0);
        assert_eq!(ids.source_ids.len(), 0);
        assert_eq!(ids.code_ids.len(), 0);
    }
}
