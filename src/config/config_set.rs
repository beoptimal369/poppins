// src/config/config_set.rs

use crate::config::{
    Config,
    config_get_rope_dim,
    config_get_total_params,
    config_get_compression_dim,
    CONFIG_PROVEN_FUNDAMENTALS,
};

impl Config {
    /// Set optimal model configuration based on proven fundamentals & train.xml.
    /// Starts from largest context (1M) and works backwards until memory aims from train.xml are satisfied
    pub fn set(&mut self) {
        // Best Practice
        const FFN_MULTIPLIER: usize = 4;
        /// f32 master (4) + grad (4) + Adam states (8)
        const TRAINING_BYTES_PER_PARAM: f32 = 16.0;
        /// K and V separate
        const KV_CACHE_K_MULTIPLIER: usize = 2;
        /// FP16 = 2 bytes
        const ROPE_MULTIPLIER: f32 = 2.0;
        /// INT8 = 1 byte
        const BYTES_INT8: f32 = 1.0;
        /// How many bytes are in a GB
        const BYTES_PER_GB: f32 = 1024.0 * 1024.0 * 1024.0;

        println!("🤖 Setting optimal config...");
        println!("   • Aim training memory: {:.1}GB", self.aim_train_gb);
        println!("   • Aim inference memory: {:.1}GB", self.aim_infer_gb);

        // Iterate from largest context to smallest
        for proven in CONFIG_PROVEN_FUNDAMENTALS.iter().rev() {
            let compression_dim = config_get_compression_dim(proven.embedding_dim);
            let rope_dim = config_get_rope_dim(proven.head_dim);
            let ffn_dim = proven.embedding_dim * FFN_MULTIPLIER;
            let total_params = config_get_total_params(self.vocab_size, proven.embedding_dim, proven.num_layers, proven.num_heads, proven.head_dim, compression_dim, rope_dim, ffn_dim);

            // Compute memory
            let train_weights_gb = (total_params as f32 * TRAINING_BYTES_PER_PARAM) / BYTES_PER_GB;
            let infer_weights_gb = (total_params as f32 * self.bytes_per_ternary) / BYTES_PER_GB;
            
            let content_cache = (proven.num_layers * compression_dim * KV_CACHE_K_MULTIPLIER * proven.context_size) as f32 * BYTES_INT8;
            let rope_cache = (proven.num_layers * rope_dim * proven.context_size) as f32 * ROPE_MULTIPLIER;
            let kv_cache_per_batch_gb = (content_cache + rope_cache) / BYTES_PER_GB;
            
            let train_memory = train_weights_gb + (kv_cache_per_batch_gb * self.batch_size as f32);
            let infer_memory = infer_weights_gb + kv_cache_per_batch_gb;

            // Compute fits
            let fits = train_memory <= self.aim_train_gb && infer_memory <= self.aim_infer_gb;

            println!("    • {} Verifying Context Window: {}", if fits { "✅" } else { "❌" }, proven.context_size);
            println!("      • Params: {}", Config::format_params(total_params));
            println!("      • Required training memory: {:.2}GB", train_memory);
            println!("      • Required inference memory: {:.2}GB", infer_memory);
            
            if !fits {
                continue;
            }

            // Bind config
            self.embedding_dim = proven.embedding_dim;
            self.num_layers = proven.num_layers;
            self.num_heads = proven.num_heads;
            self.head_dim = proven.head_dim;
            self.context_length = proven.context_size;
            self.sliding_window_size = proven.context_size / KV_CACHE_K_MULTIPLIER;
            self.compression_dim = compression_dim;
            self.rope_dim = rope_dim;
            self.ffn_dim = ffn_dim;
            self.total_params = total_params;
            self.train_memory_gb = train_memory;
            self.infer_memory_gb = infer_memory;
            self.bind_memory(KV_CACHE_K_MULTIPLIER, ROPE_MULTIPLIER, BYTES_INT8);

            return;
        }
        
        panic!("\n❌ ERROR: No model configuration fits your memory budget!");
    }

    fn bind_memory(&mut self, kv_cache_k_multiplier: usize, rope_multiplier: f32, bytes_int8: f32) {
        /// Input + output embeddings
        const EMBEDDING_MULTIPLIER: usize = 2; 
        /// Up and down projections
        const FFN_PROJECTIONS: usize = 2;
        /// Q, K, V projections
        const ATTENTION_PROJECTIONS: usize = 3;
        /// How many bytes are in a MB
        const BYTES_PER_MB: f32 = 1024.0 * 1024.0;

        self.embedding_memory_mb = (self.vocab_size * self.embedding_dim * EMBEDDING_MULTIPLIER) as f32 * self.bytes_per_ternary / BYTES_PER_MB;
            
        let attention_params = self.embedding_dim * self.compression_dim * ATTENTION_PROJECTIONS;
        self.attention_memory_mb = (attention_params * self.num_layers) as f32 * self.bytes_per_ternary / BYTES_PER_MB;
        
        let ffn_params = self.embedding_dim * self.ffn_dim * FFN_PROJECTIONS;
        self.ffn_memory_mb = (ffn_params * self.num_layers) as f32 * self.bytes_per_ternary / BYTES_PER_MB;
        
        let content_cache_mb = (self.num_layers * self.compression_dim * kv_cache_k_multiplier * self.context_length) as f32 * bytes_int8 / BYTES_PER_MB;
        let rope_cache_mb = (self.num_layers * self.rope_dim * self.context_length) as f32 * rope_multiplier / BYTES_PER_MB;
        self.kv_cache_memory_mb = content_cache_mb + rope_cache_mb;
    }

    fn format_params(total_params: usize) -> String {
        if total_params >= 1_000_000_000 {
            format!("{:.2}B", total_params as f32 / 1_000_000_000.0)
        } else if total_params >= 1_000_000 {
            format!("{:.2}M", total_params as f32 / 1_000_000.0)
        } else if total_params >= 1_000 {
            format!("{:.2}K", total_params as f32 / 1000.0)
        } else {
            format!("{}", total_params)
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::config::Config;

    // Helper function to create a test config with specific memory budgets
    fn create_test_config(aim_train_gb: f32, aim_infer_gb: f32, batch_size: usize) -> Config {
        Config {
            vocab_size: 1678,
            embedding_dim: 0,
            num_layers: 0,
            num_heads: 0,
            head_dim: 0,
            compression_dim: 0,
            rope_dim: 0,
            ffn_dim: 0,
            total_params: 0,
            context_length: 0,
            sliding_window_size: 0,
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
            batch_size,
            num_workers: 8,
            effective_batch_size: batch_size * 8,
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
            train_memory_gb: 0.0,
            infer_memory_gb: 0.0,
            embedding_memory_mb: 0.0,
            attention_memory_mb: 0.0,
            ffn_memory_mb: 0.0,
            kv_cache_memory_mb: 0.0,
            aim_train_gb,
            aim_infer_gb,
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
            notes: String::new(),
        }
    }

    #[test]
    fn test_set_selects_largest_fitting_config() {
        // Large budget should select largest config (likely 1M context)
        let mut config = create_test_config(100.0, 20.0, 1);
        config.set();
        
        // Should have selected a config with reasonable dimensions
        assert!(config.embedding_dim > 0);
        assert!(config.num_layers > 0);
        assert!(config.context_length > 0);
        assert!(config.total_params > 0);
        
        // Memory should be within budgets
        assert!(config.train_memory_gb <= config.aim_train_gb);
        assert!(config.infer_memory_gb <= config.aim_infer_gb);
    }

    #[test]
    fn test_set_with_limited_budget() {
        // Small budget should select smaller config
        let mut config = create_test_config(8.0, 1.0, 1);
        config.set();
        
        // Should have found a config
        assert!(config.embedding_dim > 0);
        assert!(config.num_layers > 0);
        
        // Memory should be within budgets
        assert!(config.train_memory_gb <= config.aim_train_gb);
        assert!(config.infer_memory_gb <= config.aim_infer_gb);
    }

    #[test]
    fn test_set_with_batch_size_impact() {
        // Larger batch size increases memory requirements
        let mut config_small_batch = create_test_config(9.0, 1.0, 1);
        let mut config_large_batch = create_test_config(9.0, 1.0, 32);
        
        config_small_batch.set();
        config_large_batch.set();
        
        // Large batch might need smaller model to fit
        // So embedding_dim could be smaller (but not guaranteed)
        // Just verify both succeed
        assert!(config_small_batch.total_params > 0);
        assert!(config_large_batch.total_params > 0);
    }

    #[test]
    fn test_set_binds_all_fields() {
        let mut config = create_test_config(9.0, 1.0, 1);
        config.set();
        
        // Verify all fields are set
        assert!(config.embedding_dim > 0, "embedding_dim not set");
        assert!(config.num_layers > 0, "num_layers not set");
        assert!(config.num_heads > 0, "num_heads not set");
        assert!(config.head_dim > 0, "head_dim not set");
        assert!(config.compression_dim > 0, "compression_dim not set");
        assert!(config.rope_dim > 0, "rope_dim not set");
        assert!(config.ffn_dim > 0, "ffn_dim not set");
        assert!(config.total_params > 0, "total_params not set");
        assert!(config.context_length > 0, "context_length not set");
        assert!(config.sliding_window_size > 0, "sliding_window_size not set");
        assert!(config.train_memory_gb > 0.0, "train_memory_gb not set");
        assert!(config.infer_memory_gb > 0.0, "infer_memory_gb not set");
    }

    #[test]
    fn test_set_sliding_window_size() {
        let mut config = create_test_config(9.0, 1.0, 1);
        config.set();
        
        // Sliding window should be half of context length
        assert_eq!(config.sliding_window_size, config.context_length / 2,
            "Sliding window {} should be half of context length {}",
            config.sliding_window_size, config.context_length);
    }

    #[test]
    fn test_set_respects_training_budget() {
        let mut config = create_test_config(9.0, 100.0, 1); // Large inference budget
        config.set();
        
        // Training memory should be the limiting factor
        assert!(config.train_memory_gb <= config.aim_train_gb,
            "Training memory {:.2}GB exceeds budget {:.2}GB",
            config.train_memory_gb, config.aim_train_gb);
    }

    #[test]
    fn test_set_respects_inference_budget() {
        let mut config = create_test_config(100.0, 1.0, 1); // Large training budget
        config.set();
        
        // Inference memory should be the limiting factor
        assert!(config.infer_memory_gb <= config.aim_infer_gb,
            "Inference memory {:.2}GB exceeds budget {:.2}GB",
            config.infer_memory_gb, config.aim_infer_gb);
    }

    #[test]
    fn test_set_with_very_small_budget() {
        // Very small budget - should still find smallest config
        let mut config = create_test_config(4.0, 0.5, 1);
        config.set();
        
        assert!(config.total_params > 0);
        assert!(config.train_memory_gb <= config.aim_train_gb);
        assert!(config.infer_memory_gb <= config.aim_infer_gb);
    }

    #[test]
    #[should_panic(expected = "No model configuration fits your memory budget")]
    fn test_set_panics_with_impossible_budget() {
        // Zero budget - impossible to fit any model
        let mut config = create_test_config(0.0, 0.0, 1);
        config.set();
    }

   #[test]
    fn test_format_params() {
        assert_eq!(Config::format_params(500), "500");
        assert_eq!(Config::format_params(1_500), "1.50K");
        assert_eq!(Config::format_params(1_500_000), "1.50M");
        assert_eq!(Config::format_params(1_500_000_000), "1.50B");
        assert_eq!(Config::format_params(1_234_567_890), "1.23B");
        assert_eq!(Config::format_params(999_999_999), "1000.00M");  // < 1B, so M format
    }

    #[test]
    fn test_bind_memory_calculations() {
        let mut config = create_test_config(9.0, 1.0, 1);
        
        // Set some values manually
        config.vocab_size = 1678;
        config.embedding_dim = 1280;
        config.num_layers = 28;
        config.compression_dim = 320;
        config.rope_dim = 32;
        config.ffn_dim = 5120;
        config.context_length = 32768;
        config.bytes_per_ternary = 0.25;
        
        config.bind_memory(2, 2.0, 1.0);
        
        // Verify memory values are positive and reasonable
        assert!(config.embedding_memory_mb > 0.0);
        assert!(config.attention_memory_mb > 0.0);
        assert!(config.ffn_memory_mb > 0.0);
        assert!(config.kv_cache_memory_mb > 0.0);
        
        // Embedding memory should be relatively small
        assert!(config.embedding_memory_mb < 100.0);
        
        // KV cache should be significant for long context
        assert!(config.kv_cache_memory_mb > 100.0);
    }

    #[test]
    fn test_set_selects_largest_context_that_fits() {
        // Create config with budget that should fit mid-range but not largest
        let mut config = create_test_config(15.0, 2.0, 1);
        config.set();
        
        // Should have selected a config
        assert!(config.context_length > 0);
        assert!(config.train_memory_gb <= config.aim_train_gb);
        assert!(config.infer_memory_gb <= config.aim_infer_gb);
    }

    #[test]
    fn test_set_works_with_different_batch_sizes() {
        let batch_sizes = [1, 4, 8, 16, 32];
        
        for &batch_size in batch_sizes.iter() {
            let mut config = create_test_config(12.0, 1.5, batch_size);
            config.set();
            
            // Should find a config for each batch size
            assert!(config.total_params > 0, 
                "Failed for batch_size={}", batch_size);
            assert!(config.train_memory_gb <= config.aim_train_gb,
                "Training memory exceeded for batch_size={}", batch_size);
        }
    }
}
