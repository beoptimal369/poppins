// src/train/train.rs

use std::fs;
use std::{path::Path, error::Error};
use crate::sample::{Samples, sample_create_samples};
use crate::train::{train_write_xmls, train_write_bins};
use crate::bpe::{bpe_get_special_tokens, bpe_train, BPETokenizerJSON};
use crate::train_xml::{TrainXMLConstantParsed, train_xml_parse, train_xml_validate};


pub fn train(train_xml_path: Option<&Path>, output_dir_path: Option<&Path>, model_version: Option<&str>) -> Result<(), Box<dyn Error>> {
    let input_path = train_xml_path.unwrap_or(Path::new("./train.xml"));

    let output_dir = output_dir_path.unwrap_or(Path::new("./.poppins"));

    let (samples, train_xml_constant_parsed) = get_samples(&input_path, &output_dir).map_err(|e| format!("❌ Failed turning train.xml into a Samples struct: {}", e))?;

    train_write_xmls(&output_dir, &samples)?;

    let tokenizer = bpe_train(
        &samples.train_samples,
        &bpe_get_special_tokens(), 
        &train_xml_constant_parsed.bpe_requested_tokens,
        train_xml_constant_parsed.bpe_min_merge_frequency,
    )?;

    BPETokenizerJSON::save(&tokenizer, &output_dir, &model_version.unwrap_or("0.1.0")).map_err(|e| format!("❌ Failed writing tokenizer.json: {}", e))?;

    train_write_bins(&output_dir, &samples, &tokenizer).map_err(|e| format!("❌ Failed writing bin files: {}", e))?;

    Ok(())
}


fn get_samples(input_path: &Path, output_dir: &Path) -> Result<(Samples, TrainXMLConstantParsed), Box<dyn std::error::Error>> {
    let train_content = fs::read_to_string(input_path).map_err(|e| format!("❌ Failed reading training file {}: {}", input_path.display(), e))?;

    let train_xml = train_xml_parse(&train_content).map_err(|e| format!("❌ Failed parsing train.xml: {}", e))?;

    let (train_xml_id_maps, train_xml_constant_parsed) = train_xml_validate(&train_xml).map_err(|e| format!("❌ Failed validating train.xml: {}", e))?;

    fs::create_dir_all(output_dir).map_err(|e| format!("❌ Failed creating output directory: {}", e))?;

    let samples = sample_create_samples(&train_xml, &train_xml_id_maps);

    Ok((samples, train_xml_constant_parsed))
}
