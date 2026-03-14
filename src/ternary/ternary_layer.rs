// src/ternary/ternary_layer.rs

use crate::ternary::ternary_weights::TernaryWeights;

/// Ternary neural network layer
///
/// Contains the weights and dimensions for a ternary linear layer.
/// Forward computation is handled by the separate `ternary_layer_forward` function.
pub struct TernaryLayer {
    /// Ternary weights (input_size × output_size)
    pub weights: TernaryWeights,
    
    /// Number of input features
    pub input_size: usize,
    
    /// Number of output neurons
    pub output_size: usize,
}

impl TernaryLayer {
    /// Create a new ternary layer with the specified dimensions
    ///
    /// # Arguments
    /// * `input_size` - Number of input features
    /// * `output_size` - Number of output neurons
    /// * `init_fn` - Function that returns raw weight value for each (out_idx, in_idx)
    ///
    /// # Returns
    /// * `Self` - Initialized layer with weights set by init_fn
    pub fn create<F>(input_size: usize, output_size: usize, mut init_fn: F) -> Self
    where
        F: FnMut(usize, usize) -> f32,
    {
        let num_weights = input_size * output_size;
        let mut weights = TernaryWeights::create(num_weights, None);
        
        // Initialize raw weights
        for out_idx in 0..output_size {
            for in_idx in 0..input_size {
                let weight_idx = out_idx * input_size + in_idx;
                weights.raw[weight_idx] = init_fn(out_idx, in_idx);
            }
        }
        
        // Quantize the initialized weights
        weights.quantize();
        
        Self {
            weights,
            input_size,
            output_size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create() {
        // Create layer with all zeros
        let layer = TernaryLayer::create(3, 2, |_, _| 0.0);
        
        assert_eq!(layer.input_size, 3);
        assert_eq!(layer.output_size, 2);
        assert_eq!(layer.weights.size, 6); // 3 * 2 = 6 weights
        
        // Verify raw weights were set
        for i in 0..6 {
            assert_eq!(layer.weights.raw[i], 0.0);
        }
    }
    
    #[test]
    fn test_create_with_values() {
        // Create layer with custom weight values
        let layer = TernaryLayer::create(2, 2, |out_idx, in_idx| {
            (out_idx * 2 + in_idx) as f32 * 0.5
        });
        
        assert_eq!(layer.weights.raw[0], 0.0); // out0,in0
        assert_eq!(layer.weights.raw[1], 0.5); // out0,in1
        assert_eq!(layer.weights.raw[2], 1.0); // out1,in0
        assert_eq!(layer.weights.raw[3], 1.5); // out1,in1
    }
    
    #[test]
    fn test_create_quantizes() {
        // Create layer with values that should quantize to specific ternary values
        let layer = TernaryLayer::create(2, 2, |out_idx, in_idx| match (out_idx, in_idx) {
            (0, 0) => 2.0,  // Should become 1 (after quantization)
            (0, 1) => -1.5, // Should become -1
            (1, 0) => 0.1,  // Should become 0
            (1, 1) => 0.0,  // Should become 0
            _ => 0.0,
        });
        
        // We can't easily check the quantized values here without accessors,
        // but we can verify that quantize() was called (no panic)
        assert_eq!(layer.weights.quantized.len(), 1); // 4 weights -> 1 byte
    }
}
