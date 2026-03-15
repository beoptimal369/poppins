// src/train_xml/train_xml_samples.rs

use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLSamples {
    /// The sequence of sample elements
    #[serde(rename = "sample-ids")]
    pub sample_ids: Option<Vec<TrainXMLSamplesSampleIds>>,

    /// The sequence of sample elements
    pub sample: Option<Vec<TrainXMLSamplesSample>>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLSamplesSampleIds {
    /// Prompt unique identifier
    #[serde(rename = "@prompt")]
    pub prompt: String,

    /// Response unique identifier
    #[serde(rename = "@response")]
    pub response: Option<String>,

    /// Source unique identifier
    #[serde(rename = "@source")]
    pub source: Option<String>,

    /// Code unique identifier
    #[serde(rename = "@code")]
    pub code: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLSamplesSample {
    /// The sequence of prompt elements
    pub prompt: TrainXMLSamplesPrompt,

    /// The sequence of response-ids elements
    #[serde(rename = "response-ids")]
    pub response_ids: Option<Vec<TrainXMLSamplesResponseIds>>,

    /// The sequence of response elements
    pub response: Option<Vec<TrainXMLSamplesResponse>>,

    /// The sequence of source elements
    pub source: Option<Vec<TrainXMLSamplesSource>>,

    /// The sequence of code elements
    pub code: Option<Vec<TrainXMLSamplesCode>>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLSamplesResponseIds {
    /// Response unique identifier
    #[serde(rename = "@response")]
    pub response: String,

    /// Source unique identifier
    #[serde(rename = "@source")]
    pub source: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLSamplesPrompt {
    /// Unique identifier for this prompt
    #[serde(rename = "@id")]
    pub id: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLSamplesResponse {
    /// Unique identifier for this response
    #[serde(rename = "@id")]
    pub id: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLSamplesSource {
    /// Unique identifier for this response
    #[serde(rename = "@id")]
    pub id: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLSamplesCode {
    /// Unique identifier for this code
    #[serde(rename = "@id")]
    pub id: String,
}
