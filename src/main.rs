// src/main.rs

mod cli;

use crate::cli::{Cli, CliCommand};
use poppins::{bootstrap, train, infer};


fn main() {
    let cli = Cli::parse_args();
    
    match cli.command {
        CliCommand::Bootstrap { output } => {
            bootstrap(output.as_deref());
        },
        CliCommand::Train { input, output , version } => {
            train(
                input.as_deref(), 
                output.as_deref(), 
                version.as_deref()
            ).expect("❌ train() Error:");
        },
        CliCommand::Infer => infer(),
    }
}
