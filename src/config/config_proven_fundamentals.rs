// src/config/config_proven_fundamentals.rs


/// Proven, battle-tested model configurations at various context window sizes.
/// Based on industry best practices from DeepSeek & BitNet architectures.
#[derive(Debug, Clone)]
pub struct ConfigProvenFundamentals {
    // Core dimensions
    pub embedding_dim: usize,
    pub num_layers: usize,
    pub num_heads: usize,
    pub head_dim: usize,
    pub context_size: usize,
}


pub const CONFIG_PROVEN_FUNDAMENTALS: &[ConfigProvenFundamentals] = &[
    // 128 token context
    ConfigProvenFundamentals {
        embedding_dim: 256,
        num_layers: 4,
        num_heads: 4,
        head_dim: 64,
        context_size: 128,
    },
    
    // 256 token context
    ConfigProvenFundamentals {
        embedding_dim: 384,
        num_layers: 6,
        num_heads: 6,
        head_dim: 64,
        context_size: 256,
    },
    
    // 512 token context
    ConfigProvenFundamentals {
        embedding_dim: 512,
        num_layers: 8,
        num_heads: 8,
        head_dim: 64,
        context_size: 512,
    },
    
    // 1K token context
    ConfigProvenFundamentals {
        embedding_dim: 512,
        num_layers: 12,
        num_heads: 8,
        head_dim: 64,
        context_size: 1024,
    },
    
    // 2K token context
    ConfigProvenFundamentals {
        embedding_dim: 768,
        num_layers: 12,
        num_heads: 12,
        head_dim: 64,
        context_size: 2048,
    },
    
    // 4K token context
    ConfigProvenFundamentals {
        embedding_dim: 768,
        num_layers: 16,
        num_heads: 12,
        head_dim: 64,
        context_size: 4096,
    },
    
    // 8K token context
    ConfigProvenFundamentals {
        embedding_dim: 1024,
        num_layers: 16,
        num_heads: 16,
        head_dim: 64,
        context_size: 8192,
    },
    
    // 16K token context
    ConfigProvenFundamentals {
        embedding_dim: 1024,
        num_layers: 20,
        num_heads: 16,
        head_dim: 64,
        context_size: 16384,
    },
    
    // 32K token context (sweet spot for most applications)
    ConfigProvenFundamentals {
        embedding_dim: 1280,
        num_layers: 24,
        num_heads: 20,
        head_dim: 64,
        context_size: 32768,
    },
    
    // 65K token context
    ConfigProvenFundamentals {
        embedding_dim: 1536,
        num_layers: 28,
        num_heads: 24,
        head_dim: 64,
        context_size: 65536,
    },
    
    // 131K token context
    ConfigProvenFundamentals {
        embedding_dim: 1536,
        num_layers: 32,
        num_heads: 24,
        head_dim: 64,
        context_size: 131072,
    },
    
    // 262K token context
    ConfigProvenFundamentals {
        embedding_dim: 2048,
        num_layers: 32,
        num_heads: 32,
        head_dim: 64,
        context_size: 262144,
    },
    
    // 524K token context
    ConfigProvenFundamentals {
        embedding_dim: 2048,
        num_layers: 40,
        num_heads: 32,
        head_dim: 64,
        context_size: 524288,
    },
    
    // 1M token context
    ConfigProvenFundamentals {
        embedding_dim: 2560,
        num_layers: 48,
        num_heads: 40,
        head_dim: 64,
        context_size: 1048576,
    },
];
