// src/train_xml/train_xml_code_snippets.rs

use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLCodeSnippets {
    /// The sequence of code elements
    pub code: Vec<TrainXMLCodeSnippetsCode>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLCodeSnippetsCode {
    /// Unique identifier for this code
    #[serde(rename = "@id")]
    pub id: String,

    /// Language of the code
    #[serde(rename = "@lang")]
    pub lang: String,

    /// The code content
    #[serde(rename = "$text")]
    pub content: String,
}
