// src/cli/cli_command.rs

use clap::Subcommand;
use std::path::PathBuf;


/// Poppins CLI Commands
#[derive(Subcommand, Debug, PartialEq)]
pub enum CliCommand {
    /// Create a sample train.xml in the current directory
    Bootstrap {
        /// Path to the output directory (defaults to current directory)
        #[clap(short = 'o', long = "output", value_name = "PATH")]
        output: Option<PathBuf>,
    },
    
    /// Train an Ai model based on the training xml
    Train {
        /// Path to the training XML file (defaults to ./train.xml)
        #[clap(short = 'i', long = "input", value_name = "PATH")]
        input: Option<PathBuf>,

        /// Path to the output directory (defaults to ./poppins)
        #[clap(short = 'o', long = "output", value_name = "PATH")]
        output: Option<PathBuf>,
    },
    
    /// Send a prompt to an Ai model & get back a response
    Infer,
}



#[cfg(test)]
mod tests {
    use clap::Parser;
    use std::path::PathBuf;
    use crate::{Cli, CliCommand};

    #[test]
    fn test_bootstrap_command() {
        let args_input_long = vec!["poppins", "bootstrap", "--output", "src/random"];
        
        match Cli::try_parse_from(args_input_long).expect("Should parse").command {
            CliCommand::Bootstrap { output } => {
                assert_eq!(output, Some(PathBuf::from("src/random")));
            }
            _ => panic!("Expected valid poppins bootstrap"),
        }

        let args_input_short = vec!["poppins", "bootstrap", "-o", "src/random"];
        
        match Cli::try_parse_from(args_input_short).expect("Should parse").command {
            CliCommand::Bootstrap { output } => {
                assert_eq!(output, Some(PathBuf::from("src/random")));
            }
            _ => panic!("Expected valid poppins bootstrap"),
        }

        let args_default = vec!["poppins", "bootstrap"];

        match Cli::try_parse_from(args_default).expect("Should parse").command {
            CliCommand::Bootstrap { output } => {
                assert_eq!(output, None);
            }
            _ => panic!("Expected valid poppins bootstrap"),
        }
    }

    #[test]
    fn test_train_command() {
        let args_input_long = vec!["poppins", "train", "--input", "custom.xml", "--output", "src/random"];
        
        match Cli::try_parse_from(args_input_long).expect("Should parse").command {
            CliCommand::Train { input, output } => {
                assert_eq!(input, Some(PathBuf::from("custom.xml")));
                assert_eq!(output, Some(PathBuf::from("src/random")));
            }
            _ => panic!("Expected Train variant"),
        }

        let args_input_short = vec!["poppins", "train", "-i", "custom.xml", "-o", "src/random"];
        
        match Cli::try_parse_from(args_input_short).expect("Should parse").command {
            CliCommand::Train { input, output } => {
                assert_eq!(input, Some(PathBuf::from("custom.xml")));
                assert_eq!(output, Some(PathBuf::from("src/random")));
            }
            _ => panic!("Expected Train variant"),
        }

        let args_default = vec!["poppins", "train"];

        match Cli::try_parse_from(args_default).expect("Should parse").command {
            CliCommand::Train { input, output } => {
                assert_eq!(input, None);
                assert_eq!(output, None);
            }
            _ => panic!("Expected Train variant"),
        }
    }

    #[test]
    fn test_unknown_command() {
        let args = vec!["poppins", "fly"];
        let result = Cli::try_parse_from(args);
        
        assert!(result.is_err());
    }
}
