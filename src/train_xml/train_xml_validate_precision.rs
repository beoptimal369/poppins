// src/train_xml/train_xml_validate_precision.rs

use crate::train_xml::TrainXML;


/// Validates precision string values are within allowed enumerations
pub fn train_xml_validate_precision(train_xml: &TrainXML) -> Result<(), String> {
    let mut errors = Vec::new();
    
    if let Some(constants) = &train_xml.constants {
        // Validate activation_precision
        if let Some(precision) = &constants.activation_precision {
            match precision.as_str() {
                "fp32" | "int8" => (),
                _ => errors.push(format!(
                    "Invalid activation_precision '{}'. Must be one of: fp32, int8",
                    precision
                )),
            }
        }
        
        // Validate kv_cache_precision
        if let Some(precision) = &constants.kv_cache_precision {
            match precision.as_str() {
                "int8" | "int4" | "fp8" => (),
                _ => errors.push(format!(
                    "Invalid kv_cache_precision '{}'. Must be one of: int8, int4, fp8",
                    precision
                )),
            }
        }
        
        // Validate rope_precision
        if let Some(precision) = &constants.rope_precision {
            match precision.as_str() {
                "fp32" | "fp16" => (),
                _ => errors.push(format!(
                    "Invalid rope_precision '{}'. Must be one of: fp32, fp16",
                    precision
                )),
            }
        }
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("\n"))
    }
}




#[cfg(test)]
mod tests {
    use super::train_xml_validate_precision;
    use crate::train_xml::{
        TrainXML,
        TrainXMLConstants
    };

    fn create_train_xml_with_precisions(
        activation: Option<&str>,
        kv_cache: Option<&str>,
        rope: Option<&str>,
    ) -> TrainXML {
        TrainXML {
            constants: Some(TrainXMLConstants {
                aim_train_gb: None,
                aim_infer_gb: None,
                aim_loss: None,
                learning_rate: None,
                warmup_steps: None,
                val_interval: None,
                batch_size: None,
                mixed_precision: None,
                gradient_accumulation_steps: None,
                activation_precision: activation.map(String::from),
                kv_cache_precision: kv_cache.map(String::from),
                rope_precision: rope.map(String::from),
                num_workers: None,
                use_flash_attention: None,
                use_tensor_cores: None,
                bpe_min_merge_frequency: None,
                bpe_requested_tokens: None,
                weight_decay_response: None,
                weight_decay_source: None,
                weight_decay_code: None,
                dropout_rate_response: None,
                dropout_rate_source: None,
                dropout_rate_code: None,
                loss_scale_response: None,
                loss_scale_source: None,
                loss_scale_code: None,
                gradient_scale_response: None,
                gradient_scale_source: None,
                gradient_scale_code: None,
                gradient_clip_response: None,
                gradient_clip_source: None,
                gradient_clip_code: None,
            }),
            ..Default::default()
        }
    }

    // ========== activation_precision Tests ==========

    #[test]
    fn test_validate_activation_precision_valid_fp32() {
        let train_xml = create_train_xml_with_precisions(Some("fp32"), None, None);
        let result = train_xml_validate_precision(&train_xml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_activation_precision_valid_int8() {
        let train_xml = create_train_xml_with_precisions(Some("int8"), None, None);
        let result = train_xml_validate_precision(&train_xml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_activation_precision_invalid_fp16() {
        let train_xml = create_train_xml_with_precisions(Some("fp16"), None, None);
        let result = train_xml_validate_precision(&train_xml);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Invalid activation_precision 'fp16'"));
        assert!(err.contains("fp32, int8"));
    }

    #[test]
    fn test_validate_activation_precision_invalid_bf16() {
        let train_xml = create_train_xml_with_precisions(Some("bf16"), None, None);
        let result = train_xml_validate_precision(&train_xml);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid activation_precision 'bf16'"));
    }

    #[test]
    fn test_validate_activation_precision_invalid_unknown() {
        let train_xml = create_train_xml_with_precisions(Some("unknown"), None, None);
        let result = train_xml_validate_precision(&train_xml);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid activation_precision 'unknown'"));
    }

    // ========== kv_cache_precision Tests ==========

    #[test]
    fn test_validate_kv_cache_precision_valid_int8() {
        let train_xml = create_train_xml_with_precisions(None, Some("int8"), None);
        let result = train_xml_validate_precision(&train_xml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_kv_cache_precision_valid_int4() {
        let train_xml = create_train_xml_with_precisions(None, Some("int4"), None);
        let result = train_xml_validate_precision(&train_xml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_kv_cache_precision_valid_fp8() {
        let train_xml = create_train_xml_with_precisions(None, Some("fp8"), None);
        let result = train_xml_validate_precision(&train_xml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_kv_cache_precision_invalid_int16() {
        let train_xml = create_train_xml_with_precisions(None, Some("int16"), None);
        let result = train_xml_validate_precision(&train_xml);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Invalid kv_cache_precision 'int16'"));
        assert!(err.contains("int8, int4, fp8"));
    }

    #[test]
    fn test_validate_kv_cache_precision_invalid_fp16() {
        let train_xml = create_train_xml_with_precisions(None, Some("fp16"), None);
        let result = train_xml_validate_precision(&train_xml);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid kv_cache_precision 'fp16'"));
    }

    #[test]
    fn test_validate_kv_cache_precision_invalid_unknown() {
        let train_xml = create_train_xml_with_precisions(None, Some("unknown"), None);
        let result = train_xml_validate_precision(&train_xml);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid kv_cache_precision 'unknown'"));
    }

    // ========== rope_precision Tests ==========

    #[test]
    fn test_validate_rope_precision_valid_fp32() {
        let train_xml = create_train_xml_with_precisions(None, None, Some("fp32"));
        let result = train_xml_validate_precision(&train_xml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_rope_precision_valid_fp16() {
        let train_xml = create_train_xml_with_precisions(None, None, Some("fp16"));
        let result = train_xml_validate_precision(&train_xml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_rope_precision_invalid_bf16() {
        let train_xml = create_train_xml_with_precisions(None, None, Some("bf16"));
        let result = train_xml_validate_precision(&train_xml);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Invalid rope_precision 'bf16'"));
        assert!(err.contains("fp32, fp16"));
    }

    #[test]
    fn test_validate_rope_precision_invalid_int8() {
        let train_xml = create_train_xml_with_precisions(None, None, Some("int8"));
        let result = train_xml_validate_precision(&train_xml);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid rope_precision 'int8'"));
    }

    #[test]
    fn test_validate_rope_precision_invalid_unknown() {
        let train_xml = create_train_xml_with_precisions(None, None, Some("unknown"));
        let result = train_xml_validate_precision(&train_xml);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid rope_precision 'unknown'"));
    }

    // ========== Combined Tests ==========

    #[test]
    fn test_validate_precision_all_valid() {
        let train_xml = create_train_xml_with_precisions(Some("int8"), Some("fp8"), Some("fp16"));
        let result = train_xml_validate_precision(&train_xml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_precision_all_invalid() {
        let train_xml = create_train_xml_with_precisions(Some("fp16"), Some("int16"), Some("bf16"));
        let result = train_xml_validate_precision(&train_xml);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Invalid activation_precision 'fp16'"));
        assert!(err.contains("Invalid kv_cache_precision 'int16'"));
        assert!(err.contains("Invalid rope_precision 'bf16'"));
    }

    #[test]
    fn test_validate_precision_mixed_valid_invalid() {
        let train_xml = create_train_xml_with_precisions(Some("int8"), Some("int16"), Some("fp16"));
        let result = train_xml_validate_precision(&train_xml);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(!err.contains("activation_precision")); // valid
        assert!(err.contains("Invalid kv_cache_precision 'int16'"));
        assert!(!err.contains("rope_precision")); // valid
    }

    #[test]
    fn test_validate_precision_none_constants() {
        let train_xml = TrainXML {
            constants: None,
            ..Default::default()
        };
        let result = train_xml_validate_precision(&train_xml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_precision_empty_constants() {
        let train_xml = TrainXML {
            constants: Some(TrainXMLConstants {
                activation_precision: None,
                kv_cache_precision: None,
                rope_precision: None,
                ..Default::default()
            }),
            ..Default::default()
        };
        let result = train_xml_validate_precision(&train_xml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_precision_case_sensitivity() {
        // Should be case-sensitive (lowercase only)
        let train_xml = create_train_xml_with_precisions(Some("FP32"), None, None);
        let result = train_xml_validate_precision(&train_xml);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid activation_precision 'FP32'"));
    }

    #[test]
    fn test_validate_precision_whitespace_handling() {
        // Should not trim whitespace
        let train_xml = create_train_xml_with_precisions(Some(" fp32 "), None, None);
        let result = train_xml_validate_precision(&train_xml);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid activation_precision ' fp32 '"));
    }

    #[test]
    fn test_validate_precision_all_precision_types_present() {
        let train_xml = create_train_xml_with_precisions(Some("int8"), Some("int4"), Some("fp32"));
        let result = train_xml_validate_precision(&train_xml);
        assert!(result.is_ok());
    }
}
