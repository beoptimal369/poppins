// src/tokens/tokens_get_special.rs

use crate::sample::{SampleLanguage, SampleIndent};


/// Returns a vector of all special XML tokens used in the corpus
pub fn tokens_get_special() -> Vec<String> {
    let mut tokens = Vec::new();

    // Unknown Tag
    tokens.push("<unknown>".to_string());

    // Structural Tags
    let structural = [
        "sample", "prompt", "ai", "text", "source"
    ];

    for tag in structural {
        tokens.push(format!("<{tag}>"));
        tokens.push(format!("</{tag}>"));
    }

    // Line Breaks Tags
    tokens.push("<line-break />".to_string());
    tokens.push("<line-break count=\"2\" />".to_string());

    for lang in SampleLanguage::ALL {
        let tag = lang.as_str();

        // Standard opening/closing
        tokens.push(format!("<{tag}>"));
        tokens.push(format!("</{tag}>"));

        // Inline variant (No indent)
        tokens.push(format!("<{tag} inline=\"true\">"));

        // Indent variants (No inline)
        for indent in SampleIndent::ALL {
            let val = indent.as_u8();

            if val > 0 { // No indent attribute if it's 0
                tokens.push(format!("<{tag} indent=\"{val}\">"));
            }
        }
    }

    tokens
}



#[cfg(test)]
mod tests {
    use super::tokens_get_special;
    use crate::sample::{SampleIndent, SampleLanguage};

    #[test]
    fn test_tokens_get_special_variants() {
        let tokens = tokens_get_special();

        // Verify unknown
        assert_eq!(tokens[0], "<unknown>", "Index 0 must be the <unknown> token");

        // Verify Structural Tags
        let expected_structs = ["<sample>", "</sample>", "<prompt>", "</prompt>", "<ai>", "</ai>"];
        for tag in expected_structs {
            assert!(tokens.contains(&tag.to_string()), "Missing structural tag: {tag}");
        }

        // Verify Line Breaks
        assert!(tokens.contains(&"<line-break />".to_string()));
        assert!(tokens.contains(&"<line-break count=\"2\" />".to_string()));

        // Verify Code
        for lang in SampleLanguage::ALL {
            let tag = lang.as_str();
            
            assert!(tokens.contains(&format!("<{tag}>")), "Missing base tag for {tag}");
            assert!(tokens.contains(&format!("</{tag}>")), "Missing closing tag for {tag}");
            assert!(tokens.contains(&format!("<{tag} inline=\"true\">")), "Missing inline tag for {tag}");

            // Verify every non-zero indent defined in SampleIndent::ALL
            for indent in SampleIndent::ALL {
                let val = indent.as_u8();
                if val > 0 {
                    let expected = format!("<{tag} indent=\"{val}\">");
                    assert!(tokens.contains(&expected), "Missing indent tag: {expected}");
                } else {
                    // Safety check: ensure indent="0" was NOT generated
                    let unexpected = format!("<{tag} indent=\"0\">");
                    assert!(!tokens.contains(&unexpected), "Forbidden token found: {unexpected}");
                }
            }
        }

        // Verify no token has inline true and an indent
        for t in &tokens {
            let both = t.contains("inline=\"true\"") && t.contains("indent=");
            assert!(!both, "Contaminated token found: {t}");
        }
    }
}
