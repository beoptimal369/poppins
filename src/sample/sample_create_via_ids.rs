// src/sample/sample_create_via_ids.rs

use crate::train_xml::{
    TrainXMLIdMaps,
    TrainXMLSamplesSampleIds
};
use crate::sample::{
    Sample,
    SampleCode,
    SampleAiEnum,
    SampleLanguage,
    SamplePromptEnum,
};


/// Create a Sample based on xml id attributes
///
/// # Arguments
/// * `samples` - The <sample-ids> element from train.xml containing attribute references
/// * `train_xml_ids` - Validated ID maps containing all prompts, responses, sources, code snippets, and system prompts
///
/// # Returns
/// * `Option<Sample>` - The constructed sample, or None if required references are missing
///
/// # Notes
/// * The sample is NOT automatically added to train/val vectors - that's handled separately
pub fn sample_create_via_ids(
    samples: &TrainXMLSamplesSampleIds,
    train_xml_ids: &TrainXMLIdMaps,
) -> Option<Sample> {
    // Get the prompt (required)
    let prompt = train_xml_ids.prompts.get(&samples.prompt)?;
    
    // Get system prompt if present - return None if ID exists but not found
    let mut system = None;

    if let Some(system_id) = &samples.system {
        if let Some(system_prompt) = train_xml_ids.system_prompts.get(system_id) {
            system = Some(system_prompt.content.clone());
        } else {
            // System ID provided but not found in train_xml_ids - treat as invalid sample
            return None;
        }
    }
    
    // Get thought if present - return None if ID exists but not found
    let mut thought = None;

    if let Some(thought_id) = &samples.thought {
        if let Some(thought_prompt) = train_xml_ids.thoughts.get(thought_id) {
            thought = Some(thought_prompt.content.clone());
        } else {
            // Thought ID provided but not found in train_xml_ids - treat as invalid sample
            return None;
        }
    }
    
    // Build prompt section (user prompts only)
    let mut prompt_section = Vec::new();
    prompt_section.push(SamplePromptEnum::Text(prompt.content.clone()));
    
    // Build AI section
    let mut ai_section = Vec::new();
    
    // Add response if present - return None if ID exists but not found
    if let Some(response_id) = &samples.response {
        if let Some(response) = train_xml_ids.responses.get(response_id) {
            ai_section.push(SampleAiEnum::Text(response.content.clone()));
        } else {
            // Response ID provided but not found - invalid sample
            return None;
        }
    }
    
    // Add source if present - return None if ID exists but not found
    if let Some(source_id) = &samples.source {
        if let Some(_source) = train_xml_ids.sources.get(source_id) {
            ai_section.push(SampleAiEnum::Source(source_id.clone()));
        } else {
            // Source ID provided but not found - invalid sample
            return None;
        }
    }
    
    // Add code if present - return None if ID exists but not found
    if let Some(code_id) = &samples.code {
        if let Some(code) = train_xml_ids.code_snippets.get(code_id) {
            let lang = SampleLanguage::from_str(&code.lang);
            
            ai_section.push(SampleAiEnum::Code(SampleCode {
                lang,
                inline: false,
                indent: None,
                content: code.content.clone(),
            }));
        } else {
            // Code ID provided but not found - invalid sample
            return None;
        }
    }
    
    // Only create sample if we have at least something in AI section
    if ai_section.is_empty() {
        None
    } else {
        Some(Sample {
            system,
            thought,
            prompt_section,
            ai_section,
        })
    }
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::sample::{
        SamplePromptEnum,
        sample_create_via_ids,
    };
    use crate::train_xml::{
        TrainXMLIdMaps,
        TrainXMLSourcesSource,
        TrainXMLPromptsPrompt,
        TrainXMLThoughtsThought,
        TrainXMLCodeSnippetsCode,
        TrainXMLResponsesResponse,
        TrainXMLSamplesSampleIds,
        TrainXMLSystemPromptsSystem,
    };
    
    // Create static test data that lives for the entire program
    fn create_test_train_xml_ids() -> TrainXMLIdMaps<'static> {
        // Create owned data and leak it to get 'static references
        let system_data = Box::new(TrainXMLSystemPromptsSystem {
            id: "sy1".to_string(),
            content: "You are a helpful computer programming assistant.".to_string(),
        });
        
        let prompts_data = Box::new(TrainXMLPromptsPrompt {
            id: "1".to_string(),
            content: "What is a computer?".to_string(),
        });
        
        let thoughts_data = Box::new(TrainXMLThoughtsThought {
            id: "th1".to_string(),
            content: "I will provide the definition of a computer.".to_string(),
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
        let system_ref: &'static TrainXMLSystemPromptsSystem = Box::leak(system_data);
        let prompts_ref: &'static TrainXMLPromptsPrompt = Box::leak(prompts_data);
        let thoughts_ref: &'static TrainXMLThoughtsThought = Box::leak(thoughts_data);
        let responses_ref: &'static TrainXMLResponsesResponse = Box::leak(responses_data);
        let sources_ref: &'static TrainXMLSourcesSource = Box::leak(sources_data);
        let code_ref: &'static TrainXMLCodeSnippetsCode = Box::leak(code_data);
        
        let mut system_prompts = HashMap::new();
        system_prompts.insert("sy1".to_string(), system_ref);
        
        let mut prompts = HashMap::new();
        prompts.insert("1".to_string(), prompts_ref);
        
        let mut thoughts = HashMap::new();
        thoughts.insert("th1".to_string(), thoughts_ref);
        
        let mut responses = HashMap::new();
        responses.insert("1".to_string(), responses_ref);
        
        let mut sources = HashMap::new();
        sources.insert("1".to_string(), sources_ref);
        
        let mut code_snippets = HashMap::new();
        code_snippets.insert("1".to_string(), code_ref);
        
        TrainXMLIdMaps {
            system_prompts,
            prompts,
            thoughts,
            responses,
            sources,
            code_snippets,
        }
    }
    
    // Create test ID map without system prompts
    fn create_test_train_xml_ids_without_system() -> TrainXMLIdMaps<'static> {
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
            system_prompts: HashMap::new(),
            prompts,
            thoughts: HashMap::new(),
            responses,
            sources,
            code_snippets,
        }
    }
    

    #[test]
    fn test_sample_create_via_ids_basic() {
        let train_xml_ids = create_test_train_xml_ids();
        let samples = TrainXMLSamplesSampleIds {
            system: None,
            prompt: "1".to_string(),
            thought: None,
            response: Some("1".to_string()),
            source: Some("1".to_string()),
            code: Some("1".to_string()),
        };
        
        let sample = sample_create_via_ids(&samples, &train_xml_ids);
        assert!(sample.is_some());
        let sample = sample.unwrap();
        
        assert_eq!(sample.system, None);
        assert_eq!(sample.thought, None);
        assert_eq!(sample.prompt_section.len(), 1);
        assert_eq!(sample.ai_section.len(), 3);
        
        // Verify prompt section contains only text prompt
        match &sample.prompt_section[0] {
            SamplePromptEnum::Text(text) => assert_eq!(text, "What is a computer?"),
            _ => panic!("Expected text prompt"),
        }
    }
    
    #[test]
    fn test_sample_create_via_ids_with_thought() {
        let train_xml_ids = create_test_train_xml_ids();
        let samples = TrainXMLSamplesSampleIds {
            system: Some("sy1".to_string()),
            prompt: "1".to_string(),
            thought: Some("th1".to_string()),
            response: Some("1".to_string()),
            source: Some("1".to_string()),
            code: Some("1".to_string()),
        };
        
        let sample = sample_create_via_ids(&samples, &train_xml_ids);
        assert!(sample.is_some());
        let sample = sample.unwrap();
        
        assert_eq!(sample.system, Some("You are a helpful computer programming assistant.".to_string()));
        assert_eq!(sample.thought, Some("I will provide the definition of a computer.".to_string()));
        assert_eq!(sample.prompt_section.len(), 1);
        assert_eq!(sample.ai_section.len(), 3);
        
        // Verify prompt section contains only text prompt
        match &sample.prompt_section[0] {
            SamplePromptEnum::Text(text) => assert_eq!(text, "What is a computer?"),
            _ => panic!("Expected text prompt"),
        }
    }
    
    #[test]
    fn test_sample_create_via_ids_with_system_prompt() {
        let train_xml_ids = create_test_train_xml_ids();
        let samples = TrainXMLSamplesSampleIds {
            system: Some("sy1".to_string()),
            prompt: "1".to_string(),
            thought: None,
            response: Some("1".to_string()),
            source: Some("1".to_string()),
            code: Some("1".to_string()),
        };
        
        let sample = sample_create_via_ids(&samples, &train_xml_ids);
        assert!(sample.is_some());
        let sample = sample.unwrap();
        
        assert_eq!(sample.system, Some("You are a helpful computer programming assistant.".to_owned()));
        assert_eq!(sample.thought, None);
        assert_eq!(sample.prompt_section.len(), 1);
        assert_eq!(sample.ai_section.len(), 3);
        
        match &sample.prompt_section[0] {
            SamplePromptEnum::Text(text) => assert_eq!(text, "What is a computer?"),
            _ => panic!("Expected text prompt"),
        }
    }
    
    #[test]
    fn test_sample_create_via_ids_with_invalid_system_prompt() {
        let train_xml_ids = create_test_train_xml_ids();
        let samples = TrainXMLSamplesSampleIds {
            system: Some("invalid_system".to_string()),
            prompt: "1".to_string(),
            thought: None,
            response: Some("1".to_string()),
            source: Some("1".to_string()),
            code: Some("1".to_string()),
        };
        
        let sample = sample_create_via_ids(&samples, &train_xml_ids);
        // Should return None because system ID is invalid
        assert!(sample.is_none());
    }
    
    #[test]
    fn test_sample_create_via_ids_multiple_samples() {
        let train_xml_ids = create_test_train_xml_ids();
        
        // Create first sample without system
        let samples1 = TrainXMLSamplesSampleIds {
            system: None,
            prompt: "1".to_string(),
            thought: None,
            response: Some("1".to_string()),
            source: None,
            code: None,
        };
        
        let sample1 = sample_create_via_ids(&samples1, &train_xml_ids);
        assert!(sample1.is_some());
        let sample1 = sample1.unwrap();
        assert_eq!(sample1.system, None);
        assert_eq!(sample1.thought, None);
        assert_eq!(sample1.prompt_section.len(), 1);
        assert_eq!(sample1.ai_section.len(), 1);
        
        // Create second sample with system
        let samples2 = TrainXMLSamplesSampleIds {
            system: Some("sy1".to_string()),
            prompt: "1".to_string(),
            thought: None,
            response: Some("1".to_string()),
            source: Some("1".to_string()),
            code: None,
        };
        
        let sample2 = sample_create_via_ids(&samples2, &train_xml_ids);
        assert!(sample2.is_some());
        let sample2 = sample2.unwrap();
        assert_eq!(sample2.system, Some("You are a helpful computer programming assistant.".to_owned()));
        assert_eq!(sample2.thought, None);
        assert_eq!(sample2.prompt_section.len(), 1);
        assert_eq!(sample2.ai_section.len(), 2);
        
        // Create third sample with system and code
        let samples3 = TrainXMLSamplesSampleIds {
            system: Some("sy1".to_string()),
            prompt: "1".to_string(),
            thought: None,
            response: Some("1".to_string()),
            source: Some("1".to_string()),
            code: Some("1".to_string()),
        };
        
        let sample3 = sample_create_via_ids(&samples3, &train_xml_ids);
        assert!(sample3.is_some());
        let sample3 = sample3.unwrap();
        assert_eq!(sample3.system, Some("You are a helpful computer programming assistant.".to_owned()));
        assert_eq!(sample3.thought, None);
        assert_eq!(sample3.prompt_section.len(), 1);
        assert_eq!(sample3.ai_section.len(), 3);
    }
    
    #[test]
    fn test_sample_create_via_ids_missing_prompt() {
        let train_xml_ids = create_test_train_xml_ids();
        let samples = TrainXMLSamplesSampleIds {
            system: None,
            prompt: "999".to_string(),
            thought: None,
            response: Some("1".to_string()),
            source: Some("1".to_string()),
            code: Some("1".to_string()),
        };
        
        let sample = sample_create_via_ids(&samples, &train_xml_ids);
        assert!(sample.is_none());
    }
    
    #[test]
    fn test_sample_create_via_ids_response_only() {
        let train_xml_ids = create_test_train_xml_ids();
        let samples = TrainXMLSamplesSampleIds {
            system: None,
            prompt: "1".to_string(),
            thought: None,
            response: Some("1".to_string()),
            source: None,
            code: None,
        };
        
        let sample = sample_create_via_ids(&samples, &train_xml_ids);
        assert!(sample.is_some());
        let sample = sample.unwrap();
        
        assert_eq!(sample.ai_section.len(), 1);
    }
    
    #[test]
    fn test_sample_create_via_ids_with_system_no_response() {
        let train_xml_ids = create_test_train_xml_ids();
        let samples = TrainXMLSamplesSampleIds {
            system: Some("sy1".to_string()),
            prompt: "1".to_string(),
            thought: None,
            response: None,
            source: None,
            code: None,
        };
        
        let sample = sample_create_via_ids(&samples, &train_xml_ids);
        assert!(sample.is_none(), "Sample should be None when no AI section content");
    }
    
    #[test]
    fn test_sample_create_via_ids_with_system_and_missing_response() {
        let train_xml_ids = create_test_train_xml_ids();
        let samples = TrainXMLSamplesSampleIds {
            system: Some("sy1".to_string()),
            prompt: "1".to_string(),
            thought: None,
            response: Some("999".to_string()),
            source: None,
            code: None,
        };
        
        let sample = sample_create_via_ids(&samples, &train_xml_ids);
        assert!(sample.is_none(), "Sample should be None when response doesn't exist");
    }
    
    #[test]
    fn test_sample_create_via_ids_without_system_prompts_in_train_xml_ids() {
        let train_xml_ids = create_test_train_xml_ids_without_system();
        let samples = TrainXMLSamplesSampleIds {
            system: Some("sy1".to_string()),
            prompt: "1".to_string(),
            thought: None,
            response: Some("1".to_string()),
            source: None,
            code: None,
        };
        
        let sample = sample_create_via_ids(&samples, &train_xml_ids);
        // Should return None because system ID exists but not in train_xml_ids
        assert!(sample.is_none());
    }
}
