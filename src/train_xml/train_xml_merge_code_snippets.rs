// src/train_xml/train_xml_merge_code_snippets.rs

use crate::train_xml::{TrainXML, TrainXMLCodeSnippets};


/// Merge code snippets from all train XML files into the target train_xml
///
/// Preserves order by iterating through train_xmls in order and appending code snippets as they're found.
/// No priority logic - all code snippets from all files are included in the order they appear.
pub fn train_xml_merge_code_snippets(
    train_xmls: &[TrainXML],
    train_xml: &mut TrainXML,
) {
    // First, check if there are any code snippets to merge
    let has_any_snippets = train_xmls.iter().any(|xml| xml.code_snippets.is_some());
    
    if !has_any_snippets {
        return;
    }
    
    // Initialize or get existing code snippets container
    let code_snippets = train_xml.code_snippets.get_or_insert_with(|| TrainXMLCodeSnippets {
        code: Vec::new(),
    });
    
    // Iterate through all train_xmls in order
    for xml in train_xmls {
        if let Some(source_snippets) = &xml.code_snippets {
            for code in &source_snippets.code {
                code_snippets.code.push(code.clone());
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::train_xml::{
        TrainXML,
        TrainXMLCodeSnippets,
        TrainXMLCodeSnippetsCode,
        train_xml_merge_code_snippets,
    };

    #[test]
    fn test_merge_code_snippets_no_snippets() {
        let train_xmls = vec![
            TrainXML::default(),
            TrainXML::default(),
        ];
        let mut merged = TrainXML::default();
        
        train_xml_merge_code_snippets(&train_xmls, &mut merged);
        
        // Should remain None since no snippets to merge
        assert!(merged.code_snippets.is_none());
    }
    
    #[test]
    fn test_merge_code_snippets_with_snippets() {
        let mut xml1 = TrainXML::default();
        xml1.code_snippets = Some(TrainXMLCodeSnippets {
            code: vec![
                TrainXMLCodeSnippetsCode {
                    id: "1".to_string(),
                    lang: "rust".to_string(),
                    content: "fn main() {}".to_string(),
                },
            ],
        });
        
        let mut xml2 = TrainXML::default();
        xml2.code_snippets = Some(TrainXMLCodeSnippets {
            code: vec![
                TrainXMLCodeSnippetsCode {
                    id: "2".to_string(),
                    lang: "python".to_string(),
                    content: "print('hello')".to_string(),
                },
            ],
        });
        
        let train_xmls = vec![xml1, xml2];
        let mut merged = TrainXML::default();
        
        train_xml_merge_code_snippets(&train_xmls, &mut merged);
        
        // Should have snippets
        assert!(merged.code_snippets.is_some());
        let snippets = merged.code_snippets.unwrap();
        assert_eq!(snippets.code.len(), 2);
        assert_eq!(snippets.code[0].id, "1");
        assert_eq!(snippets.code[1].id, "2");
    }
    
    #[test]
    fn test_merge_code_snippets_with_existing_merged_snippets() {
        let mut xml1 = TrainXML::default();
        xml1.code_snippets = Some(TrainXMLCodeSnippets {
            code: vec![
                TrainXMLCodeSnippetsCode {
                    id: "1".to_string(),
                    lang: "rust".to_string(),
                    content: "fn main() {}".to_string(),
                },
            ],
        });
        
        let xml2 = TrainXML::default();
        
        let train_xmls = vec![xml1, xml2];
        let mut merged = TrainXML::default();
        
        // Pre-populate merged with existing snippets
        merged.code_snippets = Some(TrainXMLCodeSnippets {
            code: vec![
                TrainXMLCodeSnippetsCode {
                    id: "existing".to_string(),
                    lang: "c".to_string(),
                    content: "int main() {}".to_string(),
                },
            ],
        });
        
        train_xml_merge_code_snippets(&train_xmls, &mut merged);
        
        // Should preserve existing and add new
        assert!(merged.code_snippets.is_some());
        let snippets = merged.code_snippets.unwrap();
        assert_eq!(snippets.code.len(), 2);
        assert_eq!(snippets.code[0].id, "existing");
        assert_eq!(snippets.code[1].id, "1");
    }
}
