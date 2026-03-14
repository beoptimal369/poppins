// src/ternary/ternary_computation.rs

/// O(1) computation cache for ternary operations
///
/// Pre-computes all possible multiplication results between ternary values
/// (represented as 2-bit patterns) and 8-bit activations. This eliminates
/// multiplication operations during inference.
///
/// The lookup table is indexed by:
/// - First dimension: weight bits (0=0, 1=1, 2=-1)
/// - Second dimension: activation as u8 (0-255)
pub struct TernaryComputation {
    /// Lookup table for multiplication results
    /// table[weight_idx][activation_idx] = result
    table: [[i16; 256]; 3],
}

impl TernaryComputation {
    /// Create a new computation cache with pre-computed values
    ///
    /// Pre-computes all 3 * 256 = 768 possible multiplication results.
    pub fn new() -> Self {
        let mut table = [[0i16; 256]; 3];
        
        // Pre-compute all possible multiplication results
        for activation_u8 in 0..=255 {
            let activation_i8 = activation_u8 as i8;
            
            // weight = -1 (bits: 0b10 = 2)
            table[2][activation_u8 as usize] = -(activation_i8 as i16);
            
            // weight = 0 (bits: 0b00 = 0)
            table[0][activation_u8 as usize] = 0;
            
            // weight = 1 (bits: 0b01 = 1)
            table[1][activation_u8 as usize] = activation_i8 as i16;
        }
        
        Self { table }
    }
    
    /// Multiply a ternary weight (as 2-bit pattern) with an activation
    ///
    /// # Arguments
    /// * `weight_bits` - 2-bit representation (0b00, 0b01, 0b10)
    /// * `activation` - 8-bit activation value (-128 to 127)
    ///
    /// # Returns
    /// * `i16` - Result of the multiplication
    #[inline]
    pub fn mul(&self, weight_bits: u8, activation: i8) -> i16 {
        // Map the 2-bit pattern to table index
        let weight_idx = match weight_bits & 0b11 {
            0b01 => 1,  // 1
            0b10 => 2,  // -1
            _ => 0,     // 0 (including 0b00 and invalid 0b11)
        };
        
        let activation_idx = activation as u8 as usize;
        self.table[weight_idx][activation_idx]
    }
    
    /// Compute dot product of weights and activations
    ///
    /// # Arguments
    /// * `weight_bits` - Slice of packed weight bits
    /// * `activations` - Slice of activation values
    ///
    /// # Returns
    /// * `i32` - Sum of weight[i] × activation[i] for all i
    ///
    /// # Panics
    /// * If slices have different lengths
    pub fn dot_product(&self, weight_bits: &[u8], activations: &[i8]) -> i32 {
        assert_eq!(
            weight_bits.len(), 
            activations.len(), 
            "Weight bits and activations must have same length"
        );
        
        let mut sum = 0i32;
        for i in 0..weight_bits.len() {
            sum += self.mul(weight_bits[i], activations[i]) as i32;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mul() {
        let comp = TernaryComputation::new();
        
        // Test all weight possibilities
        assert_eq!(comp.mul(0b01, 42), 42);   // 1 × 42
        assert_eq!(comp.mul(0b10, 42), -42);  // -1 × 42
        assert_eq!(comp.mul(0b00, 42), 0);    // 0 × 42
        
        // Test with negative activations
        assert_eq!(comp.mul(0b01, -127), -127);
        assert_eq!(comp.mul(0b10, -127), 127);
        assert_eq!(comp.mul(0b00, -127), 0);
        
        // Test with invalid weight bits (treated as 0)
        assert_eq!(comp.mul(0b11, 42), 0);
        assert_eq!(comp.mul(0b11, -127), 0);
    }
    
    #[test]
    fn test_dot_product() {
        let comp = TernaryComputation::new();
        
        let weights = [0b01, 0b10, 0b00, 0b01];
        let activations = [42, 42, 42, -127];
        
        // 42 + (-42) + 0 + (-127) = -127
        assert_eq!(comp.dot_product(&weights, &activations), -127);
        
        // Empty slices
        assert_eq!(comp.dot_product(&[], &[]), 0);
    }
    
    #[test]
    #[should_panic(expected = "Weight bits and activations must have same length")]
    fn test_dot_product_length_mismatch() {
        let comp = TernaryComputation::new();
        comp.dot_product(&[0b01, 0b10], &[42]);
    }
    
    #[test]
    fn test_all_activations_covered() {
        let comp = TernaryComputation::new();
        
        // Verify that all 256 activation values produce correct results
        for activation_u8 in 0..=255 {
            let activation_i8 = activation_u8 as i8;
            
            // weight = 1
            assert_eq!(
                comp.mul(0b01, activation_i8), 
                activation_i8 as i16
            );
            
            // weight = -1
            assert_eq!(
                comp.mul(0b10, activation_i8), 
                -(activation_i8 as i16)
            );
            
            // weight = 0
            assert_eq!(comp.mul(0b00, activation_i8), 0);
        }
    }
}
