// src/train/train.rs

use std::fs;
use std::{path::Path, error::Error};
use crate::sample::{Samples, sample_create_samples};
use crate::train::train_create_corpus::train_create_corpus;
use crate::train_xml::{train_xml_parse, train_xml_validate};
use crate::bpe::{bpe_get_special_tokens, bpe_train, bpe_write_tokenizer_json};


pub fn train(train_xml_path: Option<&Path>, output_dir_path: Option<&Path>, model_version: Option<&str>) -> Result<(), Box<dyn Error>> {
    let input_path = train_xml_path.unwrap_or(Path::new("./train.xml"));

    let output_dir = output_dir_path.unwrap_or(Path::new("./.poppins"));

    let samples = get_samples(&input_path, &output_dir)?;

    write_xml_corpuses(&samples, output_dir)?;

    let tokenizer = bpe_train(&samples.train_samples, &bpe_get_special_tokens(), &vec!["console.log".to_owned()])?;

    bpe_write_tokenizer_json(&tokenizer, &output_dir, &model_version.unwrap_or("0.1.0")).expect("Should write vocab.json");

    println!("tokenizer: {:?}", tokenizer);

    Ok(())
}


fn get_samples(input_path: &Path, output_dir: &Path) -> Result<Samples, Box<dyn std::error::Error>> {
    let train_content = fs::read_to_string(input_path).map_err(|e| format!("❌ Failed to read training file {}: {}", input_path.display(), e))?;

    let train_xml = train_xml_parse(&train_content)?;

    let (train_xml_id_maps, train_xml_constant_parsed) = train_xml_validate(&train_xml);

    fs::create_dir_all(output_dir).map_err(|e| format!("❌ Failed to create output directory: {}", e))?;

    let samples = sample_create_samples(&train_xml, &train_xml_id_maps, &train_xml_constant_parsed);

    Ok(samples)
}


fn write_xml_corpuses(samples: &Samples, output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let train_corpus_xml = train_create_corpus(&samples.train_samples);

    fs::write(output_dir.join("train_corpus.xml"), &train_corpus_xml)
        .map_err(|e| format!("❌ Failed to write train corpus to {}: {}", &output_dir.display(), e))?;

    let val_corpus_xml = train_create_corpus(&samples.val_samples);

    fs::write(output_dir.join("val_corpus.xml"), &val_corpus_xml)
        .map_err(|e| format!("❌ Failed to write val corpus to {}: {}", &output_dir.display(), e))?;

    Ok(())
}
