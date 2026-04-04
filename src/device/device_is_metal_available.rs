// src/device/device_is_metal_available.rs

use crate::device::Device;
use std::process::Command;


impl Device {
    /// Check if Metal is available (Apple Silicon)
    pub fn is_metal_available() -> bool {
        Self::inject_is_metal_available(|| {
            let command_output = Command::new("sysctl")
                .args(&["-n", "machdep.cpu.brand_string"])
                .output()?;

            Ok(String::from_utf8_lossy(&command_output.stdout).to_string())
        })
    }

    /// Pass command output as an argument. Seperates "command response" from "command action". Makes code highly testable.
    fn inject_is_metal_available<F: FnOnce() -> Result<String, std::io::Error>>(command_output: F) -> bool {
        match command_output() {
            Ok(output) => {
                let output_trimmed = output.trim();
                !output_trimmed.is_empty() && !output_trimmed.contains("not found") && !output_trimmed.contains("failed") && output.contains("Apple")
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
    fn test_is_metal_available_on_apple_silicon() {
        let result = Device::inject_is_metal_available(|| Ok("Apple M1 Pro".to_string()));
        assert!(result);
    }

    #[test]
    fn test_is_metal_available_command_fails() {
        let empty_res = Device::inject_is_metal_available(|| Ok("".to_string()));
        assert!(!empty_res);

        let io_err = Device::inject_is_metal_available(|| Err(Error::new(ErrorKind::NotFound, "error")));
        assert!(!io_err);
        
        let intel_err = Device::inject_is_metal_available(|| Ok("Intel Core i7".to_string()));
        assert!(!intel_err);
        
        let not_found_err = Device::inject_is_metal_available(|| Ok("command not found".to_string()));
        assert!(!not_found_err);
        
        let failed_err = Device::inject_is_metal_available(|| Ok("sysctl failed".to_string()));
        assert!(!failed_err);
    }
}
