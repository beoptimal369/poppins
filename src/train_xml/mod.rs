// src/train_xml/mod.rs

mod train_xml;
mod train_xml_read;
mod train_xml_parse;
mod train_xml_prompts;
mod train_xml_sources;
mod train_xml_phrases;
mod train_xml_samples;
mod train_xml_responses;
mod train_xml_constants;
mod train_xml_code_snippets;

pub use train_xml::TrainXML;
pub use train_xml_read::train_xml_read;
pub use train_xml_parse::train_xml_parse;
