// src/train/train.rs

use std::{path::Path, error::Error};
use crate::train_xml::{train_xml_read, train_xml_parse};


pub fn train(path: Option<&Path>) -> Result<(), Box<dyn Error>> {
    let train_content = train_xml_read(path)?;
    let train_xml = train_xml_parse(&train_content)?;

    println!("train_content: {}", train_content);
    println!("train_xml: {:?}", train_xml);

    Ok(())
}
