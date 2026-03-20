// src/main.rs

mod cli;

use crate::cli::{Cli, CliCommand};
use poppins::{bootstrap, train, infer};


fn main() {
    let cli = Cli::parse_args();
    
    match cli.command {
        CliCommand::Bootstrap => bootstrap(),
        CliCommand::Train { input, output } => {
            train(input.as_deref(), output.as_deref()).expect("Should train");
        },
        CliCommand::Infer => infer(),
    }
}
