// src/train_xml/train_xml_responses.rs

use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLResponses {
    /// The sequence of response elements
    pub response: Vec<TrainXMLResponsesResponse>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLResponsesResponse {
    /// Unique identifier for this response
    #[serde(rename = "@id")]
    pub id: String,

    /// The response markdown content
    #[serde(rename = "$text")]
    pub content: String,
}
