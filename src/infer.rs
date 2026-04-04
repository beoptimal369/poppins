// src/infer.rs

use std::path::Path;
use crate::device::Device;


pub fn infer(output_dir: &Path, prompt: String, temperature: Option<f32>, device: &Device) {
    let temperature = temperature.unwrap_or(0.7);

    println!("output_dir: {:?}", output_dir);
    println!("temperature: {}", temperature);
    println!("prompt: {}", prompt);
    println!("device: {:?}", device);
}
