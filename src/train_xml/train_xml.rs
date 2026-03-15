// src/train_xml/train_xml.rs

use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXML {
    pub prompts: Option<crate::train_xml::train_xml_prompts::TrainXMLPrompts>,
    pub responses: Option<crate::train_xml::train_xml_responses::TrainXMLResponses>,
    pub sources: Option<crate::train_xml::train_xml_sources::TrainXMLSources>,
    #[serde(rename = "code-snippets")]
    pub code_snippets: Option<crate::train_xml::train_xml_code_snippets::TrainXMLCodeSnippets>,
    pub samples: Option<crate::train_xml::train_xml_samples::TrainXMLSamples>,
    pub constants: Option<crate::train_xml::train_xml_constants::TrainXMLConstants>,
    pub phrases: Option<crate::train_xml::train_xml_phrases::TrainXMLPhrases>,
}
