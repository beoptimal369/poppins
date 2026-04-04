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
