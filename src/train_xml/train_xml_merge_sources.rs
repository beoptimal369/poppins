// src/train_xml/train_xml_merge_sources.rs

use crate::train_xml::{TrainXML, TrainXMLSources};


/// Merge sources from all train XML files into the target train_xml
///
/// Preserves order by iterating through train_xmls in order and appending sources as they're found.
/// If a source with the same ID is found twice, returns an error.
pub fn train_xml_merge_sources(
    train_xmls: &[TrainXML],
    train_xml: &mut TrainXML,
) -> Result<(), String> {
    let mut seen_ids = std::collections::HashSet::new();
    
    // First, collect all sources from existing train_xml
    let mut all_sources = Vec::new();
    
    // Add existing sources from train_xml first
    if let Some(existing_sources) = &train_xml.sources {
        for source in &existing_sources.source {
            if seen_ids.contains(&source.id) {
                return Err(format!("Duplicate source ID found in existing sources: '{}'", source.id));
            }
            seen_ids.insert(source.id.clone());
            all_sources.push(source.clone());
        }
    }
    
    // Then add sources from all train_xmls in order
    for xml in train_xmls {
        if let Some(source_sources) = &xml.sources {
            for source in &source_sources.source {
                // Check for duplicate ID
                if seen_ids.contains(&source.id) {
                    return Err(format!("Duplicate source ID found: '{}'", source.id));
                }
                
                seen_ids.insert(source.id.clone());
                all_sources.push(source.clone());
            }
        }
    }
    
    // Only set sources if we have any
    if !all_sources.is_empty() {
        train_xml.sources = Some(TrainXMLSources {
            source: all_sources,
        });
    } else {
        // No sources at all - ensure it's None
        train_xml.sources = None;
    }
    
    Ok(())
}



#[cfg(test)]
mod tests {
    use crate::train_xml::{
        TrainXML,
        TrainXMLSources,
        TrainXMLSourcesSource,
        train_xml_merge_sources,
    };

    fn create_source(id: &str, url: &str, title: Option<&str>) -> TrainXMLSourcesSource {
        TrainXMLSourcesSource {
            id: id.to_string(),
            url: url.to_string(),
            title: title.map(|s| s.to_string()),
        }
    }

    fn create_sources_section(sources: Vec<TrainXMLSourcesSource>) -> TrainXMLSources {
        TrainXMLSources {
            source: sources,
        }
    }

    #[test]
    fn test_merge_sources_no_sources() {
        let train_xmls = vec![
            TrainXML::default(),
            TrainXML::default(),
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_sources(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.sources.is_none());
    }

    #[test]
    fn test_merge_sources_single_file() {
        let source1 = create_source("src1", "https://example.com/1", None);
        let source2 = create_source("src2", "https://example.com/2", Some("Title 2"));
        
        let sources = create_sources_section(vec![source1.clone(), source2.clone()]);
        
        let train_xmls = vec![
            TrainXML {
                sources: Some(sources),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_sources(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.sources.is_some());
        let merged_sources = merged.sources.unwrap();
        assert_eq!(merged_sources.source.len(), 2);
        
        // Check both sources are present
        let ids: Vec<String> = merged_sources.source.iter().map(|s| s.id.clone()).collect();
        assert!(ids.contains(&"src1".to_string()));
        assert!(ids.contains(&"src2".to_string()));
        
        // Check content
        let src1 = merged_sources.source.iter().find(|s| s.id == "src1").unwrap();
        assert_eq!(src1.url, "https://example.com/1");
        assert_eq!(src1.title, None);
        
        let src2 = merged_sources.source.iter().find(|s| s.id == "src2").unwrap();
        assert_eq!(src2.url, "https://example.com/2");
        assert_eq!(src2.title, Some("Title 2".to_string()));
    }

    #[test]
    fn test_merge_sources_two_files_unique_ids() {
        let source1 = create_source("src1", "https://example.com/1", None);
        let source2 = create_source("src2", "https://example.com/2", None);
        
        let sources1 = create_sources_section(vec![source1]);
        let sources2 = create_sources_section(vec![source2]);
        
        let train_xmls = vec![
            TrainXML {
                sources: Some(sources1),
                ..Default::default()
            },
            TrainXML {
                sources: Some(sources2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_sources(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.sources.is_some());
        let merged_sources = merged.sources.unwrap();
        assert_eq!(merged_sources.source.len(), 2);
        
        let ids: Vec<String> = merged_sources.source.iter().map(|s| s.id.clone()).collect();
        assert!(ids.contains(&"src1".to_string()));
        assert!(ids.contains(&"src2".to_string()));
    }

    #[test]
    fn test_merge_sources_duplicate_id_error() {
        let source1 = create_source("duplicate", "https://example.com/1", None);
        let source2 = create_source("duplicate", "https://example.com/2", None);
        
        let sources1 = create_sources_section(vec![source1]);
        let sources2 = create_sources_section(vec![source2]);
        
        let train_xmls = vec![
            TrainXML {
                sources: Some(sources1),
                ..Default::default()
            },
            TrainXML {
                sources: Some(sources2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_sources(&train_xmls, &mut merged);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Duplicate source ID found: 'duplicate'");
    }

    #[test]
    fn test_merge_sources_duplicate_id_in_same_file() {
        let source1 = create_source("duplicate", "https://example.com/1", None);
        let source2 = create_source("duplicate", "https://example.com/2", None);
        
        let sources = create_sources_section(vec![source1, source2]);
        
        let train_xmls = vec![
            TrainXML {
                sources: Some(sources),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_sources(&train_xmls, &mut merged);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Duplicate source ID found: 'duplicate'");
    }

    #[test]
    fn test_merge_sources_three_files_unique_ids() {
        let source1 = create_source("src1", "https://example.com/1", None);
        let source2 = create_source("src2", "https://example.com/2", None);
        let source3 = create_source("src3", "https://example.com/3", None);
        
        let sources1 = create_sources_section(vec![source1]);
        let sources2 = create_sources_section(vec![source2]);
        let sources3 = create_sources_section(vec![source3]);
        
        let train_xmls = vec![
            TrainXML {
                sources: Some(sources1),
                ..Default::default()
            },
            TrainXML {
                sources: Some(sources2),
                ..Default::default()
            },
            TrainXML {
                sources: Some(sources3),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_sources(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.sources.is_some());
        let merged_sources = merged.sources.unwrap();
        assert_eq!(merged_sources.source.len(), 3);
        
        // Order preserved
        assert_eq!(merged_sources.source[0].id, "src1");
        assert_eq!(merged_sources.source[1].id, "src2");
        assert_eq!(merged_sources.source[2].id, "src3");
    }

    #[test]
    fn test_merge_sources_append_to_existing() {
        let existing_source = create_source("existing", "https://example.com/existing", None);
        let existing_sources = create_sources_section(vec![existing_source]);
        
        let new_source = create_source("new", "https://example.com/new", None);
        let new_sources = create_sources_section(vec![new_source]);
        
        let mut merged = TrainXML {
            sources: Some(existing_sources),
            ..Default::default()
        };
        
        let train_xmls = vec![
            TrainXML {
                sources: Some(new_sources),
                ..Default::default()
            },
        ];
        
        let result = train_xml_merge_sources(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.sources.is_some());
        let merged_sources = merged.sources.unwrap();
        assert_eq!(merged_sources.source.len(), 2);
        assert_eq!(merged_sources.source[0].id, "existing");
        assert_eq!(merged_sources.source[1].id, "new");
    }

    #[test]
    fn test_merge_sources_empty_files_skipped() {
        let source = create_source("src1", "https://example.com/1", None);
        let sources = create_sources_section(vec![source]);
        
        let train_xmls = vec![
            TrainXML {
                sources: Some(sources),
                ..Default::default()
            },
            TrainXML {
                sources: None,
                ..Default::default()
            },
            TrainXML {
                sources: Some(create_sources_section(vec![])),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_sources(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.sources.is_some());
        let merged_sources = merged.sources.unwrap();
        assert_eq!(merged_sources.source.len(), 1);
        assert_eq!(merged_sources.source[0].id, "src1");
    }

    #[test]
    fn test_merge_sources_preserves_order_with_duplicate_check() {
        let source1 = create_source("src1", "https://example.com/1", None);
        let source2 = create_source("src2", "https://example.com/2", None);
        let source3 = create_source("src3", "https://example.com/3", None);
        let source4 = create_source("src4", "https://example.com/4", None);
        
        let sources1 = create_sources_section(vec![source1, source2]);
        let sources2 = create_sources_section(vec![source3, source4]);
        
        let train_xmls = vec![
            TrainXML {
                sources: Some(sources1),
                ..Default::default()
            },
            TrainXML {
                sources: Some(sources2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_sources(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.sources.is_some());
        let merged_sources = merged.sources.unwrap();
        assert_eq!(merged_sources.source.len(), 4);
        
        // Order preserved
        assert_eq!(merged_sources.source[0].id, "src1");
        assert_eq!(merged_sources.source[1].id, "src2");
        assert_eq!(merged_sources.source[2].id, "src3");
        assert_eq!(merged_sources.source[3].id, "src4");
    }

    #[test]
    fn test_merge_sources_with_titles_and_duplicate_check() {
        let source1 = create_source("src1", "https://example.com/1", Some("First Title"));
        let source2 = create_source("src2", "https://example.com/2", None);
        
        let sources1 = create_sources_section(vec![source1]);
        let sources2 = create_sources_section(vec![source2]);
        
        let train_xmls = vec![
            TrainXML {
                sources: Some(sources1),
                ..Default::default()
            },
            TrainXML {
                sources: Some(sources2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_sources(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.sources.is_some());
        let merged_sources = merged.sources.unwrap();
        assert_eq!(merged_sources.source.len(), 2);
        
        assert_eq!(merged_sources.source[0].id, "src1");
        assert_eq!(merged_sources.source[0].title, Some("First Title".to_string()));
        assert_eq!(merged_sources.source[1].id, "src2");
        assert_eq!(merged_sources.source[1].title, None);
    }
}
