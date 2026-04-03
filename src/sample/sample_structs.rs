// src/sample/sample_structs.rs

use std::fmt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};


#[derive(Debug, Clone)]
pub struct Sample {
    pub system: String,
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
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

        impl Serialize for SampleIndent {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                // Serialize as the numeric value
                serializer.serialize_u8(*self as u8)
            }
        }

        impl<'de> Deserialize<'de> for SampleIndent {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                use serde::de::{self, Visitor, Unexpected};
                
                struct IndentVisitor;
                
                impl<'de> Visitor<'de> for IndentVisitor {
                    type Value = SampleIndent;
                    
                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("a number between 1 and 6")
                    }
                    
                    fn visit_u64<E>(self, value: u64) -> Result<SampleIndent, E>
                    where
                        E: de::Error,
                    {
                        if value >= 1 && value <= 6 {
                            if let Some(indent) = SampleIndent::from_u8(value as u8) {
                                return Ok(indent);
                            }
                        }
                        Err(E::invalid_value(Unexpected::Unsigned(value), &self))
                    }
                    
                    fn visit_i64<E>(self, value: i64) -> Result<SampleIndent, E>
                    where
                        E: de::Error,
                    {
                        if value >= 1 && value <= 6 {
                            if let Some(indent) = SampleIndent::from_u8(value as u8) {
                                return Ok(indent);
                            }
                        }
                        Err(E::invalid_value(Unexpected::Signed(value), &self))
                    }
                    
                    // For XML attributes, they come as strings, so we still need to handle strings
                    fn visit_str<E>(self, value: &str) -> Result<SampleIndent, E>
                    where
                        E: de::Error,
                    {
                        // Parse the string as a number
                        if let Ok(num) = value.parse::<u8>() {
                            if let Some(indent) = SampleIndent::from_u8(num) {
                                return Ok(indent);
                            }
                        }
                        Err(E::invalid_value(Unexpected::Str(value), &self))
                    }
                }
                
                deserializer.deserialize_any(IndentVisitor)
            }
        }
    };
}

define_indents! {
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

// #[derive(Debug, Clone)]
// pub struct SampleTokenStats {
//     pub weight_decay: f32,
//     /// Bitnet papers say we don't need this
//     pub dropout: f32,
//     pub loss_scale: f32,
//     pub gradient_scale: f32,
//     pub gradient_clip: f32,
// }
