// src/config/config_get_rope_dim.rs

/// Find RoPE dimension (d_h^R)
/// Typically 1/4 to 1/2 of head_dim
pub fn config_get_rope_dim(head_dim: usize) -> usize {
    let dim = if head_dim >= 128 {
        head_dim / 4 // Smaller ratio for larger heads
    } else {
        head_dim / 2 // Larger ratio for smaller heads
    };
    
    // Align to 8 for efficiency
    let result = ((dim + 7) / 8) * 8;

    result
}

#[cfg(test)]
mod tests {
    use super::config_get_rope_dim;

    #[test]
    fn test_large_heads_quarter_ratio() {
        // Head dimensions >= 128 should use head_dim / 4, then align to 8
        let test_cases = vec![
            (128, 32),   // 128/4 = 32, aligned to 8 = 32
            (144, 40),   // 144/4 = 36, (36+7)/8 = 43/8 = 5.375 -> 5*8 = 40
            (160, 40),   // 160/4 = 40, aligned to 8 = 40
            (192, 48),   // 192/4 = 48, aligned to 8 = 48
            (256, 64),   // 256/4 = 64, aligned to 8 = 64
            (1280, 320), // 1280/4 = 320, aligned to 8 = 320
        ];
        
        for (input, expected) in test_cases {
            let result = config_get_rope_dim(input);
            assert_eq!(result, expected, 
                "head_dim={}: expected rope_dim={}, got={}", 
                input, expected, result);
        }
    }

    #[test]
    fn test_small_heads_half_ratio() {
        // Head dimensions < 128 should use head_dim / 2, then align to 8
        let test_cases = vec![
            (8, 8),     // 8/2 = 4, (4+7)/8 = 11/8 = 1.375 -> 1*8 = 8
            (16, 8),    // 16/2 = 8, aligned to 8 = 8
            (32, 16),   // 32/2 = 16, aligned to 8 = 16
            (48, 24),   // 48/2 = 24, aligned to 8 = 24
            (64, 32),   // 64/2 = 32, aligned to 8 = 32
            (96, 48),   // 96/2 = 48, aligned to 8 = 48
            (112, 56),  // 112/2 = 56, aligned to 8 = 56
            (120, 64),  // 120/2 = 60, (60+7)/8 = 67/8 = 8.375 -> 8*8 = 64
            (127, 64),  // 127/2 = 63, (63+7)/8 = 70/8 = 8.75 -> 8*8 = 64
        ];
        
        for (input, expected) in test_cases {
            let result = config_get_rope_dim(input);
            assert_eq!(result, expected, 
                "head_dim={}: expected rope_dim={}, got={}", 
                input, expected, result);
        }
    }

    #[test]
    fn test_boundary_at_128() {
        // Test boundary between half and quarter ratio
        let just_below = config_get_rope_dim(127);
        let at_boundary = config_get_rope_dim(128);
        let just_above = config_get_rope_dim(129);
        
        // 127 < 128: uses half ratio (63 -> aligned to 64)
        // 128 >= 128: uses quarter ratio (32 -> aligned to 32)
        assert_ne!(just_below, at_boundary, 
            "Boundary at 128 should change ratio: below={}, at={}", 
            just_below, at_boundary);
        
        // 128 and 129 should both use quarter ratio
        assert_eq!(at_boundary, just_above,
            "Both 128 and 129 should use quarter ratio: {} vs {}", 
            at_boundary, just_above);
    }

    #[test]
    fn test_alignment_to_8() {
        // Verify that all results are multiples of 8
        let test_values = [
            8, 16, 32, 48, 64, 96, 112, 128, 144, 160, 192, 256, 320, 384, 512, 640, 768, 1024, 1280
        ];
        
        for &dim in test_values.iter() {
            let result = config_get_rope_dim(dim);
            assert_eq!(result % 8, 0, 
                "rope_dim={} from head_dim={} is not aligned to 8", 
                result, dim);
        }
    }

    #[test]
    fn test_rope_dim_never_exceeds_head_dim() {
        // rope_dim should always be <= head_dim (since it's a subset)
        let test_values = [
            8, 16, 32, 48, 64, 96, 112, 128, 144, 160, 192, 256, 320, 384, 512, 640, 768, 1024, 1280
        ];
        
        for &head_dim in test_values.iter() {
            let rope_dim = config_get_rope_dim(head_dim);
            assert!(rope_dim <= head_dim, 
                "rope_dim={} exceeds head_dim={}", rope_dim, head_dim);
        }
    }

    #[test]
    fn test_rope_dim_at_least_8() {
        // For head_dim >= 16, rope_dim should be at least 8
        let test_values = [16, 32, 48, 64, 96, 112, 128];
        
        for &head_dim in test_values.iter() {
            let rope_dim = config_get_rope_dim(head_dim);
            assert!(rope_dim >= 8,
                "head_dim={} gave rope_dim={}, expected >=8", 
                head_dim, rope_dim);
        }
    }

    #[test]
    fn test_common_head_dimensions() {
        // Test common head dimensions found in popular models
        let test_cases = vec![
            (64, 32),    // Common in BERT-base, GPT-2 (64/2=32)
            (128, 32),   // Common in LLaMA, GPT-3 (128/4=32)
            (256, 64),   // Common in larger models (256/4=64)
            (80, 40),    // Some models use 80 (80/2=40 -> 40 is multiple of 8)
            (96, 48),    // Some models use 96 (96/2=48)
            (192, 48),   // 192/4=48
        ];
        
        for (input, expected) in test_cases {
            let result = config_get_rope_dim(input);
            assert_eq!(result, expected, 
                "head_dim={}: expected rope_dim={}, got={}", 
                input, expected, result);
        }
    }

    #[test]
    fn test_quarter_ratio_alignment_edge_cases() {
        // Test alignment edge cases for quarter ratio
        let test_cases = vec![
            (130, 32),   // 130/4 = 32.5 -> 32, aligned to 8 = 32
            (132, 40),   // 132/4 = 33, (33+7)/8 = 40/8=5 -> 5*8=40
            (134, 40),   // 134/4 = 33.5 -> 33, (33+7)/8=40/8=5 -> 40
            (136, 40),   // 136/4 = 34, (34+7)/8=41/8=5.125 -> 5*8=40
            (140, 40),   // 140/4 = 35, (35+7)/8=42/8=5.25 -> 5*8=40
            (144, 40),   // 144/4 = 36, (36+7)/8=43/8=5.375 -> 5*8=40
            (160, 40),   // 160/4 = 40, aligned to 8 = 40
        ];
        
        for (input, expected) in test_cases {
            let result = config_get_rope_dim(input);
            assert_eq!(result, expected, 
                "head_dim={}: expected rope_dim={}, got={}", 
                input, expected, result);
        }
    }

    #[test]
    fn test_extreme_values() {
        // Test minimum valid head_dim - integer division gives 0
        let min = config_get_rope_dim(1);
        assert_eq!(min, 0, "head_dim=1: 1/2=0 (integer division), aligned to 0");
        
        // head_dim=2: 2/2=1, (1+7)/8=1, 1*8=8
        let small = config_get_rope_dim(2);
        assert_eq!(small, 8, "head_dim=2: 2/2=1, aligned to 8");
        
        // head_dim=3: 3/2=1, aligned to 8
        let three = config_get_rope_dim(3);
        assert_eq!(three, 8, "head_dim=3: 3/2=1, aligned to 8");
        
        // head_dim=4: 4/2=2, (2+7)/8=9/8=1, 1*8=8
        let tiny = config_get_rope_dim(4);
        assert_eq!(tiny, 8, "head_dim=4: 4/2=2, aligned to 8");
        
        // head_dim=8: 8/2=4, (4+7)/8=11/8=1, 1*8=8
        let eight = config_get_rope_dim(8);
        assert_eq!(eight, 8, "head_dim=8: 8/2=4, aligned to 8");
        
        // head_dim=15: 15/2=7, (7+7)/8=14/8=1, 1*8=8
        let fifteen = config_get_rope_dim(15);
        assert_eq!(fifteen, 8, "head_dim=15: 15/2=7, aligned to 8");
        
        // head_dim=16: 16/2=8, aligned to 8
        let sixteen = config_get_rope_dim(16);
        assert_eq!(sixteen, 8, "head_dim=16: 16/2=8, aligned to 8");
        
        // head_dim=17: 17/2=8, aligned to 8
        let seventeen = config_get_rope_dim(17);
        assert_eq!(seventeen, 8, "head_dim=17: 17/2=8, aligned to 8");
        
        // Test very large head_dim
        let large = config_get_rope_dim(4096);
        assert_eq!(large, 1024, "head_dim=4096: expected 1024 (4096/4=1024)");
        
        // Test maximum realistic head_dim
        let max_realistic = config_get_rope_dim(8192);
        assert_eq!(max_realistic, 2048, "head_dim=8192: expected 2048");
    }

    #[test]
    fn test_rope_dim_for_your_config() {
        // Your current config: head_dim = 128
        let result = config_get_rope_dim(128);
        assert_eq!(result, 32, "For head_dim=128, expected rope_dim=32");
        
        // Test other potential configs
        assert_eq!(config_get_rope_dim(64), 32);
        assert_eq!(config_get_rope_dim(96), 48);
        assert_eq!(config_get_rope_dim(256), 64);
    }

    #[test]
    fn test_consistency_with_different_approaches() {
        // Verify that rope_dim is consistent with different head_dim values
        // that would appear in the same model family
        
        // For a model with embedding_dim=1280, num_heads=10, head_dim=128
        assert_eq!(config_get_rope_dim(128), 32);
        
        // For a model with embedding_dim=1024, num_heads=8, head_dim=128
        assert_eq!(config_get_rope_dim(128), 32);
        
        // For a model with embedding_dim=768, num_heads=12, head_dim=64
        assert_eq!(config_get_rope_dim(64), 32);
        
        // rope_dim should be consistent across models with same head_dim
        assert_eq!(config_get_rope_dim(64), config_get_rope_dim(64));
        assert_eq!(config_get_rope_dim(128), config_get_rope_dim(128));
    }

    #[test]
    fn test_rope_dim_values_are_practical() {
        // Verify that rope_dim values are practical for real models
        let practical_head_dims = [64, 80, 96, 112, 128, 144, 160, 192, 256];
        
        for &head_dim in practical_head_dims.iter() {
            let rope_dim = config_get_rope_dim(head_dim);
            // rope_dim should be at least 8 and at most head_dim
            assert!(rope_dim >= 8 || rope_dim == 0, 
                "head_dim={} gave impractical rope_dim={}", head_dim, rope_dim);
            assert!(rope_dim <= head_dim, 
                "head_dim={} rope_dim={} exceeds head_dim", head_dim, rope_dim);
            // rope_dim should be a multiple of 8
            assert_eq!(rope_dim % 8, 0, 
                "head_dim={} rope_dim={} not multiple of 8", head_dim, rope_dim);
        }
    }
}
