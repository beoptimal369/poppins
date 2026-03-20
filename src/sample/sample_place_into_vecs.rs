// src/sample/sample_place_into_vecs.rs

use rand::RngExt;
use crate::sample::sample_structs::{Samples, Sample};


/// Places an original sample and its variants into train/validation vectors
///
/// This function implements the "Poppins hack" for validation:
/// - Takes one original sample and its variants (if any)
/// - Randomly selects ONE sample from the group (original + variants) to place in validation
/// - Places all remaining samples in training
///
/// This ensures the model trains on everything, but never sees the exact validation sample,
/// while still having seen similar patterns during training.
///
/// # Arguments
/// * `samples` - Mutable reference to Samples container
/// * `original` - The original sample
/// * `variants` - Optional vector of variant samples (from sample_get_variants)
///
/// # Notes
/// * The original sample and all variants must already have unique IDs assigned
/// * The samples are consumed and moved into train/val vectors
pub fn sample_place_into_vecs(
    samples: &mut Samples,
    original: Sample,
    variants: Option<Vec<Sample>>,
) {
    let mut rng = rand::rng();
    
    // Collect all samples (original + variants)
    let mut all_samples = Vec::new();
    all_samples.push(original);
    
    if let Some(mut vars) = variants {
        all_samples.append(&mut vars);
    }
    
    // If there's only one sample, it goes to training
    if all_samples.len() == 1 {
        samples.train_samples.push(all_samples.remove(0));
        return;
    }
    
    // Randomly select one sample for validation
    let val_index = rng.random_range(0..all_samples.len()); // Changed from gen_range()
    
    // Distribute samples
    for (i, sample) in all_samples.into_iter().enumerate() {
        if i == val_index {
            samples.val_samples.push(sample);
        } else {
            samples.train_samples.push(sample);
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::sample::{
        sample_structs::{Samples, Sample, SamplePromptEnum, SampleAiEnum, SampleText, SampleTokenStats},
        sample_place_into_vecs::sample_place_into_vecs,
    };
    
    fn create_test_sample(id: &str) -> Sample {
        Sample {
            id: id.to_string(),
            prompt_section: vec![
                SamplePromptEnum::Text(format!("Prompt {}", id)),
            ],
            ai_section: vec![
                SampleAiEnum::Text(SampleText {
                    content: format!("Response {}", id),
                    token_stats: SampleTokenStats {
                        weight_decay: 0.1,
                        dropout: 0.05,
                        loss_scale: 1.0,
                        gradient_scale: 1.0,
                        gradient_clip: 1.0,
                    },
                }),
            ],
        }
    }
    
    #[test]
    fn test_place_single_sample() {
        let mut samples = Samples {
            train_samples: Vec::new(),
            val_samples: Vec::new(),
            total_sample_count: 5, // Just for tracking
        };
        
        let original = create_test_sample("1");
        
        sample_place_into_vecs(&mut samples, original, None);
        
        assert_eq!(samples.train_samples.len(), 1);
        assert_eq!(samples.val_samples.len(), 0);
        assert_eq!(samples.train_samples[0].id, "1");
    }
    
    #[test]
    fn test_place_with_variants() {
        let mut samples = Samples {
            train_samples: Vec::new(),
            val_samples: Vec::new(),
            total_sample_count: 5,
        };
        
        let original = create_test_sample("1");
        let variants = vec![
            create_test_sample("2"),
            create_test_sample("3"),
        ];
        
        sample_place_into_vecs(&mut samples, original, Some(variants));
        
        // Total samples should be 3 (original + 2 variants)
        assert_eq!(samples.train_samples.len() + samples.val_samples.len(), 3);
        
        // Exactly one sample should be in validation
        assert_eq!(samples.val_samples.len(), 1);
        
        // The validation sample should be one of the three
        let val_id = &samples.val_samples[0].id;
        assert!(val_id == "1" || val_id == "2" || val_id == "3");
        
        // All samples should be present (no duplicates, no missing)
        let mut all_ids: Vec<String> = samples.train_samples.iter().map(|s| s.id.clone()).collect();
        all_ids.extend(samples.val_samples.iter().map(|s| s.id.clone()));
        all_ids.sort();
        
        assert_eq!(all_ids, vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    }
    
    #[test]
    fn test_place_multiple_times() {
        let mut samples = Samples {
            train_samples: Vec::new(),
            val_samples: Vec::new(),
            total_sample_count: 0,
        };
        
        // First group
        let original1 = create_test_sample("1");
        let variants1 = vec![create_test_sample("2")];
        sample_place_into_vecs(&mut samples, original1, Some(variants1));
        
        // Second group
        let original2 = create_test_sample("3");
        let variants2 = vec![create_test_sample("4"), create_test_sample("5")];
        sample_place_into_vecs(&mut samples, original2, Some(variants2));
        
        // Total samples should be 5
        assert_eq!(samples.train_samples.len() + samples.val_samples.len(), 5);
        
        // Should have 2 validation samples (one from each group)
        assert_eq!(samples.val_samples.len(), 2);
        
        // All samples should be present
        let mut all_ids: Vec<String> = samples.train_samples.iter().map(|s| s.id.clone()).collect();
        all_ids.extend(samples.val_samples.iter().map(|s| s.id.clone()));
        all_ids.sort();
        
        assert_eq!(all_ids, vec!["1", "2", "3", "4", "5"]);
    }
    
    #[test]
    fn test_random_distribution() {
        let mut val_counts = std::collections::HashMap::new();
        
        for _ in 0..100 {
            let mut test_samples = Samples {
                train_samples: Vec::new(),
                val_samples: Vec::new(),
                total_sample_count: 0,
            };
            
            sample_place_into_vecs(&mut test_samples, create_test_sample("1"), Some(vec![
                create_test_sample("2"),
                create_test_sample("3"),
                create_test_sample("4"),
            ]));
            
            let val_id = test_samples.val_samples[0].id.clone();
            *val_counts.entry(val_id).or_insert(0) += 1;
        }
        
        // Each sample should appear in validation roughly 25% of the time (100/4 = 25)
        // Allow some variance but ensure all are selected
        assert!(val_counts.get("1").unwrap_or(&0) > &15);
        assert!(val_counts.get("2").unwrap_or(&0) > &15);
        assert!(val_counts.get("3").unwrap_or(&0) > &15);
        assert!(val_counts.get("4").unwrap_or(&0) > &15);
    }
}
