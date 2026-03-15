// src/main.rs

pub mod cli;
pub mod ternary;

use crate::cli::{Cli, CliCommand};
use poppins::{bootstrap, train, infer};


fn main() {
    let cli = Cli::parse_args();
    
    match cli.command {
        CliCommand::Bootstrap => bootstrap(),
        CliCommand::Train { input } => {
            train(input.as_deref()).expect("Should train");
        },
        CliCommand::Infer => infer(),
    }
}
