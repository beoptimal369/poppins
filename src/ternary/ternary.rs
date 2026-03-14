// src/ternary/ternary.rs

/// Ternary value: 1, 0, or -1
///
/// Each value uses 2 bits when packed:
/// - One: 0b01
/// - Zero: 0b00
/// - MinusOne: 0b10
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ternary {
    One = 1,
    Zero = 0,
    MinusOne = -1,
}


impl Ternary {
    /// Convert to 2-bit representation for packing
    ///
    /// # Returns
    /// * `u8` - 2-bit encoding (lowest 2 bits are significant)
    #[inline]
    pub fn to_bits(self) -> u8 {
        match self {
            Ternary::One => 0b01,      // 1
            Ternary::MinusOne => 0b10,  // -1
            Ternary::Zero => 0b00,      // 0
        }
    }
    
    /// Multiply with an activation value
    ///
    /// # Arguments
    /// * `activation` - The activation value (typically i8 in inference)
    ///
    /// # Returns
    /// * `i16` - Result of ternary × activation
    #[inline]
    pub fn mul(self, activation: i8) -> i16 {
        match self {
            Ternary::One => activation as i16,
            Ternary::MinusOne => -(activation as i16),
            Ternary::Zero => 0,
        }
    }
}


impl std::fmt::Display for Ternary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ternary::One => write!(f, "1"),
            Ternary::Zero => write!(f, "0"),
            Ternary::MinusOne => write!(f, "-1"),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_ternary_to_bits() {
        assert_eq!(Ternary::One.to_bits(), 0b01);
        assert_eq!(Ternary::Zero.to_bits(), 0b00);
        assert_eq!(Ternary::MinusOne.to_bits(), 0b10);
    }
    
    #[test]
    fn test_ternary_mul() {
        assert_eq!(Ternary::One.mul(42), 42);
        assert_eq!(Ternary::One.mul(-127), -127);
        assert_eq!(Ternary::Zero.mul(42), 0);
        assert_eq!(Ternary::Zero.mul(-127), 0);
        assert_eq!(Ternary::MinusOne.mul(42), -42);
        assert_eq!(Ternary::MinusOne.mul(-127), 127);
    }
    
    #[test]
    fn test_ternary_display() {
        assert_eq!(format!("{}", Ternary::One), "1");
        assert_eq!(format!("{}", Ternary::Zero), "0");
        assert_eq!(format!("{}", Ternary::MinusOne), "-1");
    }
}
