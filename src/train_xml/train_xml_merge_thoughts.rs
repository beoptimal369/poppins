// src/train_xml/train_xml_merge_thoughts.rs

use crate::train_xml::{TrainXML, TrainXMLThoughts};


/// Merge thoughts from all train XML files into the target train_xml
///
/// Preserves order by iterating through train_xmls in order and appending thoughts as they're found.
/// If a thought with the same ID is found twice, returns an error.
pub fn train_xml_merge_thoughts(
    train_xmls: &[TrainXML],
    train_xml: &mut TrainXML,
) -> Result<(), String> {
    let mut seen_ids = std::collections::HashSet::new();
    
    // First, collect all thoughts from existing train_xml
    let mut all_thoughts = Vec::new();
    
    // Add existing thoughts from train_xml first
    if let Some(existing_thoughts) = &train_xml.thoughts {
        for thought in &existing_thoughts.thought {
            if seen_ids.contains(&thought.id) {
                return Err(format!("Duplicate thought ID found in existing thoughts: '{}'", thought.id));
            }
            seen_ids.insert(thought.id.clone());
            all_thoughts.push(thought.clone());
        }
    }
    
    // Then add thoughts from all train_xmls in order
    for xml in train_xmls {
        if let Some(source_thoughts) = &xml.thoughts {
            for thought in &source_thoughts.thought {
                // Check for duplicate ID
                if seen_ids.contains(&thought.id) {
                    return Err(format!("Duplicate thought ID found: '{}'", thought.id));
                }
                
                seen_ids.insert(thought.id.clone());
                all_thoughts.push(thought.clone());
            }
        }
    }
    
    // Only set thoughts if we have any
    if !all_thoughts.is_empty() {
        train_xml.thoughts = Some(TrainXMLThoughts {
            thought: all_thoughts,
        });
    } else {
        // No thoughts at all - ensure it's None
        train_xml.thoughts = None;
    }
    
    Ok(())
}



#[cfg(test)]
mod tests {
    use crate::train_xml::{
        TrainXML,
        TrainXMLThoughts,
        TrainXMLThoughtsThought,
        train_xml_merge_thoughts,
    };

    fn create_thought(id: &str, content: &str) -> TrainXMLThoughtsThought {
        TrainXMLThoughtsThought {
            id: id.to_string(),
            content: content.to_string(),
        }
    }

    fn create_thoughts_section(thoughts: Vec<TrainXMLThoughtsThought>) -> TrainXMLThoughts {
        TrainXMLThoughts {
            thought: thoughts,
        }
    }

    #[test]
    fn test_merge_thoughts_no_thoughts() {
        let train_xmls = vec![
            TrainXML::default(),
            TrainXML::default(),
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_thoughts(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.thoughts.is_none());
    }

    #[test]
    fn test_merge_thoughts_single_file() {
        let thought1 = create_thought("th1", "I will think about this.");
        let thought2 = create_thought("th2", "Another approach.");
        
        let thoughts = create_thoughts_section(vec![thought1.clone(), thought2.clone()]);
        
        let train_xmls = vec![
            TrainXML {
                thoughts: Some(thoughts),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_thoughts(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.thoughts.is_some());
        let merged_thoughts = merged.thoughts.unwrap();
        assert_eq!(merged_thoughts.thought.len(), 2);
        
        // Check both thoughts are present
        let ids: Vec<String> = merged_thoughts.thought.iter().map(|t| t.id.clone()).collect();
        assert!(ids.contains(&"th1".to_string()));
        assert!(ids.contains(&"th2".to_string()));
        
        // Check content
        let th1 = merged_thoughts.thought.iter().find(|t| t.id == "th1").unwrap();
        assert_eq!(th1.content, "I will think about this.");
        
        let th2 = merged_thoughts.thought.iter().find(|t| t.id == "th2").unwrap();
        assert_eq!(th2.content, "Another approach.");
    }

    #[test]
    fn test_merge_thoughts_two_files_unique_ids() {
        let thought1 = create_thought("th1", "Thought from file 1");
        let thought2 = create_thought("th2", "Thought from file 2");
        
        let thoughts1 = create_thoughts_section(vec![thought1]);
        let thoughts2 = create_thoughts_section(vec![thought2]);
        
        let train_xmls = vec![
            TrainXML {
                thoughts: Some(thoughts1),
                ..Default::default()
            },
            TrainXML {
                thoughts: Some(thoughts2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_thoughts(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.thoughts.is_some());
        let merged_thoughts = merged.thoughts.unwrap();
        assert_eq!(merged_thoughts.thought.len(), 2);
        
        let ids: Vec<String> = merged_thoughts.thought.iter().map(|t| t.id.clone()).collect();
        assert!(ids.contains(&"th1".to_string()));
        assert!(ids.contains(&"th2".to_string()));
    }

    #[test]
    fn test_merge_thoughts_duplicate_id_error() {
        let thought1 = create_thought("duplicate", "First version");
        let thought2 = create_thought("duplicate", "Second version");
        
        let thoughts1 = create_thoughts_section(vec![thought1]);
        let thoughts2 = create_thoughts_section(vec![thought2]);
        
        let train_xmls = vec![
            TrainXML {
                thoughts: Some(thoughts1),
                ..Default::default()
            },
            TrainXML {
                thoughts: Some(thoughts2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_thoughts(&train_xmls, &mut merged);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Duplicate thought ID found: 'duplicate'");
    }

    #[test]
    fn test_merge_thoughts_duplicate_id_in_same_file() {
        let thought1 = create_thought("duplicate", "First version");
        let thought2 = create_thought("duplicate", "Second version");
        
        let thoughts = create_thoughts_section(vec![thought1, thought2]);
        
        let train_xmls = vec![
            TrainXML {
                thoughts: Some(thoughts),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_thoughts(&train_xmls, &mut merged);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Duplicate thought ID found: 'duplicate'");
    }

    #[test]
    fn test_merge_thoughts_three_files_unique_ids() {
        let thought1 = create_thought("th1", "File 1");
        let thought2 = create_thought("th2", "File 2");
        let thought3 = create_thought("th3", "File 3");
        
        let thoughts1 = create_thoughts_section(vec![thought1]);
        let thoughts2 = create_thoughts_section(vec![thought2]);
        let thoughts3 = create_thoughts_section(vec![thought3]);
        
        let train_xmls = vec![
            TrainXML {
                thoughts: Some(thoughts1),
                ..Default::default()
            },
            TrainXML {
                thoughts: Some(thoughts2),
                ..Default::default()
            },
            TrainXML {
                thoughts: Some(thoughts3),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_thoughts(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.thoughts.is_some());
        let merged_thoughts = merged.thoughts.unwrap();
        assert_eq!(merged_thoughts.thought.len(), 3);
        
        // Order preserved
        assert_eq!(merged_thoughts.thought[0].id, "th1");
        assert_eq!(merged_thoughts.thought[1].id, "th2");
        assert_eq!(merged_thoughts.thought[2].id, "th3");
    }

    #[test]
    fn test_merge_thoughts_append_to_existing() {
        let existing_thought = create_thought("existing", "Existing thought");
        let existing_thoughts = create_thoughts_section(vec![existing_thought]);
        
        let new_thought = create_thought("new", "New thought");
        let new_thoughts = create_thoughts_section(vec![new_thought]);
        
        let mut merged = TrainXML {
            thoughts: Some(existing_thoughts),
            ..Default::default()
        };
        
        let train_xmls = vec![
            TrainXML {
                thoughts: Some(new_thoughts),
                ..Default::default()
            },
        ];
        
        let result = train_xml_merge_thoughts(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.thoughts.is_some());
        let merged_thoughts = merged.thoughts.unwrap();
        assert_eq!(merged_thoughts.thought.len(), 2);
        assert_eq!(merged_thoughts.thought[0].id, "existing");
        assert_eq!(merged_thoughts.thought[1].id, "new");
    }

    #[test]
    fn test_merge_thoughts_empty_files_skipped() {
        let thought = create_thought("th1", "Content");
        let thoughts = create_thoughts_section(vec![thought]);
        
        let train_xmls = vec![
            TrainXML {
                thoughts: Some(thoughts),
                ..Default::default()
            },
            TrainXML {
                thoughts: None,
                ..Default::default()
            },
            TrainXML {
                thoughts: Some(create_thoughts_section(vec![])),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_thoughts(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.thoughts.is_some());
        let merged_thoughts = merged.thoughts.unwrap();
        assert_eq!(merged_thoughts.thought.len(), 1);
        assert_eq!(merged_thoughts.thought[0].id, "th1");
    }

    #[test]
    fn test_merge_thoughts_preserves_order_with_duplicate_check() {
        let thought1 = create_thought("th1", "First");
        let thought2 = create_thought("th2", "Second");
        let thought3 = create_thought("th3", "Third");
        let thought4 = create_thought("th4", "Fourth");
        
        let thoughts1 = create_thoughts_section(vec![thought1, thought2]);
        let thoughts2 = create_thoughts_section(vec![thought3, thought4]);
        
        let train_xmls = vec![
            TrainXML {
                thoughts: Some(thoughts1),
                ..Default::default()
            },
            TrainXML {
                thoughts: Some(thoughts2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_thoughts(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.thoughts.is_some());
        let merged_thoughts = merged.thoughts.unwrap();
        assert_eq!(merged_thoughts.thought.len(), 4);
        
        // Order preserved
        assert_eq!(merged_thoughts.thought[0].id, "th1");
        assert_eq!(merged_thoughts.thought[1].id, "th2");
        assert_eq!(merged_thoughts.thought[2].id, "th3");
        assert_eq!(merged_thoughts.thought[3].id, "th4");
    }
}
