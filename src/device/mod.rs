// src/device/mod.rs

mod device;
mod device_detect;
mod device_defaults;
mod device_is_cuda_available;
mod device_is_metal_available;

pub use device::Device;
