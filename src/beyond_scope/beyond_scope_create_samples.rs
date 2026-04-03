// src/beyond_scope/beyond_scope_create_samples.rs

use crate::beyond_scope::BeyondScope;
use crate::train_xml::{TrainXML, TrainXMLIdMaps};
use crate::sample::{Sample, SamplePromptEnum, SampleAiEnum};


/// Add topics from enabled categories
macro_rules! add_topic {
    ($topics_to_generate:expr, $config:expr, $default_topics:expr, $field:ident) => {
        if $config.$field == Some(true) {
            for topic in &$default_topics.$field {
                $topics_to_generate.push((&topic.value, &topic.prefix));
            }
        }
    };
}


/// Creates beyond-scope samples based on the train.xml configuration
///
/// This function generates samples for topics that are outside the AI's scope.
/// It uses the beyond-scope configuration to determine which topics to include
/// and creates "What <prefix> <topic>?" prompts with the configured response.
///
/// # Arguments
/// * `train_xml` - The parsed train XML document
/// * `train_xml_ids` - Validated train.xml ID maps
///
/// # Returns
/// * `Vec<Sample>` - Vector of beyond-scope samples, or empty vector if none configured
pub fn beyond_scope_create_samples(
    train_xml: &TrainXML,
    train_xml_ids: &TrainXMLIdMaps,
) -> Vec<Sample> {
    // Get the beyond-scope configuration
    let beyond_scope_config = match &train_xml.beyond_scope {
        Some(config) => config,
        None => return Vec::new(),
    };
    
    // Get the system prompt content
    let system_content = match train_xml_ids.system_prompts.get(&beyond_scope_config.system) {
        Some(system) => &system.content,
        None => return Vec::new(),
    };
    
    // Get the response content
    let response_content = match train_xml_ids.responses.get(&beyond_scope_config.response) {
        Some(response) => &response.content,
        None => return Vec::new(),
    };
    
    // Pre-allocate with estimated capacity to avoid reallocation
    let mut topics_to_generate = Vec::new();
    
    // Add custom topics from <topic> elements
    for topic in &beyond_scope_config.topics {
        topics_to_generate.push((&topic.value, &topic.prefix));
    }
    
    // Get the default beyond-scope topics
    let default_topics = BeyondScope::default();
    
    // Add topics from enabled categories
    add_topic!(topics_to_generate, beyond_scope_config, default_topics, sports);
    add_topic!(topics_to_generate, beyond_scope_config, default_topics, food);
    add_topic!(topics_to_generate, beyond_scope_config, default_topics, movies);
    add_topic!(topics_to_generate, beyond_scope_config, default_topics, history);
    add_topic!(topics_to_generate, beyond_scope_config, default_topics, geography);
    add_topic!(topics_to_generate, beyond_scope_config, default_topics, politics);
    add_topic!(topics_to_generate, beyond_scope_config, default_topics, science);
    add_topic!(topics_to_generate, beyond_scope_config, default_topics, politics);
    add_topic!(topics_to_generate, beyond_scope_config, default_topics, health);
    add_topic!(topics_to_generate, beyond_scope_config, default_topics, art);
    add_topic!(topics_to_generate, beyond_scope_config, default_topics, music);
    add_topic!(topics_to_generate, beyond_scope_config, default_topics, fashion);
    add_topic!(topics_to_generate, beyond_scope_config, default_topics, travel);
    add_topic!(topics_to_generate, beyond_scope_config, default_topics, pets);
    add_topic!(topics_to_generate, beyond_scope_config, default_topics, cars);
    
    // Pre-allocate the samples vector with exact capacity
    let mut samples = Vec::with_capacity(topics_to_generate.len());
    
    // Create a reusable question string buffer to reduce allocations
    let mut question_buffer = String::new();
    
    // Create a sample for each topic - using references and minimal cloning
    for (topic_value, prefix) in topics_to_generate {
        // Build the question using the buffer
        question_buffer.clear();
        question_buffer.push_str("What ");
        question_buffer.push_str(prefix);
        question_buffer.push(' ');
        question_buffer.push_str(topic_value);
        question_buffer.push('?');
        
        // Create the prompt section
        let prompt_section = vec![SamplePromptEnum::Text(question_buffer.clone())];
        
        // Create the AI section with the configured response (clone once per topic)
        let ai_section = vec![SampleAiEnum::Text(response_content.clone())];
        
        samples.push(Sample {
            system: system_content.clone(),
            prompt_section,
            ai_section,
        });
    }
    
    samples
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::beyond_scope::beyond_scope_create_samples;
    use crate::train_xml::{
        TrainXML,
        TrainXMLIdMaps,
        TrainXMLBeyondScope,
        TrainXMLBeyondScopeTopic,
        TrainXMLSystemPrompts,
        TrainXMLSystemPromptsSystem,
        TrainXMLResponses,
        TrainXMLResponsesResponse,
    };
    
    fn create_test_train_xml_ids_with_system_and_response() -> TrainXMLIdMaps<'static> {
        let system_data = Box::new(TrainXMLSystemPromptsSystem {
            id: "sy_default".to_string(),
            content: "You are a computer programming assistant. If the prompt is not about computer programming, then please respond: \"I'm sorry, I don't know, I'm a computer programming assistant\"".to_string(),
        });
        
        let response_data = Box::new(TrainXMLResponsesResponse {
            id: "re_beyond_scope".to_string(),
            content: "I'm sorry, I don't know, I'm a computer programming assistant".to_string(),
        });
        
        let system_ref: &'static TrainXMLSystemPromptsSystem = Box::leak(system_data);
        let response_ref: &'static TrainXMLResponsesResponse = Box::leak(response_data);
        
        let mut system_prompts = HashMap::new();
        system_prompts.insert("sy_default".to_string(), system_ref);
        
        let mut responses = HashMap::new();
        responses.insert("re_beyond_scope".to_string(), response_ref);
        
        TrainXMLIdMaps {
            system_prompts,
            prompts: HashMap::new(),
            responses,
            sources: HashMap::new(),
            code_snippets: HashMap::new(),
        }
    }
    
    #[test]
    fn test_beyond_scope_create_samples_with_no_config() {
        let train_xml = TrainXML {
            system_prompts: None,
            prompts: None,
            responses: None,
            sources: None,
            code_snippets: None,
            samples: None,
            constants: None,
            phrases: None,
            beyond_scope: None,
        };
        
        let train_xml_ids = create_test_train_xml_ids_with_system_and_response();
        let samples = beyond_scope_create_samples(&train_xml, &train_xml_ids);
        
        assert!(samples.is_empty());
    }
    
    #[test]
    fn test_beyond_scope_create_samples_with_custom_topics() {
        let beyond_scope = TrainXMLBeyondScope {
            system: "sy_default".to_string(),
            response: "re_beyond_scope".to_string(),
            topics: vec![
                TrainXMLBeyondScopeTopic { 
                    value: "quantum computing".to_string(),
                    prefix: "is".to_string(),
                },
                TrainXMLBeyondScopeTopic { 
                    value: "blockchain".to_string(),
                    prefix: "is".to_string(),
                },
            ],
            sports: Some(false),
            food: Some(false),
            movies: Some(false),
            history: Some(false),
            geography: Some(false),
            politics: Some(false),
            science: Some(false),
            health: Some(false),
            art: Some(false),
            music: Some(false),
            fashion: Some(false),
            travel: Some(false),
            pets: Some(false),
            cars: Some(false),
        };
        
        let train_xml = TrainXML {
            system_prompts: Some(TrainXMLSystemPrompts {
                system: vec![TrainXMLSystemPromptsSystem {
                    id: "sy_default".to_string(),
                    content: "You are a computer programming assistant. If the prompt is not about computer programming, then please respond: \"I'm sorry, I don't know, I'm a computer programming assistant\"".to_string(),
                }],
            }),
            responses: Some(TrainXMLResponses {
                response: vec![TrainXMLResponsesResponse {
                    id: "re_beyond_scope".to_string(),
                    content: "I'm sorry, I don't know, I'm a computer programming assistant".to_string(),
                }],
            }),
            prompts: None,
            sources: None,
            code_snippets: None,
            samples: None,
            constants: None,
            phrases: None,
            beyond_scope: Some(beyond_scope),
        };
        
        let train_xml_ids = create_test_train_xml_ids_with_system_and_response();
        let samples = beyond_scope_create_samples(&train_xml, &train_xml_ids);
        
        assert_eq!(samples.len(), 2);
        
        // Check first sample
        assert_eq!(samples[0].system, "You are a computer programming assistant. If the prompt is not about computer programming, then please respond: \"I'm sorry, I don't know, I'm a computer programming assistant\"");
        match &samples[0].prompt_section[0] {
            crate::sample::SamplePromptEnum::Text(text) => {
                assert_eq!(text, "What is quantum computing?");
            }
            _ => panic!("Expected text prompt"),
        }
        match &samples[0].ai_section[0] {
            crate::sample::SampleAiEnum::Text(text) => {
                assert_eq!(text, "I'm sorry, I don't know, I'm a computer programming assistant");
            }
            _ => panic!("Expected text response"),
        }
        
        // Check second sample
        match &samples[1].prompt_section[0] {
            crate::sample::SamplePromptEnum::Text(text) => {
                assert_eq!(text, "What is blockchain?");
            }
            _ => panic!("Expected text prompt"),
        }
    }
    
    #[test]
    fn test_beyond_scope_create_samples_with_category() {
        let beyond_scope = TrainXMLBeyondScope {
            system: "sy_default".to_string(),
            response: "re_beyond_scope".to_string(),
            topics: vec![],
            sports: Some(true),
            food: Some(false),
            movies: Some(false),
            history: Some(false),
            geography: Some(false),
            politics: Some(false),
            science: Some(false),
            health: Some(false),
            art: Some(false),
            music: Some(false),
            fashion: Some(false),
            travel: Some(false),
            pets: Some(false),
            cars: Some(false),
        };
        
        let train_xml = TrainXML {
            system_prompts: Some(TrainXMLSystemPrompts {
                system: vec![TrainXMLSystemPromptsSystem {
                    id: "sy_default".to_string(),
                    content: "You are a computer programming assistant.".to_string(),
                }],
            }),
            responses: Some(TrainXMLResponses {
                response: vec![TrainXMLResponsesResponse {
                    id: "re_beyond_scope".to_string(),
                    content: "I'm sorry, I don't know, I'm a computer programming assistant".to_string(),
                }],
            }),
            prompts: None,
            sources: None,
            code_snippets: None,
            samples: None,
            constants: None,
            phrases: None,
            beyond_scope: Some(beyond_scope),
        };
        
        let train_xml_ids = create_test_train_xml_ids_with_system_and_response();
        let samples = beyond_scope_create_samples(&train_xml, &train_xml_ids);
        
        // Sports category has 12 topics
        assert_eq!(samples.len(), 12);
        
        // Check a few specific topics
        let questions: Vec<String> = samples.iter()
            .filter_map(|s| match &s.prompt_section[0] {
                crate::sample::SamplePromptEnum::Text(text) => Some(text.clone()),
                _ => None,
            })
            .collect();
        
        assert!(questions.contains(&"What is soccer?".to_string()));
        assert!(questions.contains(&"What is the olympics?".to_string()));
        assert!(questions.contains(&"What is the world cup?".to_string()));
    }
    
    #[test]
    fn test_beyond_scope_create_samples_with_are_prefix() {
        let beyond_scope = TrainXMLBeyondScope {
            system: "sy_default".to_string(),
            response: "re_beyond_scope".to_string(),
            topics: vec![],
            sports: Some(false),
            food: Some(false),
            movies: Some(true),
            history: Some(false),
            geography: Some(false),
            politics: Some(false),
            science: Some(false),
            health: Some(false),
            art: Some(false),
            music: Some(false),
            fashion: Some(false),
            travel: Some(false),
            pets: Some(false),
            cars: Some(false),
        };
        
        let train_xml = TrainXML {
            system_prompts: Some(TrainXMLSystemPrompts {
                system: vec![TrainXMLSystemPromptsSystem {
                    id: "sy_default".to_string(),
                    content: "You are a computer programming assistant.".to_string(),
                }],
            }),
            responses: Some(TrainXMLResponses {
                response: vec![TrainXMLResponsesResponse {
                    id: "re_beyond_scope".to_string(),
                    content: "I'm sorry, I don't know, I'm a computer programming assistant".to_string(),
                }],
            }),
            prompts: None,
            sources: None,
            code_snippets: None,
            samples: None,
            constants: None,
            phrases: None,
            beyond_scope: Some(beyond_scope),
        };
        
        let train_xml_ids = create_test_train_xml_ids_with_system_and_response();
        let samples = beyond_scope_create_samples(&train_xml, &train_xml_ids);
        
        let questions: Vec<String> = samples.iter()
            .filter_map(|s| match &s.prompt_section[0] {
                crate::sample::SamplePromptEnum::Text(text) => Some(text.clone()),
                _ => None,
            })
            .collect();
        
        // Check topics that use "are" prefix
        assert!(questions.contains(&"What are movies?".to_string()));
        assert!(questions.contains(&"What are tv shows?".to_string()));
        assert!(questions.contains(&"What are actors?".to_string()));
        assert!(questions.contains(&"What are documentaries?".to_string()));
        
        // Check topics that use "is" prefix
        assert!(questions.contains(&"What is netflix?".to_string()));
        assert!(questions.contains(&"What is hollywood?".to_string()));
        assert!(questions.contains(&"What is animation?".to_string()));
    }
    
    #[test]
    fn test_beyond_scope_create_samples_with_custom_prefix() {
        let beyond_scope = TrainXMLBeyondScope {
            system: "sy_default".to_string(),
            response: "re_beyond_scope".to_string(),
            topics: vec![
                TrainXMLBeyondScopeTopic { 
                    value: "the matrix".to_string(),
                    prefix: "is".to_string(),
                },
                TrainXMLBeyondScopeTopic { 
                    value: "electric cars".to_string(),
                    prefix: "are".to_string(),
                },
            ],
            sports: Some(false),
            food: Some(false),
            movies: Some(false),
            history: Some(false),
            geography: Some(false),
            politics: Some(false),
            science: Some(false),
            health: Some(false),
            art: Some(false),
            music: Some(false),
            fashion: Some(false),
            travel: Some(false),
            pets: Some(false),
            cars: Some(false),
        };
        
        let train_xml = TrainXML {
            system_prompts: Some(TrainXMLSystemPrompts {
                system: vec![TrainXMLSystemPromptsSystem {
                    id: "sy_default".to_string(),
                    content: "You are a computer programming assistant.".to_string(),
                }],
            }),
            responses: Some(TrainXMLResponses {
                response: vec![TrainXMLResponsesResponse {
                    id: "re_beyond_scope".to_string(),
                    content: "I'm sorry, I don't know, I'm a computer programming assistant".to_string(),
                }],
            }),
            prompts: None,
            sources: None,
            code_snippets: None,
            samples: None,
            constants: None,
            phrases: None,
            beyond_scope: Some(beyond_scope),
        };
        
        let train_xml_ids = create_test_train_xml_ids_with_system_and_response();
        let samples = beyond_scope_create_samples(&train_xml, &train_xml_ids);
        
        let questions: Vec<String> = samples.iter()
            .filter_map(|s| match &s.prompt_section[0] {
                crate::sample::SamplePromptEnum::Text(text) => Some(text.clone()),
                _ => None,
            })
            .collect();
        
        assert!(questions.contains(&"What is the matrix?".to_string()));
        assert!(questions.contains(&"What are electric cars?".to_string()));
    }
    
    #[test]
    fn test_beyond_scope_create_samples_with_missing_system() {
        let beyond_scope = TrainXMLBeyondScope {
            system: "missing_system".to_string(),
            response: "re_beyond_scope".to_string(),
            topics: vec![TrainXMLBeyondScopeTopic { 
                value: "test".to_string(),
                prefix: "is".to_string(),
            }],
            sports: Some(false),
            food: Some(false),
            movies: Some(false),
            history: Some(false),
            geography: Some(false),
            politics: Some(false),
            science: Some(false),
            health: Some(false),
            art: Some(false),
            music: Some(false),
            fashion: Some(false),
            travel: Some(false),
            pets: Some(false),
            cars: Some(false),
        };
        
        let train_xml = TrainXML {
            system_prompts: Some(TrainXMLSystemPrompts {
                system: vec![TrainXMLSystemPromptsSystem {
                    id: "sy_default".to_string(),
                    content: "You are a computer programming assistant.".to_string(),
                }],
            }),
            responses: Some(TrainXMLResponses {
                response: vec![TrainXMLResponsesResponse {
                    id: "re_beyond_scope".to_string(),
                    content: "I'm sorry, I don't know, I'm a computer programming assistant".to_string(),
                }],
            }),
            prompts: None,
            sources: None,
            code_snippets: None,
            samples: None,
            constants: None,
            phrases: None,
            beyond_scope: Some(beyond_scope),
        };
        
        let train_xml_ids = create_test_train_xml_ids_with_system_and_response();
        let samples = beyond_scope_create_samples(&train_xml, &train_xml_ids);
        
        // Should return empty because system prompt not found
        assert!(samples.is_empty());
    }
    
    #[test]
    fn test_beyond_scope_create_samples_with_missing_response() {
        let beyond_scope = TrainXMLBeyondScope {
            system: "sy_default".to_string(),
            response: "missing_response".to_string(),
            topics: vec![TrainXMLBeyondScopeTopic { 
                value: "test".to_string(),
                prefix: "is".to_string(),
            }],
            sports: Some(false),
            food: Some(false),
            movies: Some(false),
            history: Some(false),
            geography: Some(false),
            politics: Some(false),
            science: Some(false),
            health: Some(false),
            art: Some(false),
            music: Some(false),
            fashion: Some(false),
            travel: Some(false),
            pets: Some(false),
            cars: Some(false),
        };
        
        let train_xml = TrainXML {
            system_prompts: Some(TrainXMLSystemPrompts {
                system: vec![TrainXMLSystemPromptsSystem {
                    id: "sy_default".to_string(),
                    content: "You are a computer programming assistant.".to_string(),
                }],
            }),
            responses: Some(TrainXMLResponses {
                response: vec![TrainXMLResponsesResponse {
                    id: "re_beyond_scope".to_string(),
                    content: "I'm sorry, I don't know, I'm a computer programming assistant".to_string(),
                }],
            }),
            prompts: None,
            sources: None,
            code_snippets: None,
            samples: None,
            constants: None,
            phrases: None,
            beyond_scope: Some(beyond_scope),
        };
        
        let train_xml_ids = create_test_train_xml_ids_with_system_and_response();
        let samples = beyond_scope_create_samples(&train_xml, &train_xml_ids);
        
        // Should return empty because response not found
        assert!(samples.is_empty());
    }
}
