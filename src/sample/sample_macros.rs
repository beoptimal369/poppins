// src/sample/sample_macros.rs

use std::fmt;
use serde::{Deserialize, Deserializer, Serialize, Serializer};


macro_rules! define_languages {
    ($($variant:ident => $string:expr),* $(,)?) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
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
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                // Serialize as the numeric value
                serializer.serialize_u8(*self as u8)
            }
        }

        impl<'de> Deserialize<'de> for SampleIndent {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                use serde::de::{self, Visitor, Unexpected};
                
                struct IndentVisitor;
                
                impl<'de> Visitor<'de> for IndentVisitor {
                    type Value = SampleIndent;
                    
                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("a number between 1 and 6")
                    }
                    
                    fn visit_u64<E: de::Error>(self, value: u64) -> Result<SampleIndent, E> {
                        if value >= 1 && value <= 6 {
                            if let Some(indent) = SampleIndent::from_u8(value as u8) {
                                return Ok(indent);
                            }
                        }
                        Err(E::invalid_value(Unexpected::Unsigned(value), &self))
                    }
                    
                    fn visit_i64<E: de::Error>(self, value: i64) -> Result<SampleIndent, E> {
                        if value >= 1 && value <= 6 {
                            if let Some(indent) = SampleIndent::from_u8(value as u8) {
                                return Ok(indent);
                            }
                        }
                        Err(E::invalid_value(Unexpected::Signed(value), &self))
                    }
                    
                    // For XML attributes, they come as strings, so we still need to handle strings
                    fn visit_str<E: de::Error>(self, value: &str) -> Result<SampleIndent, E> {
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


#[cfg(test)]
mod tests {
    use serde_json;
    use crate::sample::{SampleIndent, SampleLanguage};

    // ========== SampleLanguage Tests ==========

    #[test]
    fn test_language_all_contains_all_variants() {
        let expected_count = 12; // Html, Css, Js, Ts, Jsx, Tsx, Rust, Bash, Xml, Json, Txt, Md
        assert_eq!(SampleLanguage::ALL.len(), expected_count);
    }

    #[test]
    fn test_language_as_str() {
        assert_eq!(SampleLanguage::Html.as_str(), "html");
        assert_eq!(SampleLanguage::Css.as_str(), "css");
        assert_eq!(SampleLanguage::Js.as_str(), "js");
        assert_eq!(SampleLanguage::Ts.as_str(), "ts");
        assert_eq!(SampleLanguage::Jsx.as_str(), "jsx");
        assert_eq!(SampleLanguage::Tsx.as_str(), "tsx");
        assert_eq!(SampleLanguage::Rust.as_str(), "rust");
        assert_eq!(SampleLanguage::Bash.as_str(), "bash");
        assert_eq!(SampleLanguage::Xml.as_str(), "xml");
        assert_eq!(SampleLanguage::Json.as_str(), "json");
        assert_eq!(SampleLanguage::Txt.as_str(), "txt");
        assert_eq!(SampleLanguage::Md.as_str(), "md");
    }

    #[test]
    fn test_language_from_str_valid() {
        assert!(matches!(SampleLanguage::from_str("html"), SampleLanguage::Html));
        assert!(matches!(SampleLanguage::from_str("css"), SampleLanguage::Css));
        assert!(matches!(SampleLanguage::from_str("js"), SampleLanguage::Js));
        assert!(matches!(SampleLanguage::from_str("ts"), SampleLanguage::Ts));
        assert!(matches!(SampleLanguage::from_str("jsx"), SampleLanguage::Jsx));
        assert!(matches!(SampleLanguage::from_str("tsx"), SampleLanguage::Tsx));
        assert!(matches!(SampleLanguage::from_str("rust"), SampleLanguage::Rust));
        assert!(matches!(SampleLanguage::from_str("bash"), SampleLanguage::Bash));
        assert!(matches!(SampleLanguage::from_str("xml"), SampleLanguage::Xml));
        assert!(matches!(SampleLanguage::from_str("json"), SampleLanguage::Json));
        assert!(matches!(SampleLanguage::from_str("txt"), SampleLanguage::Txt));
        assert!(matches!(SampleLanguage::from_str("md"), SampleLanguage::Md));
    }

    #[test]
    fn test_language_from_str_case_sensitive() {
        // Should not match uppercase variants
        assert!(matches!(SampleLanguage::from_str("HTML"), SampleLanguage::Txt));
        assert!(matches!(SampleLanguage::from_str("CSS"), SampleLanguage::Txt));
        assert!(matches!(SampleLanguage::from_str("JS"), SampleLanguage::Txt));
    }

    #[test]
    fn test_language_from_str_unknown_returns_txt() {
        assert!(matches!(SampleLanguage::from_str("unknown"), SampleLanguage::Txt));
        assert!(matches!(SampleLanguage::from_str("python"), SampleLanguage::Txt));
        assert!(matches!(SampleLanguage::from_str(""), SampleLanguage::Txt));
    }

    #[test]
    fn test_language_equality() {
        let lang1 = SampleLanguage::Rust;
        let lang2 = SampleLanguage::Rust;
        assert_eq!(lang1, lang2);
        // Different variants not equal
        assert_ne!(SampleLanguage::Html, SampleLanguage::Css);
    }

    #[test]
    fn test_language_clone() {
        let original = SampleLanguage::Rust;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_language_debug_format() {
        let lang = SampleLanguage::Rust;
        let debug_str = format!("{:?}", lang);
        assert!(debug_str.contains("Rust"));
    }

    // ========== SampleIndent Tests ==========

    #[test]
    fn test_indent_all_contains_all_variants() {
        assert_eq!(SampleIndent::ALL.len(), 6);
        let expected = [1, 2, 3, 4, 5, 6];
        for (i, indent) in SampleIndent::ALL.iter().enumerate() {
            assert_eq!(indent.as_u8(), expected[i]);
        }
    }

    #[test]
    fn test_indent_from_u8_valid() {
        assert_eq!(SampleIndent::from_u8(1), Some(SampleIndent::One));
        assert_eq!(SampleIndent::from_u8(2), Some(SampleIndent::Two));
        assert_eq!(SampleIndent::from_u8(3), Some(SampleIndent::Three));
        assert_eq!(SampleIndent::from_u8(4), Some(SampleIndent::Four));
        assert_eq!(SampleIndent::from_u8(5), Some(SampleIndent::Five));
        assert_eq!(SampleIndent::from_u8(6), Some(SampleIndent::Six));
    }

    #[test]
    fn test_indent_from_u8_invalid() {
        assert_eq!(SampleIndent::from_u8(0), None);
        assert_eq!(SampleIndent::from_u8(7), None);
        assert_eq!(SampleIndent::from_u8(100), None);
        assert_eq!(SampleIndent::from_u8(255), None);
    }

    #[test]
    fn test_indent_as_u8() {
        assert_eq!(SampleIndent::One.as_u8(), 1);
        assert_eq!(SampleIndent::Two.as_u8(), 2);
        assert_eq!(SampleIndent::Three.as_u8(), 3);
        assert_eq!(SampleIndent::Four.as_u8(), 4);
        assert_eq!(SampleIndent::Five.as_u8(), 5);
        assert_eq!(SampleIndent::Six.as_u8(), 6);
    }

    #[test]
    fn test_indent_equality() {
        assert_eq!(SampleIndent::One, SampleIndent::One);
        assert_eq!(SampleIndent::Two, SampleIndent::Two);
        assert_ne!(SampleIndent::One, SampleIndent::Two);
        assert_ne!(SampleIndent::Three, SampleIndent::Four);
    }

    #[test]
    fn test_indent_clone() {
        let original = SampleIndent::Three;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_indent_copy() {
        let original = SampleIndent::Four;
        let copied = original;
        assert_eq!(original, copied);
    }

    #[test]
    fn test_indent_debug_format() {
        let indent = SampleIndent::Five;
        let debug_str = format!("{:?}", indent);
        assert!(debug_str.contains("Five"));
    }

    // ========== Serialization Tests ==========

    #[test]
    fn test_indent_serialize_to_json() {
        let indent = SampleIndent::Three;
        let json = serde_json::to_string(&indent).unwrap();
        assert_eq!(json, "3");
    }

    #[test]
    fn test_indent_serialize_to_json_all_values() {
        let test_cases = vec![
            (SampleIndent::One, "1"),
            (SampleIndent::Two, "2"),
            (SampleIndent::Three, "3"),
            (SampleIndent::Four, "4"),
            (SampleIndent::Five, "5"),
            (SampleIndent::Six, "6"),
        ];

        for (indent, expected) in test_cases {
            let json = serde_json::to_string(&indent).unwrap();
            assert_eq!(json, expected);
        }
    }

    #[test]
    fn test_indent_deserialize_from_json_number() {
        let json = "3";
        let indent: SampleIndent = serde_json::from_str(json).unwrap();
        assert_eq!(indent, SampleIndent::Three);
    }

    #[test]
    fn test_indent_deserialize_from_json_string() {
        // XML attributes come as strings, so this is important
        let json = "\"4\"";
        let indent: SampleIndent = serde_json::from_str(json).unwrap();
        assert_eq!(indent, SampleIndent::Four);
    }

    #[test]
    fn test_indent_deserialize_from_json_all_valid() {
        let test_cases = vec![
            ("1", SampleIndent::One),
            ("2", SampleIndent::Two),
            ("3", SampleIndent::Three),
            ("4", SampleIndent::Four),
            ("5", SampleIndent::Five),
            ("6", SampleIndent::Six),
            ("\"1\"", SampleIndent::One),
            ("\"2\"", SampleIndent::Two),
            ("\"3\"", SampleIndent::Three),
            ("\"4\"", SampleIndent::Four),
            ("\"5\"", SampleIndent::Five),
            ("\"6\"", SampleIndent::Six),
        ];

        for (json, expected) in test_cases {
            let indent: SampleIndent = serde_json::from_str(json).unwrap();
            assert_eq!(indent, expected);
        }
    }

    #[test]
    fn test_indent_deserialize_invalid_returns_error() {
        let invalid_values = vec!["0", "7", "10", "100", "\"0\"", "\"7\"", "\"10\""];
        
        for json in invalid_values {
            let result: Result<SampleIndent, _> = serde_json::from_str(json);
            assert!(result.is_err(), "Should fail for value: {}", json);
        }
    }

    #[test]
    fn test_indent_deserialize_negative_numbers_error() {
        let result: Result<SampleIndent, _> = serde_json::from_str("-1");
        assert!(result.is_err());
        
        let result: Result<SampleIndent, _> = serde_json::from_str("\"-1\"");
        assert!(result.is_err());
    }

    #[test]
    fn test_indent_deserialize_non_numeric_string_error() {
        let result: Result<SampleIndent, _> = serde_json::from_str("\"invalid\"");
        assert!(result.is_err());
        
        let result: Result<SampleIndent, _> = serde_json::from_str("\"abc\"");
        assert!(result.is_err());
    }

    #[test]
    fn test_indent_deserialize_float_error() {
        let result: Result<SampleIndent, _> = serde_json::from_str("1.5");
        assert!(result.is_err());
        
        let result: Result<SampleIndent, _> = serde_json::from_str("\"1.5\"");
        assert!(result.is_err());
    }

    #[test]
    fn test_indent_deserialize_null_error() {
        let result: Result<SampleIndent, _> = serde_json::from_str("null");
        assert!(result.is_err());
    }

    #[test]
    fn test_indent_deserialize_boolean_error() {
        let result: Result<SampleIndent, _> = serde_json::from_str("true");
        assert!(result.is_err());
        
        let result: Result<SampleIndent, _> = serde_json::from_str("false");
        assert!(result.is_err());
    }

    #[test]
    fn test_indent_round_trip_serialization() {
        for &indent in SampleIndent::ALL {
            let json = serde_json::to_string(&indent).unwrap();
            let deserialized: SampleIndent = serde_json::from_str(&json).unwrap();
            assert_eq!(indent, deserialized);
        }
    }

    #[test]
    fn test_indent_round_trip_with_string_input() {
        // Simulate XML attribute parsing
        for &indent in SampleIndent::ALL {
            let value = indent.as_u8().to_string();
            let json = serde_json::to_string(&value).unwrap();
            let deserialized: SampleIndent = serde_json::from_str(&json).unwrap();
            assert_eq!(indent, deserialized);
        }
    }

    // ========== Combined Tests ==========

    #[test]
    fn test_language_and_indent_independent() {
        // Verify that Language and Indent enums don't interfere with each other
        let lang = SampleLanguage::Rust;
        let indent = SampleIndent::Three;
        
        assert_eq!(lang.as_str(), "rust");
        assert_eq!(indent.as_u8(), 3);
    }

    #[test]
    fn test_all_languages_have_unique_strings() {
        let mut strings = std::collections::HashSet::new();
        for lang in SampleLanguage::ALL {
            let s = lang.as_str();
            assert!(!strings.contains(s), "Duplicate string found: {}", s);
            strings.insert(s);
        }
        assert_eq!(strings.len(), SampleLanguage::ALL.len());
    }

    #[test]
    fn test_all_indents_have_unique_values() {
        let mut values = std::collections::HashSet::new();
        for indent in SampleIndent::ALL {
            let v = indent.as_u8();
            assert!(!values.contains(&v), "Duplicate value found: {}", v);
            values.insert(v);
        }
        assert_eq!(values.len(), SampleIndent::ALL.len());
    }

    #[test]
    fn test_from_str_round_trip() {
        for lang in SampleLanguage::ALL {
            let str_repr = lang.as_str();
            let recovered = SampleLanguage::from_str(str_repr);
            assert_eq!(*lang, recovered);  // Fixed: compare by value, not reference
        }
    }
}
