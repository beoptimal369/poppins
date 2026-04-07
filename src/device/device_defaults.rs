// src/device/device_defaults.rs

use crate::device::Device;


impl Device {
    pub fn batch_size(&self, aim_train_gb: f32) -> usize {
        let base_batch_size = 32;

        let batch_size: usize = match self {
            Device::Cpu => 1,
            Device::Metal => {
                if aim_train_gb >= 32.0 { base_batch_size / 4 }       // 8
                else if aim_train_gb >= 16.0 { base_batch_size / 8 }  // 4
                else { base_batch_size / 16 }                         // 2
            },
            Device::Cuda => {
                if aim_train_gb >= 24.0 { base_batch_size * 2 }      // 64
                else if aim_train_gb >= 16.0 { base_batch_size }     // 32
                else if aim_train_gb >= 12.0 { base_batch_size / 2 } // 16
                else { base_batch_size / 4 }                         // 8
            }
        };

        batch_size.max(1).next_power_of_two()
    }

    pub fn mixed_precision(&self) -> bool {
        match self {
            Device::Cpu => false,
            Device::Metal | Device::Cuda => true,
        }
    }

    pub fn gradient_accumulation_steps(&self) -> usize {
        match self {
            Device::Cpu => 1,
            Device::Metal => 4,
            Device::Cuda => 8,
        }
    }

    pub fn activation_precision(&self) -> String {
        match self {
            Device::Cpu => "fp32".to_string(),
            Device::Metal | Device::Cuda => "int8".to_string(),
        }
    }

    pub fn rope_precision(&self) -> String {
        match self {
            Device::Cpu => "fp32".to_string(),
            Device::Metal | Device::Cuda => "fp16".to_string(),
        }
    }

    pub fn num_cpu_threads(&self) -> usize {
        match self {
            Device::Cpu => num_cpus::get().min(4),
            Device::Metal | Device::Cuda => 0,
        }
    }

    pub fn use_flash_attention(&self) -> bool {
        match self {
            Device::Cpu => false,
            Device::Metal | Device::Cuda => true,
        }
    }

    pub fn use_tensor_cores(&self) -> bool {
        matches!(self, Device::Cuda)
    }

    pub fn num_workers(&self) -> usize {
        match self {
            Device::Cpu => num_cpus::get(),
            Device::Metal => 4,
            Device::Cuda => 8,
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::device::Device;

    #[test]
    fn test_batch_size() {
        let cpu = Device::Cpu;
        assert_eq!(cpu.batch_size(100.0), 1);
        
        let metal = Device::Metal;
        assert_eq!(metal.batch_size(32.0), 8);   // High memory
        assert_eq!(metal.batch_size(20.0), 4);    // Medium memory
        assert_eq!(metal.batch_size(10.0), 2);   // Low memory
        
        let cuda = Device::Cuda;
        assert_eq!(cuda.batch_size(32.0), 64);   // Very high memory
        assert_eq!(cuda.batch_size(20.0), 32);    // High memory
        assert_eq!(cuda.batch_size(14.0), 16);   // Medium memory
        assert_eq!(cuda.batch_size(10.0), 8);    // Low memory
        
        // All results should be powers of two
        assert!(metal.batch_size(15.0).is_power_of_two());
        assert!(cuda.batch_size(15.0).is_power_of_two());
    }

    #[test]
    fn test_device_properties() {
        // CPU properties
        assert_eq!(Device::Cpu.mixed_precision(), false);
        assert_eq!(Device::Cpu.gradient_accumulation_steps(), 1);
        assert_eq!(Device::Cpu.activation_precision(), "fp32");
        assert_eq!(Device::Cpu.rope_precision(), "fp32");
        assert_eq!(Device::Cpu.use_flash_attention(), false);
        assert_eq!(Device::Cpu.use_tensor_cores(), false);
        assert!(Device::Cpu.num_cpu_threads() >= 1);
        assert!(Device::Cpu.num_workers() >= 1);
        
        // Metal properties
        assert_eq!(Device::Metal.mixed_precision(), true);
        assert_eq!(Device::Metal.gradient_accumulation_steps(), 4);
        assert_eq!(Device::Metal.activation_precision(), "int8");
        assert_eq!(Device::Metal.rope_precision(), "fp16");
        assert_eq!(Device::Metal.use_flash_attention(), true);
        assert_eq!(Device::Metal.use_tensor_cores(), false);
        assert_eq!(Device::Metal.num_cpu_threads(), 0);
        assert_eq!(Device::Metal.num_workers(), 4);
        
        // CUDA properties
        assert_eq!(Device::Cuda.mixed_precision(), true);
        assert_eq!(Device::Cuda.gradient_accumulation_steps(), 8);
        assert_eq!(Device::Cuda.activation_precision(), "int8");
        assert_eq!(Device::Cuda.rope_precision(), "fp16");
        assert_eq!(Device::Cuda.use_flash_attention(), true);
        assert_eq!(Device::Cuda.use_tensor_cores(), true);
        assert_eq!(Device::Cuda.num_cpu_threads(), 0);
        assert_eq!(Device::Cuda.num_workers(), 8);
    }
}
