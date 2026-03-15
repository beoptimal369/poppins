// src/train_xml/train_xml_prompts.rs

use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLPrompts {
    /// The sequence of prompt elements
    pub prompt: Vec<TrainXMLPromptsPrompt>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLPromptsPrompt {
    /// Unique identifier for this prompt
    #[serde(rename = "@id")]
    pub id: String,

    /// The prompt markdown content
    #[serde(rename = "$text")]
    pub content: String,
}
