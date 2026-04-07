// src/bpe/bpe_get_special_tokens.rs

use crate::sample::{SampleIndent, SampleLanguage};


/// Returns a vector of all special XML tokens used in the corpus
///
/// This includes:
/// - `<unknown>` token (always first)
/// - Structural tags (sample, system, prompt, thought, ai, text, source)
/// - Line break tags
/// - Code language tags with all variants (standard, inline, indent)
///
/// # Returns
/// * `Vec<String>` - All special tokens in deterministic order
pub fn bpe_get_special_tokens() -> Vec<String> {
    let mut tokens = Vec::new();

    // Unknown Tag - MUST be first (index 0)
    tokens.push("<unknown>".to_string());

    // Structural Tags
    let structural = ["sample", "system", "prompt", "thought", "ai", "text", "source"];

    for tag in structural {
        tokens.push(format!("<{tag}>"));
        tokens.push(format!("</{tag}>"));
    }

    // Line Breaks Tags
    tokens.push("<line-break />".to_string());
    tokens.push("<line-break count=\"2\" />".to_string());

    // Code Language Tags
    for lang in SampleLanguage::ALL {
        let tag = lang.as_str();

        // Standard opening/closing
        tokens.push(format!("<{tag}>"));
        tokens.push(format!("</{tag}>"));

        // Inline variant (no indent)
        tokens.push(format!("<{tag} inline=\"true\">"));

        // Indent variants (no inline)
        for indent in SampleIndent::ALL {
            let val = indent.as_u8();

            if val > 0 {
                // No indent attribute if value is 0
                tokens.push(format!("<{tag} indent=\"{val}\">"));
            }
        }
    }

    tokens
}


#[cfg(test)]
mod tests {
    use super::bpe_get_special_tokens;
    use crate::sample::{SampleIndent, SampleLanguage};

    #[test]
    fn test_unknown_token_is_first() {
        let tokens = bpe_get_special_tokens();
        assert_eq!(tokens[0], "<unknown>", "Index 0 must be the <unknown> token");
    }

    #[test]
    fn test_structural_tags_present() {
        let tokens = bpe_get_special_tokens();
        let expected_structs = [
            "<sample>",
            "</sample>",
            "<system>",
            "</system>",
            "<prompt>",
            "</prompt>",
            "<thought>",
            "</thought>",
            "<ai>",
            "</ai>",
            "<text>",
            "</text>",
            "<source>",
            "</source>",
        ];

        for tag in expected_structs {
            assert!(
                tokens.contains(&tag.to_string()),
                "Missing structural tag: {tag}"
            );
        }
    }

    #[test]
    fn test_line_break_tags_present() {
        let tokens = bpe_get_special_tokens();

        assert!(
            tokens.contains(&"<line-break />".to_string()),
            "Missing <line-break /> tag"
        );
        assert!(
            tokens.contains(&"<line-break count=\"2\" />".to_string()),
            "Missing <line-break count=\"2\" /> tag"
        );
    }

    #[test]
    fn test_code_language_tags() {
        let tokens = bpe_get_special_tokens();

        for lang in SampleLanguage::ALL {
            let tag = lang.as_str();

            // Standard opening/closing
            assert!(
                tokens.contains(&format!("<{tag}>")),
                "Missing base tag for {tag}"
            );
            assert!(
                tokens.contains(&format!("</{tag}>")),
                "Missing closing tag for {tag}"
            );
            assert!(
                tokens.contains(&format!("<{tag} inline=\"true\">")),
                "Missing inline tag for {tag}"
            );

            // Verify every non-zero indent
            for indent in SampleIndent::ALL {
                let val = indent.as_u8();
                if val > 0 {
                    let expected = format!("<{tag} indent=\"{val}\">");
                    assert!(
                        tokens.contains(&expected),
                        "Missing indent tag: {expected}"
                    );
                } else {
                    // Safety check: ensure indent="0" was NOT generated
                    let unexpected = format!("<{tag} indent=\"0\">");
                    assert!(
                        !tokens.contains(&unexpected),
                        "Forbidden token found: {unexpected}"
                    );
                }
            }
        }
    }

    #[test]
    fn test_no_mixed_attributes() {
        let tokens = bpe_get_special_tokens();

        for token in &tokens {
            let has_inline = token.contains("inline=\"true\"");
            let has_indent = token.contains("indent=");

            // A token should never have both inline and indent attributes
            assert!(
                !(has_inline && has_indent),
                "Contaminated token found: {token} (has both inline and indent)"
            );
        }
    }

    #[test]
    fn test_special_tokens_count() {
        let tokens = bpe_get_special_tokens();

        // Calculate expected count
        let structural_count = 14; // 7 tags × 2 (open/close)
        let line_break_count = 2;
        let languages_count = SampleLanguage::ALL.len();
        let tags_per_language = 2 + 1 + 6; // opening/closing (2), inline (1), indents 1-6 (6)
        let code_tags_count = languages_count * tags_per_language;
        
        let expected_count = 1 + structural_count + line_break_count + code_tags_count;
        
        assert_eq!(
            tokens.len(),
            expected_count,
            "Expected {} special tokens, got {}",
            expected_count,
            tokens.len()
        );
    }

    #[test]
    fn test_no_duplicate_tokens() {
        let tokens = bpe_get_special_tokens();
        let mut seen = std::collections::HashSet::new();

        for token in &tokens {
            assert!(
                !seen.contains(token),
                "Duplicate token found: {token}"
            );
            seen.insert(token);
        }
    }

    #[test]
    fn test_structural_tags_order() {
        let tokens = bpe_get_special_tokens();
        
        // Structural tags should appear after <unknown> and before line breaks
        let sample_pos = tokens.iter().position(|t| t == "<sample>").unwrap();
        let line_break_pos = tokens.iter().position(|t| t == "<line-break />").unwrap();
        
        assert!(
            sample_pos < line_break_pos,
            "Structural tags should appear before line break tags"
        );
    }

    #[test]
    fn test_thought_tag_present() {
        let tokens = bpe_get_special_tokens();
        
        assert!(
            tokens.contains(&"<thought>".to_string()),
            "Missing <thought> opening tag"
        );
        assert!(
            tokens.contains(&"</thought>".to_string()),
            "Missing </thought> closing tag"
        );
    }

    #[test]
    fn test_thought_tag_order() {
        let tokens = bpe_get_special_tokens();
        
        let prompt_pos = tokens.iter().position(|t| t == "<prompt>").unwrap();
        let thought_pos = tokens.iter().position(|t| t == "<thought>").unwrap();
        let ai_pos = tokens.iter().position(|t| t == "<ai>").unwrap();
        
        // thought should come after prompt and before ai
        assert!(
            prompt_pos < thought_pos,
            "<thought> should appear after <prompt>"
        );
        assert!(
            thought_pos < ai_pos,
            "<thought> should appear before <ai>"
        );
    }
}
