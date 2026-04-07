// src/train_xml/train_xml_validate_ids_imports.rs

use crate::train_xml::{TrainXML, TrainXMLIdMaps};


pub fn train_xml_validate_ids_imports(
    train_xml: &TrainXML,
    ids: &TrainXMLIdMaps,
    errors: &mut Vec<String>,
) {
    if let Some(imports) = &train_xml.imports {
        for import in &imports.import {
            if let Some(system_id) = &import.system {
                if !ids.system_prompts.contains_key(system_id) {
                    errors.push(format!(
                        "Import references unknown system prompt ID '{}'",
                        system_id
                    ));
                }
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::train_xml_validate_ids_imports;
    use crate::train_xml::{
        TrainXML,
        TrainXMLIdMaps,
        TrainXMLImports,
        TrainXMLImportsImport,
        TrainXMLSystemPromptsSystem,
    };

    fn create_test_ids_with_system_prompts(system_ids: &[&str]) -> TrainXMLIdMaps<'static> {
        let mut system_prompts = HashMap::new();
        
        for &id in system_ids {
            let system = Box::new(TrainXMLSystemPromptsSystem {
                id: id.to_string(),
                content: "Test system prompt".to_string(),
            });
            let system_ref: &'static TrainXMLSystemPromptsSystem = Box::leak(system);
            system_prompts.insert(id.to_string(), system_ref);
        }
        
        TrainXMLIdMaps {
            system_prompts,
            prompts: HashMap::new(),
            thoughts: HashMap::new(),
            responses: HashMap::new(),
            sources: HashMap::new(),
            code_snippets: HashMap::new(),
        }
    }

    #[test]
    fn test_validate_ids_imports_no_imports() {
        let train_xml = TrainXML {
            imports: None,
            ..Default::default()
        };
        
        let ids = create_test_ids_with_system_prompts(&[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_imports(&train_xml, &ids, &mut errors);
        
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_ids_imports_empty_imports() {
        let train_xml = TrainXML {
            imports: Some(TrainXMLImports {
                import: vec![],
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids_with_system_prompts(&[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_imports(&train_xml, &ids, &mut errors);
        
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_ids_imports_valid_system_id() {
        let train_xml = TrainXML {
            imports: Some(TrainXMLImports {
                import: vec![
                    TrainXMLImportsImport {
                        path: "test.xml".to_string(),
                        system: Some("system1".to_string()),
                    },
                ],
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids_with_system_prompts(&["system1"]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_imports(&train_xml, &ids, &mut errors);
        
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_ids_imports_multiple_valid_system_ids() {
        let train_xml = TrainXML {
            imports: Some(TrainXMLImports {
                import: vec![
                    TrainXMLImportsImport {
                        path: "test1.xml".to_string(),
                        system: Some("system1".to_string()),
                    },
                    TrainXMLImportsImport {
                        path: "test2.xml".to_string(),
                        system: Some("system2".to_string()),
                    },
                    TrainXMLImportsImport {
                        path: "test3.xml".to_string(),
                        system: Some("system3".to_string()),
                    },
                ],
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids_with_system_prompts(&["system1", "system2", "system3"]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_imports(&train_xml, &ids, &mut errors);
        
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_ids_imports_invalid_system_id() {
        let train_xml = TrainXML {
            imports: Some(TrainXMLImports {
                import: vec![
                    TrainXMLImportsImport {
                        path: "test.xml".to_string(),
                        system: Some("nonexistent".to_string()),
                    },
                ],
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids_with_system_prompts(&["system1"]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_imports(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("Import references unknown system prompt ID 'nonexistent'"));
    }

    #[test]
    fn test_validate_ids_imports_multiple_invalid_system_ids() {
        let train_xml = TrainXML {
            imports: Some(TrainXMLImports {
                import: vec![
                    TrainXMLImportsImport {
                        path: "test1.xml".to_string(),
                        system: Some("bad1".to_string()),
                    },
                    TrainXMLImportsImport {
                        path: "test2.xml".to_string(),
                        system: Some("bad2".to_string()),
                    },
                ],
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids_with_system_prompts(&["system1"]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_imports(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 2);
        assert!(errors[0].contains("unknown system prompt ID 'bad1'"));
        assert!(errors[1].contains("unknown system prompt ID 'bad2'"));
    }

    #[test]
    fn test_validate_ids_imports_mixed_valid_and_invalid() {
        let train_xml = TrainXML {
            imports: Some(TrainXMLImports {
                import: vec![
                    TrainXMLImportsImport {
                        path: "test1.xml".to_string(),
                        system: Some("system1".to_string()),
                    },
                    TrainXMLImportsImport {
                        path: "test2.xml".to_string(),
                        system: Some("bad".to_string()),
                    },
                    TrainXMLImportsImport {
                        path: "test3.xml".to_string(),
                        system: Some("system2".to_string()),
                    },
                ],
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids_with_system_prompts(&["system1", "system2"]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_imports(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("unknown system prompt ID 'bad'"));
    }

    #[test]
    fn test_validate_ids_imports_no_system_attribute() {
        let train_xml = TrainXML {
            imports: Some(TrainXMLImports {
                import: vec![
                    TrainXMLImportsImport {
                        path: "test.xml".to_string(),
                        system: None,  // No system attribute
                    },
                ],
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids_with_system_prompts(&["system1"]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_imports(&train_xml, &ids, &mut errors);
        
        // No validation needed when system attribute is not present
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_ids_imports_multiple_imports_some_without_system() {
        let train_xml = TrainXML {
            imports: Some(TrainXMLImports {
                import: vec![
                    TrainXMLImportsImport {
                        path: "test1.xml".to_string(),
                        system: Some("system1".to_string()),
                    },
                    TrainXMLImportsImport {
                        path: "test2.xml".to_string(),
                        system: None,
                    },
                    TrainXMLImportsImport {
                        path: "test3.xml".to_string(),
                        system: Some("bad".to_string()),
                    },
                ],
            }),
            ..Default::default()
        };
        
        let ids = create_test_ids_with_system_prompts(&["system1"]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_imports(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("unknown system prompt ID 'bad'"));
    }
}
