// src/cli/cli.rs

use clap::Parser;
use poppins::Device;
use std::path::PathBuf;
use crate::cli::CliCommand;


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

    pub fn get_device(device: Option<Device>) -> Device {
        device.unwrap_or(Device::detect())
    }

    pub fn get_model_path(model_name: &String) -> PathBuf {
        PathBuf::from(".poppins").join(&model_name)
    }
}



#[cfg(test)]
mod tests {
    use crate::cli::Cli;
    use poppins::Device;

    #[test]
    fn test_get_device() {
        // Test with Some(device) - returns the provided device
        let device_cuda = Cli::get_device(Some(Device::Cuda));
        assert!(matches!(device_cuda, Device::Cuda));
        
        let device_metal = Cli::get_device(Some(Device::Metal));
        assert!(matches!(device_metal, Device::Metal));
        
        let device_cpu = Cli::get_device(Some(Device::Cpu));
        assert!(matches!(device_cpu, Device::Cpu));
        
        // Test with None - returns detected device (can't predict which, just check it's valid)
        let detected = Cli::get_device(None);
        // Device::detect() returns Cuda, Metal, or Cpu - all are valid
        match detected {
            Device::Cuda | Device::Metal | Device::Cpu => (),
        }
    }

    #[test]
    fn test_get_model_path() {
        // Test basic model name
        let path = Cli::get_model_path(&"optimus".to_string());
        assert_eq!(path, std::path::PathBuf::from(".poppins/optimus"));
        
        // Test model name with dots and hyphens
        let path = Cli::get_model_path(&"gpt-4.5".to_string());
        assert_eq!(path, std::path::PathBuf::from(".poppins/gpt-4.5"));
        
        // Test model name with underscores
        let path = Cli::get_model_path(&"my_model-v2".to_string());
        assert_eq!(path, std::path::PathBuf::from(".poppins/my_model-v2"));
        
        // Test with numbers
        let path = Cli::get_model_path(&"model123".to_string());
        assert_eq!(path, std::path::PathBuf::from(".poppins/model123"));
    }
}
