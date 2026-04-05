// src/main.rs

mod cli;

use crate::cli::{Cli, CliCommand};
use poppins::{bootstrap, train, infer};


fn main() {
    let cli = Cli::parse_args();

    match cli.command {
        CliCommand::Bootstrap { model_name } => {
            // Model name becomes the folder name within .poppins/
            bootstrap(Cli::get_model_path(&model_name).as_path());
        },
        CliCommand::Train { model_name, device } => {
            // Train using the model's train.xml and save artifacts to the same model folder
            train(
                Cli::get_model_path(&model_name).as_path(), 
                model_name, 
                &Cli::get_device(device)
            ).expect("❌ Failed to train:");
        },
        CliCommand::Infer { model_name, temperature, prompt, device } => {
            // Run inference using the trained model
            infer(
                Cli::get_model_path(&model_name).as_path(),
                prompt.join(" "),
                temperature, &Cli::get_device(device)
            );
        },
    }
}
