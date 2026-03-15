// src/train/train.rs

use crate::train::train_read_xml;
use std::{path::Path, error::Error};


pub fn train(path: Option<&Path>) -> Result<(), Box<dyn Error>> {
    let train_content = train_read_xml(path)?;

    println!("{}", train_content);

    Ok(())
}
