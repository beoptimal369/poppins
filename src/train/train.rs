// src/train/train.rs

use std::fs;
use std::{path::Path, error::Error};
use crate::sample::sample_create_samples;
use crate::train_xml::{train_xml_parse, train_xml_validate};


pub fn train(train_xml_path: Option<&Path>, output_dir_path: Option<&Path>) -> Result<(), Box<dyn Error>> {
    let input_path = train_xml_path.unwrap_or(Path::new("./train.xml"));

    let output_dir = output_dir_path.unwrap_or(Path::new("./.poppins"));

    let train_content = fs::read_to_string(input_path).map_err(|e| format!("❌ Failed to read training file {}: {}", input_path.display(), e))?;

    let train_xml = train_xml_parse(&train_content)?;

    let (train_xml_id_maps, train_xml_constant_parsed, train_xml_phrase_map) = train_xml_validate(&train_xml);

    fs::create_dir_all(output_dir) .map_err(|e| format!("❌ Failed to create output directory: {}", e))?;

    let sample = sample_create_samples(&train_xml, &train_xml_id_maps, &train_xml_constant_parsed);

    println!("train_xml: {:?}", train_xml);
    println!("train_xml_id_maps: {:?}", train_xml_id_maps);
    println!("train_xml_phrase_map: {:?}", train_xml_phrase_map);
    println!("train_xml_constant_parsed: {:?}", train_xml_constant_parsed);
    println!("sample: {:?}", sample);

    Ok(())
}
