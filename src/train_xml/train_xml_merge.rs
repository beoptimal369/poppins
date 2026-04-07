// src/train_xml/train_xml_merge.rs

use crate::train_xml::{
    TrainXML,
    train_xml_merge_constants,
    train_xml_merge_system_prompts,
    train_xml_merge_prompts,
    train_xml_merge_thoughts,
    train_xml_merge_responses,
    train_xml_merge_sources,
    train_xml_merge_code_snippets,
    train_xml_merge_samples,
    train_xml_merge_phrases,
    train_xml_merge_beyond_scope,
};


/// Merge multiple TrainXML documents into a single TrainXML struct
///
/// Priority order (first in vector = highest priority):
/// - train_xmls[0] (main file) has highest priority
/// - Later imports have lower priority
///
/// # Arguments
/// * `train_xmls` - Vector of TrainXML documents to merge (must have at least one element)
///
/// # Returns
/// * `Result<TrainXML, String>` - Merged TrainXML struct or error if duplicate IDs found
pub fn train_xml_merge(train_xmls: Vec<TrainXML>) -> Result<TrainXML, String> {
    if train_xmls.is_empty() {
        return Err("No train XML files to merge".to_string());
    }
    
    if train_xmls.len() == 1 {
        return Ok(train_xmls.into_iter().next().unwrap());
    }
    
    // Start with an empty TrainXML
    let mut merged = TrainXML::default();
    
    // Merge each section
    train_xml_merge_constants(&train_xmls, &mut merged);
    train_xml_merge_system_prompts(&train_xmls, &mut merged)?;
    train_xml_merge_prompts(&train_xmls, &mut merged)?;
    train_xml_merge_thoughts(&train_xmls, &mut merged)?;
    train_xml_merge_responses(&train_xmls, &mut merged)?;
    train_xml_merge_sources(&train_xmls, &mut merged)?;
    train_xml_merge_code_snippets(&train_xmls, &mut merged);
    train_xml_merge_samples(&train_xmls, &mut merged);
    train_xml_merge_phrases(&train_xmls, &mut merged);
    train_xml_merge_beyond_scope(&train_xmls, &mut merged);
    
    Ok(merged)
}



#[cfg(test)]
mod tests {
    use crate::train_xml::{
        TrainXML,
        train_xml_merge,
        TrainXMLResponses,
        TrainXMLPromptsPrompt,
        TrainXMLResponsesResponse,
    };

    fn create_prompt(id: &str, content: &str) -> TrainXMLPromptsPrompt {
        TrainXMLPromptsPrompt {
            id: id.to_string(),
            content: content.to_string(),
        }
    }

    fn create_response(id: &str, content: &str) -> TrainXMLResponsesResponse {
        TrainXMLResponsesResponse {
            id: id.to_string(),
            content: content.to_string(),
        }
    }

    #[test]
    fn test_merge_empty_vec_error() {
        let result = train_xml_merge(vec![]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No train XML files to merge");
    }

    #[test]
    fn test_merge_single_file() {
        let train_xml = TrainXML::default();
        let result = train_xml_merge(vec![train_xml]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_merge_two_files_without_conflicts() {
        let mut train_xml1 = TrainXML::default();
        let mut train_xml2 = TrainXML::default();
        
        train_xml1.prompts = Some(crate::train_xml::TrainXMLPrompts {
            prompt: vec![create_prompt("prompt1", "Content 1")],
        });
        
        train_xml2.responses = Some(TrainXMLResponses {
            response: vec![create_response("response1", "Response 1")],
        });
        
        let result = train_xml_merge(vec![train_xml1, train_xml2]);
        assert!(result.is_ok());
        
        let merged = result.unwrap();
        assert!(merged.prompts.is_some());
        assert!(merged.responses.is_some());
    }

    #[test]
    fn test_merge_two_files_with_duplicate_prompts_error() {
        let mut train_xml1 = TrainXML::default();
        let mut train_xml2 = TrainXML::default();
        
        train_xml1.prompts = Some(crate::train_xml::TrainXMLPrompts {
            prompt: vec![create_prompt("duplicate", "First version")],
        });
        
        train_xml2.prompts = Some(crate::train_xml::TrainXMLPrompts {
            prompt: vec![create_prompt("duplicate", "Second version")],
        });
        
        let result = train_xml_merge(vec![train_xml1, train_xml2]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Duplicate prompt ID found: 'duplicate'"));
    }

    #[test]
    fn test_merge_two_files_with_duplicate_responses_error() {
        let mut train_xml1 = TrainXML::default();
        let mut train_xml2 = TrainXML::default();
        
        train_xml1.responses = Some(TrainXMLResponses {
            response: vec![create_response("duplicate", "First version")],
        });
        
        train_xml2.responses = Some(TrainXMLResponses {
            response: vec![create_response("duplicate", "Second version")],
        });
        
        let result = train_xml_merge(vec![train_xml1, train_xml2]);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Duplicate response ID found: 'duplicate'"));
    }
}