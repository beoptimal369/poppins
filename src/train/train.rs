// src/train/train.rs

use std::{path::Path, error::Error};
use crate::train_xml::{train_xml_read, train_xml_parse, train_xml_validate};


pub fn train(path: Option<&Path>) -> Result<(), Box<dyn Error>> {
    let train_content = train_xml_read(path)?;
    let train_xml = train_xml_parse(&train_content)?;
    let train_xml_constants_parsed = train_xml_validate(&train_xml);

    println!("{:?}", train_xml_constants_parsed.constants);

    Ok(())
}
