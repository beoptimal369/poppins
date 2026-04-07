// src/train_xml/train_xml_validate_ids_beyond_scope.rs

use crate::train_xml::{TrainXML, TrainXMLIdMaps};


pub fn train_xml_validate_ids_beyond_scope(
    train_xml: &TrainXML,
    ids: &TrainXMLIdMaps,
    errors: &mut Vec<String>,
) {
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

        if let Some(thought_id) = &beyond_scope.thought {
            if !ids.thoughts.contains_key(thought_id) {
                errors.push(format!(
                    "Beyond-scope references unknown thought ID '{}'",
                    thought_id
                ));
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::train_xml_validate_ids_beyond_scope;
    use crate::train_xml::{
        TrainXML,
        TrainXMLIdMaps,
        TrainXMLBeyondScope,
        TrainXMLSystemPromptsSystem,
        TrainXMLThoughtsThought,
        TrainXMLResponsesResponse,
    };

    fn create_test_ids(
        system_ids: &[&str],
        response_ids: &[&str],
        thought_ids: &[&str],
    ) -> TrainXMLIdMaps<'static> {
        let mut system_prompts = HashMap::new();
        for &id in system_ids {
            let system = Box::new(TrainXMLSystemPromptsSystem {
                id: id.to_string(),
                content: "Test system".to_string(),
            });
            // Cast &mut T to &T by dereferencing and re-borrowing
            let system_ref: &'static TrainXMLSystemPromptsSystem = &*Box::leak(system);
            system_prompts.insert(id.to_string(), system_ref);
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
        
        let mut thoughts = HashMap::new();
        for &id in thought_ids {
            let thought = Box::new(TrainXMLThoughtsThought {
                id: id.to_string(),
                content: "Test thought".to_string(),
            });
            let thought_ref: &'static TrainXMLThoughtsThought = &*Box::leak(thought);
            thoughts.insert(id.to_string(), thought_ref);
        }
        
        TrainXMLIdMaps {
            system_prompts,
            prompts: HashMap::new(),
            thoughts,
            responses,
            sources: HashMap::new(),
            code_snippets: HashMap::new(),
        }
    }

    #[test]
    fn test_validate_ids_beyond_scope_no_beyond_scope() {
        let train_xml = TrainXML {
            beyond_scope: None,
            ..Default::default()
        };
        
        let ids = create_test_ids(&[], &[], &[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_beyond_scope(&train_xml, &ids, &mut errors);
        
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_ids_beyond_scope_valid_all_ids() {
        let train_xml = TrainXML {
            beyond_scope: Some(TrainXMLBeyondScope {
                system: "system1".to_string(),
                thought: Some("thought1".to_string()),
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
            ..Default::default()
        };
        
        let ids = create_test_ids(&["system1"], &["response1"], &["thought1"]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_beyond_scope(&train_xml, &ids, &mut errors);
        
        assert!(errors.is_empty());
    }

    #[test]
    fn test_validate_ids_beyond_scope_invalid_system_id() {
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
            ..Default::default()
        };
        
        let ids = create_test_ids(&["system1"], &["response1"], &[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_beyond_scope(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("unknown system prompt ID 'nonexistent'"));
    }

    #[test]
    fn test_validate_ids_beyond_scope_invalid_response_id() {
        let train_xml = TrainXML {
            beyond_scope: Some(TrainXMLBeyondScope {
                system: "system1".to_string(),
                thought: None,
                response: "nonexistent".to_string(),
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
            ..Default::default()
        };
        
        let ids = create_test_ids(&["system1"], &["response1"], &[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_beyond_scope(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("unknown response ID 'nonexistent'"));
    }

    #[test]
    fn test_validate_ids_beyond_scope_invalid_thought_id() {
        let train_xml = TrainXML {
            beyond_scope: Some(TrainXMLBeyondScope {
                system: "system1".to_string(),
                thought: Some("nonexistent".to_string()),
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
            ..Default::default()
        };
        
        let ids = create_test_ids(&["system1"], &["response1"], &["thought1"]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_beyond_scope(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("unknown thought ID 'nonexistent'"));
    }

    #[test]
    fn test_validate_ids_beyond_scope_multiple_errors() {
        let train_xml = TrainXML {
            beyond_scope: Some(TrainXMLBeyondScope {
                system: "bad_system".to_string(),
                thought: Some("bad_thought".to_string()),
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
            ..Default::default()
        };
        
        let ids = create_test_ids(&["system1"], &["response1"], &["thought1"]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_beyond_scope(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 3);
        assert!(errors[0].contains("unknown system prompt ID 'bad_system'"));
        assert!(errors[1].contains("unknown response ID 'bad_response'"));
        assert!(errors[2].contains("unknown thought ID 'bad_thought'"));
    }

    #[test]
    fn test_validate_ids_beyond_scope_mixed_valid_and_invalid() {
        let train_xml = TrainXML {
            beyond_scope: Some(TrainXMLBeyondScope {
                system: "system1".to_string(),  // Valid
                thought: Some("bad_thought".to_string()),  // Invalid
                response: "response1".to_string(),  // Valid
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
            ..Default::default()
        };
        
        let ids = create_test_ids(&["system1"], &["response1"], &["thought1"]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_beyond_scope(&train_xml, &ids, &mut errors);
        
        assert_eq!(errors.len(), 1);
        assert!(errors[0].contains("unknown thought ID 'bad_thought'"));
    }

    #[test]
    fn test_validate_ids_beyond_scope_no_thought() {
        let train_xml = TrainXML {
            beyond_scope: Some(TrainXMLBeyondScope {
                system: "system1".to_string(),
                thought: None,  // No thought
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
            ..Default::default()
        };
        
        let ids = create_test_ids(&["system1"], &["response1"], &[]);
        let mut errors = Vec::new();
        
        train_xml_validate_ids_beyond_scope(&train_xml, &ids, &mut errors);
        
        assert!(errors.is_empty());
    }
}
