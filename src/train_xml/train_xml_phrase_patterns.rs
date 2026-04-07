// src/train_xml/train_xml_phrase_patterns.rs

use regex::Regex;
use std::sync::Arc;
use crate::train_xml::{TrainXML, TrainXMLPhrasePattern};


/// Creates compiled phrase patterns from the train XML
pub fn train_xml_phrase_patterns(train_xml: &TrainXML) -> Vec<TrainXMLPhrasePattern> {
    let phrases_section = match &train_xml.phrases {
        Some(p) if !p.phrase.is_empty() => &p.phrase,
        _ => return Vec::new(),
    };

    let mut compiled_patterns = Vec::with_capacity(phrases_section.len());

    for phrase in phrases_section {
        match Regex::new(&phrase.pattern) {
            Ok(regex) => {
                let variants: Vec<String> = phrase.variant
                    .iter()
                    .map(|v| v.value.clone())
                    .collect();
                
                let has_captures = TrainXMLPhrasePattern::has_capture_groups(&regex);
                let has_multiple_captures = TrainXMLPhrasePattern::has_multiple_capture_groups(&regex);
                let variants_use_multiple_captures = TrainXMLPhrasePattern::variants_use_multiple_captures(&variants);
                let replacements = TrainXMLPhrasePattern::compile_replacements(&variants);
                
                compiled_patterns.push(TrainXMLPhrasePattern {
                    regex: Arc::new(regex),
                    variants: Arc::new(variants),
                    replacements: Arc::new(replacements),
                    has_captures,
                    has_multiple_captures,
                    variants_use_multiple_captures,
                });
            }
            Err(e) => {
                eprintln!("  ✗ Failed to compile regex '{}': {}", phrase.pattern, e);
            }
        }
    }
    
    compiled_patterns
}



#[cfg(test)]
mod tests {
    use crate::train_xml::{
        TrainXML,
        TrainXMLPhrases,
        TrainXMLPhrasesPhrase,
        TrainXMLPhrasesVariant,
        train_xml_phrase_patterns,
    };

    fn create_test_train_xml_with_phrases() -> TrainXML {
        TrainXML {
            imports: None, 
            system_prompts: None,
            prompts: None,
            thoughts: None,
            responses: None,
            sources: None,
            code_snippets: None,
            samples: None,
            constants: None,
            phrases: Some(TrainXMLPhrases {
                phrase: vec![
                    TrainXMLPhrasesPhrase {
                        pattern: "What (?:is|are) (?:a |an |the )?(.*?)\\?".to_string(),
                        variant: vec![
                            TrainXMLPhrasesVariant { value: "Define $1.".to_string() },
                            TrainXMLPhrasesVariant { value: "Define: $1.".to_string() },
                            TrainXMLPhrasesVariant { value: "Tell me about $1.".to_string() },
                        ],
                    },
                    TrainXMLPhrasesPhrase {
                        pattern: "ty".to_string(),
                        variant: vec![
                            TrainXMLPhrasesVariant { value: "thanks".to_string() },
                            TrainXMLPhrasesVariant { value: "thank you".to_string() },
                        ],
                    },
                ],
            }),
            beyond_scope: None,
        }
    }

    fn create_test_train_xml_with_multi_capture() -> TrainXML {
        TrainXML {
            imports: None, 
            system_prompts: None,
            prompts: None,
            thoughts: None,
            responses: None,
            sources: None,
            code_snippets: None,
            samples: None,
            constants: None,
            phrases: Some(TrainXMLPhrases {
                phrase: vec![
                    TrainXMLPhrasesPhrase {
                        pattern: "The (\\w+) is (\\w+)\\.".to_string(),
                        variant: vec![
                            TrainXMLPhrasesVariant { value: "$1 is very $2.".to_string() },
                            TrainXMLPhrasesVariant { value: "The $2 $1 is amazing.".to_string() },
                        ],
                    },
                ],
            }),
            beyond_scope: None,
        }
    }

    fn create_test_train_xml_with_no_phrases() -> TrainXML {
        TrainXML {
            imports: None, 
            system_prompts: None,
            prompts: None,
            thoughts: None,
            responses: None,
            sources: None,
            code_snippets: None,
            samples: None,
            constants: None,
            phrases: None,
            beyond_scope: None,
        }
    }

    fn create_test_train_xml_with_empty_phrases() -> TrainXML {
        TrainXML {
            imports: None, 
            system_prompts: None,
            prompts: None,
            thoughts: None,
            responses: None,
            sources: None,
            code_snippets: None,
            samples: None,
            constants: None,
            phrases: Some(TrainXMLPhrases { phrase: vec![] }),
            beyond_scope: None,
        }
    }

    fn create_test_train_xml_with_invalid_regex() -> TrainXML {
        TrainXML {
            imports: None, 
            system_prompts: None,
            prompts: None,
            thoughts: None,
            responses: None,
            sources: None,
            code_snippets: None,
            samples: None,
            constants: None,
            phrases: Some(TrainXMLPhrases {
                phrase: vec![
                    TrainXMLPhrasesPhrase {
                        pattern: "(invalid".to_string(),
                        variant: vec![
                            TrainXMLPhrasesVariant { value: "test".to_string() },
                        ],
                    },
                    TrainXMLPhrasesPhrase {
                        pattern: "valid".to_string(),
                        variant: vec![
                            TrainXMLPhrasesVariant { value: "valid variant".to_string() },
                        ],
                    },
                ],
            }),
            beyond_scope: None,
        }
    }

    #[test]
    fn test_train_xml_phrase_patterns_with_phrases() {
        let train_xml = create_test_train_xml_with_phrases();
        let patterns = train_xml_phrase_patterns(&train_xml);
        
        assert_eq!(patterns.len(), 2);
        
        // Test first pattern (single capture group)
        let first = &patterns[0];
        assert!(first.regex.is_match("What is a computer?"));
        assert!(first.regex.is_match("What are movies?"));
        assert!(first.has_captures);
        assert!(!first.has_multiple_captures);
        assert!(!first.variants_use_multiple_captures);
        assert_eq!(first.variants.len(), 3);
        assert_eq!(first.variants[0], "Define $1.");
        assert_eq!(first.variants[1], "Define: $1.");
        assert_eq!(first.variants[2], "Tell me about $1.");
        
        // Test replacement
        let result = first.replace("What is a computer?", 0);
        assert_eq!(result, Some("Define computer.".to_string()));
        
        // Test second pattern (no capture groups)
        let second = &patterns[1];
        assert!(second.regex.is_match("ty"));
        assert!(!second.has_captures);
        assert!(!second.has_multiple_captures);
        assert!(!second.variants_use_multiple_captures);
        assert_eq!(second.variants.len(), 2);
        assert_eq!(second.variants[0], "thanks");
        assert_eq!(second.variants[1], "thank you");
        
        let result = second.replace("ty", 0);
        assert_eq!(result, Some("thanks".to_string()));
    }

    #[test]
    fn test_train_xml_phrase_patterns_with_multi_capture() {
        let train_xml = create_test_train_xml_with_multi_capture();
        let patterns = train_xml_phrase_patterns(&train_xml);
        
        assert_eq!(patterns.len(), 1);
        
        let pattern = &patterns[0];
        assert!(pattern.regex.is_match("The cat is fast."));
        assert!(pattern.has_captures);
        assert!(pattern.has_multiple_captures);
        assert!(pattern.variants_use_multiple_captures);
        assert_eq!(pattern.variants.len(), 2);
        
        // Test replacement uses slow path correctly
        let result = pattern.replace("The cat is fast.", 0);
        assert_eq!(result, Some("cat is very fast.".to_string()));
        
        let result = pattern.replace("The cat is fast.", 1);
        assert_eq!(result, Some("The fast cat is amazing.".to_string()));
    }

    #[test]
    fn test_train_xml_phrase_patterns_with_no_phrases() {
        let train_xml = create_test_train_xml_with_no_phrases();
        let patterns = train_xml_phrase_patterns(&train_xml);
        assert!(patterns.is_empty());
    }

    #[test]
    fn test_train_xml_phrase_patterns_with_empty_phrases() {
        let train_xml = create_test_train_xml_with_empty_phrases();
        let patterns = train_xml_phrase_patterns(&train_xml);
        assert!(patterns.is_empty());
    }

    #[test]
    fn test_train_xml_phrase_patterns_with_invalid_regex() {
        let train_xml = create_test_train_xml_with_invalid_regex();
        let patterns = train_xml_phrase_patterns(&train_xml);
        
        // Should skip the invalid regex and only include the valid one
        assert_eq!(patterns.len(), 1);
        assert_eq!(patterns[0].variants[0], "valid variant");
    }

    #[test]
    fn test_train_xml_phrase_patterns_preserves_order() {
        let train_xml = TrainXML {
            imports: None, 
            system_prompts: None,
            prompts: None,
            thoughts: None,
            responses: None,
            sources: None,
            code_snippets: None,
            samples: None,
            constants: None,
            phrases: Some(TrainXMLPhrases {
                phrase: vec![
                    TrainXMLPhrasesPhrase {
                        pattern: "pattern1".to_string(),
                        variant: vec![
                            TrainXMLPhrasesVariant { value: "var1".to_string() },
                        ],
                    },
                    TrainXMLPhrasesPhrase {
                        pattern: "pattern2".to_string(),
                        variant: vec![
                            TrainXMLPhrasesVariant { value: "var2".to_string() },
                        ],
                    },
                    TrainXMLPhrasesPhrase {
                        pattern: "pattern3".to_string(),
                        variant: vec![
                            TrainXMLPhrasesVariant { value: "var3".to_string() },
                        ],
                    },
                ],
            }),
            beyond_scope: None,
        };
        
        let patterns = train_xml_phrase_patterns(&train_xml);
        
        assert_eq!(patterns.len(), 3);
        assert_eq!(patterns[0].variants[0], "var1");
        assert_eq!(patterns[1].variants[0], "var2");
        assert_eq!(patterns[2].variants[0], "var3");
    }

    #[test]
    fn test_train_xml_phrase_patterns_with_complex_pattern() {
        let train_xml = TrainXML {
            imports: None, 
            system_prompts: None,
            prompts: None,
            thoughts: None,
            responses: None,
            sources: None,
            code_snippets: None,
            samples: None,
            constants: None,
            phrases: Some(TrainXMLPhrases {
                phrase: vec![
                    TrainXMLPhrasesPhrase {
                        pattern: r"The (\w+) and (\w+) are (\w+)\.".to_string(),
                        variant: vec![
                            TrainXMLPhrasesVariant { value: "$1, $2, and $3".to_string() },
                        ],
                    },
                ],
            }),
            beyond_scope: None,
        };
        
        let patterns = train_xml_phrase_patterns(&train_xml);
        
        assert_eq!(patterns.len(), 1);
        let pattern = &patterns[0];
        assert!(pattern.has_captures);
        assert!(pattern.has_multiple_captures);
        assert!(pattern.variants_use_multiple_captures);
        
        let result = pattern.replace("The cat and dog are animals.", 0);
        assert_eq!(result, Some("cat, dog, and animals".to_string()));
    }

    #[test]
    fn test_train_xml_phrase_patterns_with_named_capture_groups() {
        let train_xml = TrainXML {
            imports: None, 
            system_prompts: None,
            prompts: None,
            thoughts: None,
            responses: None,
            sources: None,
            code_snippets: None,
            samples: None,
            constants: None,
            phrases: Some(TrainXMLPhrases {
                phrase: vec![
                    TrainXMLPhrasesPhrase {
                        pattern: r"(?P<first>\w+) and (?P<second>\w+)".to_string(),
                        variant: vec![
                            TrainXMLPhrasesVariant { value: "$1 and $2".to_string() },
                        ],
                    },
                ],
            }),
            beyond_scope: None,
        };
        
        let patterns = train_xml_phrase_patterns(&train_xml);
        
        assert_eq!(patterns.len(), 1);
        let pattern = &patterns[0];
        assert!(pattern.has_captures);
        assert!(pattern.has_multiple_captures);
        assert!(pattern.variants_use_multiple_captures);
        
        let result = pattern.replace("cats and dogs", 0);
        assert_eq!(result, Some("cats and dogs".to_string()));
    }

    #[test]
    fn test_train_xml_phrase_patterns_with_variants_using_placeholders() {
        let train_xml = TrainXML {
            imports: None, 
            system_prompts: None,
            prompts: None,
            thoughts: None,
            responses: None,
            sources: None,
            code_snippets: None,
            samples: None,
            constants: None,
            phrases: Some(TrainXMLPhrases {
                phrase: vec![
                    TrainXMLPhrasesPhrase {
                        pattern: r"Hello (.*?)!".to_string(),
                        variant: vec![
                            TrainXMLPhrasesVariant { value: "Greetings, $1!".to_string() },
                            TrainXMLPhrasesVariant { value: "Hi $1!".to_string() },
                            TrainXMLPhrasesVariant { value: "Hey $1!".to_string() },
                        ],
                    },
                ],
            }),
            beyond_scope: None,
        };
        
        let patterns = train_xml_phrase_patterns(&train_xml);
        
        assert_eq!(patterns.len(), 1);
        let pattern = &patterns[0];
        assert!(pattern.has_captures);
        assert!(!pattern.has_multiple_captures);
        assert!(!pattern.variants_use_multiple_captures);
        assert_eq!(pattern.variants.len(), 3);
        
        let result = pattern.replace("Hello world!", 0);
        assert_eq!(result, Some("Greetings, world!".to_string()));
        
        let result = pattern.replace("Hello world!", 1);
        assert_eq!(result, Some("Hi world!".to_string()));
        
        let result = pattern.replace("Hello world!", 2);
        assert_eq!(result, Some("Hey world!".to_string()));
    }
}
