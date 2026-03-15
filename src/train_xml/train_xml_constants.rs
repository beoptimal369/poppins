// src/train_xml/train_xml_constants.rs

use std::error::Error;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLConstants {
    /// The sequence of constant elements
    pub constant: Vec<TrainXMLConstantsConstant>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLConstantsConstant {
    /// The key for this constant
    #[serde(rename = "@key")]
    pub key: TrainXMLConstantsKey,

    /// The value for this constant
    #[serde(rename = "@value")]
    pub value: String,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrainXMLConstantsKey {
    AimTrainGb,
    AimInferF16Gb,
    LearningRate,
    WarmupSteps,
    AimLoss,
    ValInterval,
    WeightDecayResponse,
    WeightDecaySource,
    WeightDecayCode,
    DropoutRateResponse,
    DropoutRateSource,
    DropoutRateCode,
    LossScaleResponse,
    LossScaleSource,
    LossScaleCode,
    GradientScaleResponse,
    GradientScaleSource,
    GradientScaleCode,
    GradientClipResponse,
    GradientClipSource,
    GradientClipCode,
}


#[derive(Debug)]
pub struct TrainXMLConstantsParsed {
    /// The sequence of parsed constant elements
    pub constants: Vec<TrainXMLConstantParsed>,
}

impl TrainXMLConstantsParsed {
    /// The primary API: takes the raw XML and returns the collection of parsed structs
    pub fn create(train_xml_constants: &Option<TrainXMLConstants>) -> Result<Self, Box<dyn Error>> {
        let mut parsed_list = Vec::new();

        if let Some(inner) = train_xml_constants {
            // In your current XML structure, you likely have one <constants> block 
            // but we treat the collection here to populate the Vec.
            let mut parsed = TrainXMLConstantParsed::default();
            
            for c in &inner.constant {
                match c.key {
                    TrainXMLConstantsKey::WarmupSteps => parsed.warmup_steps = c.value.parse().expect("❌ warmup_steps should parse"),
                    TrainXMLConstantsKey::ValInterval => parsed.val_interval = c.value.parse().expect("❌ val_interval should parse"),
                    TrainXMLConstantsKey::AimTrainGb => parsed.aim_train_gb = c.value.parse().expect("❌ aim_train_gb should parse"),
                    TrainXMLConstantsKey::AimInferF16Gb => parsed.aim_infer_f16_gb = c.value.parse().expect("❌ aim_infer_f16_gb should parse"),
                    TrainXMLConstantsKey::LearningRate => parsed.learning_rate = c.value.parse().expect("❌ learning_rate should parse"),
                    TrainXMLConstantsKey::AimLoss => parsed.aim_loss = c.value.parse().expect("❌ aim_loss should parse"),

                    // WeightDecay
                    TrainXMLConstantsKey::WeightDecayResponse => parsed.weight_decay_response = c.value.parse().expect("❌ weight_decay_response should parse"),
                    TrainXMLConstantsKey::WeightDecaySource => parsed.weight_decay_source = c.value.parse().expect("❌ weight_decay_source should parse"),
                    TrainXMLConstantsKey::WeightDecayCode => parsed.weight_decay_code = c.value.parse().expect("❌ weight_decay_code should parse"),

                    // DropoutRate
                    TrainXMLConstantsKey::DropoutRateResponse => parsed.dropout_rate_response = c.value.parse().expect("❌ dropout_rate_response should parse"),
                    TrainXMLConstantsKey::DropoutRateSource => parsed.dropout_rate_source = c.value.parse().expect("❌ dropout_rate_source should parse"),
                    TrainXMLConstantsKey::DropoutRateCode => parsed.dropout_rate_code = c.value.parse().expect("❌ dropout_rate_code should parse"),

                    // LossScale
                    TrainXMLConstantsKey::LossScaleResponse => parsed.loss_scale_response = c.value.parse().expect("❌ loss_scale_response should parse"),
                    TrainXMLConstantsKey::LossScaleSource => parsed.loss_scale_source = c.value.parse().expect("❌ loss_scale_source should parse"),
                    TrainXMLConstantsKey::LossScaleCode => parsed.loss_scale_code = c.value.parse().expect("❌ loss_scale_code should parse"),

                    // GradientScale
                    TrainXMLConstantsKey::GradientScaleResponse => parsed.gradient_scale_response = c.value.parse().expect("❌ gradient_scale_response should parse"),
                    TrainXMLConstantsKey::GradientScaleSource => parsed.gradient_scale_source = c.value.parse().expect("❌ gradient_scale_source should parse"),
                    TrainXMLConstantsKey::GradientScaleCode => parsed.gradient_scale_code = c.value.parse().expect("❌ gradient_scale_code should parse"),

                    // GradientClip
                    TrainXMLConstantsKey::GradientClipResponse => parsed.gradient_clip_response = c.value.parse().expect("❌ gradient_clip_response should parse"),
                    TrainXMLConstantsKey::GradientClipSource => parsed.gradient_clip_source = c.value.parse().expect("❌ gradient_clip_source should parse"),
                    TrainXMLConstantsKey::GradientClipCode => parsed.gradient_clip_code = c.value.parse().expect("❌ gradient_clip_code should parse"),
                }
            }
            parsed_list.push(parsed);
        }

        Ok(TrainXMLConstantsParsed { constants: parsed_list })
    }
}


#[derive(Debug)]
pub struct TrainXMLConstantParsed {
    pub warmup_steps: usize,
    pub val_interval: usize,
    pub aim_train_gb: f32,
    pub aim_infer_f16_gb: f32,
    pub learning_rate: f32,
    pub aim_loss: f32,
    
    pub weight_decay_response: f32,
    pub weight_decay_source: f32,
    pub weight_decay_code: f32,

    pub dropout_rate_response: f32,
    pub dropout_rate_source: f32,
    pub dropout_rate_code: f32,

    pub loss_scale_response: f32,
    pub loss_scale_source: f32,
    pub loss_scale_code: f32,

    pub gradient_scale_response: f32,
    pub gradient_scale_source: f32,
    pub gradient_scale_code: f32,

    pub gradient_clip_response: f32,
    pub gradient_clip_source: f32,
    pub gradient_clip_code: f32,
}


impl Default for TrainXMLConstantParsed {
    fn default() -> Self {
        Self {
            warmup_steps: 100,
            val_interval: 10,
            aim_train_gb: 3.0,
            aim_infer_f16_gb: 0.9,
            learning_rate: 1e-3,
            aim_loss: 0.45,

            weight_decay_response: 0.1,
            weight_decay_source: 0.01,
            weight_decay_code: 0.05,

            dropout_rate_response: 0.05,
            dropout_rate_source: 0.0,
            dropout_rate_code: 0.1,

            loss_scale_response: 1.0,
            loss_scale_source: 0.2,
            loss_scale_code: 1.0,

            gradient_scale_response: 1.0,
            gradient_scale_source: 2.0,
            gradient_scale_code: 1.2,

            gradient_clip_response: 1.0,
            gradient_clip_source: 0.0,
            gradient_clip_code: 0.7,
        }
    }
}


// src/train_xml/train_xml_constants.rs (add these tests at the end of the file)

#[cfg(test)]
mod tests {
    use crate::train_xml::train_xml_constants::{
        TrainXMLConstants,
        TrainXMLConstantsKey,
        TrainXMLConstantsParsed,
        TrainXMLConstantsConstant,
    };

    #[test]
    fn test_create_success() {
        // Create test constants with all valid values
        let constants = TrainXMLConstants {
            constant: vec![
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::WarmupSteps, value: "500".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::ValInterval, value: "25".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::AimTrainGb, value: "8.5".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::AimInferF16Gb, value: "2.1".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::LearningRate, value: "5e-4".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::AimLoss, value: "0.35".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::WeightDecayResponse, value: "0.2".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::WeightDecaySource, value: "0.02".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::WeightDecayCode, value: "0.08".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::DropoutRateResponse, value: "0.1".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::DropoutRateSource, value: "0.05".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::DropoutRateCode, value: "0.15".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::LossScaleResponse, value: "1.5".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::LossScaleSource, value: "0.5".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::LossScaleCode, value: "1.2".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientScaleResponse, value: "1.1".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientScaleSource, value: "2.5".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientScaleCode, value: "1.8".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientClipResponse, value: "1.2".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientClipSource, value: "0.3".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientClipCode, value: "0.9".to_string() },
            ],
        };

        let result = TrainXMLConstantsParsed::create(&Some(constants));
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.constants.len(), 1);
        
        let parsed = &parsed.constants[0];
        assert_eq!(parsed.warmup_steps, 500);
        assert_eq!(parsed.val_interval, 25);
        assert_eq!(parsed.aim_train_gb, 8.5);
        assert_eq!(parsed.aim_infer_f16_gb, 2.1);
        assert_eq!(parsed.learning_rate, 0.0005);
        assert_eq!(parsed.aim_loss, 0.35);
        
        assert_eq!(parsed.weight_decay_response, 0.2);
        assert_eq!(parsed.weight_decay_source, 0.02);
        assert_eq!(parsed.weight_decay_code, 0.08);
        
        assert_eq!(parsed.dropout_rate_response, 0.1);
        assert_eq!(parsed.dropout_rate_source, 0.05);
        assert_eq!(parsed.dropout_rate_code, 0.15);
        
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
    #[should_panic(expected = "❌ learning_rate should parse")]
    fn test_create_error() {
        // Create constants with an invalid value that will fail to parse
        let constants = TrainXMLConstants {
            constant: vec![
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::WarmupSteps, value: "500".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::ValInterval, value: "25".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::AimTrainGb, value: "8.5".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::AimInferF16Gb, value: "2.1".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::LearningRate, value: "not-a-float".to_string() }, // This will panic
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::AimLoss, value: "0.35".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::WeightDecayResponse, value: "0.2".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::WeightDecaySource, value: "0.02".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::WeightDecayCode, value: "0.08".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::DropoutRateResponse, value: "0.1".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::DropoutRateSource, value: "0.05".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::DropoutRateCode, value: "0.15".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::LossScaleResponse, value: "1.5".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::LossScaleSource, value: "0.5".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::LossScaleCode, value: "1.2".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientScaleResponse, value: "1.1".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientScaleSource, value: "2.5".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientScaleCode, value: "1.8".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientClipResponse, value: "1.2".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientClipSource, value: "0.3".to_string() },
                TrainXMLConstantsConstant { key: TrainXMLConstantsKey::GradientClipCode, value: "0.9".to_string() },
            ],
        };

        // This will panic with the expected message
        let _ = TrainXMLConstantsParsed::create(&Some(constants));
    }
}
