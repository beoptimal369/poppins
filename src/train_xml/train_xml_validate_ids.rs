// src/train_xml/train_xml_validate_ids.rs

use crate::train_xml::{
    TrainXML,
    TrainXMLIdMaps,
    train_xml_validate_ids_beyond_scope,
    train_xml_validate_ids_sample_ids,
    train_xml_validate_ids_sample,
    train_xml_validate_ids_imports,
};


/// Validates that every ID referenced in the train XML exists in TrainXMLIdMaps
pub fn train_xml_validate_ids(train_xml: &TrainXML, ids: &TrainXMLIdMaps) -> Result<(), Vec<String>> {
    let mut errors: Vec<String> = Vec::new();
    
    train_xml_validate_ids_beyond_scope(train_xml, ids, &mut errors);
    train_xml_validate_ids_sample_ids(train_xml, ids, &mut errors);
    train_xml_validate_ids_sample(train_xml, ids, &mut errors);
    train_xml_validate_ids_imports(train_xml, ids, &mut errors);
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::train_xml_validate_ids;
    use crate::train_xml::{
        TrainXML,
        TrainXMLIdMaps,
        TrainXMLBeyondScope,
        TrainXMLSamples,
        TrainXMLSamplesSampleIds,
        TrainXMLImports,
        TrainXMLImportsImport,
    };

    // Helper to create empty ID maps
    fn create_empty_ids() -> TrainXMLIdMaps<'static> {
        TrainXMLIdMaps {
            system_prompts: HashMap::new(),
            prompts: HashMap::new(),
            thoughts: HashMap::new(),
            responses: HashMap::new(),
            sources: HashMap::new(),
            code_snippets: HashMap::new(),
        }
    }

    #[test]
    fn test_validate_ids_success_with_no_errors() {
        let train_xml = TrainXML {
            beyond_scope: None,
            samples: None,
            imports: None,
            ..Default::default()
        };
        
        let ids = create_empty_ids();
        let result = train_xml_validate_ids(&train_xml, &ids);
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_ids_collects_errors_from_beyond_scope() {
        let train_xml = TrainXML {
            beyond_scope: Some(TrainXMLBeyondScope {
                system: "nonexistent".to_string(),
                thought: None,
                response: "response1".to_string(),
                topics: vec![],
                sports: None,
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
            }),
            samples: None,
            imports: None,
            ..Default::default()
        };
        
        let ids = create_empty_ids();
        let result = train_xml_validate_ids(&train_xml, &ids);
        
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 2); // system + response
        assert!(errors[0].contains("unknown system prompt ID 'nonexistent'"));
        assert!(errors[1].contains("unknown response ID 'response1'"));
    }

    #[test]
    fn test_validate_ids_collects_errors_from_sample_ids() {
        let train_xml = TrainXML {
            beyond_scope: None,
            samples: Some(TrainXMLSamples {
                sample_ids: Some(vec![
                    TrainXMLSamplesSampleIds {
                        system: None,
                        prompt: "bad_prompt".to_string(),
                        thought: None,
                        response: None,
                        source: None,
                        code: None,
                    },
                ]),
                sample: None,
            }),
            imports: None,
            ..Default::default()
        };
        
        let ids = create_empty_ids();
        let result = train_xml_validate_ids(&train_xml, &ids);
        
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("unknown prompt ID 'bad_prompt'"));
    }

    #[test]
    fn test_validate_ids_collects_errors_from_sample() {
        // Note: Sample validation requires more complex setup,
        // but we can verify the function is called by checking
        // that errors from sample validation are collected
        let train_xml = TrainXML {
            beyond_scope: None,
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    crate::train_xml::TrainXMLSamplesSample {
                        children: vec![
                            crate::train_xml::TrainXMLSamplesSampleChildren::Prompt(
                                crate::train_xml::TrainXMLSamplesPrompt { id: "bad_prompt".to_string() }
                            ),
                        ],
                    },
                ]),
            }),
            imports: None,
            ..Default::default()
        };
        
        let ids = create_empty_ids();
        let result = train_xml_validate_ids(&train_xml, &ids);
        
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("unknown prompt ID 'bad_prompt'"));
    }

    #[test]
    fn test_validate_ids_collects_errors_from_imports() {
        let train_xml = TrainXML {
            beyond_scope: None,
            samples: None,
            imports: Some(TrainXMLImports {
                import: vec![
                    TrainXMLImportsImport {
                        path: "test.xml".to_string(),
                        system: Some("bad_system".to_string()),
                    },
                ],
            }),
            ..Default::default()
        };
        
        let ids = create_empty_ids();
        let result = train_xml_validate_ids(&train_xml, &ids);
        
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("unknown system prompt ID 'bad_system'"));
    }

    #[test]
    fn test_validate_ids_collects_multiple_errors_from_all_sources() {
        let train_xml = TrainXML {
            beyond_scope: Some(TrainXMLBeyondScope {
                system: "bad_system".to_string(),
                thought: None,
                response: "bad_response".to_string(),
                topics: vec![],
                sports: None,
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
            }),
            samples: Some(TrainXMLSamples {
                sample_ids: Some(vec![
                    TrainXMLSamplesSampleIds {
                        system: None,
                        prompt: "bad_prompt".to_string(),
                        thought: None,
                        response: None,
                        source: None,
                        code: None,
                    },
                ]),
                sample: None,
            }),
            imports: Some(TrainXMLImports {
                import: vec![
                    TrainXMLImportsImport {
                        path: "test.xml".to_string(),
                        system: Some("bad_import_system".to_string()),
                    },
                ],
            }),
            ..Default::default()
        };
        
        let ids = create_empty_ids();
        let result = train_xml_validate_ids(&train_xml, &ids);
        
        assert!(result.is_err());
        let errors = result.unwrap_err();
        
        // Should have errors from beyond-scope (2), sample-ids (1), imports (1) = 4
        assert_eq!(errors.len(), 4);
        assert!(errors.iter().any(|e| e.contains("unknown system prompt ID 'bad_system'")));
        assert!(errors.iter().any(|e| e.contains("unknown response ID 'bad_response'")));
        assert!(errors.iter().any(|e| e.contains("unknown prompt ID 'bad_prompt'")));
        assert!(errors.iter().any(|e| e.contains("unknown system prompt ID 'bad_import_system'")));
    }

    #[test]
    fn test_validate_ids_returns_ok_when_no_errors() {
        // Create a valid train_xml with no references that would cause errors
        let train_xml = TrainXML {
            beyond_scope: None,
            samples: Some(TrainXMLSamples {
                sample_ids: Some(vec![]),
                sample: Some(vec![]),
            }),
            imports: Some(TrainXMLImports {
                import: vec![],
            }),
            ..Default::default()
        };
        
        let ids = create_empty_ids();
        let result = train_xml_validate_ids(&train_xml, &ids);
        
        assert!(result.is_ok());
    }
}
