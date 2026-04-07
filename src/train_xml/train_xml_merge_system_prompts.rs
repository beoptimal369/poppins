// src/train_xml/train_xml_merge_system_prompts.rs

use crate::train_xml::{TrainXML, TrainXMLSystemPrompts};


/// Merge system prompts from all train XML files into the target train_xml
///
/// Preserves order by iterating through train_xmls in order and appending system prompts as they're found.
/// If a system prompt with the same ID is found twice, returns an error.
pub fn train_xml_merge_system_prompts(
    train_xmls: &[TrainXML],
    train_xml: &mut TrainXML,
) -> Result<(), String> {
    let mut seen_ids = std::collections::HashSet::new();
    
    // First, collect all system prompts from existing train_xml
    let mut all_system_prompts = Vec::new();
    
    // Add existing system prompts from train_xml first
    if let Some(existing_system_prompts) = &train_xml.system_prompts {
        for system_prompt in &existing_system_prompts.system {
            if seen_ids.contains(&system_prompt.id) {
                return Err(format!("Duplicate system prompt ID found in existing prompts: '{}'", system_prompt.id));
            }
            seen_ids.insert(system_prompt.id.clone());
            all_system_prompts.push(system_prompt.clone());
        }
    }
    
    // Then add system prompts from all train_xmls in order
    for xml in train_xmls {
        if let Some(source_system_prompts) = &xml.system_prompts {
            for system_prompt in &source_system_prompts.system {
                // Check for duplicate ID
                if seen_ids.contains(&system_prompt.id) {
                    return Err(format!("Duplicate system prompt ID found: '{}'", system_prompt.id));
                }
                
                seen_ids.insert(system_prompt.id.clone());
                all_system_prompts.push(system_prompt.clone());
            }
        }
    }
    
    // Only set system_prompts if we have any
    if !all_system_prompts.is_empty() {
        train_xml.system_prompts = Some(TrainXMLSystemPrompts {
            system: all_system_prompts,
        });
    } else {
        // No system prompts at all - ensure it's None
        train_xml.system_prompts = None;
    }
    
    Ok(())
}



#[cfg(test)]
mod tests {
    use crate::train_xml::{
        TrainXML,
        TrainXMLSystemPrompts,
        TrainXMLSystemPromptsSystem,
        train_xml_merge_system_prompts,
    };

    fn create_system_prompt(id: &str, content: &str) -> TrainXMLSystemPromptsSystem {
        TrainXMLSystemPromptsSystem {
            id: id.to_string(),
            content: content.to_string(),
        }
    }

    fn create_system_prompts_section(system_prompts: Vec<TrainXMLSystemPromptsSystem>) -> TrainXMLSystemPrompts {
        TrainXMLSystemPrompts {
            system: system_prompts,
        }
    }

    #[test]
    fn test_merge_system_prompts_no_system_prompts() {
        let train_xmls = vec![
            TrainXML::default(),
            TrainXML::default(),
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_system_prompts(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.system_prompts.is_none());
    }

    #[test]
    fn test_merge_system_prompts_single_file() {
        let system1 = create_system_prompt("sy1", "You are a helpful assistant.");
        let system2 = create_system_prompt("sy2", "You are a coding expert.");
        
        let system_prompts = create_system_prompts_section(vec![system1.clone(), system2.clone()]);
        
        let train_xmls = vec![
            TrainXML {
                system_prompts: Some(system_prompts),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_system_prompts(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.system_prompts.is_some());
        let merged_system_prompts = merged.system_prompts.unwrap();
        assert_eq!(merged_system_prompts.system.len(), 2);
        
        // Check both system prompts are present
        let ids: Vec<String> = merged_system_prompts.system.iter().map(|s| s.id.clone()).collect();
        assert!(ids.contains(&"sy1".to_string()));
        assert!(ids.contains(&"sy2".to_string()));
        
        // Check content
        let sy1 = merged_system_prompts.system.iter().find(|s| s.id == "sy1").unwrap();
        assert_eq!(sy1.content, "You are a helpful assistant.");
        
        let sy2 = merged_system_prompts.system.iter().find(|s| s.id == "sy2").unwrap();
        assert_eq!(sy2.content, "You are a coding expert.");
    }

    #[test]
    fn test_merge_system_prompts_two_files_unique_ids() {
        let system1 = create_system_prompt("sy1", "System from file 1");
        let system2 = create_system_prompt("sy2", "System from file 2");
        
        let system_prompts1 = create_system_prompts_section(vec![system1]);
        let system_prompts2 = create_system_prompts_section(vec![system2]);
        
        let train_xmls = vec![
            TrainXML {
                system_prompts: Some(system_prompts1),
                ..Default::default()
            },
            TrainXML {
                system_prompts: Some(system_prompts2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_system_prompts(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.system_prompts.is_some());
        let merged_system_prompts = merged.system_prompts.unwrap();
        assert_eq!(merged_system_prompts.system.len(), 2);
        
        let ids: Vec<String> = merged_system_prompts.system.iter().map(|s| s.id.clone()).collect();
        assert!(ids.contains(&"sy1".to_string()));
        assert!(ids.contains(&"sy2".to_string()));
    }

    #[test]
    fn test_merge_system_prompts_duplicate_id_error() {
        let system1 = create_system_prompt("duplicate", "First version");
        let system2 = create_system_prompt("duplicate", "Second version");
        
        let system_prompts1 = create_system_prompts_section(vec![system1]);
        let system_prompts2 = create_system_prompts_section(vec![system2]);
        
        let train_xmls = vec![
            TrainXML {
                system_prompts: Some(system_prompts1),
                ..Default::default()
            },
            TrainXML {
                system_prompts: Some(system_prompts2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_system_prompts(&train_xmls, &mut merged);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Duplicate system prompt ID found: 'duplicate'");
    }

    #[test]
    fn test_merge_system_prompts_duplicate_id_in_same_file() {
        let system1 = create_system_prompt("duplicate", "First version");
        let system2 = create_system_prompt("duplicate", "Second version");
        
        let system_prompts = create_system_prompts_section(vec![system1, system2]);
        
        let train_xmls = vec![
            TrainXML {
                system_prompts: Some(system_prompts),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_system_prompts(&train_xmls, &mut merged);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Duplicate system prompt ID found: 'duplicate'");
    }

    #[test]
    fn test_merge_system_prompts_three_files_unique_ids() {
        let system1 = create_system_prompt("sy1", "File 1");
        let system2 = create_system_prompt("sy2", "File 2");
        let system3 = create_system_prompt("sy3", "File 3");
        
        let system_prompts1 = create_system_prompts_section(vec![system1]);
        let system_prompts2 = create_system_prompts_section(vec![system2]);
        let system_prompts3 = create_system_prompts_section(vec![system3]);
        
        let train_xmls = vec![
            TrainXML {
                system_prompts: Some(system_prompts1),
                ..Default::default()
            },
            TrainXML {
                system_prompts: Some(system_prompts2),
                ..Default::default()
            },
            TrainXML {
                system_prompts: Some(system_prompts3),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_system_prompts(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.system_prompts.is_some());
        let merged_system_prompts = merged.system_prompts.unwrap();
        assert_eq!(merged_system_prompts.system.len(), 3);
        
        // Order preserved
        assert_eq!(merged_system_prompts.system[0].id, "sy1");
        assert_eq!(merged_system_prompts.system[1].id, "sy2");
        assert_eq!(merged_system_prompts.system[2].id, "sy3");
    }

    #[test]
    fn test_merge_system_prompts_append_to_existing() {
        let existing_system = create_system_prompt("existing", "Existing system prompt");
        let existing_system_prompts = create_system_prompts_section(vec![existing_system]);
        
        let new_system = create_system_prompt("new", "New system prompt");
        let new_system_prompts = create_system_prompts_section(vec![new_system]);
        
        let mut merged = TrainXML {
            system_prompts: Some(existing_system_prompts),
            ..Default::default()
        };
        
        let train_xmls = vec![
            TrainXML {
                system_prompts: Some(new_system_prompts),
                ..Default::default()
            },
        ];
        
        let result = train_xml_merge_system_prompts(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.system_prompts.is_some());
        let merged_system_prompts = merged.system_prompts.unwrap();
        assert_eq!(merged_system_prompts.system.len(), 2);
        assert_eq!(merged_system_prompts.system[0].id, "existing");
        assert_eq!(merged_system_prompts.system[1].id, "new");
    }

    #[test]
    fn test_merge_system_prompts_empty_files_skipped() {
        let system = create_system_prompt("sy1", "Content");
        let system_prompts = create_system_prompts_section(vec![system]);
        
        let train_xmls = vec![
            TrainXML {
                system_prompts: Some(system_prompts),
                ..Default::default()
            },
            TrainXML {
                system_prompts: None,
                ..Default::default()
            },
            TrainXML {
                system_prompts: Some(create_system_prompts_section(vec![])),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_system_prompts(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.system_prompts.is_some());
        let merged_system_prompts = merged.system_prompts.unwrap();
        assert_eq!(merged_system_prompts.system.len(), 1);
        assert_eq!(merged_system_prompts.system[0].id, "sy1");
    }

    #[test]
    fn test_merge_system_prompts_preserves_order_with_duplicate_check() {
        let system1 = create_system_prompt("sy1", "First");
        let system2 = create_system_prompt("sy2", "Second");
        let system3 = create_system_prompt("sy3", "Third");
        let system4 = create_system_prompt("sy4", "Fourth");
        
        let system_prompts1 = create_system_prompts_section(vec![system1, system2]);
        let system_prompts2 = create_system_prompts_section(vec![system3, system4]);
        
        let train_xmls = vec![
            TrainXML {
                system_prompts: Some(system_prompts1),
                ..Default::default()
            },
            TrainXML {
                system_prompts: Some(system_prompts2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_system_prompts(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.system_prompts.is_some());
        let merged_system_prompts = merged.system_prompts.unwrap();
        assert_eq!(merged_system_prompts.system.len(), 4);
        
        // Order preserved
        assert_eq!(merged_system_prompts.system[0].id, "sy1");
        assert_eq!(merged_system_prompts.system[1].id, "sy2");
        assert_eq!(merged_system_prompts.system[2].id, "sy3");
        assert_eq!(merged_system_prompts.system[3].id, "sy4");
    }
}
