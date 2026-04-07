// src/train_xml/train_xml_phrase_pattern.rs

use regex::Regex;
use std::{fmt, sync::Arc};


/// Pre-compiled replacement function for fast variant generation
type TrainXMLPhrasePatternReplacementFn = Arc<dyn Fn(&str) -> String + Send + Sync>;

/// Compiled phrase pattern with its variants and pre-compiled replacement functions
pub struct TrainXMLPhrasePattern {
    /// The compiled regex pattern
    pub regex: Arc<Regex>,
    /// The variant strings to generate
    pub variants: Arc<Vec<String>>,
    /// Pre-compiled replacement functions for each variant (for fast path)
    pub replacements: Arc<Vec<TrainXMLPhrasePatternReplacementFn>>,
    /// Whether the pattern has capture groups (for fast path optimization)
    pub has_captures: bool,
    /// Whether the pattern has multiple capture groups (requires slow path)
    pub has_multiple_captures: bool,
    /// Whether any variant uses multiple capture groups ($1, $2, etc.)
    pub variants_use_multiple_captures: bool,
}

impl fmt::Debug for TrainXMLPhrasePattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TrainXMLPhrasePattern")
            .field("regex", &self.regex)
            .field("variants", &self.variants)
            .field("has_captures", &self.has_captures)
            .field("has_multiple_captures", &self.has_multiple_captures)
            .field("variants_use_multiple_captures", &self.variants_use_multiple_captures)
            .field("replacements", &"<compiled functions>")
            .finish()
    }
}

impl TrainXMLPhrasePattern {
    /// Pre-compile replacement functions for all variants
    pub fn compile_replacements(variants: &[String]) -> Vec<TrainXMLPhrasePatternReplacementFn> {
        variants
            .iter()
            .map(|variant| {
                let variant = variant.clone();
                let parts: Vec<String> = variant.split("$1").map(|s| s.to_string()).collect();
                let parts = Arc::new(parts);

                let replacement_fn: TrainXMLPhrasePatternReplacementFn = Arc::new(move |capture: &str| -> String {
                    if parts.len() == 1 {
                        return parts[0].clone();
                    }

                    let mut result = String::with_capacity(variant.len() + capture.len() * (parts.len() - 1));

                    for i in 0..parts.len() - 1 {
                        result.push_str(&parts[i]);
                        result.push_str(capture);
                    }

                    result.push_str(&parts[parts.len() - 1]);
                    result
                });
                replacement_fn
            })
            .collect()
    }
    
    /// Check if a regex pattern has capture groups
    pub fn has_capture_groups(regex: &Regex) -> bool {
        Self::count_capture_groups(regex) > 0
    }
    
    /// Check if a regex pattern has multiple capture groups
    pub fn has_multiple_capture_groups(regex: &Regex) -> bool {
        Self::count_capture_groups(regex) > 1
    }
    
    /// Count actual capture groups (ignoring non-capturing groups like (?:...))
    fn count_capture_groups(regex: &Regex) -> usize {
        let pattern_str = regex.as_str();
        let mut in_escape = false;
        let mut in_char_class = false;
        let mut capture_count = 0;
        let mut chars = pattern_str.chars().peekable();
        
        while let Some(c) = chars.next() {
            if in_escape {
                in_escape = false;
                continue;
            }
            if c == '\\' {
                in_escape = true;
                continue;
            }
            if c == '[' && !in_char_class {
                in_char_class = true;
                continue;
            }
            if c == ']' && in_char_class {
                in_char_class = false;
                continue;
            }
            if in_char_class {
                continue;
            }
            if c == '(' {
                // Look ahead to check if this is a special group
                let mut temp_chars = chars.clone();
                if let Some(&next) = temp_chars.peek() {
                    if next == '?' {
                        temp_chars.next(); // consume '?'
                        if let Some(&next2) = temp_chars.peek() {
                            match next2 {
                                ':' => {
                                    // Non-capturing group (?:...)
                                    chars.next(); // consume '?'
                                    chars.next(); // consume ':'
                                    continue;
                                }
                                '<' => {
                                    // Named capture group (?<name>...) - these ARE captures
                                    capture_count += 1;
                                    chars.next(); // consume '?'
                                    chars.next(); // consume '<'
                                    // Skip until matching '>'
                                    while let Some(&nc) = chars.peek() {
                                        chars.next();
                                        if nc == '>' {
                                            break;
                                        }
                                    }
                                }
                                '=' | '!' => {
                                    // Lookahead/lookbehind (?!), (?=) - not captures
                                    chars.next(); // consume '?'
                                    chars.next(); // consume '=' or '!'
                                    continue;
                                }
                                _ => {
                                    // Other special groups
                                    capture_count += 1;
                                    chars.next(); // consume '?'
                                }
                            }
                        } else {
                            capture_count += 1;
                        }
                    } else {
                        capture_count += 1;
                    }
                } else {
                    capture_count += 1;
                }
            }
        }
        capture_count
    }
    
    /// Check if any variant uses multiple capture groups ($1, $2, etc.)
    pub fn variants_use_multiple_captures(variants: &[String]) -> bool {
        variants.iter().any(|variant| {
            variant.contains("$2") || variant.contains("$3") || variant.contains("$4") || variant.contains("$5")
        })
    }
    
    /// Intelligent replace that chooses the optimal path
    /// 
    /// Uses fast path when:
    /// - Patterns with a single capture group (most common case)
    /// - Patterns with NO capture groups
    /// - Patterns where only the LAST capture group matters
    /// 
    /// Uses slow path (full regex) when:
    /// - Patterns with multiple capture groups that need to be used in different positions
    /// - Patterns where capture groups are used out of order
    /// - Patterns with nested capture groups
    /// - Patterns with complex regex features (lookaheads, lookbehinds, etc.)
    pub fn replace(&self, text: &str, variant_idx: usize) -> Option<String> {
        // First check if the pattern matches at all
        if !self.regex.is_match(text) {
            return None;
        }
        
        // Fast path: pattern has no capture groups
        if !self.has_captures {
            return Some(self.variants[variant_idx].clone());
        }
        
        // Fast path: single capture group and variants only use $1
        if !self.has_multiple_captures && !self.variants_use_multiple_captures {
            return self.fast_replace(text, variant_idx);
        }
        
        // Slow path: multiple capture groups or variants use $2, $3, etc.
        self.slow_replace(text, variant_idx)
    }
    
    fn fast_replace(&self, text: &str, variant_idx: usize) -> Option<String> {
        let caps = self.regex.captures(text)?;
        
        if caps.len() <= 1 {
            return Some(self.variants[variant_idx].clone());
        }
        
        // Get the first capture group (index 1)
        let capture = caps.get(1)?.as_str();
        
        Some((self.replacements[variant_idx])(capture))
    }
    
    fn slow_replace(&self, text: &str, variant_idx: usize) -> Option<String> {
        let variant = &self.variants[variant_idx];
        let result = self.regex.replace_all(text, |caps: &regex::Captures| {
            let mut result = variant.clone();
            for i in 1..caps.len() {
                if let Some(capture) = caps.get(i) {
                    let placeholder = format!("${}", i);
                    result = result.replace(&placeholder, capture.as_str());
                }
            }
            result
        }).into_owned();
        Some(result)
    }
}



#[cfg(test)]
mod tests {
    use regex::Regex;
    use std::sync::Arc;
    use crate::train_xml::TrainXMLPhrasePattern;

    fn create_test_pattern() -> TrainXMLPhrasePattern {
        let regex = Regex::new(r"What (?:is|are) (?:a |an |the )?(.*?)\?").unwrap();
        let variants = vec![
            "Define $1.".to_string(),
            "Define: $1.".to_string(),
            "Tell me about $1.".to_string(),
        ];
        let has_captures = TrainXMLPhrasePattern::has_capture_groups(&regex);
        let has_multiple_captures = TrainXMLPhrasePattern::has_multiple_capture_groups(&regex);
        let variants_use_multiple_captures = TrainXMLPhrasePattern::variants_use_multiple_captures(&variants);
        let replacements = TrainXMLPhrasePattern::compile_replacements(&variants);
        
        TrainXMLPhrasePattern {
            regex: Arc::new(regex),
            variants: Arc::new(variants),
            replacements: Arc::new(replacements),
            has_captures,
            has_multiple_captures,
            variants_use_multiple_captures,
        }
    }

    fn create_simple_pattern() -> TrainXMLPhrasePattern {
        let regex = Regex::new(r"ty").unwrap();
        let variants = vec![
            "thanks".to_string(),
            "thank you".to_string(),
        ];
        let has_captures = TrainXMLPhrasePattern::has_capture_groups(&regex);
        let has_multiple_captures = TrainXMLPhrasePattern::has_multiple_capture_groups(&regex);
        let variants_use_multiple_captures = TrainXMLPhrasePattern::variants_use_multiple_captures(&variants);
        let replacements = TrainXMLPhrasePattern::compile_replacements(&variants);
        
        TrainXMLPhrasePattern {
            regex: Arc::new(regex),
            variants: Arc::new(variants),
            replacements: Arc::new(replacements),
            has_captures,
            has_multiple_captures,
            variants_use_multiple_captures,
        }
    }

    fn create_multi_capture_pattern() -> TrainXMLPhrasePattern {
        let regex = Regex::new(r"The (\w+) is (\w+)\.").unwrap();
        let variants = vec![
            "$1 is very $2.".to_string(),
            "The $2 $1 is amazing.".to_string(),
        ];
        let has_captures = TrainXMLPhrasePattern::has_capture_groups(&regex);
        let has_multiple_captures = TrainXMLPhrasePattern::has_multiple_capture_groups(&regex);
        let variants_use_multiple_captures = TrainXMLPhrasePattern::variants_use_multiple_captures(&variants);
        let replacements = TrainXMLPhrasePattern::compile_replacements(&variants);
        
        TrainXMLPhrasePattern {
            regex: Arc::new(regex),
            variants: Arc::new(variants),
            replacements: Arc::new(replacements),
            has_captures,
            has_multiple_captures,
            variants_use_multiple_captures,
        }
    }

    fn create_two_capture_pattern() -> TrainXMLPhrasePattern {
        let regex = Regex::new(r"(\w+) and (\w+)").unwrap();
        let variants = vec![
            "$1 & $2".to_string(),
            "both $1 and $2".to_string(),
        ];
        let has_captures = TrainXMLPhrasePattern::has_capture_groups(&regex);
        let has_multiple_captures = TrainXMLPhrasePattern::has_multiple_capture_groups(&regex);
        let variants_use_multiple_captures = TrainXMLPhrasePattern::variants_use_multiple_captures(&variants);
        let replacements = TrainXMLPhrasePattern::compile_replacements(&variants);
        
        TrainXMLPhrasePattern {
            regex: Arc::new(regex),
            variants: Arc::new(variants),
            replacements: Arc::new(replacements),
            has_captures,
            has_multiple_captures,
            variants_use_multiple_captures,
        }
    }

    #[test]
    fn test_has_capture_groups() {
        let regex_with_capture = Regex::new(r"(hello)").unwrap();
        assert!(TrainXMLPhrasePattern::has_capture_groups(&regex_with_capture));
        
        let regex_with_non_capturing = Regex::new(r"(?:hello)").unwrap();
        assert!(!TrainXMLPhrasePattern::has_capture_groups(&regex_with_non_capturing));
        
        let regex_no_capture = Regex::new(r"hello").unwrap();
        assert!(!TrainXMLPhrasePattern::has_capture_groups(&regex_no_capture));
        
        let regex_escaped = Regex::new(r"\(hello\)").unwrap();
        assert!(!TrainXMLPhrasePattern::has_capture_groups(&regex_escaped));
        
        let regex_nested = Regex::new(r"((hello) world)").unwrap();
        assert!(TrainXMLPhrasePattern::has_capture_groups(&regex_nested));
    }

    #[test]
    fn test_has_multiple_capture_groups() {
        let regex_single = Regex::new(r"(hello)").unwrap();
        assert!(!TrainXMLPhrasePattern::has_multiple_capture_groups(&regex_single));
        
        let regex_double = Regex::new(r"(hello) (world)").unwrap();
        assert!(TrainXMLPhrasePattern::has_multiple_capture_groups(&regex_double));
        
        let regex_non_capturing = Regex::new(r"(?:hello) (world)").unwrap();
        assert!(!TrainXMLPhrasePattern::has_multiple_capture_groups(&regex_non_capturing));
        
        let regex_nested = Regex::new(r"((hello) world)").unwrap();
        assert!(TrainXMLPhrasePattern::has_multiple_capture_groups(&regex_nested));
        
        let regex_no_capture = Regex::new(r"hello world").unwrap();
        assert!(!TrainXMLPhrasePattern::has_multiple_capture_groups(&regex_no_capture));
    }

    #[test]
    fn test_variants_use_multiple_captures() {
        let variants_single = vec!["Define $1.".to_string()];
        assert!(!TrainXMLPhrasePattern::variants_use_multiple_captures(&variants_single));
        
        let variants_multiple = vec!["$1 and $2".to_string()];
        assert!(TrainXMLPhrasePattern::variants_use_multiple_captures(&variants_multiple));
        
        let variants_mixed = vec!["$1 only".to_string(), "$1 and $2".to_string()];
        assert!(TrainXMLPhrasePattern::variants_use_multiple_captures(&variants_mixed));
        
        let variants_none = vec!["no placeholders".to_string()];
        assert!(!TrainXMLPhrasePattern::variants_use_multiple_captures(&variants_none));
    }

    #[test]
    fn test_compile_replacements() {
        let variants = vec!["Define $1.".to_string(), "Define: $1.".to_string()];
        let replacements = TrainXMLPhrasePattern::compile_replacements(&variants);
        
        assert_eq!(replacements.len(), 2);
        
        let result1 = replacements[0]("computer");
        assert_eq!(result1, "Define computer.");
        
        let result2 = replacements[1]("computer");
        assert_eq!(result2, "Define: computer.");
    }

    #[test]
    fn test_compile_replacements_no_placeholder() {
        let variants = vec!["thanks".to_string()];
        let replacements = TrainXMLPhrasePattern::compile_replacements(&variants);
        
        let result = replacements[0]("anything");
        assert_eq!(result, "thanks");
    }

    #[test]
    fn test_compile_replacements_multiple_placeholders() {
        let variants = vec!["$1 and $2".to_string()];
        let replacements = TrainXMLPhrasePattern::compile_replacements(&variants);
        
        // Only $1 is replaced in fast path
        let result = replacements[0]("cats");
        assert_eq!(result, "cats and $2");
    }

    #[test]
    fn test_fast_replace_single_capture() {
        let pattern = create_test_pattern();
        
        let result = pattern.fast_replace("What is a computer?", 0);
        assert_eq!(result, Some("Define computer.".to_string()));
        
        let result = pattern.fast_replace("What are movies?", 1);
        assert_eq!(result, Some("Define: movies.".to_string()));
        
        let result = pattern.fast_replace("What is the olympics?", 2);
        assert_eq!(result, Some("Tell me about olympics.".to_string()));
    }

    #[test]
    fn test_fast_replace_no_capture() {
        let pattern = create_simple_pattern();
        
        let result = pattern.fast_replace("ty", 0);
        assert_eq!(result, Some("thanks".to_string()));
        
        let result = pattern.fast_replace("ty", 1);
        assert_eq!(result, Some("thank you".to_string()));
    }

    #[test]
    fn test_fast_replace_no_match() {
        let pattern = create_test_pattern();
        
        let result = pattern.fast_replace("Hello world", 0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_slow_replace_multi_capture() {
        let pattern = create_multi_capture_pattern();
        
        let result = pattern.slow_replace("The cat is fast.", 0);
        assert_eq!(result, Some("cat is very fast.".to_string()));
        
        let result = pattern.slow_replace("The cat is fast.", 1);
        assert_eq!(result, Some("The fast cat is amazing.".to_string()));
    }

    #[test]
    fn test_replace_chooses_fast_path_for_single_capture() {
        let pattern = create_test_pattern();
        
        let result = pattern.replace("What is a computer?", 0);
        assert_eq!(result, Some("Define computer.".to_string()));
    }

    #[test]
    fn test_replace_chooses_fast_path_for_no_capture() {
        let pattern = create_simple_pattern();
        
        let result = pattern.replace("ty", 0);
        assert_eq!(result, Some("thanks".to_string()));
    }

    #[test]
    fn test_replace_chooses_slow_path_for_multi_capture() {
        let pattern = create_multi_capture_pattern();
        
        let result = pattern.replace("The cat is fast.", 1);
        assert_eq!(result, Some("The fast cat is amazing.".to_string()));
    }

    #[test]
    fn test_replace_with_two_capture_groups() {
        let pattern = create_two_capture_pattern();
        
        // Replace should use slow path because pattern has multiple captures
        let result = pattern.replace("cats and dogs", 0);
        assert_eq!(result, Some("cats & dogs".to_string()));
    }

    #[test]
    fn test_replace_no_match_returns_none() {
        let pattern = create_test_pattern();
        
        let result = pattern.replace("Hello world", 0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_replace_with_optional_groups() {
        let pattern = create_test_pattern();
        
        let result = pattern.replace("What is a computer?", 0);
        assert_eq!(result, Some("Define computer.".to_string()));
        
        let result = pattern.replace("What is an apple?", 0);
        assert_eq!(result, Some("Define apple.".to_string()));
        
        let result = pattern.replace("What is the internet?", 0);
        assert_eq!(result, Some("Define internet.".to_string()));
        
        let result = pattern.replace("What is AI?", 0);
        assert_eq!(result, Some("Define AI.".to_string()));
        
        let result = pattern.replace("What are movies?", 1);
        assert_eq!(result, Some("Define: movies.".to_string()));
    }

    #[test]
    fn test_pattern_flags_correctly() {
        let single_capture = create_test_pattern();
        assert!(single_capture.has_captures);
        assert!(!single_capture.has_multiple_captures);
        assert!(!single_capture.variants_use_multiple_captures);
        
        let no_capture = create_simple_pattern();
        assert!(!no_capture.has_captures);
        assert!(!no_capture.has_multiple_captures);
        assert!(!no_capture.variants_use_multiple_captures);
        
        let multi_capture = create_multi_capture_pattern();
        assert!(multi_capture.has_captures);
        assert!(multi_capture.has_multiple_captures);
        assert!(multi_capture.variants_use_multiple_captures);
        
        let two_capture = create_two_capture_pattern();
        assert!(two_capture.has_captures);
        assert!(two_capture.has_multiple_captures);
        assert!(two_capture.variants_use_multiple_captures);
    }

    #[test]
    fn test_debug_output() {
        let pattern = create_test_pattern();
        let debug_str = format!("{:?}", pattern);
        
        assert!(debug_str.contains("regex"));
        assert!(debug_str.contains("variants"));
        assert!(debug_str.contains("has_captures"));
        assert!(debug_str.contains("has_multiple_captures"));
        assert!(debug_str.contains("variants_use_multiple_captures"));
        assert!(debug_str.contains("<compiled functions>"));
        assert!(!debug_str.contains("Replacements"));
    }
}
