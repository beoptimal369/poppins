// src/cli/cli_command.rs

use clap::Subcommand;
use regex::Regex;

/// Validates that a model name contains only characters safe for folder names
/// Allowed: alphanumeric, dots, hyphens, underscores
/// Disallowed: path separators, spaces, special chars that could cause issues
pub fn validate_model_name(model_name: &str) -> Result<String, String> {
    let re = Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9._-]*$").unwrap();
    
    if model_name.is_empty() {
        return Err("Model name cannot be empty".to_string());
    }
    
    if !re.is_match(model_name) {
        return Err(format!(
            "Invalid model name '{}'. Model names must:\n\
             - Start with a letter or number\n\
             - Contain only letters, numbers, dots (.), hyphens (-), or underscores (_)\n\
             - Not contain spaces, slashes, or special characters",
            model_name
        ));
    }
    
    Ok(model_name.to_string())
}

/// Poppins CLI Commands
#[derive(Subcommand, Debug, PartialEq)]
pub enum CliCommand {
    /// Create a sample train.xml file for a model
    /// 
    /// This will create a .poppins/{MODEL_NAME}/ directory in the current working directory
    /// and populate it with a sample train.xml configuration file that you can customize.
    /// 
    /// Example:
    /// 
    /// poppins bootstrap optimus-4.5
    /// 
    /// Creates .poppins/optimus-4.5/train.xml
    Bootstrap {
        /// Name of the model to bootstrap
        /// 
        /// This will be used as the folder name inside .poppins/
        /// Allowed characters: letters, numbers, dots (.), hyphens (-), underscores (_)
        #[clap(value_parser = validate_model_name)]
        model_name: String,
    },
    
    /// Train an AI model based on the training XML file
    /// 
    /// Reads train.xml from .poppins/{MODEL_NAME}/train.xml and trains the model.
    /// Trained model artifacts will be stored in .poppins/{MODEL_NAME}/artifacts/
    /// 
    /// Examples:
    ///   poppins train optimus            # Trains using .poppins/optimus/train.xml
    ///   poppins train optimus-4.5        # Trains using .poppins/optimus-4.5/train.xml
    Train {
        /// Name of the model to train
        /// 
        /// The model must have been bootstrapped first (ex: .poppins/{MODEL_NAME}/train.xml must exist)
        /// Allowed characters: letters, numbers, dots (.), hyphens (-), underscores (_)
        #[clap(value_parser = validate_model_name)]
        model_name: String,
    },
    
    /// Send a prompt to a trained AI model and get back a response
    /// 
    /// Loads the trained model from .poppins/{MODEL_NAME} and runs inference.
    /// Flags (like --temperature) can come before or after the model name, but the prompt
    /// must be the last thing on the command line - no quotes surrounding the prompt required!
    /// 
    /// Examples:
    ///   poppins infer optimus What is Rust?
    ///   poppins infer optimus -t 0.8 What's up?
    ///   poppins infer --temperature 1.2 optimus Aloha! How are you?
    Infer {
        /// Name of the model to use for inference
        /// 
        /// The model must have been trained first (ex: .poppins/{MODEL_NAME}/artifacts/ must exist)
        /// Allowed characters: letters, numbers, dots (.), hyphens (-), underscores (_)
        #[clap(value_parser = validate_model_name)]
        model_name: String,
        
        /// Temperature for response randomness (0.0 = deterministic, 1.0 = creative)
        /// 
        /// Higher values make output more random/creative, lower values make it more focused/deterministic.
        /// Defaults to 0.7 if not specified.
        #[clap(short = 't', long = "temperature", value_name = "FLOAT")]
        temperature: Option<f32>,
        
        /// The prompt to send to the model
        /// 
        /// Everything after the model name and any flags is treated as the prompt.
        /// No quotes needed - spaces, apostrophes, and exclamation marks are all fine!
        /// The prompt must be the last thing on the command line.
        #[clap(trailing_var_arg = true, allow_hyphen_values = true)]
        prompt: Vec<String>,
    },
}

#[cfg(test)]
mod tests {
    use clap::Parser;
    use crate::{Cli, CliCommand};
    use super::validate_model_name;

    #[test]
    fn test_validate_model_name() {
        // Valid names
        assert!(validate_model_name("optimus").is_ok());
        assert!(validate_model_name("gpt-4.5").is_ok());
        assert!(validate_model_name("my_model-v2").is_ok());
        assert!(validate_model_name("model123").is_ok());
        assert!(validate_model_name("test.model").is_ok());
        assert!(validate_model_name("v1.2.3").is_ok());
        
        // Invalid names
        assert!(validate_model_name("").is_err());
        assert!(validate_model_name("my model").is_err());  // space
        assert!(validate_model_name("model/name").is_err()); // slash
        assert!(validate_model_name("model\\name").is_err()); // backslash
        assert!(validate_model_name("model:name").is_err()); // colon
        assert!(validate_model_name("-invalid").is_err()); // starts with hyphen
        assert!(validate_model_name(".invalid").is_err()); // starts with dot
        assert!(validate_model_name("invalid!").is_err()); // exclamation
        assert!(validate_model_name("model?name").is_err()); // question mark
    }

    #[test]
    fn test_bootstrap_command() {
        let args = vec!["poppins", "bootstrap", "optimus"];
        
        match Cli::try_parse_from(args).expect("Should parse").command {
            CliCommand::Bootstrap { model_name } => {
                assert_eq!(model_name, "optimus");
            }
            _ => panic!("Expected Bootstrap variant"),
        }
    }

    #[test]
    fn test_bootstrap_command_with_dots_and_hyphens() {
        let args = vec!["poppins", "bootstrap", "gpt-4.5"];
        
        match Cli::try_parse_from(args).expect("Should parse").command {
            CliCommand::Bootstrap { model_name } => {
                assert_eq!(model_name, "gpt-4.5");
            }
            _ => panic!("Expected Bootstrap variant"),
        }
    }

    #[test]
    fn test_train_command() {
        let args = vec!["poppins", "train", "optimus"];
        
        match Cli::try_parse_from(args).expect("Should parse").command {
            CliCommand::Train { model_name } => {
                assert_eq!(model_name, "optimus");
            }
            _ => panic!("Expected Train variant"),
        }
    }

    #[test]
    fn test_infer_command_default_temperature() {
        let args = vec!["poppins", "infer", "optimus", "Hello"];
        
        match Cli::try_parse_from(args).expect("Should parse").command {
            CliCommand::Infer { model_name, temperature, prompt } => {
                assert_eq!(model_name, "optimus");
                assert_eq!(temperature, None);  // Should be None, default handled in lib
                assert_eq!(prompt, vec!["Hello"]);
            }
            _ => panic!("Expected Infer variant"),
        }
    }

    #[test]
    fn test_infer_command_with_temperature_short() {
        let args = vec!["poppins", "infer", "-t", "0.5", "optimus", "Hello"];
        
        match Cli::try_parse_from(args).expect("Should parse").command {
            CliCommand::Infer { model_name, temperature, prompt } => {
                assert_eq!(model_name, "optimus");
                assert_eq!(temperature, Some(0.5));
                assert_eq!(prompt, vec!["Hello"]);
            }
            _ => panic!("Expected Infer variant"),
        }
    }

    #[test]
    fn test_infer_command_with_temperature_long() {
        let args = vec!["poppins", "infer", "--temperature", "1.2", "optimus", "Hello", "world"];
        
        match Cli::try_parse_from(args).expect("Should parse").command {
            CliCommand::Infer { model_name, temperature, prompt } => {
                assert_eq!(model_name, "optimus");
                assert_eq!(temperature, Some(1.2));
                assert_eq!(prompt, vec!["Hello", "world"]);
            }
            _ => panic!("Expected Infer variant"),
        }
    }

    #[test]
    fn test_infer_command_multiple_words() {
        let args = vec!["poppins", "infer", "optimus", "Hello", "world!", "How", "are", "you?"];
        
        match Cli::try_parse_from(args).expect("Should parse").command {
            CliCommand::Infer { model_name, temperature, prompt } => {
                assert_eq!(model_name, "optimus");
                assert_eq!(temperature, None);
                assert_eq!(prompt, vec!["Hello", "world!", "How", "are", "you?"]);
            }
            _ => panic!("Expected Infer variant"),
        }
    }

    #[test]
    fn test_infer_command_with_temperature_and_multiple_words() {
        let args = vec!["poppins", "infer", "-t", "0.9", "optimus", "What's", "up?", "Hello!"];
        
        match Cli::try_parse_from(args).expect("Should parse").command {
            CliCommand::Infer { model_name, temperature, prompt } => {
                assert_eq!(model_name, "optimus");
                assert_eq!(temperature, Some(0.9));
                assert_eq!(prompt, vec!["What's", "up?", "Hello!"]);
            }
            _ => panic!("Expected Infer variant"),
        }
    }

    #[test]
    fn test_infer_command_no_prompt() {
        let args = vec!["poppins", "infer", "optimus"];
        
        match Cli::try_parse_from(args).expect("Should parse").command {
            CliCommand::Infer { model_name, temperature, prompt } => {
                assert_eq!(model_name, "optimus");
                assert_eq!(temperature, None);
                assert!(prompt.is_empty(), "Prompt should be empty when not provided");
            }
            _ => panic!("Expected Infer variant"),
        }
    }

    #[test]
    fn test_validation_rejects_invalid_model_name() {
        let args = vec!["poppins", "bootstrap", "invalid name"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_err());
        
        let args = vec!["poppins", "train", "model/path"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_err());
    }

    #[test]
    fn test_unknown_command() {
        let args = vec!["poppins", "fly"];
        let result = Cli::try_parse_from(args);
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_missing_model_name() {
        let args = vec!["poppins", "bootstrap"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_err());
        
        let args = vec!["poppins", "train"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_err());
        
        let args = vec!["poppins", "infer"];
        let result = Cli::try_parse_from(args);
        assert!(result.is_err());
    }
}
