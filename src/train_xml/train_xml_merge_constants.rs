// src/train_xml/train_xml_merge_constants.rs

use crate::train_xml::{TrainXML, TrainXMLConstants};


/// Merge constants from all train XML files into the target train_xml
///
/// Priority order (first in vector = highest priority):
/// - train_xmls[0] (main file) has highest priority
/// - Later imports have lower priority
/// - Values from higher priority files override lower priority ones
pub fn train_xml_merge_constants(
    train_xmls: &[TrainXML],
    train_xml: &mut TrainXML,
) {
    // Collect all constants in priority order (first = highest priority)
    let mut all_constants = Vec::new();
    
    for xml in train_xmls {
        if let Some(constants) = &xml.constants {
            all_constants.push(constants);
        }
    }
    
    if all_constants.is_empty() {
        return;
    }
    
    // Start with the HIGHEST priority (first in vector)
    // Then merge LOWER priority into it (lower priority only fills gaps)
    let mut merged = (*all_constants[0]).clone();
    
    for constants in &all_constants[1..] {
        merge_constants_into(&mut merged, constants);
    }
    
    train_xml.constants = Some(merged);
}


fn merge_constants_into(target: &mut TrainXMLConstants, source: &TrainXMLConstants) {
    // IMPORTANT: Only update target if source has a value AND target DOES NOT have a value
    // This preserves higher priority values (target) and only fills missing values from lower priority (source)
    
    if source.aim_train_gb.is_some() && target.aim_train_gb.is_none() {
        target.aim_train_gb = source.aim_train_gb;
    }
    if source.aim_infer_gb.is_some() && target.aim_infer_gb.is_none() {
        target.aim_infer_gb = source.aim_infer_gb;
    }
    if source.aim_loss.is_some() && target.aim_loss.is_none() {
        target.aim_loss = source.aim_loss;
    }
    if source.learning_rate.is_some() && target.learning_rate.is_none() {
        target.learning_rate = source.learning_rate;
    }
    if source.warmup_steps.is_some() && target.warmup_steps.is_none() {
        target.warmup_steps = source.warmup_steps;
    }
    if source.val_interval.is_some() && target.val_interval.is_none() {
        target.val_interval = source.val_interval;
    }
    if source.batch_size.is_some() && target.batch_size.is_none() {
        target.batch_size = source.batch_size;
    }
    if source.mixed_precision.is_some() && target.mixed_precision.is_none() {
        target.mixed_precision = source.mixed_precision;
    }
    if source.gradient_accumulation_steps.is_some() && target.gradient_accumulation_steps.is_none() {
        target.gradient_accumulation_steps = source.gradient_accumulation_steps;
    }
    if source.activation_precision.is_some() && target.activation_precision.is_none() {
        target.activation_precision = source.activation_precision.clone();
    }
    if source.kv_cache_precision.is_some() && target.kv_cache_precision.is_none() {
        target.kv_cache_precision = source.kv_cache_precision.clone();
    }
    if source.rope_precision.is_some() && target.rope_precision.is_none() {
        target.rope_precision = source.rope_precision.clone();
    }
    if source.num_workers.is_some() && target.num_workers.is_none() {
        target.num_workers = source.num_workers;
    }
    if source.use_flash_attention.is_some() && target.use_flash_attention.is_none() {
        target.use_flash_attention = source.use_flash_attention;
    }
    if source.use_tensor_cores.is_some() && target.use_tensor_cores.is_none() {
        target.use_tensor_cores = source.use_tensor_cores;
    }
    if source.bpe_min_merge_frequency.is_some() && target.bpe_min_merge_frequency.is_none() {
        target.bpe_min_merge_frequency = source.bpe_min_merge_frequency;
    }
    if source.bpe_requested_tokens.is_some() && target.bpe_requested_tokens.is_none() {
        target.bpe_requested_tokens = source.bpe_requested_tokens.clone();
    }
    if source.weight_decay_response.is_some() && target.weight_decay_response.is_none() {
        target.weight_decay_response = source.weight_decay_response;
    }
    if source.weight_decay_source.is_some() && target.weight_decay_source.is_none() {
        target.weight_decay_source = source.weight_decay_source;
    }
    if source.weight_decay_code.is_some() && target.weight_decay_code.is_none() {
        target.weight_decay_code = source.weight_decay_code;
    }
    if source.dropout_rate_response.is_some() && target.dropout_rate_response.is_none() {
        target.dropout_rate_response = source.dropout_rate_response;
    }
    if source.dropout_rate_source.is_some() && target.dropout_rate_source.is_none() {
        target.dropout_rate_source = source.dropout_rate_source;
    }
    if source.dropout_rate_code.is_some() && target.dropout_rate_code.is_none() {
        target.dropout_rate_code = source.dropout_rate_code;
    }
    if source.loss_scale_response.is_some() && target.loss_scale_response.is_none() {
        target.loss_scale_response = source.loss_scale_response;
    }
    if source.loss_scale_source.is_some() && target.loss_scale_source.is_none() {
        target.loss_scale_source = source.loss_scale_source;
    }
    if source.loss_scale_code.is_some() && target.loss_scale_code.is_none() {
        target.loss_scale_code = source.loss_scale_code;
    }
    if source.gradient_scale_response.is_some() && target.gradient_scale_response.is_none() {
        target.gradient_scale_response = source.gradient_scale_response;
    }
    if source.gradient_scale_source.is_some() && target.gradient_scale_source.is_none() {
        target.gradient_scale_source = source.gradient_scale_source;
    }
    if source.gradient_scale_code.is_some() && target.gradient_scale_code.is_none() {
        target.gradient_scale_code = source.gradient_scale_code;
    }
    if source.gradient_clip_response.is_some() && target.gradient_clip_response.is_none() {
        target.gradient_clip_response = source.gradient_clip_response;
    }
    if source.gradient_clip_source.is_some() && target.gradient_clip_source.is_none() {
        target.gradient_clip_source = source.gradient_clip_source;
    }
    if source.gradient_clip_code.is_some() && target.gradient_clip_code.is_none() {
        target.gradient_clip_code = source.gradient_clip_code;
    }
}



#[cfg(test)]
mod tests {
    use crate::train_xml::{
        TrainXML,
        TrainXMLConstants,
        train_xml_merge_constants,
        TrainXMLBpeRequestedTokens,
    };

    fn create_constants_with_values() -> TrainXMLConstants {
        TrainXMLConstants {
            aim_train_gb: Some(10.0),
            aim_infer_gb: Some(5.0),
            aim_loss: Some(0.5),
            learning_rate: Some(0.01),
            warmup_steps: Some(1000),
            val_interval: Some(100),
            batch_size: Some(64),
            mixed_precision: Some(true),
            gradient_accumulation_steps: Some(4),
            activation_precision: Some("int8".to_string()),
            kv_cache_precision: Some("int4".to_string()),
            rope_precision: Some("fp16".to_string()),
            num_workers: Some(8),
            use_flash_attention: Some(true),
            use_tensor_cores: Some(true),
            bpe_min_merge_frequency: Some(5),
            bpe_requested_tokens: Some(TrainXMLBpeRequestedTokens {
                values: vec!["token1".to_string(), "token2".to_string()],
            }),
            weight_decay_response: Some(0.1),
            weight_decay_source: Some(0.05),
            weight_decay_code: Some(0.02),
            dropout_rate_response: Some(0.2),
            dropout_rate_source: Some(0.1),
            dropout_rate_code: Some(0.05),
            loss_scale_response: Some(2.0),
            loss_scale_source: Some(1.5),
            loss_scale_code: Some(1.0),
            gradient_scale_response: Some(3.0),
            gradient_scale_source: Some(2.0),
            gradient_scale_code: Some(1.0),
            gradient_clip_response: Some(2.0),
            gradient_clip_source: Some(1.5),
            gradient_clip_code: Some(1.0),
        }
    }

    fn create_constants_with_override_values() -> TrainXMLConstants {
        TrainXMLConstants {
            aim_train_gb: Some(20.0),
            aim_infer_gb: Some(10.0),
            aim_loss: Some(0.3),
            learning_rate: Some(0.02),
            warmup_steps: Some(2000),
            val_interval: Some(200),
            batch_size: Some(128),
            mixed_precision: Some(false),
            gradient_accumulation_steps: Some(8),
            activation_precision: Some("fp32".to_string()),
            kv_cache_precision: Some("int8".to_string()),
            rope_precision: Some("fp32".to_string()),
            num_workers: Some(16),
            use_flash_attention: Some(false),
            use_tensor_cores: Some(false),
            bpe_min_merge_frequency: Some(10),
            bpe_requested_tokens: Some(TrainXMLBpeRequestedTokens {
                values: vec!["token3".to_string(), "token4".to_string()],
            }),
            weight_decay_response: Some(0.2),
            weight_decay_source: Some(0.1),
            weight_decay_code: Some(0.04),
            dropout_rate_response: Some(0.4),
            dropout_rate_source: Some(0.2),
            dropout_rate_code: Some(0.1),
            loss_scale_response: Some(4.0),
            loss_scale_source: Some(3.0),
            loss_scale_code: Some(2.0),
            gradient_scale_response: Some(6.0),
            gradient_scale_source: Some(4.0),
            gradient_scale_code: Some(2.0),
            gradient_clip_response: Some(4.0),
            gradient_clip_source: Some(3.0),
            gradient_clip_code: Some(2.0),
        }
    }

    fn create_partial_constants() -> TrainXMLConstants {
        TrainXMLConstants {
            aim_train_gb: Some(15.0),
            warmup_steps: Some(1500),
            batch_size: Some(96),
            ..Default::default()
        }
    }

    #[test]
    fn test_merge_constants_no_constants() {
        let train_xmls = vec![
            TrainXML::default(),
            TrainXML::default(),
        ];
        let mut merged = TrainXML::default();
        
        train_xml_merge_constants(&train_xmls, &mut merged);
        
        assert!(merged.constants.is_none());
    }

    #[test]
    fn test_merge_constants_single_file() {
        let constants = create_constants_with_values();
        let train_xmls = vec![
            TrainXML {
                constants: Some(constants.clone()),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        train_xml_merge_constants(&train_xmls, &mut merged);
        
        assert!(merged.constants.is_some());
        let merged_constants = merged.constants.unwrap();
        assert_eq!(merged_constants.aim_train_gb, Some(10.0));
        assert_eq!(merged_constants.warmup_steps, Some(1000));
        assert_eq!(merged_constants.batch_size, Some(64));
    }

    #[test]
    fn test_merge_constants_two_files_priority() {
        let constants1 = create_constants_with_values();  // Higher priority (10.0)
        let constants2 = create_constants_with_override_values();  // Lower priority (20.0)
        
        let train_xmls = vec![
            TrainXML {
                constants: Some(constants1),
                ..Default::default()
            },
            TrainXML {
                constants: Some(constants2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        train_xml_merge_constants(&train_xmls, &mut merged);
        
        assert!(merged.constants.is_some());
        let merged_constants = merged.constants.unwrap();
        
        // First file (higher priority) should keep its values
        assert_eq!(merged_constants.aim_train_gb, Some(10.0));  // Not 20.0
        assert_eq!(merged_constants.warmup_steps, Some(1000));
        assert_eq!(merged_constants.batch_size, Some(64));
        assert_eq!(merged_constants.mixed_precision, Some(true));
        assert_eq!(merged_constants.activation_precision, Some("int8".to_string()));
    }

    #[test]
    fn test_merge_constants_three_files_priority() {
        let constants1 = create_constants_with_values();  // Highest priority (10.0)
        let constants2 = create_constants_with_override_values();  // Medium priority (20.0)
        let constants3 = create_partial_constants();  // Lowest priority (15.0)
        
        let train_xmls = vec![
            TrainXML {
                constants: Some(constants1),
                ..Default::default()
            },
            TrainXML {
                constants: Some(constants2),
                ..Default::default()
            },
            TrainXML {
                constants: Some(constants3),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        train_xml_merge_constants(&train_xmls, &mut merged);
        
        assert!(merged.constants.is_some());
        let merged_constants = merged.constants.unwrap();
        
        // First file has highest priority - keeps its values
        assert_eq!(merged_constants.aim_train_gb, Some(10.0));  // Not 15.0 or 20.0
        assert_eq!(merged_constants.warmup_steps, Some(1000));  // Not 1500 or 2000
        assert_eq!(merged_constants.batch_size, Some(64));  // Not 96 or 128
    }

    #[test]
    fn test_merge_constants_partial_in_high_priority() {
        let constants1 = create_partial_constants();  // High priority (partial)
        let constants2 = create_constants_with_values();  // Low priority (full)
        
        let train_xmls = vec![
            TrainXML {
                constants: Some(constants1),
                ..Default::default()
            },
            TrainXML {
                constants: Some(constants2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        train_xml_merge_constants(&train_xmls, &mut merged);
        
        assert!(merged.constants.is_some());
        let merged_constants = merged.constants.unwrap();
        
        // High priority's values should win where they exist
        assert_eq!(merged_constants.aim_train_gb, Some(15.0));
        assert_eq!(merged_constants.warmup_steps, Some(1500));
        assert_eq!(merged_constants.batch_size, Some(96));
        
        // Low priority fills missing fields
        assert_eq!(merged_constants.aim_infer_gb, Some(5.0));
        assert_eq!(merged_constants.learning_rate, Some(0.01));
        assert_eq!(merged_constants.mixed_precision, Some(true));
    }

    #[test]
    fn test_merge_constants_skip_none_values() {
        let constants1 = TrainXMLConstants {
            aim_train_gb: Some(10.0),
            warmup_steps: None,
            batch_size: Some(64),
            ..Default::default()
        };
        
        let constants2 = TrainXMLConstants {
            aim_train_gb: None,
            warmup_steps: Some(1000),
            batch_size: None,
            ..Default::default()
        };
        
        let train_xmls = vec![
            TrainXML {
                constants: Some(constants1),
                ..Default::default()
            },
            TrainXML {
                constants: Some(constants2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        train_xml_merge_constants(&train_xmls, &mut merged);
        
        assert!(merged.constants.is_some());
        let merged_constants = merged.constants.unwrap();
        
        // First file's values take precedence
        assert_eq!(merged_constants.aim_train_gb, Some(10.0));
        assert_eq!(merged_constants.warmup_steps, Some(1000)); // Second file fills None
        assert_eq!(merged_constants.batch_size, Some(64));
    }

    #[test]
    fn test_merge_constants_bpe_requested_tokens() {
        let constants1 = TrainXMLConstants {
            bpe_requested_tokens: Some(TrainXMLBpeRequestedTokens {
                values: vec!["high_priority".to_string()],
            }),
            ..Default::default()
        };
        
        let constants2 = TrainXMLConstants {
            bpe_requested_tokens: Some(TrainXMLBpeRequestedTokens {
                values: vec!["low_priority".to_string()],
            }),
            ..Default::default()
        };
        
        let train_xmls = vec![
            TrainXML {
                constants: Some(constants1),
                ..Default::default()
            },
            TrainXML {
                constants: Some(constants2),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        train_xml_merge_constants(&train_xmls, &mut merged);
        
        assert!(merged.constants.is_some());
        let merged_constants = merged.constants.unwrap();
        
        // High priority should win
        assert_eq!(
            merged_constants.bpe_requested_tokens.unwrap().values,
            vec!["high_priority".to_string()]
        );
    }

    #[test]
    fn test_merge_constants_all_fields() {
        let constants = create_constants_with_values();
        
        let train_xmls = vec![
            TrainXML {
                constants: Some(constants),
                ..Default::default()
            },
        ];
        let mut merged = TrainXML::default();
        
        train_xml_merge_constants(&train_xmls, &mut merged);
        
        assert!(merged.constants.is_some());
        let merged_constants = merged.constants.unwrap();
        
        // Verify all fields are present
        assert!(merged_constants.aim_train_gb.is_some());
        assert!(merged_constants.aim_infer_gb.is_some());
        assert!(merged_constants.aim_loss.is_some());
        assert!(merged_constants.learning_rate.is_some());
        assert!(merged_constants.warmup_steps.is_some());
        assert!(merged_constants.val_interval.is_some());
        assert!(merged_constants.batch_size.is_some());
        assert!(merged_constants.mixed_precision.is_some());
        assert!(merged_constants.gradient_accumulation_steps.is_some());
        assert!(merged_constants.activation_precision.is_some());
        assert!(merged_constants.kv_cache_precision.is_some());
        assert!(merged_constants.rope_precision.is_some());
        assert!(merged_constants.num_workers.is_some());
        assert!(merged_constants.use_flash_attention.is_some());
        assert!(merged_constants.use_tensor_cores.is_some());
        assert!(merged_constants.bpe_min_merge_frequency.is_some());
        assert!(merged_constants.bpe_requested_tokens.is_some());
        assert!(merged_constants.weight_decay_response.is_some());
        assert!(merged_constants.weight_decay_source.is_some());
        assert!(merged_constants.weight_decay_code.is_some());
        assert!(merged_constants.dropout_rate_response.is_some());
        assert!(merged_constants.dropout_rate_source.is_some());
        assert!(merged_constants.dropout_rate_code.is_some());
        assert!(merged_constants.loss_scale_response.is_some());
        assert!(merged_constants.loss_scale_source.is_some());
        assert!(merged_constants.loss_scale_code.is_some());
        assert!(merged_constants.gradient_scale_response.is_some());
        assert!(merged_constants.gradient_scale_source.is_some());
        assert!(merged_constants.gradient_scale_code.is_some());
        assert!(merged_constants.gradient_clip_response.is_some());
        assert!(merged_constants.gradient_clip_source.is_some());
        assert!(merged_constants.gradient_clip_code.is_some());
    }
}
