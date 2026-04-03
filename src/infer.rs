// src/infer.rs

use std::path::Path;


pub fn infer(output_dir: &Path, prompt: String, temperature: Option<f32>) {
    let temperature = temperature.unwrap_or(0.7);

    println!("output_dir: {:?}", output_dir);
    println!("temperature: {}", temperature);
    println!("prompt: {}", prompt);
}
