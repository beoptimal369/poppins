// src/ternary/ternary_layer_forward.rs

use crate::ternary::ternary_weights::TernaryWeights;
use crate::ternary::ternary_computation::TernaryComputation;

/// Get packing position for a weight index
///
/// # Returns
/// * `(usize, usize)` - (byte_index, bit_shift)
#[inline]
fn get_packing_position(index: usize) -> (usize, usize) {
    (index / 4, (index % 4) * 2)
}

/// Forward pass through a ternary layer
///
/// Processes a batch of inputs through a ternary layer using O(1)
/// lookup tables for multiplication.
///
/// # Arguments
/// * `weights` - Ternary weights (input_size × output_size)
/// * `input_size` - Number of input features
/// * `output_size` - Number of output neurons
/// * `inputs_batch` - Slice of input vectors, each of length `input_size`
/// * `computation` - Pre-computed lookup table for ternary operations
///
/// # Returns
/// * `Vec<Vec<i32>>` - Batch of output vectors, each of length `output_size`
///
/// # Panics
/// * If any input vector length doesn't match `input_size`
pub fn ternary_layer_forward(
    weights: &TernaryWeights,
    input_size: usize,
    output_size: usize,
    inputs_batch: &[Vec<i8>],
    computation: &TernaryComputation,
) -> Vec<Vec<i32>> {
    let mut results = Vec::with_capacity(inputs_batch.len());
    
    for inputs in inputs_batch {
        assert_eq!(
            inputs.len(), 
            input_size, 
            "Input vector length {} does not match layer input_size {}",
            inputs.len(), 
            input_size
        );
        
        let mut outputs = vec![0i32; output_size];
        
        // For each output neuron
        for out_idx in 0..output_size {
            let mut sum = 0i32;
            
            // Dot product with ternary weights for this output neuron
            for in_idx in 0..input_size {
                let weight_idx = out_idx * input_size + in_idx;
                
                // Get weight bits directly from packed storage
                if weight_idx < weights.size {
                    let (byte_idx, shift) = get_packing_position(weight_idx);
                    
                    if byte_idx < weights.quantized.len() {
                        let bits = (weights.quantized[byte_idx] >> shift) & 0b11;
                        
                        // Convert to weight bits for computation (0b00, 0b01, 0b10)
                        let weight_bits = match bits {
                            0b01 => 0b01, // 1
                            0b10 => 0b10, // -1
                            _ => 0b00,    // 0 (including 0b00 and invalid 0b11)
                        };
                        
                        sum += computation.mul(weight_bits, inputs[in_idx]) as i32;
                    }
                }
            }
            
            outputs[out_idx] = sum;
        }
        
        results.push(outputs);
    }
    
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ternary::ternary_weights::TernaryWeights;
    use crate::ternary::ternary_computation::TernaryComputation;
    
    fn create_test_weights() -> TernaryWeights {
        let mut weights = TernaryWeights::create(6, None); // 3×2 = 6 weights
        
        // Initialize raw weights
        weights.raw = vec![
            1.5,  // out0,in0 -> should quantize to 1
            -0.8, // out0,in1 -> should quantize to -1
            0.2,  // out0,in2 -> should quantize to 0
            -1.2, // out1,in0 -> should quantize to -1
            0.6,  // out1,in1 -> should quantize to 1
            0.0,  // out1,in2 -> should quantize to 0
        ];
        
        weights.quantize();
        weights
    }
    
    #[test]
    fn test_forward_basic() {
        let weights = create_test_weights();
        let computation = TernaryComputation::new();
        
        let batch = vec![
            vec![100, -50, 25],
            vec![50, 25, -10],
        ];
        
        let results = ternary_layer_forward(
            &weights,
            3, // input_size
            2, // output_size
            &batch,
            &computation
        );
        
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].len(), 2);
        assert_eq!(results[1].len(), 2);
        
        // First batch: [100, -50, 25]
        // out[0] = 100*1 + (-50)*(-1) + 25*0 = 100 + 50 + 0 = 150
        // out[1] = 100*(-1) + (-50)*1 + 25*0 = -100 + -50 + 0 = -150
        assert_eq!(results[0][0], 150);
        assert_eq!(results[0][1], -150);
        
        // Second batch: [50, 25, -10]
        // out[0] = 50*1 + 25*(-1) + (-10)*0 = 50 - 25 + 0 = 25
        // out[1] = 50*(-1) + 25*1 + (-10)*0 = -50 + 25 + 0 = -25
        assert_eq!(results[1][0], 25);
        assert_eq!(results[1][1], -25);
    }
    
    #[test]
    fn test_forward_with_zeros() {
        let mut weights = TernaryWeights::create(4, None); // 2×2 = 4 weights
        weights.raw = vec![0.0, 0.0, 0.0, 0.0];
        weights.quantize();
        
        let computation = TernaryComputation::new();
        
        let batch = vec![
            vec![100, 50],
            vec![-100, -50],
        ];
        
        let results = ternary_layer_forward(
            &weights,
            2, // input_size
            2, // output_size
            &batch,
            &computation
        );
        
        assert_eq!(results[0][0], 0);
        assert_eq!(results[0][1], 0);
        assert_eq!(results[1][0], 0);
        assert_eq!(results[1][1], 0);
    }
    
    #[test]
    fn test_forward_empty_batch() {
        let weights = create_test_weights();
        let computation = TernaryComputation::new();
        
        let batch: Vec<Vec<i8>> = vec![];
        
        let results = ternary_layer_forward(
            &weights,
            3,
            2,
            &batch,
            &computation
        );
        
        assert_eq!(results.len(), 0);
    }
    
    #[test]
    #[should_panic(expected = "Input vector length 2 does not match layer input_size 3")]
    fn test_forward_wrong_input_size() {
        let weights = create_test_weights();
        let computation = TernaryComputation::new();
        
        let batch = vec![vec![100, 50]]; // Length 2, should be 3
        
        ternary_layer_forward(
            &weights,
            3,
            2,
            &batch,
            &computation
        );
    }
    
    #[test]
    fn test_packing_position() {
        assert_eq!(get_packing_position(0), (0, 0));
        assert_eq!(get_packing_position(1), (0, 2));
        assert_eq!(get_packing_position(2), (0, 4));
        assert_eq!(get_packing_position(3), (0, 6));
        assert_eq!(get_packing_position(4), (1, 0));
        assert_eq!(get_packing_position(5), (1, 2));
        assert_eq!(get_packing_position(6), (1, 4));
        assert_eq!(get_packing_position(7), (1, 6));
    }
}
