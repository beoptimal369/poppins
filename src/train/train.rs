// src/train/train.rs

use std::fs;
use std::{path::Path, error::Error};
use crate::train_xml::{train_xml_parse, train_xml_validate};


pub fn train(train_xml_path: Option<&Path>, output_dir_path: Option<&Path>) -> Result<(), Box<dyn Error>> {
    let train_content = fs::read_to_string(train_xml_path.unwrap_or(Path::new("./train.xml"))).expect("❌ Should read training file");

    let train_xml = train_xml_parse(&train_content)?;

    let train_xml_constants_parsed = train_xml_validate(&train_xml);

    println!("{:?}", train_xml_constants_parsed.constants);

    fs::create_dir_all(output_dir_path.unwrap_or(Path::new("./.poppins"))).expect("❌ Should create output directory");

    Ok(())
}
