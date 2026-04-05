// src/device/device.rs


#[derive(Debug, Clone, PartialEq)]
pub enum Device {
    /// NVIDIA GPU with CUDA support
    Cuda,
    /// Apple Silicon GPU with Metal support
    Metal,
    /// CPU (fallback)
    Cpu,
}


impl Device {
    /// IF requested device is supported on machine THEN return that device
    /// ELSE auto detect optimal device AND return that device
    pub fn new(requested_device: Option<Device>) -> Result<Self, String> {
        Self::inject_new(requested_device, || Self::is_cuda_available(), || Self::is_metal_available(), || Self::detect())
    }

    /// Pass detection logic as arguments. Seperates "detect responses" from "hardware detection". Makes code highly testable.
    fn inject_new<F: FnOnce() -> bool, G: FnOnce() -> bool, H: FnOnce() -> Self>(
        requested_device: Option<Device>,
        is_cuda_available: F,
        is_metal_available: G,
        detect: H,
    ) -> Result<Self, String> {
        match requested_device {
            Some(requested) => {
                match requested {
                    // IF CUDA is requested AND CUDA is available THEN return CUDA ELSE error
                    Device::Cuda => {
                        if is_cuda_available() {
                            Ok(Device::Cuda)
                        } else {
                            Err("❌ CUDA requested but not available. Install CUDA toolkit, use '-d cpu' or use '-d metal'".to_string())
                        }
                    }

                    // IF Metal is requested AND Metal is available THEN return Metal ELSE error
                    Device::Metal => {
                        if is_metal_available() {
                            Ok(Device::Metal)
                        } else {
                            Err("❌ Metal requested but not available. Use an Apple Silicon Mac, use '-d cpu' or use '-d cuda'".to_string())
                        }
                    }

                    // IF CPU is requested THEN return Cpu
                    Device::Cpu => Ok(Device::Cpu),
                }
            }

            // IF no device is requested THEN auto detect optimal device AND return that device
            None => Ok(detect()),
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::device::Device;

    #[test]
    fn test_new_cuda() {
        let available = Device::inject_new(
            Some(Device::Cuda),
            || true,
            || false,
            || Device::Cpu,
        );
        assert_eq!(available.unwrap(), Device::Cuda);

        let unavailable = Device::inject_new(
            Some(Device::Cuda),
            || false,
            || false,
            || Device::Cpu,
        );
        assert!(unavailable.is_err());
        assert!(unavailable.unwrap_err().contains("CUDA requested but not available"));

        let detect = Device::inject_new(
            None,
            || true,
            || false,
            || Device::Cuda,
        );
        assert_eq!(detect.unwrap(), Device::Cuda);
    }

    #[test]
    fn test_new_metal() {
        let available = Device::inject_new(
            Some(Device::Metal),
            || true,
            || true,
            || Device::Cpu,
        );
        assert_eq!(available.unwrap(), Device::Metal);

        let unavailable = Device::inject_new(
            Some(Device::Metal),
            || false,
            || false,
            || Device::Cpu,
        );
        assert!(unavailable.is_err());
        assert!(unavailable.unwrap_err().contains("Metal requested but not available"));

        let detect = Device::inject_new(
            None,
            || false,
            || true,
            || Device::Metal,
        );
        assert_eq!(detect.unwrap(), Device::Metal);
    }

    #[test]
    fn test_new_cpu() {
        let available = Device::inject_new(
            Some(Device::Cpu),
            || true,
            || true,
            || Device::Cpu,
        );
        assert_eq!(available.unwrap(), Device::Cpu);

        let detect = Device::inject_new(
            None,
            || false,
            || false,
            || Device::Cpu,
        );
        assert_eq!(detect.unwrap(), Device::Cpu);
    }
}
