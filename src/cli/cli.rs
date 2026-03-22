// src/cli/cli.rs

use clap::Parser;
use super::cli_command::CliCommand;


/// Poppins CLI
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// The commands we support
    #[clap(subcommand)]
    pub command: CliCommand,
}

impl Cli {
    /// Parse command line arguments and return the CLI struct
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
