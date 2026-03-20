// src/sample/sample_structs.rs

use serde::{Serialize, Deserialize};


#[derive(Debug)]
pub struct Sample {
    pub id: String,
    pub prompt_section: Vec<SamplePromptEnum>,
    pub ai_section: Vec<SampleAiEnum>,
}

#[derive(Debug, Clone)]
pub enum SamplePromptEnum {
    /// Not using SampleText b/c we don't need token_stats w/in the prompt
    Text(String),
    Code(SamplePromptCode),
    LineBreak(SampleLineBreak),
}

#[derive(Debug, Clone)]
pub enum SampleAiEnum {
    Text(SampleText),
    Source(SampleSource),
    Code(SampleAiCode),
    LineBreak(SampleLineBreak),
}

#[derive(Debug, Clone)]
pub struct SampleText {
    pub content: String,
    pub token_stats: SampleTokenStats,
}

#[derive(Debug, Clone)]
pub struct SampleSource {
    pub id: String,
    pub token_stats: SampleTokenStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleLineBreak {
    pub count: u8,
}

/// The model does not predict prompts so no need for token_stats here
#[derive(Debug, Clone)]
pub struct SamplePromptCode {
    pub lang: SampleLanguage,
    pub inline: bool,
    pub indent: SampleIndent,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct SampleAiCode {
    pub lang: SampleLanguage,
    pub inline: bool,
    pub indent: SampleIndent,
    pub content: String,
    pub token_stats: SampleTokenStats,
}

#[derive(Debug, Clone)]
pub enum SampleLanguage {
    Html,
    Css,
    Js,
    Ts,
    Jsx,
    Tsx,
    Rust,
    Bash,
    Xml,
    Json,
    Txt,
    Md,
}

impl SampleLanguage {
    pub fn as_str(&self) -> &'static str {
        match self {
            SampleLanguage::Html => "html",
            SampleLanguage::Css => "css",
            SampleLanguage::Js => "js",
            SampleLanguage::Ts => "ts",
            SampleLanguage::Jsx => "jsx",
            SampleLanguage::Tsx => "tsx",
            SampleLanguage::Rust => "rust",
            SampleLanguage::Bash => "bash",
            SampleLanguage::Xml => "xml",
            SampleLanguage::Json => "json",
            SampleLanguage::Txt => "txt",
            SampleLanguage::Md => "md",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "html" => SampleLanguage::Html,
            "css" => SampleLanguage::Css,
            "js" => SampleLanguage::Js,
            "ts" => SampleLanguage::Ts,
            "jsx" => SampleLanguage::Jsx,
            "tsx" => SampleLanguage::Tsx,
            "rust" => SampleLanguage::Rust,
            "bash" => SampleLanguage::Bash,
            "xml" => SampleLanguage::Xml,
            "json" => SampleLanguage::Json,
            "txt" => SampleLanguage::Txt,
            "md" => SampleLanguage::Md,
            _ => SampleLanguage::Txt,
        }
    }
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SampleIndent {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
}

impl SampleIndent {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(SampleIndent::Zero),
            1 => Some(SampleIndent::One),
            2 => Some(SampleIndent::Two),
            3 => Some(SampleIndent::Three),
            4 => Some(SampleIndent::Four),
            5 => Some(SampleIndent::Five),
            6 => Some(SampleIndent::Six),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SampleTokenStats {
    pub weight_decay: f32,
    pub dropout: f32,
    pub loss_scale: f32,
    pub gradient_scale: f32,
    pub gradient_clip: f32,
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
    
    /// Total number of samples created (used to assign unique IDs)
    /// This counter increments with each new sample added
    pub total_sample_count: usize,
}

impl Samples {
    /// Get the next available ID and increment the counter
    ///
    /// # Returns
    /// * `String` - The next ID as a string (e.g., "1", "2", "3")
    pub fn next_id(&mut self) -> String {
        self.total_sample_count += 1;
        self.total_sample_count.to_string()
    }
}

/// Where is this sample w/in train.bin or val.bin
pub struct SampleIndex {
    /// Helps us randomly jump to samples w/in bin files
    pub sample_start: usize,
    /// Helps us extract samples from bin files
    pub sample_length: usize,
    /// Helps us provide a starting point of known information for the model (mimic inference where the prompt is known) (predictions will begin after this token in training & validation)
    pub prompt_length: usize,
}
