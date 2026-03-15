// src/train_xml/train_xml_validate.rs

use crate::train_xml::{TrainXMLConstantsParsed, train_xml_validate_ids};
pub use crate::train_xml::{TrainXML, TrainXMLIds};


pub fn train_xml_validate(train_xml: &TrainXML) -> TrainXMLConstantsParsed {
    let train_xml_ids = TrainXMLIds::create(train_xml).expect("❌ Should have valid id's");
    let train_xml_constants_parsed = TrainXMLConstantsParsed::create(&train_xml.constants).expect("❌ Should have valid constants:");

    train_xml_validate_ids(train_xml, &train_xml_ids).expect("❌ Should have valid id's");

    train_xml_constants_parsed
}
