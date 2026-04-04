// src/config/config_huggingface.rs

use core::f32;
use super::config::Config;
use serde::{Serialize, Deserialize};


/// Hugging Face compatible configuration format
#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigHuggingFace {
    // Core architecture
    pub architectures: Vec<String>,
    pub model_type: String,
    pub vocab_size: usize,
    pub hidden_size: usize,
    pub num_hidden_layers: usize,
    pub num_attention_heads: usize,
    pub head_dim: usize,
    pub intermediate_size: usize,
    pub max_position_embeddings: usize,
    
    // MLA specific config
    pub mla_config: ConfigHuggingFaceMLA,
    
    // Sliding window attention
    pub sliding_window: usize,
    pub attention_bias: bool,
    
    // Token IDs
    pub bos_token_id: Option<u32>,
    pub eos_token_id: Option<u32>,
    pub unk_token_id: Option<u32>,
    pub pad_token_id: Option<u32>,
    
    // Rotary embeddings
    pub rope_theta: f32,
    pub rope_scaling: Option<serde_json::Value>,
    
    // Normalization
    pub norm_type: String,
    pub norm_eps: f32,
    pub hidden_act: String,
    
    // Dropout
    pub attention_dropout: f32,
    pub dropout: f32,
    pub embedding_dropout: f32,
    
    // Initialization
    pub initializer_range: f32,
    
    // Precision
    pub torch_dtype: String,
    pub tie_word_embeddings: bool,
    pub use_cache: bool,
    
    // Quantization config
    pub quantization_config: ConfigHuggingFaceQuantization,
    
    // Transformers version compatibility
    pub transformers_version: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigHuggingFaceMLA {
    pub compression_dim: usize,
    pub rope_dim: usize,
    pub kv_lora_rank: usize,
    pub q_lora_rank: Option<usize>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigHuggingFaceQuantization {
    pub bits: u8,
    pub rope_precision: String,
    pub weight_precision: String,
    pub kv_cache_precision: String,
    pub activation_precision: String,
    pub use_per_token_quantization: bool,
}


impl ConfigHuggingFace {
    /// Create a Hugging Face compatible config from a Config instance
    pub fn new(config: &Config) -> Self {
        Self {
            // Core architecture
            architectures: vec!["TernaryMLA".to_string()],
            model_type: "ternary_mla".to_string(),
            vocab_size: config.vocab_size,
            hidden_size: config.embedding_dim,
            num_hidden_layers: config.num_layers,
            num_attention_heads: config.num_heads,
            head_dim: config.head_dim,
            intermediate_size: config.ffn_dim,
            max_position_embeddings: config.context_length,
            
            // MLA specific config
            mla_config: ConfigHuggingFaceMLA {
                compression_dim: config.compression_dim,
                rope_dim: config.rope_dim,
                kv_lora_rank: config.compression_dim,
                q_lora_rank: None,
            },
            
            // Sliding window attention
            sliding_window: config.sliding_window_size,
            attention_bias: false,
            
            // Token IDs
            bos_token_id: config.bos_token_id,
            eos_token_id: config.eos_token_id,
            unk_token_id: config.unk_token_id,
            pad_token_id: config.pad_token_id,
            
            // Rotary embeddings
            rope_theta: 10000.0,
            rope_scaling: None,
            
            // Normalization
            norm_type: config.norm_type.clone(),
            norm_eps: f32::EPSILON,
            hidden_act: config.activation_fn.clone(),
            
            // Dropout
            dropout: 0.0,
            attention_dropout: 0.0,
            embedding_dropout: 0.0,
            
            // Initialization
            initializer_range: 0.0,
            
            // Precision
            torch_dtype: "bfloat16".to_string(),
            tie_word_embeddings: true,
            use_cache: true,
            
            // Quantization config
            quantization_config: ConfigHuggingFaceQuantization {
                weight_precision: config.weight_precision.clone(),
                bits: config.bits_per_weight,
                activation_precision: config.activation_precision.clone(),
                kv_cache_precision: config.kv_cache_precision.clone(),
                rope_precision: config.rope_precision.clone(),
                use_per_token_quantization: config.use_per_token_quantization,
            },
            
            // Transformers version compatibility
            transformers_version: "4.36.0".to_string(),
        }
    }
}
