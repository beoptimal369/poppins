// src/train_xml/train_xml_parse.rs

use crate::train_xml::TrainXML;


pub fn train_xml_parse(train_xml_content: &str) -> Result<TrainXML, Box<dyn std::error::Error>> {
    let train_xml: TrainXML = quick_xml::de::from_str(train_xml_content)?;
    Ok(train_xml)
}



#[cfg(test)]
mod tests {
    use crate::train_xml::{
        train_xml_parse,
        TrainXMLSamplesSampleChildren,
    };

    #[test]
    fn test_train_xml_parse_success() {
        // Updated XML to match new element-based structure
        let xml_content = r#"
            <train>
                <constants>
                    <aim_train_gb>3.0</aim_train_gb>
                    <batch_size>32</batch_size>
                    <mixed_precision>true</mixed_precision>
                    <bpe_requested_tokens>
                        <value>function</value>
                        <value>console.log</value>
                    </bpe_requested_tokens>
                </constants>
                <samples>
                    <sample>
                        <prompt id="1" />
                        <response-ids response="6" source="3" />
                    </sample>
                </samples>
            </train>
        "#;

        let result = train_xml_parse(xml_content);
        
        // Assertions
        assert!(result.is_ok(), "Parser failed on valid XML: {:?}", result.err());
        let train_xml = result.unwrap();
        
        // Verify Constants - now accessing fields directly
        let constants = train_xml.constants.as_ref().unwrap();
        assert_eq!(constants.aim_train_gb, Some(3.0));
        assert_eq!(constants.batch_size, Some(32));
        assert_eq!(constants.mixed_precision, Some(true));
        
        // Verify BPE requested tokens
        let bpe_tokens = constants.bpe_requested_tokens.as_ref().unwrap();
        assert_eq!(bpe_tokens.values.len(), 2);
        assert_eq!(bpe_tokens.values[0], "function");
        assert_eq!(bpe_tokens.values[1], "console.log");

        // Verify nested Sample and ResponseIds via children
        let samples = train_xml.samples.as_ref().unwrap();
        let first_sample = &samples.sample.as_ref().unwrap()[0];
        
        // Find the ResponseIds in the children
        let resp_ids = first_sample.children.iter().find_map(|child| {
            if let TrainXMLSamplesSampleChildren::ResponseIds(ids) = child {
                Some(ids)
            } else {
                None
            }
        }).expect("Should find ResponseIds in children");
        
        assert_eq!(resp_ids.response, "6");
        assert_eq!(resp_ids.source.as_deref(), Some("3"));
    }

    #[test]
    fn test_train_xml_parse_with_defaults() {
        // Test with minimal XML (only required fields)
        let xml_content = r#"
            <train>
                <constants>
                    <aim_train_gb>4.5</aim_train_gb>
                </constants>
                <samples>
                    <sample>
                        <prompt id="1" />
                    </sample>
                </samples>
            </train>
        "#;

        let result = train_xml_parse(xml_content);
        assert!(result.is_ok());
        let train_xml = result.unwrap();
        
        let constants = train_xml.constants.as_ref().unwrap();
        assert_eq!(constants.aim_train_gb, Some(4.5));
        // Other fields should be None (will use defaults later)
        assert_eq!(constants.batch_size, None);
        assert_eq!(constants.learning_rate, None);
    }

    #[test]
    fn test_train_xml_parse_empty_constants() {
        // Test with no constants section
        let xml_content = r#"
            <train>
                <samples>
                    <sample>
                        <prompt id="1" />
                    </sample>
                </samples>
            </train>
        "#;

        let result = train_xml_parse(xml_content);
        assert!(result.is_ok());
        let train_xml = result.unwrap();
        
        // Constants should be None (will use all defaults)
        assert!(train_xml.constants.is_none());
    }

    #[test]
    fn test_train_xml_parse_fail_malformed() {
        // XML with an unclosed tag
        let malformed_xml = r#"
            <train>
                <constants>
                    <aim_train_gb>3.0
                </constants>
            </train>
        "#;

        let result = train_xml_parse(malformed_xml);
        
        assert!(result.is_err(), "Expected parsing to fail for malformed XML");
    }

    #[test]
    fn test_train_xml_parse_type_mismatch() {
        // XML with wrong type for a float field
        let xml_content = r#"
            <train>
                <constants>
                    <aim_train_gb>not_a_number</aim_train_gb>
                </constants>
                <samples>
                    <sample>
                        <prompt id="1" />
                    </sample>
                </samples>
            </train>
        "#;

        let result = train_xml_parse(xml_content);
        
        // Should fail because string can't be parsed as f32
        assert!(result.is_err(), "Expected parsing to fail for type mismatch");
    }
}
