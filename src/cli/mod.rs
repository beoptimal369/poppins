// src/cli/mod.rs

mod cli;
mod cli_command;
mod cli_validate;

pub use cli::Cli;
pub use cli_command::CliCommand;
pub use cli_validate::{cli_validate_device, cli_validate_model_name};
