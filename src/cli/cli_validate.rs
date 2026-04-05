// src/cli/cli_validate.rs

use regex::Regex;
use poppins::Device;


/// Validate model name
/// IF contains other then alphanumeric, dots, hyphens, underscores THEN error provided
pub fn cli_validate_model_name(model_name: &str) -> Result<String, String> {
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



/// Validate device (current hardware)
/// IF defined AND not a valid option THEN error provided
pub fn cli_validate_device(device_str: &str) -> Result<Device, String> {
    match device_str.to_lowercase().as_str() {
        "cuda" => Ok(Device::Cuda),
        "metal" => Ok(Device::Metal),
        "cpu" => Ok(Device::Cpu),
        _ => Err(format!("Invalid device '{}'. Must be one of: cuda, metal, cpu", device_str)),
    }
}



#[cfg(test)]
mod tests {
    use poppins::Device;
    use crate::cli::{cli_validate_device, cli_validate_model_name};

    #[test]
    fn test_validate_model_name_valid() {
        // Test various valid model names
        assert!(cli_validate_model_name("optimus").is_ok());
        assert!(cli_validate_model_name("gpt-4.5").is_ok());
        assert!(cli_validate_model_name("my_model-v2").is_ok());
        assert!(cli_validate_model_name("model123").is_ok());
        assert!(cli_validate_model_name("test.model").is_ok());
        assert!(cli_validate_model_name("v1.2.3").is_ok());
        assert!(cli_validate_model_name("a").is_ok());
        assert!(cli_validate_model_name("A1").is_ok());
    }

    #[test]
    fn test_validate_model_name_invalid() {
        // Test empty string
        let result = cli_validate_model_name("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));

        // Test spaces
        let result = cli_validate_model_name("my model");
        assert!(result.is_err());
        
        // Test starts with invalid characters
        let result = cli_validate_model_name("-invalid");
        assert!(result.is_err());
        
        let result = cli_validate_model_name(".invalid");
        assert!(result.is_err());
        
        let result = cli_validate_model_name("_invalid");
        assert!(result.is_err());
        
        // Test special characters
        let result = cli_validate_model_name("model/name");
        assert!(result.is_err());
        
        let result = cli_validate_model_name("model\\name");
        assert!(result.is_err());
        
        let result = cli_validate_model_name("model:name");
        assert!(result.is_err());
        
        let result = cli_validate_model_name("model!name");
        assert!(result.is_err());
        
        let result = cli_validate_model_name("model?name");
        assert!(result.is_err());
        
        let result = cli_validate_model_name("model@name");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_device_valid() {
        // Test case-insensitive matching
        let result = cli_validate_device("cuda");
        assert!(matches!(result, Ok(Device::Cuda)));
        
        let result = cli_validate_device("CUDA");
        assert!(matches!(result, Ok(Device::Cuda)));
        
        let result = cli_validate_device("Cuda");
        assert!(matches!(result, Ok(Device::Cuda)));
        
        let result = cli_validate_device("metal");
        assert!(matches!(result, Ok(Device::Metal)));
        
        let result = cli_validate_device("METAL");
        assert!(matches!(result, Ok(Device::Metal)));
        
        let result = cli_validate_device("cpu");
        assert!(matches!(result, Ok(Device::Cpu)));
        
        let result = cli_validate_device("CPU");
        assert!(matches!(result, Ok(Device::Cpu)));
    }

    #[test]
    fn test_validate_device_invalid() {
        // Test invalid device strings
        let result = cli_validate_device("invalid");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid device"));
        
        let result = cli_validate_device("gpu");
        assert!(result.is_err());
        
        let result = cli_validate_device("");
        assert!(result.is_err());
        
        let result = cli_validate_device("cuda2");
        assert!(result.is_err());
        
        let result = cli_validate_device("nvidia");
        assert!(result.is_err());
    }
}
