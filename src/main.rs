// src/main.rs

pub mod cli;
pub mod train;
pub mod infer;
pub mod bootstrap;

use crate::{
    train::train,
    infer::infer,
    bootstrap::bootstrap,
    cli::{Cli, CliCommand},
};

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
