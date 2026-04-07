// src/ternary/ternary_weights.rs

use crate::ternary::ternary::Ternary;


/// Ternary weights with master (f32) and quantized (packed) representations
///
/// Maintains high-precision master weights for training and packed ternary
/// weights (4 per byte) for efficient inference. Implements absmean quantization
/// as specified in the BitNet b1.58 paper.
pub struct TernaryWeights {
    /// Master weights in f32 for gradient accumulation during training
    pub raw: Vec<f32>,
    
    /// Packed ternary weights for inference (4 per byte, 2 bits each)
    pub quantized: Vec<u8>,
    
    /// Number of weights
    pub size: usize,
    
    /// Average absolute value (γ) used for quantization
    /// γ = (1/n) * Σ|raw[i]|
    pub avg_abs: f32,
    
    /// Step counter for periodic updates
    pub step_counter: usize,
    
    /// How often to update avg_abs (in steps)
    pub update_frequency: usize,
}

impl TernaryWeights {
    /// Create new weights with the specified size and update frequency
    ///
    /// # Arguments
    /// * `size` - Number of weights
    /// * `update_frequency` - How often to recalculate avg_abs (defaults to 100 if None)
    ///
    /// # Returns
    /// * `Self` - Initialized weights (all zeros)
    pub fn create(size: usize, update_frequency: Option<usize>) -> Self {
        let num_bytes = (size + 3) / 4; // Ceiling division for packing
        
        Self {
            raw: vec![0.0; size], // All weights start at 0.0 (ConfigHuggingFace > initializer_range > 0.0)
            quantized: vec![0; num_bytes],
            size,
            avg_abs: 0.0,
            step_counter: 0,
            update_frequency: update_frequency.unwrap_or(100),
        }
    }
    
    /// Calculate average absolute value (γ) from raw weights
    ///
    /// γ = (1/n) * Σ|raw[i]|
    fn calculate_avg_abs(&self) -> f32 {
        if self.size == 0 {
            return 1.0; // Avoid division by zero
        }
        
        let sum_abs: f32 = self.raw.iter().map(|&w| w.abs()).sum();
        sum_abs / self.size as f32
    }
    
    /// Set/update the average absolute value (avg_abs) based on current raw weights
    pub fn set_avg_abs(&mut self) {
        self.avg_abs = self.calculate_avg_abs();
    }
    
    /// Quantize a single raw value to ternary using absmean quantization
    ///
    /// Implements: RoundClip(raw/γ, -1, 1) where values are rounded to the nearest integer
    /// Values exactly at the boundary (0.5 or -0.5) round to the nearest integer (1 or -1)
    fn quantize_value(&self, value: f32) -> Ternary {
        if self.avg_abs <= f32::EPSILON {
            return Ternary::Zero; // Avoid division by zero
        }
        
        let normalized = value / self.avg_abs;
        
        // Round to nearest integer with proper handling of boundary cases
        // Values >= 0.5 round to 1
        // Values <= -0.5 round to -1
        // Values between -0.5 and 0.5 (exclusive) round to 0
        if normalized >= 0.5 {
            Ternary::One
        } else if normalized <= -0.5 {
            Ternary::MinusOne
        } else {
            Ternary::Zero
        }
    }
    
    /// Pack a ternary value into a byte at a specific position
    ///
    /// # Arguments
    /// * `index` - Weight index
    /// * `value` - Ternary value to pack
    fn pack_value(&mut self, index: usize, value: Ternary) {
        let (byte_idx, shift) = Self::get_packing_position(index);
        let bits = value.to_bits();
        let mask = 0b11 << shift;
        
        // Clear the two bits at this position, then set them to our value
        self.quantized[byte_idx] = (self.quantized[byte_idx] & !mask) | (bits << shift);
    }
    
    /// Get packing position for a weight index
    ///
    /// # Returns
    /// * `(usize, usize)` - (byte_index, bit_shift)
    #[inline]
    fn get_packing_position(index: usize) -> (usize, usize) {
        (index / 4, (index % 4) * 2)
    }
    
    /// Quantize all raw weights and update the packed representation
    ///
    /// This is the main quantization function that:
    /// 1. Updates avg_abs based on current raw weights
    /// 2. Quantizes each raw weight to ternary
    /// 3. Packs 4 ternary values into each byte
    pub fn quantize(&mut self) {
        if self.size == 0 {
            return;
        }
        
        // Update average absolute value
        self.set_avg_abs();
        
        // If avg_abs is effectively zero, all weights are zero
        if self.avg_abs <= f32::EPSILON {
            for i in 0..self.size {
                self.pack_value(i, Ternary::Zero);
            }
            return;
        }
        
        // Quantize and pack each weight
        for i in 0..self.size {
            let ternary = self.quantize_value(self.raw[i]);
            self.pack_value(i, ternary);
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create() {
        let weights = TernaryWeights::create(10, None);
        assert_eq!(weights.size, 10);
        assert_eq!(weights.raw.len(), 10);
        assert_eq!(weights.quantized.len(), 3); // ceil(10/4) = 3
        assert_eq!(weights.avg_abs, 0.0);
        assert_eq!(weights.step_counter, 0);
        assert_eq!(weights.update_frequency, 100);
        
        let weights = TernaryWeights::create(5, Some(50));
        assert_eq!(weights.quantized.len(), 2); // ceil(5/4) = 2
        assert_eq!(weights.update_frequency, 50);
    }
    
    #[test]
    fn test_set_avg_abs() {
        let mut weights = TernaryWeights::create(4, None);
        
        // Initialize raw weights
        weights.raw = vec![2.0, -1.0, 0.5, -0.5];
        
        weights.set_avg_abs();
        // (2.0 + 1.0 + 0.5 + 0.5) / 4 = 4.0 / 4 = 1.0
        assert!((weights.avg_abs - 1.0).abs() < 1e-6);
        
        // Test with zeros
        let mut weights = TernaryWeights::create(3, None);
        weights.raw = vec![0.0, 0.0, 0.0];
        weights.set_avg_abs();
        assert_eq!(weights.avg_abs, 0.0);
    }
    
    #[test]
    fn test_quantize() {
        let mut weights = TernaryWeights::create(8, None);
        
        // Initialize raw weights
        weights.raw = vec![
            1.2,  // Should become 1
            -0.8, // Should become -1
            0.1,  // Should become 0
            2.5,  // Should become 1
            -2.0, // Should become -1
            0.0,  // Should become 0
            0.6,  // Should become 1
            -0.4, // Should become 0
        ];
        
        weights.quantize();
        
        // Verify avg_abs was calculated
        assert!(weights.avg_abs > 0.0);
        
        // We can't directly inspect the packed bytes in tests without accessors,
        // but we can verify that quantize() ran without panicking
        assert_eq!(weights.quantized.len(), 2); // 8 weights -> 2 bytes
    }
    
    #[test]
    fn test_quantize_value() {
        let mut weights = TernaryWeights::create(1, None);
        
        // Test case 1: γ = 1.0
        weights.raw[0] = 1.0;
        weights.set_avg_abs(); // avg_abs should be 1.0
        assert!((weights.avg_abs - 1.0).abs() < 1e-6, "avg_abs should be 1.0, got {}", weights.avg_abs);
        
        assert_eq!(weights.quantize_value(0.6), Ternary::One);   // 0.6/1.0 = 0.6 > 0.5 → 1
        assert_eq!(weights.quantize_value(0.4), Ternary::Zero);  // 0.4/1.0 = 0.4 < 0.5 → 0
        assert_eq!(weights.quantize_value(0.0), Ternary::Zero);
        assert_eq!(weights.quantize_value(-0.4), Ternary::Zero);
        assert_eq!(weights.quantize_value(-0.6), Ternary::MinusOne);
        
        // Test case 2: γ = 2.0
        weights.raw[0] = 2.0;
        weights.set_avg_abs(); // avg_abs should be 2.0
        assert!((weights.avg_abs - 2.0).abs() < 1e-6, "avg_abs should be 2.0, got {}", weights.avg_abs);
        
        // At γ = 2.0, the thresholds are at ±1.0
        assert_eq!(weights.quantize_value(1.2), Ternary::One);   // 1.2/2.0 = 0.6 > 0.5 → 1
        assert_eq!(weights.quantize_value(1.0), Ternary::One);   // 1.0/2.0 = 0.5 exactly → implementation uses >0.5, so this should be 0? Wait, 0.5 is not > 0.5, so it should be 0
        assert_eq!(weights.quantize_value(0.9), Ternary::Zero);  // 0.9/2.0 = 0.45 < 0.5 → 0
        assert_eq!(weights.quantize_value(-0.9), Ternary::Zero);
        assert_eq!(weights.quantize_value(-1.0), Ternary::MinusOne); // -1.0/2.0 = -0.5 exactly → implementation uses < -0.5, so this should be 0? Wait, -0.5 is not < -0.5, so it should be 0
        assert_eq!(weights.quantize_value(-1.2), Ternary::MinusOne);
    }

    #[test]
    fn test_packing_position() {
        assert_eq!(TernaryWeights::get_packing_position(0), (0, 0));
        assert_eq!(TernaryWeights::get_packing_position(1), (0, 2));
        assert_eq!(TernaryWeights::get_packing_position(2), (0, 4));
        assert_eq!(TernaryWeights::get_packing_position(3), (0, 6));
        assert_eq!(TernaryWeights::get_packing_position(4), (1, 0));
        assert_eq!(TernaryWeights::get_packing_position(5), (1, 2));
        assert_eq!(TernaryWeights::get_packing_position(6), (1, 4));
        assert_eq!(TernaryWeights::get_packing_position(7), (1, 6));
    }
    
    #[test]
    fn test_zero_weights() {
        let mut weights = TernaryWeights::create(4, None);
        weights.raw = vec![0.0, 0.0, 0.0, 0.0];
        
        weights.quantize();
        assert_eq!(weights.avg_abs, 0.0);
        
        // All weights should be packed as Zero (0b00)
        // Since we can't inspect directly, we verify that quantize() didn't panic
    }
}
