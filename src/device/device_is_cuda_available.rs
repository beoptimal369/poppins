// src/device/device_is_cuda_available.rs

use crate::device::Device;
use std::process::Command;


impl Device {
    /// Check if CUDA is available
    pub fn is_cuda_available() -> bool {
        Self::inject_is_cuda_available(|| {
            let command_output = Command::new("nvidia-smi")
                .arg("--query-gpu=name")
                .arg("--format=csv,noheader")
                .output()?;

            Ok(String::from_utf8_lossy(&command_output.stdout).to_string())
        })
    }

    /// Pass command output as an argument. Seperates "command response" from "command action". Makes code highly testable.
    fn inject_is_cuda_available<F: FnOnce() -> Result<String, std::io::Error>>(command_output: F) -> bool {
        match command_output() {
            Ok(output) => {
                let output_trimmed = output.trim();
                !output_trimmed.is_empty() && !output_trimmed.contains("not found") && !output_trimmed.contains("failed")
            },
            Err(_) => false,
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::device::Device;
    use std::io::{Error, ErrorKind};

    #[test]
    fn test_is_cuda_available_with_gpu() {
        let result = Device::inject_is_cuda_available(|| Ok("NVIDIA GeForce RTX 4090".to_string()));
        assert!(result);
    }

    #[test]
    fn test_is_cuda_available_command_fails() {
        let empty_res = Device::inject_is_cuda_available(|| Ok("".to_string()));
        assert!(!empty_res);

        let io_err = Device::inject_is_cuda_available(|| Err(Error::new(ErrorKind::NotFound, "error")));
        assert!(!io_err);
        
        let not_found_err = Device::inject_is_cuda_available(|| Ok("nvidia-smi not found".to_string()));
        assert!(!not_found_err);
        
        let failed_err = Device::inject_is_cuda_available(|| Ok("nvidia-smi failed".to_string()));
        assert!(!failed_err);
    }
}
