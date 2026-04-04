// src/cli/cli.rs

use clap::Parser;
use poppins::device::Device;
use super::cli_command::CliCommand;


/// Poppins CLI
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// The commands we support
    #[clap(subcommand)]
    pub command: CliCommand,

    /// Device to use for computation (auto-detects if not specified)
    /// 
    /// Options: cuda, metal, cpu
    /// If not specified, automatically detects best available device (CUDA > Metal > CPU)
    #[clap(short = 'd', long = "device", value_parser = parse_device, global = true)]
    pub device: Option<Device>,
}


impl Cli {
    /// Parse command line arguments and return the CLI struct
    pub fn parse_args() -> Self {
        Self::parse()
    }
}


/// Parse device from string (used by clap) (type is an Option so error only throws if defined & not valid)
pub fn parse_device(device_str: &str) -> Result<Device, String> {
    match device_str.to_lowercase().as_str() {
        "cuda" => Ok(Device::Cuda),
        "metal" => Ok(Device::Metal),
        "cpu" => Ok(Device::Cpu),
        _ => Err(format!("Invalid device '{}'. Must be one of: cuda, metal, cpu", device_str)),
    }
}
