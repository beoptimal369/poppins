// src/train_xml/train_xml_merge_beyond_scope.rs

use crate::train_xml::TrainXML;


/// Merge beyond-scope configurations from all train XML files into the target train_xml
///
/// For scalar fields (system, response, thought, boolean flags): first non-None value wins
/// For topics: all topics from all files are collected (order preserved, deduplicated by value+prefix)
pub fn train_xml_merge_beyond_scope(
    train_xmls: &[TrainXML],
    train_xml: &mut TrainXML,
) {
    let mut merged = None;
    let mut all_topics = Vec::new();
    let mut topic_set = std::collections::HashSet::new();
    
    // Iterate through all train_xmls in order (first = highest priority)
    for xml in train_xmls {
        if let Some(beyond_scope) = &xml.beyond_scope {
            if merged.is_none() {
                // Start with a clone of the first beyond-scope config
                merged = Some(beyond_scope.clone());
            } else {
                let merged_ref = merged.as_mut().unwrap();
                
                // For each field, only update if current value is None and source has value
                if merged_ref.system.is_empty() && !beyond_scope.system.is_empty() {
                    merged_ref.system = beyond_scope.system.clone();
                }
                if merged_ref.response.is_empty() && !beyond_scope.response.is_empty() {
                    merged_ref.response = beyond_scope.response.clone();
                }
                if merged_ref.thought.is_none() && beyond_scope.thought.is_some() {
                    merged_ref.thought = beyond_scope.thought.clone();
                }
                if merged_ref.sports.is_none() && beyond_scope.sports.is_some() {
                    merged_ref.sports = beyond_scope.sports;
                }
                if merged_ref.food.is_none() && beyond_scope.food.is_some() {
                    merged_ref.food = beyond_scope.food;
                }
                if merged_ref.movies.is_none() && beyond_scope.movies.is_some() {
                    merged_ref.movies = beyond_scope.movies;
                }
                if merged_ref.history.is_none() && beyond_scope.history.is_some() {
                    merged_ref.history = beyond_scope.history;
                }
                if merged_ref.geography.is_none() && beyond_scope.geography.is_some() {
                    merged_ref.geography = beyond_scope.geography;
                }
                if merged_ref.politics.is_none() && beyond_scope.politics.is_some() {
                    merged_ref.politics = beyond_scope.politics;
                }
                if merged_ref.science.is_none() && beyond_scope.science.is_some() {
                    merged_ref.science = beyond_scope.science;
                }
                if merged_ref.health.is_none() && beyond_scope.health.is_some() {
                    merged_ref.health = beyond_scope.health;
                }
                if merged_ref.art.is_none() && beyond_scope.art.is_some() {
                    merged_ref.art = beyond_scope.art;
                }
                if merged_ref.music.is_none() && beyond_scope.music.is_some() {
                    merged_ref.music = beyond_scope.music;
                }
                if merged_ref.fashion.is_none() && beyond_scope.fashion.is_some() {
                    merged_ref.fashion = beyond_scope.fashion;
                }
                if merged_ref.travel.is_none() && beyond_scope.travel.is_some() {
                    merged_ref.travel = beyond_scope.travel;
                }
                if merged_ref.pets.is_none() && beyond_scope.pets.is_some() {
                    merged_ref.pets = beyond_scope.pets;
                }
                if merged_ref.cars.is_none() && beyond_scope.cars.is_some() {
                    merged_ref.cars = beyond_scope.cars;
                }
            }
            
            // Collect topics from this file (with deduplication)
            for topic in &beyond_scope.topics {
                let key = format!("{}|{}", topic.value, topic.prefix);
                if !topic_set.contains(&key) {
                    topic_set.insert(key);
                    all_topics.push(topic.clone());
                }
            }
        }
    }
    
    // Apply merged data
    if let Some(mut merged_config) = merged {
        merged_config.topics = all_topics;
        train_xml.beyond_scope = Some(merged_config);
    }
}



#[cfg(test)]
mod tests {
    use crate::train_xml::{
        TrainXML,
        TrainXMLBeyondScope,
        TrainXMLBeyondScopeTopic,
        train_xml_merge_beyond_scope,
    };

    fn create_beyond_scope(
        system: &str,
        thought: Option<&str>,
        response: &str,
        sports: Option<bool>,
        food: Option<bool>,
        movies: Option<bool>,
        history: Option<bool>,
        geography: Option<bool>,
        politics: Option<bool>,
        science: Option<bool>,
        health: Option<bool>,
        art: Option<bool>,
        music: Option<bool>,
        fashion: Option<bool>,
        travel: Option<bool>,
        pets: Option<bool>,
        cars: Option<bool>,
        topics: Vec<TrainXMLBeyondScopeTopic>,
    ) -> TrainXMLBeyondScope {
        TrainXMLBeyondScope {
            system: system.to_string(),
            thought: thought.map(|s| s.to_string()),
            response: response.to_string(),
            sports,
            food,
            movies,
            history,
            geography,
            politics,
            science,
            health,
            art,
            music,
            fashion,
            travel,
            pets,
            cars,
            topics,
        }
    }

    fn create_topic(value: &str, prefix: &str) -> TrainXMLBeyondScopeTopic {
        TrainXMLBeyondScopeTopic {
            value: value.to_string(),
            prefix: prefix.to_string(),
        }
    }

    #[test]
    fn test_merge_beyond_scope_no_beyond_scope() {
        let train_xmls = vec![
            TrainXML::default(),
            TrainXML::default(),
        ];
        let mut merged = TrainXML::default();
        
        train_xml_merge_beyond_scope(&train_xmls, &mut merged);
        
        assert!(merged.beyond_scope.is_none());
    }

    #[test]
    fn test_merge_beyond_scope_single_file() {
        let beyond_scope = create_beyond_scope(
            "sy1",
            Some("thought1"),
            "resp1",
            Some(true),
            Some(false),
            Some(true),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            vec![create_topic("quantum computing", "is")],
        );
        
        let train_xmls = vec![
            TrainXML {
                beyond_scope: Some(beyond_scope.clone()),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        train_xml_merge_beyond_scope(&train_xmls, &mut merged);
        
        assert!(merged.beyond_scope.is_some());
        let merged_beyond = merged.beyond_scope.unwrap();
        assert_eq!(merged_beyond.system, "sy1");
        assert_eq!(merged_beyond.thought, Some("thought1".to_string()));
        assert_eq!(merged_beyond.response, "resp1");
        assert_eq!(merged_beyond.sports, Some(true));
        assert_eq!(merged_beyond.movies, Some(true));
        assert_eq!(merged_beyond.topics.len(), 1);
        assert_eq!(merged_beyond.topics[0].value, "quantum computing");
    }

    #[test]
    fn test_merge_beyond_scope_two_files_field_level_priority() {
        let beyond_scope1 = create_beyond_scope(
            "sys_high",
            Some("thought_high"),
            "resp_high",
            Some(true),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            vec![create_topic("high_priority", "is")],
        );
        
        let beyond_scope2 = create_beyond_scope(
            "sys_low",
            Some("thought_low"),
            "resp_low",
            Some(false),
            Some(true),  // New field not in first
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            vec![create_topic("low_priority", "is")],
        );
        
        let train_xmls = vec![
            TrainXML {
                beyond_scope: Some(beyond_scope1),
                ..Default::default()
            },
            TrainXML {
                beyond_scope: Some(beyond_scope2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        train_xml_merge_beyond_scope(&train_xmls, &mut merged);
        
        assert!(merged.beyond_scope.is_some());
        let merged_beyond = merged.beyond_scope.unwrap();
        
        // First file values win for fields they have
        assert_eq!(merged_beyond.system, "sys_high");
        assert_eq!(merged_beyond.thought, Some("thought_high".to_string()));
        assert_eq!(merged_beyond.response, "resp_high");
        assert_eq!(merged_beyond.sports, Some(true));
        
        // Second file's new field is included
        assert_eq!(merged_beyond.food, Some(true));
        
        // Both topics are included
        assert_eq!(merged_beyond.topics.len(), 2);
        let topic_values: Vec<String> = merged_beyond.topics.iter().map(|t| t.value.clone()).collect();
        assert!(topic_values.contains(&"high_priority".to_string()));
        assert!(topic_values.contains(&"low_priority".to_string()));
    }

    #[test]
    fn test_merge_beyond_scope_three_files_all_fields_collected() {
        let beyond_scope1 = create_beyond_scope(
            "sy1",
            None,
            "",
            Some(true),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            vec![create_topic("topic1", "is")],
        );
        
        let beyond_scope2 = create_beyond_scope(
            "",
            Some("thought2"),
            "resp2",
            None,
            Some(true),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            vec![create_topic("topic2", "are")],
        );
        
        let beyond_scope3 = create_beyond_scope(
            "sy2",
            None,
            "",
            None,
            None,
            Some(true),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            vec![create_topic("topic3", "is")],
        );
        
        let train_xmls = vec![
            TrainXML {
                beyond_scope: Some(beyond_scope1),
                ..Default::default()
            },
            TrainXML {
                beyond_scope: Some(beyond_scope2),
                ..Default::default()
            },
            TrainXML {
                beyond_scope: Some(beyond_scope3),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        train_xml_merge_beyond_scope(&train_xmls, &mut merged);
        
        assert!(merged.beyond_scope.is_some());
        let merged_beyond = merged.beyond_scope.unwrap();
        
        // First file wins for system
        assert_eq!(merged_beyond.system, "sy1");
        // Second file wins for thought (first had None)
        assert_eq!(merged_beyond.thought, Some("thought2".to_string()));
        // Second file wins for response (first had empty)
        assert_eq!(merged_beyond.response, "resp2");
        // First file wins for sports
        assert_eq!(merged_beyond.sports, Some(true));
        // Second file adds food
        assert_eq!(merged_beyond.food, Some(true));
        // Third file adds movies
        assert_eq!(merged_beyond.movies, Some(true));
        
        // All topics collected
        assert_eq!(merged_beyond.topics.len(), 3);
    }

    #[test]
    fn test_merge_beyond_scope_topics_deduplication() {
        let beyond_scope1 = create_beyond_scope(
            "sy1",
            None,
            "resp1",
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            vec![
                create_topic("topic1", "is"),
                create_topic("topic2", "are"),
            ],
        );
        
        let beyond_scope2 = create_beyond_scope(
            "sy1",
            None,
            "resp1",
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            vec![
                create_topic("topic2", "are"),  // Duplicate
                create_topic("topic3", "is"),
            ],
        );
        
        let train_xmls = vec![
            TrainXML {
                beyond_scope: Some(beyond_scope1),
                ..Default::default()
            },
            TrainXML {
                beyond_scope: Some(beyond_scope2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        train_xml_merge_beyond_scope(&train_xmls, &mut merged);
        
        assert!(merged.beyond_scope.is_some());
        let merged_beyond = merged.beyond_scope.unwrap();
        
        // No duplicate topics
        assert_eq!(merged_beyond.topics.len(), 3);
        let topic_keys: Vec<String> = merged_beyond.topics.iter()
            .map(|t| format!("{}|{}", t.value, t.prefix))
            .collect();
        assert!(topic_keys.contains(&"topic1|is".to_string()));
        assert!(topic_keys.contains(&"topic2|are".to_string()));
        assert!(topic_keys.contains(&"topic3|is".to_string()));
    }

    #[test]
    fn test_merge_beyond_scope_preserves_topic_order() {
        let beyond_scope1 = create_beyond_scope(
            "sy1",
            None,
            "resp1",
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            vec![
                create_topic("first", "is"),
                create_topic("second", "are"),
            ],
        );
        
        let beyond_scope2 = create_beyond_scope(
            "sy1",
            None,
            "resp1",
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            vec![
                create_topic("third", "is"),
                create_topic("fourth", "are"),
            ],
        );
        
        let train_xmls = vec![
            TrainXML {
                beyond_scope: Some(beyond_scope1),
                ..Default::default()
            },
            TrainXML {
                beyond_scope: Some(beyond_scope2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        train_xml_merge_beyond_scope(&train_xmls, &mut merged);
        
        assert!(merged.beyond_scope.is_some());
        let merged_beyond = merged.beyond_scope.unwrap();
        
        // Order preserved
        assert_eq!(merged_beyond.topics.len(), 4);
        assert_eq!(merged_beyond.topics[0].value, "first");
        assert_eq!(merged_beyond.topics[1].value, "second");
        assert_eq!(merged_beyond.topics[2].value, "third");
        assert_eq!(merged_beyond.topics[3].value, "fourth");
    }

    #[test]
    fn test_merge_beyond_scope_missing_fields_filled_by_later_files() {
        let beyond_scope1 = create_beyond_scope(
            "sy1",
            None,
            "resp1",
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            vec![],
        );
        
        let beyond_scope2 = create_beyond_scope(
            "",
            Some("thought2"),
            "",
            Some(true),
            Some(false),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            vec![],
        );
        
        let train_xmls = vec![
            TrainXML {
                beyond_scope: Some(beyond_scope1),
                ..Default::default()
            },
            TrainXML {
                beyond_scope: Some(beyond_scope2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        train_xml_merge_beyond_scope(&train_xmls, &mut merged);
        
        assert!(merged.beyond_scope.is_some());
        let merged_beyond = merged.beyond_scope.unwrap();
        
        // First file's values preserved where present
        assert_eq!(merged_beyond.system, "sy1");
        assert_eq!(merged_beyond.response, "resp1");
        
        // Second file fills in missing fields
        assert_eq!(merged_beyond.thought, Some("thought2".to_string()));
        assert_eq!(merged_beyond.sports, Some(true));
        assert_eq!(merged_beyond.food, Some(false));
    }
}
