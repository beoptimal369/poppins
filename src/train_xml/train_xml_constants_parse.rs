// src/train_xml/train_xml_constants_parse.rs

use std::error::Error;
use crate::device::Device;
use crate::train_xml::{
    TrainXMLConstants,
    TrainXMLConstantsKey,
    TrainXMLConstantParsed,
};


pub fn train_xml_constants_parse(train_xml_constants: &Option<TrainXMLConstants>, device: &Device) -> Result<TrainXMLConstantParsed, Box<dyn Error>> {
    let mut parsed = TrainXMLConstantParsed::create(device);

    if let Some(inner) = train_xml_constants {
        for c in &inner.constant {
            // Parse based on key - all values are parsed as needed
            match c.key {
                TrainXMLConstantsKey::WarmupSteps => {
                    parsed.warmup_steps = c.value.parse()
                        .map_err(|_| format!("❌ warmup_steps must be an integer, got '{}'", c.value))?;
                }
                TrainXMLConstantsKey::ValInterval => {
                    parsed.val_interval = c.value.parse()
                        .map_err(|_| format!("❌ val_interval must be an integer, got '{}'", c.value))?;
                }
                TrainXMLConstantsKey::AimTrainGb => {
                    parsed.aim_train_gb = c.value.parse()
                        .map_err(|_| format!("❌ aim_train_gb must be a number, got '{}'", c.value))?;
                }
                TrainXMLConstantsKey::AimInferGb => {
                    parsed.aim_infer_gb = c.value.parse()
                        .map_err(|_| format!("❌ aim_infer_gb must be a number, got '{}'", c.value))?;
                }
                TrainXMLConstantsKey::LearningRate => {
                    parsed.learning_rate = c.value.parse()
                        .map_err(|_| format!("❌ learning_rate must be a number, got '{}'", c.value))?;
                }
                TrainXMLConstantsKey::AimLoss => {
                    parsed.aim_loss = c.value.parse()
                        .map_err(|_| format!("❌ aim_loss must be a number, got '{}'", c.value))?;
                }
                TrainXMLConstantsKey::BatchSize => {
                    parsed.batch_size = c.value.parse()
                        .map_err(|_| format!("❌ batch_size must be a number, got '{}'", c.value))?;
                }
                TrainXMLConstantsKey::MixedPrecision => {
                    parsed.mixed_precision = c.value.parse()
                        .map_err(|_| format!("❌ mixed_precision must be a number, got '{}'", c.value))?;
                }
                TrainXMLConstantsKey::GradientAccumulationSteps => {
                    parsed.gradient_accumulation_steps = c.value.parse()
                        .map_err(|_| format!("❌ gradient_accumulation_steps must be a number, got '{}'", c.value))?;
                }
                TrainXMLConstantsKey::ActivationPrecision => {
                    parsed.activation_precision = c.value.parse()
                        .map_err(|_| format!("❌ activation_precision must be a string, got '{}'", c.value))?;
                }
                TrainXMLConstantsKey::KvCachePrecision => {
                    parsed.kv_cache_precision = c.value.parse()
                        .map_err(|_| format!("❌ kv_cache_precision must be a string, got '{}'", c.value))?;
                }
                TrainXMLConstantsKey::RopePrecision => {
                    parsed.rope_precision = c.value.parse()
                        .map_err(|_| format!("❌ rope_precision must be a string, got '{}'", c.value))?;
                }
                TrainXMLConstantsKey::NumWorkers => {
                    parsed.num_workers = c.value.parse()
                        .map_err(|_| format!("❌ num_workers must be a number, got '{}'", c.value))?;
                }
                TrainXMLConstantsKey::UseFlashAttention => {
                    parsed.use_flash_attention = c.value.parse()
                        .map_err(|_| format!("❌ use_flash_attention must be a bool, got '{}'", c.value))?;
                }
                TrainXMLConstantsKey::UseTensorCores => {
                    parsed.use_tensor_cores = c.value.parse()
                        .map_err(|_| format!("❌ use_tensor_cores must be a bool, got '{}'", c.value))?;
                }

                TrainXMLConstantsKey::BpeMinMergeFrequency => {
                    parsed.bpe_min_merge_frequency = c.value.parse()
                        .map_err(|_| format!("❌ bpe_min_merge_frequency must be a number, got '{}'", c.value))?;
                }
                TrainXMLConstantsKey::BpeRequestedTokens => {
                    // Determine delimiter: use provided delimiter or default to "|"
                    let delimiter = c.delimiter.as_ref().map(|s| s.as_str()).unwrap_or("|");
                    
                    // Split the value by delimiter
                    // No trimming - preserve leading/trailing spaces as requested
                    let tokens: Vec<String> = c.value
                        .split(delimiter)
                        .map(|s| s.to_string())
                        .collect();
                    
                    parsed.bpe_requested_tokens = tokens;
                }

                // WeightDecay
                TrainXMLConstantsKey::WeightDecayResponse => {
                    parsed.weight_decay_response = c.value.parse()
                        .map_err(|_| format!("❌ weight_decay_response must be a number, got '{}'", c.value))?;
                }
                TrainXMLConstantsKey::WeightDecaySource => {
                    parsed.weight_decay_source = c.value.parse()
                        .map_err(|_| format!("❌ weight_decay_source must be a number, got '{}'", c.value))?;
                }
                TrainXMLConstantsKey::WeightDecayCode => {
                    parsed.weight_decay_code = c.value.parse()
                        .map_err(|_| format!("❌ weight_decay_code must be a number, got '{}'", c.value))?;
                }

                // LossScale
                TrainXMLConstantsKey::LossScaleResponse => {
                    parsed.loss_scale_response = c.value.parse()
                        .map_err(|_| format!("❌ loss_scale_response must be a number, got '{}'", c.value))?;
                }
                TrainXMLConstantsKey::LossScaleSource => {
                    parsed.loss_scale_source = c.value.parse()
                        .map_err(|_| format!("❌ loss_scale_source must be a number, got '{}'", c.value))?;
                }
                TrainXMLConstantsKey::LossScaleCode => {
                    parsed.loss_scale_code = c.value.parse()
                        .map_err(|_| format!("❌ loss_scale_code must be a number, got '{}'", c.value))?;
                }

                // GradientScale
                TrainXMLConstantsKey::GradientScaleResponse => {
                    parsed.gradient_scale_response = c.value.parse()
                        .map_err(|_| format!("❌ gradient_scale_response must be a number, got '{}'", c.value))?;
                }
                TrainXMLConstantsKey::GradientScaleSource => {
                    parsed.gradient_scale_source = c.value.parse()
                        .map_err(|_| format!("❌ gradient_scale_source must be a number, got '{}'", c.value))?;
                }
                TrainXMLConstantsKey::GradientScaleCode => {
                    parsed.gradient_scale_code = c.value.parse()
                        .map_err(|_| format!("❌ gradient_scale_code must be a number, got '{}'", c.value))?;
                }

                // GradientClip
                TrainXMLConstantsKey::GradientClipResponse => {
                    parsed.gradient_clip_response = c.value.parse()
                        .map_err(|_| format!("❌ gradient_clip_response must be a number, got '{}'", c.value))?;
                }
                TrainXMLConstantsKey::GradientClipSource => {
                    parsed.gradient_clip_source = c.value.parse()
                        .map_err(|_| format!("❌ gradient_clip_source must be a number, got '{}'", c.value))?;
                }
                TrainXMLConstantsKey::GradientClipCode => {
                    parsed.gradient_clip_code = c.value.parse()
                        .map_err(|_| format!("❌ gradient_clip_code must be a number, got '{}'", c.value))?;
                }
            }
        }
    }

    Ok(parsed)
}



#[cfg(test)]
mod tests {
    use crate::device::Device;
    use crate::train_xml::{
        TrainXMLConstants,
        TrainXMLConstantsKey,
        train_xml_constants_parse,
        train_xml_structs::TrainXMLConstantsConstant,
    };

    fn test_device() -> Device {
        Device::Cpu
    }

    #[test]
    fn test_create_success() {
        let device = test_device();
        
        // Create test constants with all valid values
        let constants = TrainXMLConstants {
            constant: vec![
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::WarmupSteps, value: "500".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::ValInterval, value: "25".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::AimTrainGb, value: "8.5".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::AimInferGb, value: "2.1".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::LearningRate, value: "5e-4".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::AimLoss, value: "0.35".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::BatchSize, value: "16".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::MixedPrecision, value: "true".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientAccumulationSteps, value: "4".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::ActivationPrecision, value: "int8".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::KvCachePrecision, value: "int8".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::RopePrecision, value: "fp16".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::NumWorkers, value: "8".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::UseFlashAttention, value: "true".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::UseTensorCores, value: "true".to_string(), delimiter: None },

                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::BpeMinMergeFrequency, value: "6".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::BpeRequestedTokens, value: "function|console.log|hi world".to_string(), delimiter: Some("|".to_owned()) },

                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::WeightDecayResponse, value: "0.2".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::WeightDecaySource, value: "0.02".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::WeightDecayCode, value: "0.08".to_string(), delimiter: None },

                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::LossScaleResponse, value: "1.5".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::LossScaleSource, value: "0.5".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::LossScaleCode, value: "1.2".to_string(), delimiter: None },

                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientScaleResponse, value: "1.1".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientScaleSource, value: "2.5".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientScaleCode, value: "1.8".to_string(), delimiter: None },

                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientClipResponse, value: "1.2".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientClipSource, value: "0.3".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientClipCode, value: "0.9".to_string(), delimiter: None },
            ],
        };

        let result = train_xml_constants_parse(&Some(constants), &device);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.warmup_steps, 500);
        assert_eq!(parsed.val_interval, 25);
        assert_eq!(parsed.aim_train_gb, 8.5);
        assert_eq!(parsed.aim_infer_gb, 2.1);
        assert_eq!(parsed.learning_rate, 0.0005);
        assert_eq!(parsed.aim_loss, 0.35);
        assert_eq!(parsed.batch_size, 16);
        assert_eq!(parsed.mixed_precision, true);
        assert_eq!(parsed.gradient_accumulation_steps, 4);
        assert_eq!(parsed.activation_precision, "int8");
        assert_eq!(parsed.kv_cache_precision, "int8");
        assert_eq!(parsed.rope_precision, "fp16");
        assert_eq!(parsed.num_workers, 8);
        assert_eq!(parsed.use_flash_attention, true);
        assert_eq!(parsed.use_tensor_cores, true);
        assert_eq!(parsed.bpe_min_merge_frequency, 6);
        
        assert_eq!(parsed.weight_decay_response, 0.2);
        assert_eq!(parsed.weight_decay_source, 0.02);
        assert_eq!(parsed.weight_decay_code, 0.08);
        
        assert_eq!(parsed.loss_scale_response, 1.5);
        assert_eq!(parsed.loss_scale_source, 0.5);
        assert_eq!(parsed.loss_scale_code, 1.2);
        
        assert_eq!(parsed.gradient_scale_response, 1.1);
        assert_eq!(parsed.gradient_scale_source, 2.5);
        assert_eq!(parsed.gradient_scale_code, 1.8);
        
        assert_eq!(parsed.gradient_clip_response, 1.2);
        assert_eq!(parsed.gradient_clip_source, 0.3);
        assert_eq!(parsed.gradient_clip_code, 0.9);
    }

    #[test]
    fn test_create_error() {
        let device = test_device();
        
        // Create constants with an invalid value that will fail to parse
        let constants = TrainXMLConstants {
            constant: vec![
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::WarmupSteps, value: "500".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::ValInterval, value: "25".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::AimTrainGb, value: "8.5".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::AimInferGb, value: "2.1".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::LearningRate, value: "not-a-float".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::AimLoss, value: "0.35".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::BatchSize, value: "16".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::MixedPrecision, value: "true".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientAccumulationSteps, value: "4".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::ActivationPrecision, value: "int8".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::KvCachePrecision, value: "int8".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::RopePrecision, value: "fp16".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::NumWorkers, value: "8".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::UseFlashAttention, value: "true".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::UseTensorCores, value: "true".to_string(), delimiter: None },

                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::BpeMinMergeFrequency, value: "9".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::BpeRequestedTokens, value: "function|console.log|hi world".to_string(), delimiter: Some("|".to_owned()) },

                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::WeightDecayResponse, value: "0.2".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::WeightDecaySource, value: "0.02".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::WeightDecayCode, value: "0.08".to_string(), delimiter: None },

                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::LossScaleResponse, value: "1.5".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::LossScaleSource, value: "0.5".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::LossScaleCode, value: "1.2".to_string(), delimiter: None },

                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientScaleResponse, value: "1.1".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientScaleSource, value: "2.5".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientScaleCode, value: "1.8".to_string(), delimiter: None },

                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientClipResponse, value: "1.2".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientClipSource, value: "0.3".to_string(), delimiter: None },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientClipCode, value: "0.9".to_string(), delimiter: None },
            ],
        };

        let result = train_xml_constants_parse(&Some(constants), &device);
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        let error_string = error.to_string();
        assert!(error_string.contains("learning_rate"));
        assert!(error_string.contains("not-a-float"));
    }

    #[test]
    fn test_empty_constants() {
        let device = test_device();
        let result = train_xml_constants_parse(&None, &device);
        assert!(result.is_ok());
        
        let parsed = result.unwrap();
        // Should return defaults from TrainXMLConstantParsed::create()
        assert_eq!(parsed.warmup_steps, 11520);
        assert_eq!(parsed.val_interval, 10);
        assert_eq!(parsed.aim_train_gb, 7.0);
        assert_eq!(parsed.aim_infer_gb, 0.9);
        assert_eq!(parsed.aim_loss, 0.45);
        assert_eq!(parsed.bpe_min_merge_frequency, 3);
        assert_eq!(parsed.batch_size, 1); // CPU default
        assert_eq!(parsed.mixed_precision, false); // CPU default
        assert_eq!(parsed.gradient_accumulation_steps, 1); // CPU default
        assert_eq!(parsed.activation_precision, "fp32"); // CPU default
        assert_eq!(parsed.kv_cache_precision, "int8");
        assert_eq!(parsed.rope_precision, "fp32"); // CPU default
        assert_eq!(parsed.num_workers, num_cpus::get());
        assert_eq!(parsed.use_flash_attention, false); // CPU default
        assert_eq!(parsed.use_tensor_cores, false); // CPU default
    }
}
