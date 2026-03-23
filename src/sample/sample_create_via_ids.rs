// src/sample/sample_create_via_ids.rs

use crate::train_xml::{
    TrainXMLIdMaps,
    TrainXMLSamplesSampleIds
};
use crate::sample::{
    Sample,
    Samples,
    SampleText,
    SampleSource, 
    SampleAiEnum,
    SampleAiCode,
    SampleIndent,
    SampleLanguage,
    SampleTokenStatsContainer,
    SamplePromptEnum,
};


/// Create a Sample based on xml id attributes
///
/// # Arguments
/// * `samples` - Mutable reference to Samples container (for ID assignment)
/// * `sample_ids` - The <sample-ids> element from train.xml containing attribute references
/// * `id_map` - Validated ID maps containing all prompts, responses, sources, and code snippets
/// * `token_stats_map` - Token stats map for different component types
///
/// # Returns
/// * `Option<Sample>` - The constructed sample with a unique ID, or None if required references are missing
///
/// # Notes
/// * The sample ID is automatically assigned using samples.next_id()
/// * The sample is NOT automatically added to train/val vectors - that's handled separately
pub fn sample_create_via_ids(
    samples: &mut Samples,
    sample_ids: &TrainXMLSamplesSampleIds,
    id_map: &TrainXMLIdMaps,
    token_stats_map: &SampleTokenStatsContainer,
) -> Option<Sample> {
    // Get the prompt (required)
    let prompt = id_map.prompts.get(&sample_ids.prompt)?;
    
    // Build prompt section
    let mut prompt_section = Vec::new();
    prompt_section.push(SamplePromptEnum::Text(prompt.content.clone()));
    
    // Build AI section
    let mut ai_section = Vec::new();
    
    // Add response if present
    if let Some(response_id) = &sample_ids.response {
        if let Some(response) = id_map.responses.get(response_id) {
            if let Some(token_stats) = token_stats_map.get("response") {
                ai_section.push(SampleAiEnum::Text(SampleText {
                    content: response.content.clone(),
                    token_stats: token_stats.clone(),
                }));
            }
        }
    }
    
    // Add source if present
    if let Some(source_id) = &sample_ids.source {
        if let Some(_source) = id_map.sources.get(source_id) {
            if let Some(token_stats) = token_stats_map.get("source") {
                ai_section.push(SampleAiEnum::Source(SampleSource {
                    id: source_id.clone(),
                    token_stats: token_stats.clone(),
                }));
            }
        }
    }
    
    // Add code if present
    if let Some(code_id) = &sample_ids.code {
        if let Some(code) = id_map.code_snippets.get(code_id) {
            if let Some(token_stats) = token_stats_map.get("code") {
                let lang = SampleLanguage::from_str(&code.lang);
                
                ai_section.push(SampleAiEnum::Code(SampleAiCode {
                    lang,
                    inline: false, // in the future we may wanna add an attribute to <sample-ids /> to identify if code should be inline & how many indents
                    indent: SampleIndent::Zero,
                    content: code.content.clone(),
                    token_stats: token_stats.clone(),
                }));
            }
        }
    }
    
    // Only create sample if we have at least something in AI section
    if ai_section.is_empty() {
        None
    } else {
        Some(Sample {
            id: samples.next_id(), // Use the Samples counter for unique ID
            prompt_section,
            ai_section,
        })
    }
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::sample::{
        Samples,
        SampleAiEnum,
        SampleTokenStatsContainer,
        sample_create_via_ids,
    };
    use crate::train_xml::{
        TrainXMLIdMaps,
        TrainXMLSourcesSource,
        TrainXMLPromptsPrompt,
        TrainXMLCodeSnippetsCode,
        TrainXMLResponsesResponse,
        TrainXMLSamplesSampleIds,
        TrainXMLConstantParsed,
    };
    
    // Create static test data that lives for the entire program
    fn create_test_id_map() -> TrainXMLIdMaps<'static> {
        // Create owned data and leak it to get 'static references
        let prompts_data = Box::new(TrainXMLPromptsPrompt {
            id: "1".to_string(),
            content: "What is a computer?".to_string(),
        });
        
        let responses_data = Box::new(TrainXMLResponsesResponse {
            id: "1".to_string(),
            content: "A computer is a computing device.".to_string(),
        });
        
        let sources_data = Box::new(TrainXMLSourcesSource {
            id: "1".to_string(),
            url: "https://example.com".to_string(),
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
        let sources_ref: &'static TrainXMLSourcesSource = Box::leak(sources_data);
        let code_ref: &'static TrainXMLCodeSnippetsCode = Box::leak(code_data);
        
        let mut prompts = HashMap::new();
        prompts.insert("1".to_string(), prompts_ref);
        
        let mut responses = HashMap::new();
        responses.insert("1".to_string(), responses_ref);
        
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
    
    fn create_test_token_stats_map() -> SampleTokenStatsContainer {
        let constants = TrainXMLConstantParsed::default();
        SampleTokenStatsContainer::new(&constants)
    }
    

    #[test]
    fn test_sample_create_via_ids_basic() {
        let mut samples = Samples {
            train_samples: Vec::new(),
            val_samples: Vec::new(),
            total_sample_count: 0,
        };
        let id_map = create_test_id_map();
        let token_stats_map = create_test_token_stats_map(); // Uses actual defaults
        let sample_ids = TrainXMLSamplesSampleIds {
            prompt: "1".to_string(),
            response: Some("1".to_string()),
            source: Some("1".to_string()),
            code: Some("1".to_string()),
        };
        
        let sample = sample_create_via_ids(&mut samples, &sample_ids, &id_map, &token_stats_map);
        assert!(sample.is_some());
        let sample = sample.unwrap();
        
        assert_eq!(sample.id, "1");
        assert_eq!(samples.total_sample_count, 1);
        
        assert_eq!(sample.prompt_section.len(), 1);
        assert_eq!(sample.ai_section.len(), 3);
        
        // Match actual defaults from TrainXMLConstantParsed::default()
        if let SampleAiEnum::Text(text) = &sample.ai_section[0] {
            assert_eq!(text.token_stats.weight_decay, 0.1);   // Default: 0.1
            assert_eq!(text.token_stats.dropout, 0.05);      // Default: 0.05
        } else {
            panic!("Expected Text variant");
        }
        
        if let SampleAiEnum::Source(source) = &sample.ai_section[1] {
            assert_eq!(source.token_stats.weight_decay, 0.01); // Default: 0.01
            assert_eq!(source.token_stats.dropout, 0.0);      // Default: 0.0
        } else {
            panic!("Expected Source variant");
        }
        
        if let SampleAiEnum::Code(code) = &sample.ai_section[2] {
            assert_eq!(code.token_stats.weight_decay, 0.05);   // Default: 0.05
            assert_eq!(code.token_stats.dropout, 0.1);        // Default: 0.1
        } else {
            panic!("Expected Code variant");
        }
    }
    
    #[test]
    fn test_sample_create_via_ids_multiple_samples() {
        let mut samples = Samples {
            train_samples: Vec::new(),
            val_samples: Vec::new(),
            total_sample_count: 0,
        };
        let id_map = create_test_id_map();
        let token_stats_map = create_test_token_stats_map();
        
        // Create first sample
        let sample_ids1 = TrainXMLSamplesSampleIds {
            prompt: "1".to_string(),
            response: Some("1".to_string()),
            source: None,
            code: None,
        };
        
        let sample1 = sample_create_via_ids(&mut samples, &sample_ids1, &id_map, &token_stats_map);
        assert!(sample1.is_some());
        assert_eq!(sample1.unwrap().id, "1");
        assert_eq!(samples.total_sample_count, 1);
        
        // Create second sample
        let sample_ids2 = TrainXMLSamplesSampleIds {
            prompt: "1".to_string(),
            response: Some("1".to_string()),
            source: Some("1".to_string()),
            code: None,
        };
        
        let sample2 = sample_create_via_ids(&mut samples, &sample_ids2, &id_map, &token_stats_map);
        assert!(sample2.is_some());
        assert_eq!(sample2.unwrap().id, "2");
        assert_eq!(samples.total_sample_count, 2);
        
        // Create third sample
        let sample_ids3 = TrainXMLSamplesSampleIds {
            prompt: "1".to_string(),
            response: Some("1".to_string()),
            source: Some("1".to_string()),
            code: Some("1".to_string()),
        };
        
        let sample3 = sample_create_via_ids(&mut samples, &sample_ids3, &id_map, &token_stats_map);
        assert!(sample3.is_some());
        assert_eq!(sample3.unwrap().id, "3");
        assert_eq!(samples.total_sample_count, 3);
    }
    
    #[test]
    fn test_sample_create_via_ids_missing_prompt() {
        let mut samples = Samples {
            train_samples: Vec::new(),
            val_samples: Vec::new(),
            total_sample_count: 0,
        };
        let id_map = create_test_id_map();
        let token_stats_map = create_test_token_stats_map();
        let sample_ids = TrainXMLSamplesSampleIds {
            prompt: "999".to_string(), // Non-existent
            response: Some("1".to_string()),
            source: Some("1".to_string()),
            code: Some("1".to_string()),
        };
        
        let sample = sample_create_via_ids(&mut samples, &sample_ids, &id_map, &token_stats_map);
        assert!(sample.is_none());
        assert_eq!(samples.total_sample_count, 0); // Count should not increment
    }
    
    #[test]
    fn test_sample_create_via_ids_response_only() {
        let mut samples = Samples {
            train_samples: Vec::new(),
            val_samples: Vec::new(),
            total_sample_count: 0,
        };
        let id_map = create_test_id_map();
        let token_stats_map = create_test_token_stats_map();
        let sample_ids = TrainXMLSamplesSampleIds {
            prompt: "1".to_string(),
            response: Some("1".to_string()),
            source: None,
            code: None,
        };
        
        let sample = sample_create_via_ids(&mut samples, &sample_ids, &id_map, &token_stats_map);
        assert!(sample.is_some());
        let sample = sample.unwrap();
        
        assert_eq!(sample.id, "1");
        assert_eq!(samples.total_sample_count, 1);
        assert_eq!(sample.ai_section.len(), 1);
        
        // Verify token stats match actual defaults from TrainXMLConstantParsed::default()
        match &sample.ai_section[0] {
            SampleAiEnum::Text(text) => {
                assert_eq!(text.token_stats.weight_decay, 0.1);
                assert_eq!(text.token_stats.dropout, 0.05);
            },
            _ => panic!("Expected Text variant"),
        }
    }

    #[test]
    fn test_sample_create_via_ids_line_break_uses_response_stats() {
        // This test verifies that "line-break" component type maps to response stats
        let token_stats_map = create_test_token_stats_map();
        
        // Get stats for "line-break" - should be the same as "response"
        let line_break_stats = token_stats_map.get("line-break").unwrap();
        let response_stats = token_stats_map.get("response").unwrap();
        
        assert_eq!(line_break_stats.weight_decay, response_stats.weight_decay);
        assert_eq!(line_break_stats.dropout, response_stats.dropout);
        assert_eq!(line_break_stats.loss_scale, response_stats.loss_scale);
        assert_eq!(line_break_stats.gradient_scale, response_stats.gradient_scale);
        assert_eq!(line_break_stats.gradient_clip, response_stats.gradient_clip);
    }
}
