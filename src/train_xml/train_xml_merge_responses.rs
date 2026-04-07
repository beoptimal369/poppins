// src/train_xml/train_xml_merge_responses.rs

use crate::train_xml::{TrainXML, TrainXMLResponses};


/// Merge responses from all train XML files into the target train_xml
///
/// Preserves order by iterating through train_xmls in order and appending responses as they're found.
/// If a response with the same ID is found twice, returns an error.
pub fn train_xml_merge_responses(
    train_xmls: &[TrainXML],
    train_xml: &mut TrainXML,
) -> Result<(), String> {
    let mut seen_ids = std::collections::HashSet::new();
    
    // First, collect all responses from existing train_xml
    let mut all_responses = Vec::new();
    
    // Add existing responses from train_xml first
    if let Some(existing_responses) = &train_xml.responses {
        for response in &existing_responses.response {
            if seen_ids.contains(&response.id) {
                return Err(format!("Duplicate response ID found in existing responses: '{}'", response.id));
            }
            seen_ids.insert(response.id.clone());
            all_responses.push(response.clone());
        }
    }
    
    // Then add responses from all train_xmls in order
    for xml in train_xmls {
        if let Some(source_responses) = &xml.responses {
            for response in &source_responses.response {
                // Check for duplicate ID
                if seen_ids.contains(&response.id) {
                    return Err(format!("Duplicate response ID found: '{}'", response.id));
                }
                
                seen_ids.insert(response.id.clone());
                all_responses.push(response.clone());
            }
        }
    }
    
    // Only set responses if we have any
    if !all_responses.is_empty() {
        train_xml.responses = Some(TrainXMLResponses {
            response: all_responses,
        });
    } else {
        // No responses at all - ensure it's None
        train_xml.responses = None;
    }
    
    Ok(())
}



#[cfg(test)]
mod tests {
    use crate::train_xml::{
        TrainXML,
        TrainXMLResponses,
        TrainXMLResponsesResponse,
        train_xml_merge_responses,
    };

    fn create_response(id: &str, content: &str) -> TrainXMLResponsesResponse {
        TrainXMLResponsesResponse {
            id: id.to_string(),
            content: content.to_string(),
        }
    }

    fn create_responses_section(responses: Vec<TrainXMLResponsesResponse>) -> TrainXMLResponses {
        TrainXMLResponses {
            response: responses,
        }
    }

    #[test]
    fn test_merge_responses_no_responses() {
        let train_xmls = vec![
            TrainXML::default(),
            TrainXML::default(),
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_responses(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.responses.is_none());
    }

    #[test]
    fn test_merge_responses_single_file() {
        let response1 = create_response("resp1", "Response content 1");
        let response2 = create_response("resp2", "Response content 2");
        
        let responses = create_responses_section(vec![response1.clone(), response2.clone()]);
        
        let train_xmls = vec![
            TrainXML {
                responses: Some(responses),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_responses(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.responses.is_some());
        let merged_responses = merged.responses.unwrap();
        assert_eq!(merged_responses.response.len(), 2);
        
        // Check both responses are present
        let ids: Vec<String> = merged_responses.response.iter().map(|r| r.id.clone()).collect();
        assert!(ids.contains(&"resp1".to_string()));
        assert!(ids.contains(&"resp2".to_string()));
        
        // Check content
        let resp1 = merged_responses.response.iter().find(|r| r.id == "resp1").unwrap();
        assert_eq!(resp1.content, "Response content 1");
        
        let resp2 = merged_responses.response.iter().find(|r| r.id == "resp2").unwrap();
        assert_eq!(resp2.content, "Response content 2");
    }

    #[test]
    fn test_merge_responses_two_files_unique_ids() {
        let response1 = create_response("resp1", "Response from file 1");
        let response2 = create_response("resp2", "Response from file 2");
        
        let responses1 = create_responses_section(vec![response1]);
        let responses2 = create_responses_section(vec![response2]);
        
        let train_xmls = vec![
            TrainXML {
                responses: Some(responses1),
                ..Default::default()
            },
            TrainXML {
                responses: Some(responses2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_responses(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.responses.is_some());
        let merged_responses = merged.responses.unwrap();
        assert_eq!(merged_responses.response.len(), 2);
        
        let ids: Vec<String> = merged_responses.response.iter().map(|r| r.id.clone()).collect();
        assert!(ids.contains(&"resp1".to_string()));
        assert!(ids.contains(&"resp2".to_string()));
    }

    #[test]
    fn test_merge_responses_duplicate_id_error() {
        let response1 = create_response("duplicate", "First version");
        let response2 = create_response("duplicate", "Second version");
        
        let responses1 = create_responses_section(vec![response1]);
        let responses2 = create_responses_section(vec![response2]);
        
        let train_xmls = vec![
            TrainXML {
                responses: Some(responses1),
                ..Default::default()
            },
            TrainXML {
                responses: Some(responses2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_responses(&train_xmls, &mut merged);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Duplicate response ID found: 'duplicate'");
    }

    #[test]
    fn test_merge_responses_duplicate_id_in_same_file() {
        let response1 = create_response("duplicate", "First version");
        let response2 = create_response("duplicate", "Second version");
        
        let responses = create_responses_section(vec![response1, response2]);
        
        let train_xmls = vec![
            TrainXML {
                responses: Some(responses),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_responses(&train_xmls, &mut merged);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Duplicate response ID found: 'duplicate'");
    }

    #[test]
    fn test_merge_responses_three_files_unique_ids() {
        let response1 = create_response("resp1", "File 1");
        let response2 = create_response("resp2", "File 2");
        let response3 = create_response("resp3", "File 3");
        
        let responses1 = create_responses_section(vec![response1]);
        let responses2 = create_responses_section(vec![response2]);
        let responses3 = create_responses_section(vec![response3]);
        
        let train_xmls = vec![
            TrainXML {
                responses: Some(responses1),
                ..Default::default()
            },
            TrainXML {
                responses: Some(responses2),
                ..Default::default()
            },
            TrainXML {
                responses: Some(responses3),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_responses(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.responses.is_some());
        let merged_responses = merged.responses.unwrap();
        assert_eq!(merged_responses.response.len(), 3);
        
        // Order preserved
        assert_eq!(merged_responses.response[0].id, "resp1");
        assert_eq!(merged_responses.response[1].id, "resp2");
        assert_eq!(merged_responses.response[2].id, "resp3");
    }

    #[test]
    fn test_merge_responses_append_to_existing() {
        let existing_response = create_response("existing", "Existing response");
        let existing_responses = create_responses_section(vec![existing_response]);
        
        let new_response = create_response("new", "New response");
        let new_responses = create_responses_section(vec![new_response]);
        
        let mut merged = TrainXML {
            responses: Some(existing_responses),
            ..Default::default()
        };
        
        let train_xmls = vec![
            TrainXML {
                responses: Some(new_responses),
                ..Default::default()
            },
        ];
        
        let result = train_xml_merge_responses(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.responses.is_some());
        let merged_responses = merged.responses.unwrap();
        assert_eq!(merged_responses.response.len(), 2);
        assert_eq!(merged_responses.response[0].id, "existing");
        assert_eq!(merged_responses.response[1].id, "new");
    }

    #[test]
    fn test_merge_responses_empty_files_skipped() {
        let response = create_response("resp1", "Content");
        let responses = create_responses_section(vec![response]);
        
        let train_xmls = vec![
            TrainXML {
                responses: Some(responses),
                ..Default::default()
            },
            TrainXML {
                responses: None,
                ..Default::default()
            },
            TrainXML {
                responses: Some(create_responses_section(vec![])),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_responses(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.responses.is_some());
        let merged_responses = merged.responses.unwrap();
        assert_eq!(merged_responses.response.len(), 1);
        assert_eq!(merged_responses.response[0].id, "resp1");
    }

    #[test]
    fn test_merge_responses_preserves_order_with_duplicate_check() {
        let response1 = create_response("resp1", "First");
        let response2 = create_response("resp2", "Second");
        let response3 = create_response("resp3", "Third");
        let response4 = create_response("resp4", "Fourth");
        
        let responses1 = create_responses_section(vec![response1, response2]);
        let responses2 = create_responses_section(vec![response3, response4]);
        
        let train_xmls = vec![
            TrainXML {
                responses: Some(responses1),
                ..Default::default()
            },
            TrainXML {
                responses: Some(responses2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        let result = train_xml_merge_responses(&train_xmls, &mut merged);
        
        assert!(result.is_ok());
        assert!(merged.responses.is_some());
        let merged_responses = merged.responses.unwrap();
        assert_eq!(merged_responses.response.len(), 4);
        
        // Order preserved
        assert_eq!(merged_responses.response[0].id, "resp1");
        assert_eq!(merged_responses.response[1].id, "resp2");
        assert_eq!(merged_responses.response[2].id, "resp3");
        assert_eq!(merged_responses.response[3].id, "resp4");
    }
}
