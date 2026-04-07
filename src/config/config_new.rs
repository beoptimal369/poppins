// src/config/config_new.rs

use chrono::Utc;
use crate::config::Config;
use crate::bpe::BPETokenizer;
use crate::train_xml::TrainXMLConstantParsed;


impl Config {
    pub fn new(train_xml_constant_parsed: &TrainXMLConstantParsed, tokenizer: &BPETokenizer) -> Self {
        let current_effective_batch_size = train_xml_constant_parsed.batch_size * train_xml_constant_parsed.gradient_accumulation_steps;

        let mut config = Config {
            // ========== Core Architecture ==========
            vocab_size: tokenizer.vocab.len(),
            embedding_dim: 0,
            num_layers: 0,
            num_heads: 0,
            head_dim: 0,
            compression_dim: 0,
            rope_dim: 0,
            ffn_dim: 0,
            total_params: 0,
            
            // ========== Context & Attention ==========
            context_length: 0,
            sliding_window_size: 0,
            attention_type: "sliding_window".to_string(),
            use_per_token_quantization: true,
            attention_bias: false,
            
            // ========== Precision Settings ==========
            bits_per_weight: 2,
            bytes_per_ternary: 0.25,
            weight_precision: "ternary".to_string(),
            rope_precision: train_xml_constant_parsed.rope_precision.clone(),
            kv_cache_precision: train_xml_constant_parsed.kv_cache_precision.clone(),
            activation_precision: train_xml_constant_parsed.activation_precision.clone(),
            
            // ========== Normalization & Activation ==========
            norm_type: "rms_norm".to_string(),
            activation_fn: "squared_relu".to_string(),
            
            // ========== Training Configuration ==========
            optimizer: "AdamW".to_string(),
            lr_scheduler: "cosine".to_string(),
            batch_size: train_xml_constant_parsed.batch_size,
            num_workers: train_xml_constant_parsed.num_workers,
            effective_batch_size: current_effective_batch_size,
            warmup_steps: train_xml_constant_parsed.warmup_steps,
            val_interval: train_xml_constant_parsed.val_interval,
            learning_rate: train_xml_constant_parsed.learning_rate,
            mixed_precision: train_xml_constant_parsed.mixed_precision,
            use_tensor_cores: train_xml_constant_parsed.use_tensor_cores,
            use_flash_attention: train_xml_constant_parsed.use_flash_attention,
            gradient_accumulation_steps: train_xml_constant_parsed.gradient_accumulation_steps,

            weight_decay_response: train_xml_constant_parsed.weight_decay_response,
            weight_decay_source: train_xml_constant_parsed.weight_decay_source,
            weight_decay_code: train_xml_constant_parsed.weight_decay_code,

            loss_scale_response: train_xml_constant_parsed.loss_scale_response,
            loss_scale_source: train_xml_constant_parsed.loss_scale_source,
            loss_scale_code: train_xml_constant_parsed.loss_scale_code,

            gradient_scale_response: train_xml_constant_parsed.gradient_scale_response,
            gradient_scale_source: train_xml_constant_parsed.gradient_scale_source,
            gradient_scale_code: train_xml_constant_parsed.gradient_scale_code,

            gradient_clip_response: train_xml_constant_parsed.gradient_clip_response,
            gradient_clip_source: train_xml_constant_parsed.gradient_clip_source,
            gradient_clip_code: train_xml_constant_parsed.gradient_clip_code,
            
            // ========== Memory Budgets ==========
            train_memory_gb: 0.0,
            infer_memory_gb: 0.0,
            embedding_memory_mb: 0.0,
            attention_memory_mb: 0.0,
            ffn_memory_mb: 0.0,
            kv_cache_memory_mb: 0.0,
            aim_train_gb: train_xml_constant_parsed.aim_train_gb,
            aim_infer_gb: train_xml_constant_parsed.aim_infer_gb,
            
            // ========== Infer Defaults ==========
            infer_default_temperature: 0.7,
            infer_default_top_p: 0.9,
            infer_default_top_k: 50,
            infer_default_repetition_penalty: 1.1,
            infer_default_num_beams: 1,
            
            // ========== Tokenizer Settings ==========
            bos_token_id: tokenizer.token_to_id.get("<sample>").copied(),
            eos_token_id: tokenizer.token_to_id.get("</sample>").copied(),
            unk_token_id: tokenizer.token_to_id.get("<unknown>").copied(),
            pad_token_id: None,
            
            // ========== Paths & Metadata ==========
            model_type: "ternary_mla".to_string(),
            poppins_version: env!("CARGO_PKG_VERSION").to_string(),
            architecture: "TernaryMLA".to_string(),
            tokenizer_path: "tokenizer.json".to_string(),
            checkpoint_path: "checkpoints/model.pt".to_string(),
            created_at: Utc::now().to_rfc3339(),
            notes: String::new(),
        };

        // Set model config based on train.xml
        config.set();

        config.notes = format!(
            "Optimized for {:.1}GB training / {:.1}GB inference budget. \
            Architecture optimizations: \
            Ternary weights (2 bits/weight, 4 per byte) with INT8 activations; \
            MLA Flash Attention with {}x compression (d_c={}); \
            INT8 KV cache (1 byte vs 2 bytes FP16) with per-token quantization; \
            RoPE kept in FP16 for positional precision; \
            Sliding window attention (window={}); \
            {:.1}% training memory headroom, {:.1}% inference headroom",
            config.aim_train_gb, 
            config.aim_infer_gb,
            config.embedding_dim / config.compression_dim,
            config.compression_dim,
            config.sliding_window_size,
            (1.0 - (config.train_memory_gb / config.aim_train_gb)) * 100.0,
            (1.0 - (config.infer_memory_gb / config.aim_infer_gb)) * 100.0);

        config
    }
}



#[cfg(test)]
mod tests {
    use crate::config::Config;
    use crate::bpe::BPETokenizer;
    use std::collections::HashMap;
    use crate::train_xml::TrainXMLConstantParsed;

    // Helper function to create a test tokenizer
    fn create_test_tokenizer() -> BPETokenizer {
        let mut tokenizer = BPETokenizer {
            vocab: vec![
                "<unknown>".to_string(),
                "<sample>".to_string(),
                "</sample>".to_string(),
                "<system>".to_string(),
                "</system>".to_string(),
                "<prompt>".to_string(),
                "</prompt>".to_string(),
                "<ai>".to_string(),
                "</ai>".to_string(),
                "<text>".to_string(),
                "</text>".to_string(),
                "<source>".to_string(),
                "</source>".to_string(),
            ],
            token_to_id: HashMap::new(),
            merges: vec![],
            special_token_count: 1,
            initial_token_count: 1,
        };
        
        // Build token_to_id mapping
        for (id, token) in tokenizer.vocab.iter().enumerate() {
            tokenizer.token_to_id.insert(token.clone(), id as u32);
        }
        
        tokenizer
    }

    // Helper function to create test TrainXMLConstantParsed
    fn create_test_constants() -> TrainXMLConstantParsed {
        TrainXMLConstantParsed {
            warmup_steps: 360,
            val_interval: 10,
            aim_train_gb: 7.0,
            aim_infer_gb: 0.9,
            learning_rate: 1e-3,
            aim_loss: 0.45,
            batch_size: 32,
            num_workers: 8,
            kv_cache_precision: "int8".to_string(),
            rope_precision: "fp16".to_string(),
            mixed_precision: true,
            activation_precision: "int8".to_string(),
            gradient_accumulation_steps: 4,
            use_tensor_cores: true,
            use_flash_attention: true,

            bpe_min_merge_frequency: 3,
            bpe_requested_tokens: Vec::new(),

            weight_decay_response: 0.01,
            weight_decay_source: 0.05,
            weight_decay_code: 0.02,

            loss_scale_response: 1.0,
            loss_scale_source: 0.2,
            loss_scale_code: 1.0,

            gradient_scale_response: 1.0,
            gradient_scale_source: 2.0,
            gradient_scale_code: 1.2,

            gradient_clip_response: 1.0,
            gradient_clip_source: 0.0,
            gradient_clip_code: 0.7,
        }
    }

    #[test]
    fn test_config_new_basic_initialization() {
        let tokenizer = create_test_tokenizer();
        let constants = create_test_constants();
        
        let config = Config::new(&constants, &tokenizer);
        
        // Verify basic fields are set correctly
        assert_eq!(config.vocab_size, tokenizer.vocab.len());
        assert_eq!(config.bits_per_weight, 2);
        assert_eq!(config.bytes_per_ternary, 0.25);
        assert_eq!(config.weight_precision, "ternary");
        assert_eq!(config.norm_type, "rms_norm");
        assert_eq!(config.activation_fn, "squared_relu");
        assert_eq!(config.optimizer, "AdamW");
        assert_eq!(config.lr_scheduler, "cosine");
        assert_eq!(config.attention_type, "sliding_window");
        assert_eq!(config.use_per_token_quantization, true);
        assert_eq!(config.attention_bias, false);
    }

    #[test]
    fn test_config_new_precision_settings() {
        let tokenizer = create_test_tokenizer();
        let constants = create_test_constants();
        
        let config = Config::new(&constants, &tokenizer);
        
        // Verify precision settings are passed through
        assert_eq!(config.rope_precision, constants.rope_precision);
        assert_eq!(config.kv_cache_precision, constants.kv_cache_precision);
        assert_eq!(config.activation_precision, constants.activation_precision);
    }

    #[test]
    fn test_config_new_training_configuration() {
        let tokenizer = create_test_tokenizer();
        let constants = create_test_constants();
        
        let config = Config::new(&constants, &tokenizer);
        
        // Verify training config is correctly set
        assert_eq!(config.batch_size, constants.batch_size);
        assert_eq!(config.num_workers, constants.num_workers);
        assert_eq!(config.warmup_steps, constants.warmup_steps);
        assert_eq!(config.val_interval, constants.val_interval);
        assert_eq!(config.learning_rate, constants.learning_rate);
        assert_eq!(config.mixed_precision, constants.mixed_precision);
        assert_eq!(config.use_tensor_cores, constants.use_tensor_cores);
        assert_eq!(config.use_flash_attention, constants.use_flash_attention);
        assert_eq!(config.gradient_accumulation_steps, constants.gradient_accumulation_steps);
    }

    #[test]
    fn test_config_new_effective_batch_size() {
        let tokenizer = create_test_tokenizer();
        let constants = create_test_constants();
        
        let expected_effective_batch = constants.batch_size * constants.gradient_accumulation_steps;
        let config = Config::new(&constants, &tokenizer);
        
        assert_eq!(config.effective_batch_size, expected_effective_batch);
    }

    #[test]
    fn test_config_new_memory_budgets() {
        let tokenizer = create_test_tokenizer();
        let constants = create_test_constants();
        
        let config = Config::new(&constants, &tokenizer);
        
        // Verify memory budgets are set (values come from set() method)
        assert_eq!(config.aim_train_gb, constants.aim_train_gb);
        assert_eq!(config.aim_infer_gb, constants.aim_infer_gb);
        
        // Memory values should be calculated (non-zero)
        assert!(config.train_memory_gb > 0.0);
        assert!(config.infer_memory_gb > 0.0);
    }

    #[test]
    fn test_config_new_weight_decay_per_component() {
        let tokenizer = create_test_tokenizer();
        let constants = create_test_constants();
        
        let config = Config::new(&constants, &tokenizer);
        
        assert_eq!(config.weight_decay_response, constants.weight_decay_response);
        assert_eq!(config.weight_decay_source, constants.weight_decay_source);
        assert_eq!(config.weight_decay_code, constants.weight_decay_code);
    }

    #[test]
    fn test_config_new_loss_scale_per_component() {
        let tokenizer = create_test_tokenizer();
        let constants = create_test_constants();
        
        let config = Config::new(&constants, &tokenizer);
        
        assert_eq!(config.loss_scale_response, constants.loss_scale_response);
        assert_eq!(config.loss_scale_source, constants.loss_scale_source);
        assert_eq!(config.loss_scale_code, constants.loss_scale_code);
    }

    #[test]
    fn test_config_new_gradient_scale_per_component() {
        let tokenizer = create_test_tokenizer();
        let constants = create_test_constants();
        
        let config = Config::new(&constants, &tokenizer);
        
        assert_eq!(config.gradient_scale_response, constants.gradient_scale_response);
        assert_eq!(config.gradient_scale_source, constants.gradient_scale_source);
        assert_eq!(config.gradient_scale_code, constants.gradient_scale_code);
    }

    #[test]
    fn test_config_new_gradient_clip_per_component() {
        let tokenizer = create_test_tokenizer();
        let constants = create_test_constants();
        
        let config = Config::new(&constants, &tokenizer);
        
        assert_eq!(config.gradient_clip_response, constants.gradient_clip_response);
        assert_eq!(config.gradient_clip_source, constants.gradient_clip_source);
        assert_eq!(config.gradient_clip_code, constants.gradient_clip_code);
    }

    #[test]
    fn test_config_new_tokenizer_ids() {
        let tokenizer = create_test_tokenizer();
        let constants = create_test_constants();
        
        let config = Config::new(&constants, &tokenizer);
        
        // Verify token IDs are correctly set from tokenizer
        assert_eq!(config.bos_token_id, tokenizer.token_to_id.get("<sample>").copied());
        assert_eq!(config.eos_token_id, tokenizer.token_to_id.get("</sample>").copied());
        assert_eq!(config.unk_token_id, tokenizer.token_to_id.get("<unknown>").copied());
        assert_eq!(config.pad_token_id, None);
    }

    #[test]
    fn test_config_new_infer_defaults() {
        let tokenizer = create_test_tokenizer();
        let constants = create_test_constants();
        
        let config = Config::new(&constants, &tokenizer);
        
        assert_eq!(config.infer_default_temperature, 0.7);
        assert_eq!(config.infer_default_top_p, 0.9);
        assert_eq!(config.infer_default_top_k, 50);
        assert_eq!(config.infer_default_repetition_penalty, 1.1);
        assert_eq!(config.infer_default_num_beams, 1);
    }

    #[test]
    fn test_config_new_metadata() {
        let tokenizer = create_test_tokenizer();
        let constants = create_test_constants();
        
        let config = Config::new(&constants, &tokenizer);
        
        assert_eq!(config.model_type, "ternary_mla");
        assert_eq!(config.architecture, "TernaryMLA");
        assert_eq!(config.tokenizer_path, "tokenizer.json");
        assert_eq!(config.checkpoint_path, "checkpoints/model.pt");
        assert!(!config.created_at.is_empty());
        assert!(!config.notes.is_empty());
        assert!(config.notes.contains("Optimized for"));
        assert!(config.notes.contains(&format!("{:.1}GB training", constants.aim_train_gb)));
    }

    #[test]
    fn test_config_new_with_different_batch_sizes() {
        let tokenizer = create_test_tokenizer();
        
        // Test with small batch size
        let mut constants_small = create_test_constants();
        constants_small.batch_size = 1;
        constants_small.gradient_accumulation_steps = 1;
        
        let config_small = Config::new(&constants_small, &tokenizer);
        assert_eq!(config_small.effective_batch_size, 1);
        
        // Test with large batch size
        let mut constants_large = create_test_constants();
        constants_large.batch_size = 64;
        constants_large.gradient_accumulation_steps = 8;
        
        let config_large = Config::new(&constants_large, &tokenizer);
        assert_eq!(config_large.effective_batch_size, 512);
    }

    #[test]
    fn test_config_new_with_different_memory_budgets() {
        let tokenizer = create_test_tokenizer();
        
        // Test with smaller memory budget
        let mut constants_small = create_test_constants();
        constants_small.aim_train_gb = 4.0;
        constants_small.aim_infer_gb = 0.5;
        
        let config_small = Config::new(&constants_small, &tokenizer);
        assert_eq!(config_small.aim_train_gb, 4.0);
        assert_eq!(config_small.aim_infer_gb, 0.5);
        
        // Test with larger memory budget
        let mut constants_large = create_test_constants();
        constants_large.aim_train_gb = 16.0;
        constants_large.aim_infer_gb = 2.0;
        
        let config_large = Config::new(&constants_large, &tokenizer);
        assert_eq!(config_large.aim_train_gb, 16.0);
        assert_eq!(config_large.aim_infer_gb, 2.0);
    }

    #[test]
    fn test_config_new_notes_contains_memory_headroom() {
        let tokenizer = create_test_tokenizer();
        let constants = create_test_constants();
        
        let config = Config::new(&constants, &tokenizer);
        
        // Notes should contain memory headroom percentages
        assert!(config.notes.contains("% training memory headroom") || 
                config.notes.contains("% inference headroom"));
        assert!(config.notes.contains("INT8 KV cache"));
        assert!(config.notes.contains("Sliding window attention"));
    }

    #[test]
    fn test_config_new_all_fields_populated() {
        let tokenizer = create_test_tokenizer();
        let constants = create_test_constants();
        
        let config = Config::new(&constants, &tokenizer);
        
        // Verify architecture fields are set by set() method
        assert!(config.embedding_dim > 0);
        assert!(config.num_layers > 0);
        assert!(config.num_heads > 0);
        assert!(config.head_dim > 0);
        assert!(config.compression_dim > 0);
        assert!(config.rope_dim > 0);
        assert!(config.ffn_dim > 0);
        assert!(config.total_params > 0);
        
        // Verify context fields are set
        assert!(config.context_length > 0);
        assert!(config.sliding_window_size > 0);
    }
}
