// src/cli/cli_command.rs

use regex::Regex;
use clap::Subcommand;


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
    /// poppins -d cuda bootstrap optimus-4.5
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
    ///   poppins train optimus                    # Auto-detect device
    ///   poppins -d cuda train optimus            # Force CUDA
    ///   poppins --device metal train optimus     # Force Metal
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
    ///   poppins infer optimus What is Rust?                    # Auto-detect device
    ///   poppins -d cuda infer optimus -t 0.8 What's up?        # Force CUDA
    ///   poppins --device metal infer --temperature 1.2 optimus Aloha!
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
    use poppins::device::Device;
    use super::{validate_model_name};
    use crate::{Cli, CliCommand, cli::cli::parse_device};

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
    fn test_parse_device() {
        assert!(matches!(parse_device("cuda"), Ok(Device::Cuda)));
        assert!(matches!(parse_device("CUDA"), Ok(Device::Cuda)));
        assert!(matches!(parse_device("metal"), Ok(Device::Metal)));
        assert!(matches!(parse_device("METAL"), Ok(Device::Metal)));
        assert!(matches!(parse_device("cpu"), Ok(Device::Cpu)));
        assert!(matches!(parse_device("CPU"), Ok(Device::Cpu)));
        assert!(parse_device("invalid").is_err());
    }

    #[test]
    fn test_bootstrap_command_without_device() {
        let args = vec!["poppins", "bootstrap", "optimus"];
        
        match Cli::try_parse_from(args).expect("Should parse").command {
            CliCommand::Bootstrap { model_name } => {
                assert_eq!(model_name, "optimus");
            }
            _ => panic!("Expected Bootstrap variant"),
        }
    }

    #[test]
    fn test_bootstrap_command_with_device() {
        let args = vec!["poppins", "-d", "cuda", "bootstrap", "optimus"];
        let cli = Cli::try_parse_from(args).expect("Should parse");
        
        assert!(matches!(cli.device, Some(Device::Cuda)));
        match cli.command {
            CliCommand::Bootstrap { model_name } => {
                assert_eq!(model_name, "optimus");
            }
            _ => panic!("Expected Bootstrap variant"),
        }
    }

    #[test]
    fn test_train_command_without_device() {
        let args = vec!["poppins", "train", "optimus"];
        
        match Cli::try_parse_from(args).expect("Should parse").command {
            CliCommand::Train { model_name } => {
                assert_eq!(model_name, "optimus");
            }
            _ => panic!("Expected Train variant"),
        }
    }

    #[test]
    fn test_train_command_with_device() {
        let args = vec!["poppins", "--device", "metal", "train", "optimus"];
        let cli = Cli::try_parse_from(args).expect("Should parse");
        
        assert!(matches!(cli.device, Some(Device::Metal)));
        match cli.command {
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
                assert_eq!(temperature, None);
                assert_eq!(prompt, vec!["Hello"]);
            }
            _ => panic!("Expected Infer variant"),
        }
    }

    #[test]
    fn test_infer_command_with_device_and_temperature() {
        let args = vec!["poppins", "-d", "cuda", "infer", "-t", "0.5", "optimus", "Hello"];
        let cli = Cli::try_parse_from(args).expect("Should parse");
        
        assert!(matches!(cli.device, Some(Device::Cuda)));
        match cli.command {
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
    fn test_device_flag_global() {
        // Device flag should work before any subcommand
        let args = vec!["poppins", "-d", "cpu", "train", "optimus"];
        let cli = Cli::try_parse_from(args).expect("Should parse");
        assert!(matches!(cli.device, Some(Device::Cpu)));
        
        // Device flag with long form
        let args = vec!["poppins", "--device", "metal", "bootstrap", "test"];
        let cli = Cli::try_parse_from(args).expect("Should parse");
        assert!(matches!(cli.device, Some(Device::Metal)));
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
