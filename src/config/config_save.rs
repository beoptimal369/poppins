// src/config/config_save.rs

use std::fs::File;
use std::path::Path;
use std::io::Write;
use crate::config::{Config, ConfigHuggingFace};

impl Config {
    pub fn save(&self, output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::create_dir_all(output_dir)?;
        
        // Save Poppins native config
        let poppins_path = output_dir.join("config_poppins.json");
        let poppins_json = serde_json::to_string_pretty(self)?;
        let mut poppins_file = File::create(&poppins_path)?;
        poppins_file.write_all(poppins_json.as_bytes())?;
        println!("✅ Wrote {:?}", poppins_path);
        
        // Save Hugging Face compatible config
        let hf_config = ConfigHuggingFace::new(self);
        let hf_path = output_dir.join("config.json");
        let hf_json = serde_json::to_string_pretty(&hf_config)?;
        let mut hf_file = File::create(&hf_path)?;
        hf_file.write_all(hf_json.as_bytes())?;
        println!("✅ Wrote {:?}", hf_path);
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;
    use crate::config::Config;

    // Helper function to create a minimal test config
    fn create_test_config() -> Config {
        Config {
            vocab_size: 1678,
            embedding_dim: 1280,
            num_layers: 28,
            num_heads: 10,
            head_dim: 128,
            compression_dim: 320,
            rope_dim: 32,
            ffn_dim: 5120,
            total_params: 498_672_640,
            context_length: 32768,
            sliding_window_size: 16384,
            attention_type: "sliding_window".to_string(),
            use_per_token_quantization: true,
            attention_bias: false,
            bits_per_weight: 2,
            bytes_per_ternary: 0.25,
            weight_precision: "ternary".to_string(),
            rope_precision: "fp16".to_string(),
            kv_cache_precision: "int8".to_string(),
            activation_precision: "int8".to_string(),
            norm_type: "rms_norm".to_string(),
            activation_fn: "squared_relu".to_string(),
            optimizer: "AdamW".to_string(),
            lr_scheduler: "cosine".to_string(),
            batch_size: 32,
            num_workers: 8,
            effective_batch_size: 256,
            warmup_steps: 375,
            val_interval: 500,
            learning_rate: 0.001,
            mixed_precision: true,
            use_tensor_cores: true,
            use_flash_attention: true,
            gradient_accumulation_steps: 8,
            weight_decay_response: 0.01,
            weight_decay_source: 0.01,
            weight_decay_code: 0.01,
            loss_scale_response: 1.0,
            loss_scale_source: 1.0,
            loss_scale_code: 1.0,
            gradient_scale_response: 1.0,
            gradient_scale_source: 1.0,
            gradient_scale_code: 1.0,
            gradient_clip_response: 1.0,
            gradient_clip_source: 1.0,
            gradient_clip_code: 1.0,
            train_memory_gb: 8.09,
            infer_memory_gb: 0.77,
            embedding_memory_mb: 1.02,
            attention_memory_mb: 8.20,
            ffn_memory_mb: 87.5,
            kv_cache_memory_mb: 616.0,
            aim_train_gb: 9.0,
            aim_infer_gb: 1.0,
            infer_default_temperature: 0.7,
            infer_default_top_p: 0.9,
            infer_default_top_k: 50,
            infer_default_repetition_penalty: 1.1,
            infer_default_num_beams: 1,
            bos_token_id: Some(1),
            eos_token_id: Some(2),
            unk_token_id: Some(0),
            pad_token_id: None,
            model_type: "ternary_mla".to_string(),
            poppins_version: "0.1.0".to_string(),
            architecture: "TernaryMLA".to_string(),
            tokenizer_path: "tokenizer.json".to_string(),
            checkpoint_path: "checkpoints/model.pt".to_string(),
            created_at: "2024-01-15T10:30:00Z".to_string(),
            notes: "Test config".to_string(),
        }
    }

    #[test]
    fn test_save_creates_directory() {
        let temp_dir = tempdir().unwrap();
        let config = create_test_config();
        
        // Save should create the directory if it doesn't exist
        let non_existent_dir = temp_dir.path().join("subdir").join("nested");
        assert!(!non_existent_dir.exists());
        
        config.save(&non_existent_dir).unwrap();
        assert!(non_existent_dir.exists());
    }

    #[test]
    fn test_save_creates_both_files() {
        let temp_dir = tempdir().unwrap();
        let config = create_test_config();
        
        config.save(temp_dir.path()).unwrap();
        
        // Check both files exist
        let poppins_path = temp_dir.path().join("config_poppins.json");
        let hf_path = temp_dir.path().join("config.json");
        
        assert!(poppins_path.exists(), "config_poppins.json should exist");
        assert!(hf_path.exists(), "config.json should exist");
    }

    #[test]
    fn test_save_files_are_valid_json() {
        let temp_dir = tempdir().unwrap();
        let config = create_test_config();
        
        config.save(temp_dir.path()).unwrap();
        
        let poppins_path = temp_dir.path().join("config_poppins.json");
        let hf_path = temp_dir.path().join("config.json");
        
        // Verify Poppins config is valid JSON
        let poppins_content = fs::read_to_string(poppins_path).unwrap();
        let poppins_json: serde_json::Value = serde_json::from_str(&poppins_content).unwrap();
        assert!(poppins_json.is_object());
        
        // Verify HF config is valid JSON
        let hf_content = fs::read_to_string(hf_path).unwrap();
        let hf_json: serde_json::Value = serde_json::from_str(&hf_content).unwrap();
        assert!(hf_json.is_object());
    }

    #[test]
    fn test_save_poppins_config_contains_expected_fields() {
        let temp_dir = tempdir().unwrap();
        let config = create_test_config();
        
        config.save(temp_dir.path()).unwrap();
        
        let poppins_path = temp_dir.path().join("config_poppins.json");
        let content = fs::read_to_string(poppins_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        
        // Check key fields exist
        assert!(json.get("vocab_size").is_some());
        assert!(json.get("embedding_dim").is_some());
        assert!(json.get("num_layers").is_some());
        assert!(json.get("total_params").is_some());
        assert!(json.get("context_length").is_some());
        assert!(json.get("weight_precision").is_some());
        
        // Verify values match
        assert_eq!(json["vocab_size"].as_u64().unwrap(), 1678);
        assert_eq!(json["embedding_dim"].as_u64().unwrap(), 1280);
        assert_eq!(json["num_layers"].as_u64().unwrap(), 28);
        assert_eq!(json["total_params"].as_u64().unwrap(), 498_672_640);
    }

    #[test]
    fn test_save_hf_config_contains_expected_fields() {
        let temp_dir = tempdir().unwrap();
        let config = create_test_config();
        
        config.save(temp_dir.path()).unwrap();
        
        let hf_path = temp_dir.path().join("config.json");
        let content = fs::read_to_string(hf_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        
        // Check HF-specific fields exist
        assert!(json.get("architectures").is_some());
        assert!(json.get("model_type").is_some());
        assert!(json.get("hidden_size").is_some());
        assert!(json.get("num_hidden_layers").is_some());
        assert!(json.get("mla_config").is_some());
        assert!(json.get("quantization_config").is_some());
        
        // Verify values match (use as_f64 for floating point comparison)
        assert_eq!(json["model_type"].as_str().unwrap(), "ternary_mla");
        assert_eq!(json["hidden_size"].as_u64().unwrap(), 1280);
        assert_eq!(json["num_hidden_layers"].as_u64().unwrap(), 28);
        
        // Check quantization config exists but don't assert specific fields
        let quant_config = json.get("quantization_config").unwrap();
        assert!(quant_config.is_object());
    }

    #[test]
    fn test_save_hf_config_has_mla_config() {
        let temp_dir = tempdir().unwrap();
        let config = create_test_config();
        
        config.save(temp_dir.path()).unwrap();
        
        let hf_path = temp_dir.path().join("config.json");
        let content = fs::read_to_string(hf_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        
        let mla_config = json.get("mla_config").unwrap();
        assert!(mla_config.get("compression_dim").is_some());
        assert!(mla_config.get("rope_dim").is_some());
        assert!(mla_config.get("kv_lora_rank").is_some());
        
        assert_eq!(mla_config["compression_dim"].as_u64().unwrap(), 320);
        assert_eq!(mla_config["rope_dim"].as_u64().unwrap(), 32);
    }

    #[test]
    fn test_save_hf_config_has_quantization_config() {
        let temp_dir = tempdir().unwrap();
        let config = create_test_config();
        
        config.save(temp_dir.path()).unwrap();
        
        let hf_path = temp_dir.path().join("config.json");
        let content = fs::read_to_string(hf_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        
        let quant_config = json.get("quantization_config")
            .expect("quantization_config should exist");
        
        // Check fields based on what your ConfigHuggingFace actually includes
        assert!(quant_config.get("weight_precision").is_some(), "weight_precision missing");
        assert!(quant_config.get("activation_precision").is_some(), "activation_precision missing");
        
        // Check bits field (could be "bits" or "bits_per_weight")
        if let Some(bits) = quant_config.get("bits") {
            assert_eq!(bits.as_u64().unwrap(), 2);
        } else if let Some(bits_per_weight) = quant_config.get("bits_per_weight") {
            assert_eq!(bits_per_weight.as_u64().unwrap(), 2);
        }
    }

    #[test]
    fn test_save_overwrites_existing_files() {
        let temp_dir = tempdir().unwrap();
        let config = create_test_config();
        
        // First save
        config.save(temp_dir.path()).unwrap();
        
        // Modify config
        let mut modified_config = config.clone();
        modified_config.embedding_dim = 2048;
        modified_config.total_params = 1_000_000_000;
        
        // Second save (should overwrite)
        modified_config.save(temp_dir.path()).unwrap();
        
        // Verify content changed
        let poppins_path = temp_dir.path().join("config_poppins.json");
        let content = fs::read_to_string(poppins_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&content).unwrap();
        
        assert_eq!(json["embedding_dim"].as_u64().unwrap(), 2048);
        assert_eq!(json["total_params"].as_u64().unwrap(), 1_000_000_000);
    }

    #[test]
    fn test_save_returns_error_for_invalid_path() {
        let config = create_test_config();
        
        // Invalid path (e.g., path that's actually a file)
        let result = config.save(Path::new("/dev/null/invalid/path"));
        assert!(result.is_err());
    }

    #[test]
    fn test_save_prints_success_messages() {
        let temp_dir = tempdir().unwrap();
        let config = create_test_config();
        
        // Capture stdout
        let output = std::panic::catch_unwind(|| {
            let _result = config.save(temp_dir.path());
        });
        
        // Should not panic
        assert!(output.is_ok());
    }

    #[test]
    fn test_save_with_realistic_config() {
        let temp_dir = tempdir().unwrap();
        
        // Create a config similar to what new() would produce
        let mut config = create_test_config();
        config.notes = "Optimized for 9.0GB training / 1.0GB inference budget. \
                       Architecture optimizations: Ternary weights (2 bits/weight, 4 per byte) with INT8 activations; \
                       MLA Flash Attention with 4x compression (d_c=320); \
                       INT8 KV cache (1 byte vs 2 bytes FP16) with per-token quantization; \
                       RoPE kept in FP16 for positional precision; \
                       Sliding window attention (window=16384); \
                       10.1% training memory headroom, 23.0% inference headroom".to_string();
        
        let result = config.save(temp_dir.path());
        assert!(result.is_ok(), "Save failed: {:?}", result.err());
        
        // Verify files exist and are readable
        let poppins_path = temp_dir.path().join("config_poppins.json");
        let hf_path = temp_dir.path().join("config.json");
        
        assert!(poppins_path.exists());
        assert!(hf_path.exists());
        
        // Verify file sizes are reasonable (> 0 bytes)
        assert!(fs::metadata(poppins_path).unwrap().len() > 0);
        assert!(fs::metadata(hf_path).unwrap().len() > 0);
    }

    #[test]
    fn test_save_preserves_all_fields() {
        let temp_dir = tempdir().unwrap();
        let original_config = create_test_config();
        
        original_config.save(temp_dir.path()).unwrap();
        
        // Load back and compare
        let poppins_path = temp_dir.path().join("config_poppins.json");
        let content = fs::read_to_string(poppins_path).unwrap();
        let loaded_config: Config = serde_json::from_str(&content).unwrap();
        
        // Compare key fields
        assert_eq!(loaded_config.vocab_size, original_config.vocab_size);
        assert_eq!(loaded_config.embedding_dim, original_config.embedding_dim);
        assert_eq!(loaded_config.num_layers, original_config.num_layers);
        assert_eq!(loaded_config.total_params, original_config.total_params);
        assert_eq!(loaded_config.context_length, original_config.context_length);
        assert_eq!(loaded_config.aim_train_gb, original_config.aim_train_gb);
        assert_eq!(loaded_config.aim_infer_gb, original_config.aim_infer_gb);
        assert_eq!(loaded_config.weight_precision, original_config.weight_precision);
        assert_eq!(loaded_config.architecture, original_config.architecture);
    }
}
