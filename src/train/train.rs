// src/train/train.rs

use std::fs;
use std::{path::Path, error::Error};
use crate::sample::{Samples, sample_create_samples};
use crate::train::{train_write_txts, train_write_bins};
use crate::bpe::{bpe_get_special_tokens, bpe_train, BPETokenizerJSON};
use crate::train_xml::{TrainXMLConstantParsed, train_xml_parse, train_xml_validate};


pub fn train(output_dir: &Path, model_name: String) -> Result<(), Box<dyn Error>> {
    println!("output_dir: {:?}", output_dir);
    let (samples, train_xml_constant_parsed) = get_samples(&output_dir)?;

    train_write_txts(&output_dir, &samples)?;

    let tokenizer = bpe_train(
        &samples.train_samples,
        &bpe_get_special_tokens(), 
        &train_xml_constant_parsed.bpe_requested_tokens,
        train_xml_constant_parsed.bpe_min_merge_frequency,
    )?;

    BPETokenizerJSON::save(&tokenizer, &output_dir, &model_name)?;

    train_write_bins(&output_dir, &samples, &tokenizer)?;

    Ok(())
}


fn get_samples(output_dir: &Path) -> Result<(Samples, TrainXMLConstantParsed), Box<dyn std::error::Error>> {
    let train_xml_path = output_dir.join("train.xml");

    let train_content = fs::read_to_string(&train_xml_path)?;

    let train_xml = train_xml_parse(&train_content)?;

    let (train_xml_id_maps, train_xml_constant_parsed, train_xml_patterns) = train_xml_validate(&train_xml)?;

    fs::create_dir_all(output_dir)?;

    let samples = sample_create_samples(&train_xml, &train_xml_id_maps, &train_xml_patterns);

    Ok((samples, train_xml_constant_parsed))
}
