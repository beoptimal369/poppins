// src/bpe/bpe_create_pair_counts_map.rs

use crate::bpe::BPETokenizer;
use std::collections::HashMap;


/// Count frequencies of adjacent token pairs in the sequence
///
/// This function counts how often each pair of consecutive tokens appears.
/// Pairs that involve special tokens (IDs < special_token_count) are skipped
/// to prevent merging special tokens with other tokens.
///
/// # Arguments
/// * `tokenizer` - Reference to the tokenizer with special_token_count
/// * `token_sequence` - Slice of token IDs to analyze
///
/// # Returns
/// * `HashMap<(u32, u32), usize>` - Map of (left_token, right_token) to frequency count
pub fn bpe_create_pair_counts_map(
    tokenizer: &BPETokenizer,
    token_sequence: &[u32],
) -> HashMap<(u32, u32), usize> {
    let mut counts = HashMap::new();
    
    for window in token_sequence.windows(2) {
        let a = window[0];
        let b = window[1];
        
        // Skip merging special tokens
        if a < tokenizer.special_token_count || b < tokenizer.special_token_count {
            continue;
        }
        
        *counts.entry((a, b)).or_insert(0) += 1;
    }
    
    counts
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::bpe::{BPETokenizer, bpe_create_pair_counts_map};

    fn create_test_tokenizer(special_count: u32) -> BPETokenizer {
        BPETokenizer {
            vocab: vec![],
            token_to_id: HashMap::new(),
            merges: vec![],
            special_token_count: special_count,
        }
    }

    #[test]
    fn test_empty_sequence() {
        let tokenizer = create_test_tokenizer(3);
        let sequence: Vec<u32> = vec![];
        
        let counts = bpe_create_pair_counts_map(&tokenizer, &sequence);
        
        assert!(counts.is_empty());
    }

    #[test]
    fn test_single_token_sequence() {
        let tokenizer = create_test_tokenizer(3);
        let sequence = vec![5];
        
        let counts = bpe_create_pair_counts_map(&tokenizer, &sequence);
        
        assert!(counts.is_empty());
    }

    #[test]
    fn test_two_token_sequence_no_special() {
        let tokenizer = create_test_tokenizer(3);
        let sequence = vec![5, 6];
        
        let counts = bpe_create_pair_counts_map(&tokenizer, &sequence);
        
        assert_eq!(counts.len(), 1);
        assert_eq!(counts.get(&(5, 6)), Some(&1));
    }

    #[test]
    fn test_multiple_pairs() {
        let tokenizer = create_test_tokenizer(3);
        let sequence = vec![5, 6, 5, 6, 7, 8];
        
        let counts = bpe_create_pair_counts_map(&tokenizer, &sequence);
        
        // Pairs: (5,6), (6,5), (5,6), (6,7), (7,8)
        // Unique pairs: (5,6):2, (6,5):1, (6,7):1, (7,8):1
        assert_eq!(counts.len(), 4);
        assert_eq!(counts.get(&(5, 6)), Some(&2));
        assert_eq!(counts.get(&(6, 5)), Some(&1));
        assert_eq!(counts.get(&(6, 7)), Some(&1));
        assert_eq!(counts.get(&(7, 8)), Some(&1));
    }

    #[test]
    fn test_skip_special_tokens() {
        let tokenizer = create_test_tokenizer(5); // IDs 0-4 are special
        let sequence = vec![
            1,  // special
            2,  // special
            10, // regular
            11, // regular
            3,  // special
            12, // regular
            13, // regular
            4,  // special
        ];
        
        let counts = bpe_create_pair_counts_map(&tokenizer, &sequence);
        
        // Only pairs of regular tokens should be counted
        // (10,11) and (12,13) are regular pairs
        // (2,10), (11,3), (13,4) are skipped because they involve special tokens
        assert_eq!(counts.len(), 2);
        assert_eq!(counts.get(&(10, 11)), Some(&1));
        assert_eq!(counts.get(&(12, 13)), Some(&1));
    }

    #[test]
    fn test_regular_tokens_only() {
        let tokenizer = create_test_tokenizer(3);
        let sequence: Vec<u32> = (10..=20).collect();
        
        let counts = bpe_create_pair_counts_map(&tokenizer, &sequence);
        
        // Should have (len - 1) pairs
        assert_eq!(counts.len(), 10);
        
        // Each consecutive pair should have count 1
        for i in 10..=19 {
            assert_eq!(counts.get(&(i, i + 1)), Some(&1));
        }
    }

    #[test]
    fn test_special_tokens_at_boundaries() {
        let tokenizer = create_test_tokenizer(5);
        let sequence = vec![
            0,  // special at start
            10, // regular
            11, // regular
            4,  // special at end
        ];
        
        let counts = bpe_create_pair_counts_map(&tokenizer, &sequence);
        
        // Only (10,11) should be counted
        assert_eq!(counts.len(), 1);
        assert_eq!(counts.get(&(10, 11)), Some(&1));
    }

    #[test]
    fn test_all_tokens_special() {
        let tokenizer = create_test_tokenizer(5);
        let sequence = vec![0, 1, 2, 3, 4];
        
        let counts = bpe_create_pair_counts_map(&tokenizer, &sequence);
        
        // No pairs should be counted
        assert!(counts.is_empty());
    }

    #[test]
    fn test_special_token_count_zero() {
        let tokenizer = create_test_tokenizer(0);
        let sequence = vec![0, 1, 2, 3, 4];
        
        let counts = bpe_create_pair_counts_map(&tokenizer, &sequence);
        
        // With special_token_count = 0, all tokens are considered regular
        assert_eq!(counts.len(), 4);
        assert_eq!(counts.get(&(0, 1)), Some(&1));
        assert_eq!(counts.get(&(1, 2)), Some(&1));
        assert_eq!(counts.get(&(2, 3)), Some(&1));
        assert_eq!(counts.get(&(3, 4)), Some(&1));
    }

    #[test]
    fn test_large_sequence() {
        let tokenizer = create_test_tokenizer(10);
        let sequence: Vec<u32> = (0..1000).collect();
        
        let counts = bpe_create_pair_counts_map(&tokenizer, &sequence);
        
        // Total windows: 999
        // Windows where a < 10 or b < 10 are skipped
        // First window with both >= 10 is (10, 11) - window index 10
        // Last window is (998, 999) - window index 998
        // Number of windows from index 10 to 998 inclusive = 998 - 10 + 1 = 989
        let expected_pairs = 989;
        assert_eq!(counts.len(), expected_pairs);
        
        for i in 10..999 {
            assert_eq!(counts.get(&(i, i + 1)), Some(&1));
        }
    }
}
