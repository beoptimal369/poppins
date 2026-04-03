// src/main.rs

mod cli;

use crate::cli::{Cli, CliCommand};
use poppins::{bootstrap, train, infer};
use std::path::PathBuf;

fn main() {
    let cli = Cli::parse_args();
    
    match cli.command {
        CliCommand::Bootstrap { model_name } => {
            // Model name becomes the folder name within .poppins/
            let model_path = PathBuf::from(".poppins").join(&model_name);
            bootstrap(model_path.as_path());
        },
        CliCommand::Train { model_name } => {
            // Train using the model's train.xml and save artifacts to the same model folder
            let model_path = PathBuf::from(".poppins").join(&model_name);
            train(model_path.as_path(), model_name).expect("❌ Failed to train:");
        },
        CliCommand::Infer { model_name, temperature, prompt } => {
            // Run inference using the trained model
            let model_path = PathBuf::from(".poppins").join(&model_name);
            infer(model_path.as_path(), prompt.join(" "), temperature);
        },
    }
}
