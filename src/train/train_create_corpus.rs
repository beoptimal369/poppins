// src/train/train_create_corpus.rs

use crate::sample::{
    Sample, 
    SampleAiEnum, 
    SamplePromptEnum, 
};


/// Creates a corpus string from training samples in the specified XML format
pub fn train_create_corpus(samples: &[Sample]) -> String {
    let mut corpus = String::new();
    
    // Explicit indentation variables for clarity and consistency
    let i1 = "  ";      // 2 spaces
    let i2 = "    ";    // 4 spaces
    let i3 = "      ";  // 6 spaces

    corpus.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    corpus.push_str("<samples>\n");
    
    for sample in samples {
        corpus.push_str(&format!("{i1}<sample>\n"));
        
        // --- Prompt Section ---
        corpus.push_str(&format!("{i2}<prompt>"));
        for prompt_item in &sample.prompt_section {
            match prompt_item {
                SamplePromptEnum::Text(text) => corpus.push_str(text),
                SamplePromptEnum::Code(code) => {
                    let inline = if code.inline { " inline=\"true\"" } else { "" };
                    let indent = if code.indent as u8 > 0 { format!(" indent=\"{}\"", code.indent as u8) } else { String::new() };
                    let tag = code.lang.as_str();
                    // Prompt code is usually inline/compact
                    corpus.push_str(&format!("<{tag}{inline}{indent}>{}</{tag}>", code.content));
                }
                SamplePromptEnum::LineBreak(lb) => {
                    if lb.count > 1 {
                        corpus.push_str(&format!("<line-break count=\"{}\" />", lb.count));
                    } else {
                        corpus.push_str("<line-break />");
                    }
                }
            }
        }
        corpus.push_str("</prompt>\n");
        
        // --- AI Section ---
        corpus.push_str(&format!("{i2}<ai>\n"));
        for ai_item in &sample.ai_section {
            match ai_item {
                SampleAiEnum::Text(text) => {
                    corpus.push_str(&format!("{i3}<text>{}</text>\n", text.content));
                }
                SampleAiEnum::Source(source) => {
                    corpus.push_str(&format!("{i3}<source>{}</source>\n", source.id));
                }
                SampleAiEnum::Code(code) => {
                    let inline = if code.inline { " inline=\"true\"" } else { "" };
                    let indent_attr = if code.indent as u8 > 0 { format!(" indent=\"{}\"", code.indent as u8) } else { String::new() };
                    let tag = code.lang.as_str();
                    
                    corpus.push_str(&format!("{i3}<{tag}{inline}{indent_attr}>{}</{tag}>\n", code.content));
                }
                SampleAiEnum::LineBreak(lb) => {
                    let lb_tag = if lb.count > 1 { format!("<line-break count=\"{}\" />", lb.count) } else { "<line-break />".to_string() };
                    corpus.push_str(&format!("{i3}{lb_tag}\n"));
                }
            }
        }
        corpus.push_str(&format!("{i2}</ai>\n"));
        corpus.push_str(&format!("{i1}</sample>\n"));
    }
    
    corpus.push_str("</samples>");
    corpus
}



#[cfg(test)]
mod tests {
    use crate::train::train_create_corpus::train_create_corpus;
    use crate::sample::{
        Sample,
        SampleAiEnum,
        SampleAiCode,
        SampleLanguage,
        SampleIndent,
        SampleTokenStats,
        SampleText,
        SamplePromptEnum,
    };

    #[test]
    fn test_train_create_corpus_formatting() {
        let stats = SampleTokenStats {
            weight_decay: 0.1, dropout: 0.1, loss_scale: 1.0, gradient_scale: 1.0, gradient_clip: 1.0,
        };

        let samples = vec![Sample {
            id: "1".to_string(),
            prompt_section: vec![SamplePromptEnum::Text("Hello".to_string())],
            ai_section: vec![
                SampleAiEnum::Text(SampleText { content: "Hi".to_string(), token_stats: stats.clone() }),
                SampleAiEnum::Code(SampleAiCode {
                    lang: SampleLanguage::Ts,
                    inline: false,
                    indent: SampleIndent::Zero,
                    content: "function() {\n  console.log('test')\n}".to_string(),
                    token_stats: stats,
                }),
            ],
        }];

        let result = train_create_corpus(&samples);

        // Assert Header and Root
        assert!(result.starts_with("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<samples>"));
        assert!(result.ends_with("</samples>"));

        // Assert Sample and Section Indentation
        assert!(result.contains("  <sample>"));
        assert!(result.contains("    <prompt>Hello</prompt>"));
        assert!(result.contains("    <ai>"));

        // Assert AI Content and closing tag indentation (i3 = 6 spaces)
        // Note: The content itself preserves its internal newlines, 
        // but the closing tag must immediately follow the content.
        let expected_code_line = "      <ts>function() {\n  console.log('test')\n}</ts>\n";
        assert!(result.contains(expected_code_line));
        
        assert!(result.contains("    </ai>"));
        assert!(result.contains("  </sample>"));
    }
}
