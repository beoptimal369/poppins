// src/main.rs

mod cli;

use std::path::PathBuf;
use crate::cli::{Cli, CliCommand};
use poppins::{bootstrap, train, infer, Device};


fn main() {
    let cli = Cli::parse_args();

    // Device is either user-specified or auto-detected
    let device = match cli.device {
        Some(requested) => Device::new(Some(requested)).unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        }),
        None => Device::new(None).unwrap(),  // Auto-detect
    };
    
    match cli.command {
        CliCommand::Bootstrap { model_name } => {
            // Model name becomes the folder name within .poppins/
            let model_path = PathBuf::from(".poppins").join(&model_name);
            bootstrap(model_path.as_path());
        },
        CliCommand::Train { model_name } => {
            // Train using the model's train.xml and save artifacts to the same model folder
            let model_path = PathBuf::from(".poppins").join(&model_name);
            train(model_path.as_path(), model_name, &device).expect("❌ Failed to train:");
        },
        CliCommand::Infer { model_name, temperature, prompt } => {
            // Run inference using the trained model
            let model_path = PathBuf::from(".poppins").join(&model_name);
            infer(model_path.as_path(), prompt.join(" "), temperature, &device);
        },
    }
}
