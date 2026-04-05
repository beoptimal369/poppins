// src/device/device_detect.rs

use crate::device::Device;


impl Device {
    /// Detect optimal device
    ///
    /// Priority order: CUDA > Metal > CPU
    pub fn detect() -> Self {
        Self::inject_detect(|| Self::is_cuda_available(), || Self::is_metal_available())
    }

    /// Pass detection logic as arguments. Seperates "detect responses" from "hardware detection". Makes code highly testable.
    fn inject_detect<F: FnOnce() -> bool, G: FnOnce() -> bool>(is_cuda_available: F, is_metal_available: G) -> Self {
        if is_cuda_available() {
            return Device::Cuda;
        }
        
        if is_metal_available() {
            return Device::Metal;
        }

        Device::Cpu // fallback
    }
}



#[cfg(test)]
mod tests {
    use crate::device::Device;

    #[test]
    fn test_detect_returns_cuda_when_available() {
        let result = Device::inject_detect(|| true, || false);
        assert!(matches!(result, Device::Cuda));
    }

    #[test]
    fn test_detect_returns_metal_when_cuda_not_available() {
        let result = Device::inject_detect(|| false, || true);
        assert!(matches!(result, Device::Metal));
    }

    #[test]
    fn test_detect_returns_cpu_when_neither_available() {
        let result = Device::inject_detect(|| false,|| false);
        assert!(matches!(result, Device::Cpu));
    }

    #[test]
    fn test_detect_prioritizes_cuda_over_metal() {
        let result = Device::inject_detect(|| true, || true);
        assert!(matches!(result, Device::Cuda));
    }
}
