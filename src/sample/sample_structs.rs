// src/sample/sample_structs.rs

use serde::{Deserialize, Serialize};
use crate::sample::sample_macros::{SampleIndent, SampleLanguage};

#[derive(Debug, Clone)]
pub struct Sample {
    pub system: Option<String>,
    pub thought: Option<String>,
    pub prompt_section: Vec<SamplePromptEnum>,
    pub ai_section: Vec<SampleAiEnum>,
}

#[derive(Debug, Clone)]
pub enum SamplePromptEnum {
    Text(String),
    Code(SampleCode),
    LineBreak(SampleLineBreak),
}

#[derive(Debug, Clone)]
pub enum SampleAiEnum {
    Text(String),
    Source(String),
    Code(SampleCode),
    LineBreak(SampleLineBreak),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleLineBreak {
    pub count: u8,
}

#[derive(Debug, Clone)]
pub struct SampleCode {
    pub lang: SampleLanguage,
    pub inline: bool,
    pub indent: Option<SampleIndent>,
    pub content: String,
}

/// Container for all training and validation samples
///
/// This struct manages the collection of samples and provides
/// unique sequential IDs for all samples created.
#[derive(Debug)]
pub struct Samples {
    /// Samples used for training
    pub train_samples: Vec<Sample>,
    
    /// Samples used for validation
    pub val_samples: Vec<Sample>,
}

// #[derive(Debug, Clone)]
// pub struct SampleTokenStats {
//     pub weight_decay: f32,
//     /// Bitnet papers say we don't need this
//     pub dropout: f32,
//     pub loss_scale: f32,
//     pub gradient_scale: f32,
//     pub gradient_clip: f32,
// }
