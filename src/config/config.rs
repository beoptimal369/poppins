// src/config/config.rs

use serde::{Serialize, Deserialize};


/// Model configuration calculated from aim memory constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    // ========== Core Architecture ==========
    pub vocab_size: usize,
    pub embedding_dim: usize,
    pub num_layers: usize,
    pub num_heads: usize,
    pub head_dim: usize,
    pub compression_dim: usize,
    pub rope_dim: usize,
    pub ffn_dim: usize,
    pub total_params: usize,
    
    // ========== Context & Attention ==========
    pub context_length: usize,
    pub sliding_window_size: usize,
    pub attention_type: String,
    pub use_per_token_quantization: bool,
    pub attention_bias: bool,
    
    // ========== Precision Settings ==========
    pub weight_precision: String,
    pub bits_per_weight: u8,
    pub bytes_per_ternary: f32,
    pub activation_precision: String,
    pub kv_cache_precision: String,
    pub rope_precision: String,
    
    // ========== Normalization & Activation ==========
    pub norm_type: String,
    pub activation_fn: String,

    // ========== Training Configuration ==========
    pub batch_size: usize,
    pub gradient_accumulation_steps: usize,
    /// batch_size * gradient_accumulation_steps
    pub effective_batch_size: usize,
    pub learning_rate: f32,
    pub warmup_steps: usize,
    pub optimizer: String,
    pub lr_scheduler: String,
    pub mixed_precision: bool,
    pub val_interval: usize,
    pub num_workers: usize,
    pub use_tensor_cores: bool,
    pub use_flash_attention: bool,

    pub weight_decay_response: f32,
    pub weight_decay_source: f32,
    pub weight_decay_code: f32,

    pub loss_scale_response: f32,
    pub loss_scale_source: f32,
    pub loss_scale_code: f32,

    pub gradient_scale_response: f32,
    pub gradient_scale_source: f32,
    pub gradient_scale_code: f32,

    pub gradient_clip_response: f32,
    pub gradient_clip_source: f32,
    pub gradient_clip_code: f32,

    // ========== Memory Budgets ==========
    pub train_memory_gb: f32,
    pub infer_memory_gb: f32,
    pub embedding_memory_mb: f32,
    pub attention_memory_mb: f32,
    pub ffn_memory_mb: f32,
    pub kv_cache_memory_mb: f32,
    pub aim_train_gb: f32,
    pub aim_infer_gb: f32,
    
    // ========== Infer Defaults ==========
    pub infer_default_temperature: f32,
    pub infer_default_top_p: f32,
    pub infer_default_top_k: usize,
    pub infer_default_repetition_penalty: f32,
    pub infer_default_num_beams: usize,
    
    // ========== Tokenizer Settings ==========
    pub bos_token_id: Option<u32>,
    pub eos_token_id: Option<u32>,
    pub unk_token_id: Option<u32>,
    pub pad_token_id: Option<u32>,
    
    // ========== Paths & Metadata ==========
    pub model_type: String,
    pub poppins_version: String,
    pub architecture: String,
    pub tokenizer_path: String,
    pub checkpoint_path: String,
    pub created_at: String,
    pub notes: String,
}
