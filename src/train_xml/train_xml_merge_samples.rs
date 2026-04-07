// src/train_xml/train_xml_merge_samples.rs

use crate::train_xml::{
    TrainXML, 
    TrainXMLSamples, 
    TrainXMLSamplesSample, 
    TrainXMLSamplesSampleChildren,
    TrainXMLSamplesSystem,
};


/// Merge samples from all train XML files into the target train_xml
///
/// Preserves order by iterating through train_xmls in order and appending samples as they're found.
/// For imported files, injects the system prompt specified in the root train.xml's import element.
pub fn train_xml_merge_samples(
    train_xmls: &[TrainXML],
    train_xml: &mut TrainXML,
) {
    // First, build a map of import paths to their system prompt IDs
    let mut import_system_map = std::collections::HashMap::new();
    
    // Get the root train.xml (index 0) which contains the imports
    if let Some(root) = train_xmls.first() {
        if let Some(imports) = &root.imports {
            for import in &imports.import {
                if let Some(system_id) = &import.system {
                    import_system_map.insert(import.path.clone(), system_id.clone());
                }
            }
        }
    }
    
    // Track if we have any samples at all
    let mut has_sample_ids = false;
    let mut has_samples = false;
    
    // First, collect existing samples from train_xml
    let mut all_sample_ids = Vec::new();
    let mut all_samples = Vec::new();
    
    // Add existing sample-ids from train_xml first
    if let Some(existing_samples) = &train_xml.samples {
        if let Some(sample_ids) = &existing_samples.sample_ids {
            has_sample_ids = true;
            all_sample_ids.extend(sample_ids.iter().cloned());
        }
        if let Some(samples_list) = &existing_samples.sample {
            has_samples = true;
            all_samples.extend(samples_list.iter().cloned());
        }
    }
    
    // Then add samples from ALL train_xmls (including root, preserving order)
    for (idx, xml) in train_xmls.iter().enumerate() {
        // For imports (idx > 0), check if they need system prompt injection
        let import_system_id = if idx > 0 {
            if let Some(root) = train_xmls.first() {
                if let Some(imports) = &root.imports {
                    imports.import.get(idx - 1).and_then(|imp| {
                        import_system_map.get(&imp.path).cloned()
                    })
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None  // Root samples keep their original system prompts
        };
        
        if let Some(source_samples) = &xml.samples {
            // Handle sample-ids
            if let Some(sample_ids) = &source_samples.sample_ids {
                for mut sample_id in sample_ids.iter().cloned() {
                    // Inject system prompt only for imported files
                    if let Some(system_id) = &import_system_id {
                        sample_id.system = Some(system_id.clone());
                    }
                    all_sample_ids.push(sample_id);
                    has_sample_ids = true;
                }
            }
            
            // Handle samples with tags
            if let Some(samples_list) = &source_samples.sample {
                for sample in samples_list {
                    let processed_sample = if let Some(system_id) = &import_system_id {
                        inject_system_prompt(sample, system_id)
                    } else {
                        sample.clone()  // Root samples pass through unchanged
                    };
                    all_samples.push(processed_sample);
                    has_samples = true;
                }
            }
        }
    }
    
    // Only set samples if we have any
    if has_sample_ids || has_samples {
        train_xml.samples = Some(TrainXMLSamples {
            sample_ids: if has_sample_ids { Some(all_sample_ids) } else { None },
            sample: if has_samples { Some(all_samples) } else { None },
        });
    } else {
        train_xml.samples = None;
    }
}

/// Inject a system prompt into a sample (industry standard: system BEFORE prompt)
/// - Removes any existing <system> tags
/// - Adds a new <system> tag at the BEGINNING
fn inject_system_prompt(
    sample: &TrainXMLSamplesSample,
    system_id: &str,
) -> TrainXMLSamplesSample {
    let mut new_children = Vec::new();
    
    // Collect all children except existing system tags
    for child in &sample.children {
        if !matches!(child, TrainXMLSamplesSampleChildren::System(_)) {
            new_children.push(child.clone());
        }
    }
    
    // Insert system tag at the BEGINNING (industry standard)
    new_children.insert(0, TrainXMLSamplesSampleChildren::System(TrainXMLSamplesSystem {
        id: system_id.to_string(),
    }));
    
    TrainXMLSamplesSample {
        children: new_children,
    }
}



#[cfg(test)]
mod tests {
    use crate::train_xml::{
        TrainXML,
        TrainXMLImports,
        TrainXMLSamples,
        TrainXMLSamplesSystem,
        TrainXMLSamplesSample,
        TrainXMLSamplesPrompt,
        TrainXMLImportsImport,
        TrainXMLSystemPrompts,
        train_xml_merge_samples,
        TrainXMLSamplesSampleIds,
        TrainXMLSystemPromptsSystem,
        TrainXMLSamplesSampleChildren,
    };

    fn create_sample_id(prompt_id: &str) -> TrainXMLSamplesSampleIds {
        TrainXMLSamplesSampleIds {
            system: None,
            prompt: prompt_id.to_string(),
            thought: None,
            response: None,
            source: None,
            code: None,
        }
    }

    fn create_sample_with_system(prompt_id: &str, system_id: Option<&str>) -> TrainXMLSamplesSample {
        let mut children = vec![
            TrainXMLSamplesSampleChildren::Prompt(TrainXMLSamplesPrompt {
                id: prompt_id.to_string(),
            }),
        ];
        
        if let Some(sid) = system_id {
            children.insert(1, TrainXMLSamplesSampleChildren::System(TrainXMLSamplesSystem {
                id: sid.to_string(),
            }));
        }
        
        TrainXMLSamplesSample { children }
    }

    fn create_root_train_xml_with_imports() -> TrainXML {
        TrainXML {
            imports: Some(TrainXMLImports {
                import: vec![
                    TrainXMLImportsImport {
                        path: "math.xml".to_string(),
                        system: Some("sy::be::default".to_string()),
                    },
                    TrainXMLImportsImport {
                        path: "english.xml".to_string(),
                        system: Some("sy::be::default".to_string()),
                    },
                ],
            }),
            system_prompts: Some(TrainXMLSystemPrompts {
                system: vec![
                    TrainXMLSystemPromptsSystem {
                        id: "sy::be::default".to_string(),
                        content: "You are a helpful assistant.".to_string(),
                    },
                ],
            }),
            ..Default::default()
        }
    }

    #[test]
    fn test_merge_samples_injects_system_prompt_into_sample_ids() {
        let root = create_root_train_xml_with_imports();
        
        let imported_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: Some(vec![
                    create_sample_id("pr::be::mathematics"),
                    create_sample_id("pr::be::math"),
                ]),
                sample: None,
            }),
            ..Default::default()
        };
        
        let train_xmls = vec![root, imported_xml];
        let mut merged = TrainXML::default();
        
        train_xml_merge_samples(&train_xmls, &mut merged);
        
        assert!(merged.samples.is_some());
        let merged_samples = merged.samples.unwrap();
        let sample_ids = merged_samples.sample_ids.unwrap();
        
        // Both sample-ids should have system attribute set
        assert_eq!(sample_ids.len(), 2);
        assert_eq!(sample_ids[0].system, Some("sy::be::default".to_string()));
        assert_eq!(sample_ids[1].system, Some("sy::be::default".to_string()));
    }

    #[test]
    fn test_merge_samples_injects_system_prompt_into_samples() {
        let root = create_root_train_xml_with_imports();
        
        let imported_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    create_sample_with_system("pr::be::mathematics", None),
                    create_sample_with_system("pr::be::math", None),
                ]),
            }),
            ..Default::default()
        };
        
        let train_xmls = vec![root, imported_xml];
        let mut merged = TrainXML::default();
        
        train_xml_merge_samples(&train_xmls, &mut merged);
        
        assert!(merged.samples.is_some());
        let merged_samples = merged.samples.unwrap();
        let samples = merged_samples.sample.unwrap();
        
        // Both samples should have system tag injected
        assert_eq!(samples.len(), 2);
        
        for sample in samples {
            let has_system = sample.children.iter().any(|child| {
                matches!(child, TrainXMLSamplesSampleChildren::System(_))
            });
            assert!(has_system, "Sample should have system tag injected");
            
            // Industry standard: System tag should be at index 0 (BEFORE prompt)
            if let Some(TrainXMLSamplesSampleChildren::System(system)) = sample.children.get(0) {
                assert_eq!(system.id, "sy::be::default");
            } else {
                panic!("System tag not at expected position (index 0)");
            }
            
            // Verify prompt is at index 1 (after system)
            if let Some(TrainXMLSamplesSampleChildren::Prompt(_)) = sample.children.get(1) {
                // Good - prompt follows system
            } else {
                panic!("Prompt should be at index 1 after system tag");
            }
        }
    }

    #[test]
    fn test_merge_samples_removes_existing_system_tags() {
        let root = create_root_train_xml_with_imports();
        
        let imported_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    create_sample_with_system("pr::be::mathematics", Some("old::system")),
                ]),
            }),
            ..Default::default()
        };
        
        let train_xmls = vec![root, imported_xml];
        let mut merged = TrainXML::default();
        
        train_xml_merge_samples(&train_xmls, &mut merged);
        
        assert!(merged.samples.is_some());
        let merged_samples = merged.samples.unwrap();
        let samples = merged_samples.sample.unwrap();
        
        let sample = &samples[0];
        
        // Should have exactly one system tag (the new one)
        let system_tags: Vec<_> = sample.children.iter()
            .filter(|child| matches!(child, TrainXMLSamplesSampleChildren::System(_)))
            .collect();
        
        assert_eq!(system_tags.len(), 1, "Should have exactly one system tag");
        
        // New system tag should be at index 0 (industry standard)
        if let Some(TrainXMLSamplesSampleChildren::System(system)) = sample.children.get(0) {
            assert_eq!(system.id, "sy::be::default");
        } else {
            panic!("System tag should be at index 0");
        }
        
        // Old system tag should be removed
        let old_system_exists = sample.children.iter().any(|child| {
            if let TrainXMLSamplesSampleChildren::System(system) = child {
                system.id == "old::system"
            } else {
                false
            }
        });
        assert!(!old_system_exists, "Old system tag should have been removed");
    }
}
