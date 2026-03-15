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



#[cfg(test)]
mod tests {
    use clap::Parser;
    use std::path::PathBuf;
    use crate::{Cli, CliCommand};

    #[test]
    fn test_cli_parse_args() {
        // We simulate the exact array of strings the OS would pass to the binary
        let simulated_args = vec!["poppins", "train", "--input", "config.xml"];

        // try_parse_from is the testable sibling of parse()
        let cli = Cli::try_parse_from(simulated_args)
            .expect("Failed to parse simulated args");

        if let CliCommand::Train { input } = cli.command {
            assert_eq!(input, Some(PathBuf::from("config.xml")));
        } else {
            panic!("Parsed the wrong command variant");
        }
    }
}
