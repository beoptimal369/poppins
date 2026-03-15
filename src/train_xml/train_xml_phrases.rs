// src/train_xml/train_xml_phrases.rs

use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLPhrases {
    /// The sequence of phrase elements
    pub phrase: Vec<TrainXMLPhrasesPhrase>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLPhrasesPhrase {
    /// IF we find this phrase in the prompt THEN variant prompts will be created
    #[serde(rename = "@key")]
    pub key: String,

    /// The sequence of variant elements
    pub variant: Vec<TrainXMLPhrasesVariant>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLPhrasesVariant {
    /// When this variant is used the value is this
    #[serde(rename = "@value")]
    pub value: String,
}
