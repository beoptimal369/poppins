// src/sample/sample_structs.rs

use serde::{Serialize, Deserialize};


#[derive(Debug)]
pub struct Sample {
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
    pub indent: SampleIndent,
    pub content: String,
}

macro_rules! define_languages {
    ($($variant:ident => $string:expr),* $(,)?) => {
        #[derive(Debug, Clone)]
        pub enum SampleLanguage {
            $($variant),*
        }

        impl SampleLanguage {
            /// Generated automatically from the macro list
            pub const ALL: &[SampleLanguage] = &[
                $(SampleLanguage::$variant),*
            ];

            pub fn as_str(&self) -> &'static str {
                match self {
                    $(SampleLanguage::$variant => $string),*
                }
            }

            pub fn from_str(s: &str) -> Self {
                match s {
                    $($string => SampleLanguage::$variant,)*
                    _ => SampleLanguage::Txt,
                }
            }
        }
    };
}

define_languages! {
    Html => "html",
    Css  => "css",
    Js   => "js",
    Ts   => "ts",
    Jsx  => "jsx",
    Tsx  => "tsx",
    Rust => "rust",
    Bash => "bash",
    Xml  => "xml",
    Json => "json",
    Txt  => "txt",
    Md   => "md",
}

macro_rules! define_indents {
    ($($variant:ident => $value:expr),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        #[repr(u8)]
        pub enum SampleIndent {
            $($variant = $value),*
        }

        impl SampleIndent {
            /// Generated automatically: all valid indent levels
            pub const ALL: &[SampleIndent] = &[
                $(SampleIndent::$variant),*
            ];

            /// Safely converts a u8 to a SampleIndent variant
            pub fn from_u8(value: u8) -> Option<Self> {
                match value {
                    $($value => Some(SampleIndent::$variant),)*
                    _ => None,
                }
            }

            /// Returns the numeric value for formatting
            pub fn as_u8(&self) -> u8 {
                *self as u8
            }
        }
    };
}

define_indents! {
    Zero  => 0,
    One   => 1,
    Two   => 2,
    Three => 3,
    Four  => 4,
    Five  => 5,
    Six   => 6,
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

#[derive(Debug, Clone)]
pub struct SampleTokenStats {
    pub weight_decay: f32,
    /// Bitnet papers say we don't need this
    pub dropout: f32,
    pub loss_scale: f32,
    pub gradient_scale: f32,
    pub gradient_clip: f32,
}
