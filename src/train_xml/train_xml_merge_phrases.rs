// src/train_xml/train_xml_merge_phrases.rs

use std::collections::HashMap;
use crate::train_xml::{TrainXML, TrainXMLPhrases, TrainXMLPhrasesPhrase};


/// Merge phrases from all train XML files into the target train_xml
///
/// For phrases with the same pattern:
/// - The pattern from the highest priority file wins
/// - Variants are merged (no duplicates), preserving order from higher priority files
/// - No errors for duplicates - they are merged together
pub fn train_xml_merge_phrases(
    train_xmls: &[TrainXML],
    train_xml: &mut TrainXML,
) {
    // Map to store merged phrases by pattern
    let mut phrase_map: HashMap<String, TrainXMLPhrasesPhrase> = HashMap::new();
    let mut pattern_order: Vec<String> = Vec::new();
    
    // FIRST: Add any existing phrases from train_xml (preserve them)
    if let Some(existing_phrases) = &train_xml.phrases {
        for phrase in &existing_phrases.phrase {
            phrase_map.insert(phrase.pattern.clone(), phrase.clone());
            pattern_order.push(phrase.pattern.clone());
        }
    }
    
    // SECOND: Iterate through all train_xmls in order (first = highest priority)
    for xml in train_xmls {
        if let Some(source_phrases) = &xml.phrases {
            for phrase in &source_phrases.phrase {
                if let Some(existing) = phrase_map.get_mut(&phrase.pattern) {
                    // Pattern exists - merge variants (no duplicates)
                    let mut existing_variants: HashMap<String, bool> = existing.variant
                        .iter()
                        .map(|v| (v.value.clone(), true))
                        .collect();
                    
                    // Add new variants that don't already exist
                    for variant in &phrase.variant {
                        if !existing_variants.contains_key(&variant.value) {
                            existing.variant.push(variant.clone());
                            existing_variants.insert(variant.value.clone(), true);
                        }
                    }
                } else {
                    // New pattern - add it
                    phrase_map.insert(phrase.pattern.clone(), phrase.clone());
                    pattern_order.push(phrase.pattern.clone());
                }
            }
        }
    }
    
    // Convert map back to Vec in original order
    if !phrase_map.is_empty() {
        let mut phrases = Vec::new();
        for pattern in pattern_order {
            if let Some(phrase) = phrase_map.remove(&pattern) {
                phrases.push(phrase);
            }
        }
        
        train_xml.phrases = Some(TrainXMLPhrases {
            phrase: phrases,
        });
    }
}



#[cfg(test)]
mod tests {
    use crate::train_xml::{
        TrainXML,
        TrainXMLPhrases,
        TrainXMLPhrasesPhrase,
        TrainXMLPhrasesVariant,
        train_xml_merge_phrases,
    };

    fn create_phrase(pattern: &str, variants: Vec<&str>) -> TrainXMLPhrasesPhrase {
        TrainXMLPhrasesPhrase {
            pattern: pattern.to_string(),
            variant: variants.into_iter().map(|v| TrainXMLPhrasesVariant {
                value: v.to_string(),
            }).collect(),
        }
    }

    fn create_phrases_section(phrases: Vec<TrainXMLPhrasesPhrase>) -> TrainXMLPhrases {
        TrainXMLPhrases {
            phrase: phrases,
        }
    }

    #[test]
    fn test_merge_phrases_no_phrases() {
        let train_xmls = vec![
            TrainXML::default(),
            TrainXML::default(),
        ];
        let mut merged = TrainXML::default();
        
        train_xml_merge_phrases(&train_xmls, &mut merged);
        
        assert!(merged.phrases.is_none());
    }

    #[test]
    fn test_merge_phrases_single_file() {
        let phrase1 = create_phrase("pattern1", vec!["var1", "var2"]);
        let phrase2 = create_phrase("pattern2", vec!["var3"]);
        
        let phrases = create_phrases_section(vec![phrase1.clone(), phrase2.clone()]);
        
        let train_xmls = vec![
            TrainXML {
                phrases: Some(phrases),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        train_xml_merge_phrases(&train_xmls, &mut merged);
        
        assert!(merged.phrases.is_some());
        let merged_phrases = merged.phrases.unwrap();
        assert_eq!(merged_phrases.phrase.len(), 2);
        
        assert_eq!(merged_phrases.phrase[0].pattern, "pattern1");
        assert_eq!(merged_phrases.phrase[0].variant.len(), 2);
        assert_eq!(merged_phrases.phrase[0].variant[0].value, "var1");
        assert_eq!(merged_phrases.phrase[0].variant[1].value, "var2");
        
        assert_eq!(merged_phrases.phrase[1].pattern, "pattern2");
        assert_eq!(merged_phrases.phrase[1].variant.len(), 1);
        assert_eq!(merged_phrases.phrase[1].variant[0].value, "var3");
    }

    #[test]
    fn test_merge_phrases_two_files_same_pattern_merge_variants() {
        let phrase1 = create_phrase("common", vec!["var1", "var2"]);
        let phrase2 = create_phrase("common", vec!["var2", "var3", "var4"]);
        
        let phrases1 = create_phrases_section(vec![phrase1]);
        let phrases2 = create_phrases_section(vec![phrase2]);
        
        let train_xmls = vec![
            TrainXML {
                phrases: Some(phrases1),
                ..Default::default()
            },
            TrainXML {
                phrases: Some(phrases2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        train_xml_merge_phrases(&train_xmls, &mut merged);
        
        assert!(merged.phrases.is_some());
        let merged_phrases = merged.phrases.unwrap();
        assert_eq!(merged_phrases.phrase.len(), 1);
        assert_eq!(merged_phrases.phrase[0].pattern, "common");
        
        // Variants should be merged with order from first file, then new ones appended
        assert_eq!(merged_phrases.phrase[0].variant.len(), 4);
        assert_eq!(merged_phrases.phrase[0].variant[0].value, "var1");
        assert_eq!(merged_phrases.phrase[0].variant[1].value, "var2");
        assert_eq!(merged_phrases.phrase[0].variant[2].value, "var3");
        assert_eq!(merged_phrases.phrase[0].variant[3].value, "var4");
    }

    #[test]
    fn test_merge_phrases_three_files_same_pattern() {
        let phrase1 = create_phrase("common", vec!["var1"]);
        let phrase2 = create_phrase("common", vec!["var2"]);
        let phrase3 = create_phrase("common", vec!["var1", "var3"]);
        
        let phrases1 = create_phrases_section(vec![phrase1]);
        let phrases2 = create_phrases_section(vec![phrase2]);
        let phrases3 = create_phrases_section(vec![phrase3]);
        
        let train_xmls = vec![
            TrainXML {
                phrases: Some(phrases1),
                ..Default::default()
            },
            TrainXML {
                phrases: Some(phrases2),
                ..Default::default()
            },
            TrainXML {
                phrases: Some(phrases3),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        train_xml_merge_phrases(&train_xmls, &mut merged);
        
        assert!(merged.phrases.is_some());
        let merged_phrases = merged.phrases.unwrap();
        assert_eq!(merged_phrases.phrase.len(), 1);
        assert_eq!(merged_phrases.phrase[0].pattern, "common");
        
        // Variants should be merged without duplicates
        assert_eq!(merged_phrases.phrase[0].variant.len(), 3);
        assert_eq!(merged_phrases.phrase[0].variant[0].value, "var1");
        assert_eq!(merged_phrases.phrase[0].variant[1].value, "var2");
        assert_eq!(merged_phrases.phrase[0].variant[2].value, "var3");
    }

    #[test]
    fn test_merge_phrases_different_patterns_combined() {
        let phrase1 = create_phrase("pattern1", vec!["var1"]);
        let phrase2 = create_phrase("pattern2", vec!["var2"]);
        let phrase3 = create_phrase("pattern1", vec!["var3"]);
        let phrase4 = create_phrase("pattern3", vec!["var4"]);
        
        let phrases1 = create_phrases_section(vec![phrase1, phrase2]);
        let phrases2 = create_phrases_section(vec![phrase3, phrase4]);
        
        let train_xmls = vec![
            TrainXML {
                phrases: Some(phrases1),
                ..Default::default()
            },
            TrainXML {
                phrases: Some(phrases2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        train_xml_merge_phrases(&train_xmls, &mut merged);
        
        assert!(merged.phrases.is_some());
        let merged_phrases = merged.phrases.unwrap();
        
        // Should have 3 unique patterns (pattern1, pattern2, pattern3)
        assert_eq!(merged_phrases.phrase.len(), 3);
        
        // pattern1 should have merged variants from both files
        let pattern1 = merged_phrases.phrase.iter().find(|p| p.pattern == "pattern1").unwrap();
        assert_eq!(pattern1.variant.len(), 2);
        assert_eq!(pattern1.variant[0].value, "var1");
        assert_eq!(pattern1.variant[1].value, "var3");
        
        // pattern2 only from first file
        let pattern2 = merged_phrases.phrase.iter().find(|p| p.pattern == "pattern2").unwrap();
        assert_eq!(pattern2.variant.len(), 1);
        assert_eq!(pattern2.variant[0].value, "var2");
        
        // pattern3 only from second file
        let pattern3 = merged_phrases.phrase.iter().find(|p| p.pattern == "pattern3").unwrap();
        assert_eq!(pattern3.variant.len(), 1);
        assert_eq!(pattern3.variant[0].value, "var4");
    }

    #[test]
    fn test_merge_phrases_preserves_order() {
        let phrase1 = create_phrase("patternA", vec!["var1"]);
        let phrase2 = create_phrase("patternB", vec!["var2"]);
        let phrase3 = create_phrase("patternC", vec!["var3"]);
        
        let phrases1 = create_phrases_section(vec![phrase1, phrase2]);
        let phrases2 = create_phrases_section(vec![phrase3]);
        
        let train_xmls = vec![
            TrainXML {
                phrases: Some(phrases1),
                ..Default::default()
            },
            TrainXML {
                phrases: Some(phrases2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        train_xml_merge_phrases(&train_xmls, &mut merged);
        
        assert!(merged.phrases.is_some());
        let merged_phrases = merged.phrases.unwrap();
        
        // Order should be preserved: patternA, patternB, patternC
        assert_eq!(merged_phrases.phrase[0].pattern, "patternA");
        assert_eq!(merged_phrases.phrase[1].pattern, "patternB");
        assert_eq!(merged_phrases.phrase[2].pattern, "patternC");
    }

    #[test]
    fn test_merge_phrases_append_to_existing() {
        // Create existing phrases in the target
        let existing_phrase = create_phrase("existing", vec!["var1"]);
        let existing_phrases = create_phrases_section(vec![existing_phrase]);
        
        let mut merged = TrainXML {
            phrases: Some(existing_phrases),
            ..Default::default()
        };
        
        // New phrases to merge
        let new_phrase = create_phrase("new", vec!["var2"]);
        let new_phrases = create_phrases_section(vec![new_phrase]);
        
        let train_xmls = vec![
            TrainXML {
                phrases: Some(new_phrases),
                ..Default::default()
            },
        ];
        
        train_xml_merge_phrases(&train_xmls, &mut merged);
        
        assert!(merged.phrases.is_some());
        let merged_phrases = merged.phrases.unwrap();
        
        // Should have both existing and new phrases (2 total)
        assert_eq!(merged_phrases.phrase.len(), 2);
        assert_eq!(merged_phrases.phrase[0].pattern, "existing");
        assert_eq!(merged_phrases.phrase[1].pattern, "new");
    }

    #[test]
    fn test_merge_phrases_empty_files_skipped() {
        let phrase = create_phrase("pattern1", vec!["var1"]);
        let phrases = create_phrases_section(vec![phrase]);
        
        let train_xmls = vec![
            TrainXML {
                phrases: Some(phrases),
                ..Default::default()
            },
            TrainXML {
                phrases: None,
                ..Default::default()
            },
            TrainXML {
                phrases: Some(create_phrases_section(vec![])),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        train_xml_merge_phrases(&train_xmls, &mut merged);
        
        assert!(merged.phrases.is_some());
        let merged_phrases = merged.phrases.unwrap();
        assert_eq!(merged_phrases.phrase.len(), 1);
        assert_eq!(merged_phrases.phrase[0].pattern, "pattern1");
    }

    #[test]
    fn test_merge_phrases_variant_deduplication_across_multiple_files() {
        let phrase1 = create_phrase("pattern", vec!["var1", "var2", "var3"]);
        let phrase2 = create_phrase("pattern", vec!["var2", "var3", "var4"]);
        let phrase3 = create_phrase("pattern", vec!["var3", "var4", "var5"]);
        
        let phrases1 = create_phrases_section(vec![phrase1]);
        let phrases2 = create_phrases_section(vec![phrase2]);
        let phrases3 = create_phrases_section(vec![phrase3]);
        
        let train_xmls = vec![
            TrainXML {
                phrases: Some(phrases1),
                ..Default::default()
            },
            TrainXML {
                phrases: Some(phrases2),
                ..Default::default()
            },
            TrainXML {
                phrases: Some(phrases3),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        train_xml_merge_phrases(&train_xmls, &mut merged);
        
        assert!(merged.phrases.is_some());
        let merged_phrases = merged.phrases.unwrap();
        assert_eq!(merged_phrases.phrase.len(), 1);
        
        // All unique variants should be included in order of first appearance
        assert_eq!(merged_phrases.phrase[0].variant.len(), 5);
        assert_eq!(merged_phrases.phrase[0].variant[0].value, "var1");
        assert_eq!(merged_phrases.phrase[0].variant[1].value, "var2");
        assert_eq!(merged_phrases.phrase[0].variant[2].value, "var3");
        assert_eq!(merged_phrases.phrase[0].variant[3].value, "var4");
        assert_eq!(merged_phrases.phrase[0].variant[4].value, "var5");
    }

    #[test]
    fn test_merge_phrases_no_error_on_duplicate_patterns() {
        let phrase1 = create_phrase("duplicate", vec!["var1"]);
        let phrase2 = create_phrase("duplicate", vec!["var2"]);
        
        let phrases1 = create_phrases_section(vec![phrase1]);
        let phrases2 = create_phrases_section(vec![phrase2]);
        
        let train_xmls = vec![
            TrainXML {
                phrases: Some(phrases1),
                ..Default::default()
            },
            TrainXML {
                phrases: Some(phrases2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        // Should not panic or return error
        train_xml_merge_phrases(&train_xmls, &mut merged);
        
        assert!(merged.phrases.is_some());
        let merged_phrases = merged.phrases.unwrap();
        assert_eq!(merged_phrases.phrase.len(), 1);
        assert_eq!(merged_phrases.phrase[0].variant.len(), 2);
    }
}
