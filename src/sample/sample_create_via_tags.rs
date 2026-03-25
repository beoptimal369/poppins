// src/sample/sample_create_via_tags.rs

use crate::train_xml::{
    TrainXMLIdMaps,
    TrainXMLSamplesSample,
    TrainXMLSamplesSampleChildren,
};
use crate::sample::{
    Sample,
    SampleCode,
    SampleAiEnum,
    SampleIndent,
    SampleLanguage,
    SampleLineBreak,
    SamplePromptEnum,
};


/// Create a Sample via its children xml tags
///
/// # Arguments
/// * `sample_tags` - The <sample> element from train.xml containing child tags
/// * `ids` - The parsed IDs container with all prompts, responses, sources, and code snippets
///
/// # Returns
/// * `Option<Sample>` - The constructed sample with a unique ID, or None if required references are missing
///
/// # Notes
/// * The sample ID is automatically assigned using samples.next_id()
/// * The sample is NOT automatically added to train/val vectors - that's handled separately
pub fn sample_create_via_tags(
    sample_tags: &TrainXMLSamplesSample,
    ids: &TrainXMLIdMaps,
) -> Option<Sample> {
    // Get the prompt (required)
    let prompt = ids.prompts.get(&sample_tags.prompt.id)?;
    
    // Build prompt section
    let mut prompt_section = Vec::new();
    prompt_section.push(SamplePromptEnum::Text(prompt.content.clone()));
    
    // Build AI section
    let mut ai_section = Vec::new();
    
    // Process children in the order they appear in XML
    for child in &sample_tags.children {
        match child {
            TrainXMLSamplesSampleChildren::Response(response_ref) => {
                if let Some(response) = ids.responses.get(&response_ref.id) {
                    ai_section.push(SampleAiEnum::Text(response.content.clone()));
                }
            },
            
            TrainXMLSamplesSampleChildren::Source(source_ref) => {
                if let Some(_source) = ids.sources.get(&source_ref.id) {
                    ai_section.push(SampleAiEnum::Source(source_ref.id.clone()));
                }
            },
            
            TrainXMLSamplesSampleChildren::Code(code_ref) => {
                if let Some(code) = ids.code_snippets.get(&code_ref.id) {
                    let lang = SampleLanguage::from_str(&code.lang);
                    let indent = code_ref.indent.as_ref().copied().unwrap_or(SampleIndent::Zero);
                    
                    ai_section.push(SampleAiEnum::Code(SampleCode {
                        lang,
                        inline: code_ref.inline.unwrap_or(false),
                        indent,
                        content: code.content.clone(),
                    }));
                }
            },
            
            TrainXMLSamplesSampleChildren::ResponseIds(response_id_ref) => {
                // Add the response first
                if let Some(response) = ids.responses.get(&response_id_ref.response) {
                    ai_section.push(SampleAiEnum::Text(response.content.clone()));
                }
                
                // Then add the source if present
                if let Some(source_id) = &response_id_ref.source {
                    if let Some(_source) = ids.sources.get(source_id) {
                        ai_section.push(SampleAiEnum::Source(source_id.to_string()));
                    }
                }
            },
            
            TrainXMLSamplesSampleChildren::LineBreak(line_break) => {
                ai_section.push(SampleAiEnum::LineBreak(SampleLineBreak { count: line_break.count }));
            },
        }
    }
    
    // Only create sample if we have at least something in AI section
    if ai_section.is_empty() {
        None
    } else {
        Some(Sample {
            prompt_section,
            ai_section,
        })
    }
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::sample::{
        SampleAiEnum,
        sample_create_via_tags,
    };
    use crate::train_xml::{
        TrainXMLIdMaps,
        TrainXMLLineBreak,
        TrainXMLSamplesCode,
        TrainXMLSamplesPrompt,
        TrainXMLSamplesSource,
        TrainXMLSourcesSource,
        TrainXMLSamplesSample,
        TrainXMLPromptsPrompt,
        TrainXMLSamplesResponse,
        TrainXMLCodeSnippetsCode,
        TrainXMLResponsesResponse,
        TrainXMLSamplesResponseIds,
        TrainXMLSamplesSampleChildren,
    };

    fn create_test_ids() -> TrainXMLIdMaps<'static> {
        // Box and leak the data to get 'static references
        let prompts_data = Box::new(TrainXMLPromptsPrompt {
            id: "1".to_string(),
            content: "What is a computer network?".to_string(),
        });
        
        let responses_data = Box::new(TrainXMLResponsesResponse {
            id: "1".to_string(),
            content: "A computer network is group of communicating computers.".to_string(),
        });
        
        let responses_data_2 = Box::new(TrainXMLResponsesResponse {
            id: "2".to_string(),
            content: "Additional response about networks.".to_string(),
        });
        
        let sources_data = Box::new(TrainXMLSourcesSource {
            id: "1".to_string(),
            url: "https://example.com/1".to_string(),
            title: None,
        });
        
        let code_data = Box::new(TrainXMLCodeSnippetsCode {
            id: "1".to_string(),
            lang: "rust".to_string(),
            content: "fn main() {}".to_string(),
        });
        
        // Leak the boxes to get 'static references
        let prompts_ref: &'static TrainXMLPromptsPrompt = Box::leak(prompts_data);
        let responses_ref: &'static TrainXMLResponsesResponse = Box::leak(responses_data);
        let responses_ref_2: &'static TrainXMLResponsesResponse = Box::leak(responses_data_2);
        let sources_ref: &'static TrainXMLSourcesSource = Box::leak(sources_data);
        let code_ref: &'static TrainXMLCodeSnippetsCode = Box::leak(code_data);
        
        let mut prompts = HashMap::new();
        prompts.insert("1".to_string(), prompts_ref);
        
        let mut responses = HashMap::new();
        responses.insert("1".to_string(), responses_ref);
        responses.insert("2".to_string(), responses_ref_2);
        
        let mut sources = HashMap::new();
        sources.insert("1".to_string(), sources_ref);
        
        let mut code_snippets = HashMap::new();
        code_snippets.insert("1".to_string(), code_ref);
        
        TrainXMLIdMaps {
            prompts,
            responses,
            sources,
            code_snippets,
        }
    }

    #[test]
    fn test_sample_create_via_tags_preserves_order() {
        let ids = create_test_ids();
        
        // Create a sample with interleaved elements to test order preservation
        let sample_tags = TrainXMLSamplesSample {
            prompt: TrainXMLSamplesPrompt { id: "1".to_string() },
            children: vec![
                // First response
                TrainXMLSamplesSampleChildren::Response(TrainXMLSamplesResponse { id: "1".to_string() }),
                
                // Then a source
                TrainXMLSamplesSampleChildren::Source(TrainXMLSamplesSource { id: "1".to_string() }),
                
                // Then another response
                TrainXMLSamplesSampleChildren::Response(TrainXMLSamplesResponse { id: "2".to_string() }),
                
                // Then code
                TrainXMLSamplesSampleChildren::Code(TrainXMLSamplesCode { 
                    id: "1".to_string(),
                    indent: None,
                    inline: None,
                }),
                
                // Then a response-id with source
                TrainXMLSamplesSampleChildren::ResponseIds(TrainXMLSamplesResponseIds { 
                    response: "1".to_string(),
                    source: Some("1".to_string()),
                }),
                
                // Then a line break
                TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 1 }),
                
                // Then a final response
                TrainXMLSamplesSampleChildren::Response(TrainXMLSamplesResponse { id: "1".to_string() }),
            ],
        };
        
        let sample = sample_create_via_tags(&sample_tags, &ids).unwrap();
        
        // Verify the order matches the XML
        assert_eq!(sample.ai_section.len(), 8); // 4 responses + 2 sources + 1 code + 1 line break
        
        // Position 0: First response
        match &sample.ai_section[0] {
            SampleAiEnum::Text(text) => assert_eq!(text, "A computer network is group of communicating computers."),
            _ => panic!("Expected Text at position 0"),
        }
        
        // Position 1: Source
        match &sample.ai_section[1] {
            SampleAiEnum::Source(source) => assert_eq!(source, "1"),
            _ => panic!("Expected Source at position 1"),
        }
        
        // Position 2: Second response
        match &sample.ai_section[2] {
            SampleAiEnum::Text(text) => assert_eq!(text, "Additional response about networks."),
            _ => panic!("Expected Text at position 2"),
        }
        
        // Position 3: Code
        match &sample.ai_section[3] {
            SampleAiEnum::Code(code) => assert_eq!(code.content, "fn main() {}"),
            _ => panic!("Expected Code at position 3"),
        }
        
        // Position 4: Response from response-ids
        match &sample.ai_section[4] {
            SampleAiEnum::Text(text) => assert_eq!(text, "A computer network is group of communicating computers."),
            _ => panic!("Expected Text at position 4"),
        }
        
        // Position 5: Source from response-ids
        match &sample.ai_section[5] {
            SampleAiEnum::Source(source) => assert_eq!(source, "1"),
            _ => panic!("Expected Source at position 5"),
        }
        
        // Position 6: Line break
        match &sample.ai_section[6] {
            SampleAiEnum::LineBreak(_) => (),
            _ => panic!("Expected LineBreak at position 6"),
        }
        
        // Position 7: Final response
        match &sample.ai_section[7] {
            SampleAiEnum::Text(text) => assert_eq!(text, "A computer network is group of communicating computers."),
            _ => panic!("Expected Text at position 7"),
        }
    }

    #[test]
    fn test_sample_create_via_tags_line_break_counts() {
        let ids = create_test_ids();
        
        let sample_tags = TrainXMLSamplesSample {
            prompt: TrainXMLSamplesPrompt { id: "1".to_string() },
            children: vec![
                TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 1 }),
                TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 2 }),
                TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 1 }),
            ],
        };
        
        let sample = sample_create_via_tags(&sample_tags, &ids).unwrap();
        
        // With struct approach, we have 3 line break items, each with a count
        assert_eq!(sample.ai_section.len(), 3);
        
        // Verify each line break has the correct count
        let expected_counts = [1, 2, 1];
        for (i, item) in sample.ai_section.iter().enumerate() {
            match item {
                SampleAiEnum::LineBreak(line_break) => {
                    assert_eq!(line_break.count, expected_counts[i]);
                },
                _ => panic!("Expected LineBreak at position {}", i),
            }
        }
    }
    
    #[test]
    fn test_sample_create_via_tags_missing_prompt() {
        let ids = create_test_ids();
        
        let sample_tags = TrainXMLSamplesSample {
            prompt: TrainXMLSamplesPrompt { id: "999".to_string() }, // Non-existent
            children: vec![
                TrainXMLSamplesSampleChildren::Response(TrainXMLSamplesResponse { id: "1".to_string() }),
            ],
        };
        
        let sample = sample_create_via_tags(&sample_tags, &ids);
        assert!(sample.is_none());
    }
}
