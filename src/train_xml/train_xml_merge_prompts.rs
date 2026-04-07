// src/train_xml/train_xml_merge_prompts.rs

use crate::train_xml::{TrainXML, TrainXMLPrompts};


/// Merge prompts from all train XML files into the target train_xml
///
/// Preserves order by iterating through train_xmls in order and appending prompts as they're found.
/// If a prompt with the same ID is found twice, returns an error.
pub fn train_xml_merge_prompts(
    train_xmls: &[TrainXML],
    train_xml: &mut TrainXML,
) -> Result<(), String> {
    let mut seen_ids = std::collections::HashSet::new();
    
    // First, collect all prompts from existing train_xml
    let mut all_prompts = Vec::new();
    
    // Add existing prompts from train_xml first
    if let Some(existing_prompts) = &train_xml.prompts {
        for prompt in &existing_prompts.prompt {
            if seen_ids.contains(&prompt.id) {
                return Err(format!("Duplicate prompt ID found in existing prompts: '{}'", prompt.id));
            }
            seen_ids.insert(prompt.id.clone());
            all_prompts.push(prompt.clone());
        }
    }
    
    // Then add prompts from all train_xmls in order
    for xml in train_xmls {
        if let Some(source_prompts) = &xml.prompts {
            for prompt in &source_prompts.prompt {
                // Check for duplicate ID
                if seen_ids.contains(&prompt.id) {
                    return Err(format!("Duplicate prompt ID found: '{}'", prompt.id));
                }
                
                seen_ids.insert(prompt.id.clone());
                all_prompts.push(prompt.clone());
            }
        }
    }
    
    // Only set prompts if we have any
    if !all_prompts.is_empty() {
        train_xml.prompts = Some(TrainXMLPrompts {
            prompt: all_prompts,
        });
    } else {
        // No prompts at all - ensure it's None
        train_xml.prompts = None;
    }
    
    Ok(())
}



#[cfg(test)]
mod tests {
    use crate::train_xml::{
        TrainXML,
        TrainXMLPrompts,
        TrainXMLPromptsPrompt,
        train_xml_merge_prompts,
    };

    fn create_prompt(id: &str, content: &str) -> TrainXMLPromptsPrompt {
        TrainXMLPromptsPrompt {
            id: id.to_string(),
            content: content.to_string(),
        }
    }

    fn create_prompts_section(prompts: Vec<TrainXMLPromptsPrompt>) -> TrainXMLPrompts {
        TrainXMLPrompts {
            prompt: prompts,
        }
    }

    #[test]
    fn test_merge_prompts_no_prompts() {
        let train_xmls = vec![
            TrainXML::default(),
            TrainXML::default(),
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_prompts(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.prompts.is_none());
    }

    #[test]
    fn test_merge_prompts_single_file() {
        let prompt1 = create_prompt("prompt1", "What is Rust?");
        let prompt2 = create_prompt("prompt2", "What is Python?");
        
        let prompts = create_prompts_section(vec![prompt1.clone(), prompt2.clone()]);
        
        let train_xmls = vec![
            TrainXML {
                prompts: Some(prompts),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_prompts(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.prompts.is_some());
        let merged_prompts = merged.prompts.unwrap();
        assert_eq!(merged_prompts.prompt.len(), 2);
        
        // Check both prompts are present
        let ids: Vec<String> = merged_prompts.prompt.iter().map(|p| p.id.clone()).collect();
        assert!(ids.contains(&"prompt1".to_string()));
        assert!(ids.contains(&"prompt2".to_string()));
        
        // Check content
        let p1 = merged_prompts.prompt.iter().find(|p| p.id == "prompt1").unwrap();
        assert_eq!(p1.content, "What is Rust?");
        
        let p2 = merged_prompts.prompt.iter().find(|p| p.id == "prompt2").unwrap();
        assert_eq!(p2.content, "What is Python?");
    }

    #[test]
    fn test_merge_prompts_two_files_unique_ids() {
        let prompt1 = create_prompt("prompt1", "Content from file 1");
        let prompt2 = create_prompt("prompt2", "Content from file 2");
        
        let prompts1 = create_prompts_section(vec![prompt1]);
        let prompts2 = create_prompts_section(vec![prompt2]);
        
        let train_xmls = vec![
            TrainXML {
                prompts: Some(prompts1),
                ..Default::default()
            },
            TrainXML {
                prompts: Some(prompts2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_prompts(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.prompts.is_some());
        let merged_prompts = merged.prompts.unwrap();
        assert_eq!(merged_prompts.prompt.len(), 2);
        
        let ids: Vec<String> = merged_prompts.prompt.iter().map(|p| p.id.clone()).collect();
        assert!(ids.contains(&"prompt1".to_string()));
        assert!(ids.contains(&"prompt2".to_string()));
    }

    #[test]
    fn test_merge_prompts_duplicate_id_error() {
        let prompt1 = create_prompt("duplicate", "First version");
        let prompt2 = create_prompt("duplicate", "Second version");
        
        let prompts1 = create_prompts_section(vec![prompt1]);
        let prompts2 = create_prompts_section(vec![prompt2]);
        
        let train_xmls = vec![
            TrainXML {
                prompts: Some(prompts1),
                ..Default::default()
            },
            TrainXML {
                prompts: Some(prompts2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_prompts(&train_xmls, &mut merged);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Duplicate prompt ID found: 'duplicate'");
    }

    #[test]
    fn test_merge_prompts_duplicate_id_in_same_file() {
        let prompt1 = create_prompt("duplicate", "First version");
        let prompt2 = create_prompt("duplicate", "Second version");
        
        let prompts = create_prompts_section(vec![prompt1, prompt2]);
        
        let train_xmls = vec![
            TrainXML {
                prompts: Some(prompts),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_prompts(&train_xmls, &mut merged);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Duplicate prompt ID found: 'duplicate'");
    }

    #[test]
    fn test_merge_prompts_three_files_unique_ids() {
        let prompt1 = create_prompt("prompt1", "File 1");
        let prompt2 = create_prompt("prompt2", "File 2");
        let prompt3 = create_prompt("prompt3", "File 3");
        
        let prompts1 = create_prompts_section(vec![prompt1]);
        let prompts2 = create_prompts_section(vec![prompt2]);
        let prompts3 = create_prompts_section(vec![prompt3]);
        
        let train_xmls = vec![
            TrainXML {
                prompts: Some(prompts1),
                ..Default::default()
            },
            TrainXML {
                prompts: Some(prompts2),
                ..Default::default()
            },
            TrainXML {
                prompts: Some(prompts3),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_prompts(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.prompts.is_some());
        let merged_prompts = merged.prompts.unwrap();
        assert_eq!(merged_prompts.prompt.len(), 3);
        
        // Order preserved
        assert_eq!(merged_prompts.prompt[0].id, "prompt1");
        assert_eq!(merged_prompts.prompt[1].id, "prompt2");
        assert_eq!(merged_prompts.prompt[2].id, "prompt3");
    }

    #[test]
    fn test_merge_prompts_append_to_existing() {
        let existing_prompt = create_prompt("existing", "Existing prompt");
        let existing_prompts = create_prompts_section(vec![existing_prompt]);
        
        let new_prompt = create_prompt("new", "New prompt");
        let new_prompts = create_prompts_section(vec![new_prompt]);
        
        let mut merged = TrainXML {
            prompts: Some(existing_prompts),
            ..Default::default()
        };
        
        let train_xmls = vec![
            TrainXML {
                prompts: Some(new_prompts),
                ..Default::default()
            },
        ];
        
        let result = train_xml_merge_prompts(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.prompts.is_some());
        let merged_prompts = merged.prompts.unwrap();
        assert_eq!(merged_prompts.prompt.len(), 2);
        assert_eq!(merged_prompts.prompt[0].id, "existing");
        assert_eq!(merged_prompts.prompt[1].id, "new");
    }

    #[test]
    fn test_merge_prompts_empty_files_skipped() {
        let prompt = create_prompt("prompt1", "Content");
        let prompts = create_prompts_section(vec![prompt]);
        
        let train_xmls = vec![
            TrainXML {
                prompts: Some(prompts),
                ..Default::default()
            },
            TrainXML {
                prompts: None,
                ..Default::default()
            },
            TrainXML {
                prompts: Some(create_prompts_section(vec![])),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_prompts(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.prompts.is_some());
        let merged_prompts = merged.prompts.unwrap();
        assert_eq!(merged_prompts.prompt.len(), 1);
        assert_eq!(merged_prompts.prompt[0].id, "prompt1");
    }

    #[test]
    fn test_merge_prompts_preserves_order_with_duplicate_check() {
        let prompt1 = create_prompt("prompt1", "First");
        let prompt2 = create_prompt("prompt2", "Second");
        let prompt3 = create_prompt("prompt3", "Third");
        let prompt4 = create_prompt("prompt4", "Fourth");
        
        let prompts1 = create_prompts_section(vec![prompt1, prompt2]);
        let prompts2 = create_prompts_section(vec![prompt3, prompt4]);
        
        let train_xmls = vec![
            TrainXML {
                prompts: Some(prompts1),
                ..Default::default()
            },
            TrainXML {
                prompts: Some(prompts2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_prompts(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.prompts.is_some());
        let merged_prompts = merged.prompts.unwrap();
        assert_eq!(merged_prompts.prompt.len(), 4);
        
        // Order preserved
        assert_eq!(merged_prompts.prompt[0].id, "prompt1");
        assert_eq!(merged_prompts.prompt[1].id, "prompt2");
        assert_eq!(merged_prompts.prompt[2].id, "prompt3");
        assert_eq!(merged_prompts.prompt[3].id, "prompt4");
    }
}
