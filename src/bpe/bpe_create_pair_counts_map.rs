// src/bpe/bpe_create_pair_counts_map.rs

use crate::bpe::BPETokenizer;
use std::collections::HashMap;


/// Count frequencies of adjacent token pairs in the sequence
///
/// This function counts how often each pair of consecutive tokens appears.
/// Pairs that involve tokens from the special vocabulary 
/// are skipped to prevent merging them with other tokens. This ensures:
/// - Special tokens (e.g., <sample>, <text>) remain atomic
/// - Requested tokens (e.g., "console.log") are allowed to be merged
/// - Only learned merges between regular characters/merged tokens are counted
///
/// # Arguments
/// * `tokenizer` - Reference to the tokenizer with special_token_count and initial_token_count
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
        
        // Skip merging if either token is a special token, allow merging of requested tokens
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

    fn create_test_tokenizer(special_count: u32, initial_count: u32) -> BPETokenizer {
        BPETokenizer {
            vocab: vec![],
            token_to_id: HashMap::new(),
            merges: vec![],
            special_token_count: special_count,
            initial_token_count: initial_count,
        }
    }

    #[test]
    fn test_empty_sequence() {
        let tokenizer = create_test_tokenizer(3, 5);
        let sequence: Vec<u32> = vec![];
        
        let counts = bpe_create_pair_counts_map(&tokenizer, &sequence);
        
        assert!(counts.is_empty());
    }

    #[test]
    fn test_single_token_sequence() {
        let tokenizer = create_test_tokenizer(3, 5);
        let sequence = vec![5];
        
        let counts = bpe_create_pair_counts_map(&tokenizer, &sequence);
        
        assert!(counts.is_empty());
    }

    #[test]
    fn test_two_token_sequence_both_regular() {
        let tokenizer = create_test_tokenizer(3, 5);
        let sequence = vec![10, 11]; // Both IDs >= initial_token_count (5)
        
        let counts = bpe_create_pair_counts_map(&tokenizer, &sequence);
        
        assert_eq!(counts.len(), 1);
        assert_eq!(counts.get(&(10, 11)), Some(&1));
    }

    #[test]
    fn test_skip_special_tokens() {
        let tokenizer = create_test_tokenizer(5, 8);
        let sequence = vec![
            1,  // special (ID < 5)
            2,  // special
            10, // regular (ID >= 8)
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
    fn test_allow_requested_tokens_to_merge() {
        let tokenizer = create_test_tokenizer(3, 8);
        // special tokens: IDs 0-2
        // requested tokens: IDs 3-7
        // regular tokens: IDs >= 8
        let sequence = vec![
            3,  // requested token
            4,  // requested token
            10, // regular
            11, // regular
            5,  // requested token
            12, // regular
            13, // regular
            6,  // requested token
        ];
        
        let counts = bpe_create_pair_counts_map(&tokenizer, &sequence);
        
        // Requested tokens should be allowed to merge!
        // Pairs to count:
        // (3,4): requested+requested -> SHOULD be counted
        // (4,10): requested+regular -> SHOULD be counted
        // (10,11): regular+regular -> counted
        // (11,5): regular+requested -> SHOULD be counted
        // (5,12): requested+regular -> SHOULD be counted
        // (12,13): regular+regular -> counted
        // (13,6): regular+requested -> SHOULD be counted
        assert_eq!(counts.len(), 7);
        assert_eq!(counts.get(&(3, 4)), Some(&1));
        assert_eq!(counts.get(&(4, 10)), Some(&1));
        assert_eq!(counts.get(&(10, 11)), Some(&1));
        assert_eq!(counts.get(&(11, 5)), Some(&1));
        assert_eq!(counts.get(&(5, 12)), Some(&1));
        assert_eq!(counts.get(&(12, 13)), Some(&1));
        assert_eq!(counts.get(&(13, 6)), Some(&1));
    }

    #[test]
    fn test_skip_special_allow_requested() {
        let tokenizer = create_test_tokenizer(3, 8);
        let sequence = vec![
            1,  // special
            3,  // requested
            10, // regular
            4,  // requested
            11, // regular
            2,  // special
            12, // regular
            5,  // requested
        ];
        
        let counts = bpe_create_pair_counts_map(&tokenizer, &sequence);
        
        // Pairs:
        // (1,3): special+requested -> SKIP (special)
        // (3,10): requested+regular -> COUNT
        // (10,4): regular+requested -> COUNT
        // (4,11): requested+regular -> COUNT
        // (11,2): regular+special -> SKIP (special)
        // (2,12): special+regular -> SKIP (special)
        // (12,5): regular+requested -> COUNT
        assert_eq!(counts.len(), 4);
        assert_eq!(counts.get(&(3, 10)), Some(&1));
        assert_eq!(counts.get(&(10, 4)), Some(&1));
        assert_eq!(counts.get(&(4, 11)), Some(&1));
        assert_eq!(counts.get(&(12, 5)), Some(&1));
    }

    #[test]
    fn test_multiple_pairs_regular_only() {
        let tokenizer = create_test_tokenizer(3, 8);
        let sequence = vec![10, 11, 10, 11, 12, 13];
        
        let counts = bpe_create_pair_counts_map(&tokenizer, &sequence);
        
        // Pairs: (10,11), (11,10), (10,11), (11,12), (12,13)
        assert_eq!(counts.len(), 4);
        assert_eq!(counts.get(&(10, 11)), Some(&2));
        assert_eq!(counts.get(&(11, 10)), Some(&1));
        assert_eq!(counts.get(&(11, 12)), Some(&1));
        assert_eq!(counts.get(&(12, 13)), Some(&1));
    }

    #[test]
    fn test_regular_tokens_only() {
        let tokenizer = create_test_tokenizer(3, 8);
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
    fn test_mixed_sequence_with_all_types() {
        let tokenizer = create_test_tokenizer(2, 5);
        // special: IDs 0-1
        // requested: IDs 2-4
        // regular: IDs >= 5
        let sequence = vec![
            0, // special
            2, // requested
            5, // regular
            6, // regular
            3, // requested
            7, // regular
            1, // special
            4, // requested
            8, // regular
        ];
        
        let counts = bpe_create_pair_counts_map(&tokenizer, &sequence);
        
        // Pairs:
        // (0,2): special+requested -> SKIP (special)
        // (2,5): requested+regular -> COUNT
        // (5,6): regular+regular -> COUNT
        // (6,3): regular+requested -> COUNT
        // (3,7): requested+regular -> COUNT
        // (7,1): regular+special -> SKIP (special)
        // (1,4): special+requested -> SKIP (special)
        // (4,8): requested+regular -> COUNT
        assert_eq!(counts.len(), 5);
        assert_eq!(counts.get(&(2, 5)), Some(&1));
        assert_eq!(counts.get(&(5, 6)), Some(&1));
        assert_eq!(counts.get(&(6, 3)), Some(&1));
        assert_eq!(counts.get(&(3, 7)), Some(&1));
        assert_eq!(counts.get(&(4, 8)), Some(&1));
    }

    #[test]
    fn test_special_token_count_zero() {
        let tokenizer = create_test_tokenizer(0, 5);
        // special: none
        // requested: IDs 0-4
        // regular: IDs >= 5
        let sequence = vec![
            0, // requested
            1, // requested
            5, // regular
            6, // regular
            2, // requested
            7, // regular
            3, // requested
            4, // requested
            8, // regular
        ];
        
        let counts = bpe_create_pair_counts_map(&tokenizer, &sequence);
        
        // With no special tokens, all pairs should be counted
        // (0,1): requested+requested -> COUNT
        // (1,5): requested+regular -> COUNT
        // (5,6): regular+regular -> COUNT
        // (6,2): regular+requested -> COUNT
        // (2,7): requested+regular -> COUNT
        // (7,3): regular+requested -> COUNT
        // (3,4): requested+requested -> COUNT
        // (4,8): requested+regular -> COUNT
        assert_eq!(counts.len(), 8);
        assert_eq!(counts.get(&(0, 1)), Some(&1));
        assert_eq!(counts.get(&(1, 5)), Some(&1));
        assert_eq!(counts.get(&(5, 6)), Some(&1));
        assert_eq!(counts.get(&(6, 2)), Some(&1));
        assert_eq!(counts.get(&(2, 7)), Some(&1));
        assert_eq!(counts.get(&(7, 3)), Some(&1));
        assert_eq!(counts.get(&(3, 4)), Some(&1));
        assert_eq!(counts.get(&(4, 8)), Some(&1));
    }

    #[test]
    fn test_initial_token_count_equals_special_count() {
        // No requested tokens
        let tokenizer = create_test_tokenizer(5, 5);
        let sequence = vec![
            1, // special
            2, // special
            10, // regular
            11, // regular
            3, // special
            12, // regular
        ];
        
        let counts = bpe_create_pair_counts_map(&tokenizer, &sequence);
        
        // Pairs:
        // (1,2): special+special -> SKIP
        // (2,10): special+regular -> SKIP
        // (10,11): regular+regular -> COUNT
        // (11,3): regular+special -> SKIP
        // (3,12): special+regular -> SKIP
        assert_eq!(counts.len(), 1);
        assert_eq!(counts.get(&(10, 11)), Some(&1));
    }

    #[test]
    fn test_large_sequence_with_initial_tokens() {
        let tokenizer = create_test_tokenizer(10, 20);
        let sequence: Vec<u32> = (0..1000).collect();
        
        let counts = bpe_create_pair_counts_map(&tokenizer, &sequence);
        
        // Total windows: 999
        // Windows where a < 10 or b < 10 are skipped (special tokens)
        // Requested tokens (10-19) are NOT skipped!
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
