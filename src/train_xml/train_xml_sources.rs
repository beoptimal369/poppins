// src/train_xml/train_xml_sources.rs

use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLSources {
    /// The sequence of source elements
    pub source: Vec<TrainXMLSourcesSource>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLSourcesSource {
    /// Unique identifier for this source
    #[serde(rename = "@id")]
    pub id: String,

    /// URL for this source
    #[serde(rename = "@url")]
    pub url: String
}
