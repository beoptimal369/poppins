// src/train_xml/train_xml_parse.rs

use crate::train_xml::TrainXML;


pub fn train_xml_parse(train_xml_content: &str) -> Result<TrainXML, Box<dyn std::error::Error>> {
    let train_xml: TrainXML = quick_xml::de::from_str(train_xml_content)
        .map_err(|e| format!("Failed to parse Training XML: {}", e))?;

    Ok(train_xml)
}



#[cfg(test)]
mod tests {
    use crate::train_xml::train_xml_parse;

    #[test]
    fn test_train_xml_parse_success() {
        // Minimal valid XML representing the core structures
        let xml_content = r#"
            <train>
                <constants>
                    <constant key="aim_train_gb" value="3.0" />
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
        
        // Verify Constants
        let constants = train_xml.constants.as_ref().unwrap();
        assert_eq!(constants.constant[0].value, "3.0");

        // Verify nested Sample and ResponseIds
        let samples = train_xml.samples.as_ref().unwrap();
        let first_sample = &samples.sample.as_ref().unwrap()[0];
        let resp_ids = first_sample.response_ids.as_ref().unwrap();
        
        assert_eq!(resp_ids[0].response, "6");
        assert_eq!(resp_ids[0].source.as_deref(), Some("3"));
    }

    #[test]
    fn test_train_xml_parse_fail_malformed() {
        // XML with an unclosed tag
        let malformed_xml = r#"
            <train>
                <constants>
                    <constant key="aim_train_gb" value="3.0">
                </constants>
            </train>
        "#;

        let result = train_xml_parse(malformed_xml);
        
        assert!(result.is_err());
        let err_msg = result.err().unwrap().to_string();
        assert!(err_msg.contains("Failed to parse Training XML"));
    }
}
