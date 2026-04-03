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
        sample_place_into_vecs::sample_place_into_vecs,
        sample_structs::{Samples, Sample, SamplePromptEnum, SampleAiEnum},
    };
    
    fn create_test_sample(id: &str) -> Sample {
        Sample {
            system: String::new(),
            prompt_section: vec![
                SamplePromptEnum::Text(format!("Prompt {}", id)),
            ],
            ai_section: vec![
                SampleAiEnum::Text(format!("Response {}", id)),
            ],
        }
    }
    
    fn create_test_sample_with_system(id: &str, system: &str) -> Sample {
        Sample {
            system: system.to_string(),
            prompt_section: vec![
                SamplePromptEnum::Text(format!("Prompt {}", id)),
            ],
            ai_section: vec![
                SampleAiEnum::Text(format!("Response {}", id)),
            ],
        }
    }
    
    // Helper function to extract prompt text from a sample
    fn get_prompt_text(sample: &Sample) -> &str {
        match &sample.prompt_section[0] {
            SamplePromptEnum::Text(text) => text.as_str(),
            _ => panic!("Expected Text prompt"),
        }
    }
    
    #[test]
    fn test_place_single_sample() {
        let mut samples = Samples {
            train_samples: Vec::new(),
            val_samples: Vec::new(),
        };
        
        let original = create_test_sample("1");
        
        sample_place_into_vecs(&mut samples, original, None);
        
        assert_eq!(samples.train_samples.len(), 1);
        assert_eq!(samples.val_samples.len(), 0);
    }
    
    #[test]
    fn test_place_with_variants() {
        let mut samples = Samples {
            train_samples: Vec::new(),
            val_samples: Vec::new(),
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
    }
    
    #[test]
    fn test_place_with_system_prompts() {
        let mut samples = Samples {
            train_samples: Vec::new(),
            val_samples: Vec::new(),
        };
        
        let original = create_test_sample_with_system("1", "You are a helpful assistant.");
        let variants = vec![
            create_test_sample_with_system("2", "You are a helpful assistant."),
            create_test_sample_with_system("3", "You are a helpful assistant."),
        ];
        
        sample_place_into_vecs(&mut samples, original, Some(variants));
        
        // Total samples should be 3 (original + 2 variants)
        assert_eq!(samples.train_samples.len() + samples.val_samples.len(), 3);
        
        // Exactly one sample should be in validation
        assert_eq!(samples.val_samples.len(), 1);
        
        // Verify system prompts are preserved
        for sample in &samples.train_samples {
            assert_eq!(sample.system, "You are a helpful assistant.");
        }
        for sample in &samples.val_samples {
            assert_eq!(sample.system, "You are a helpful assistant.");
        }
    }
    
    #[test]
    fn test_place_multiple_times() {
        let mut samples = Samples {
            train_samples: Vec::new(),
            val_samples: Vec::new(),
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
    }
    
    #[test]
    fn test_preserves_system_prompts_across_groups() {
        let mut samples = Samples {
            train_samples: Vec::new(),
            val_samples: Vec::new(),
        };
        
        // First group with system A
        let original1 = create_test_sample_with_system("1", "System A");
        let variants1 = vec![create_test_sample_with_system("2", "System A")];
        sample_place_into_vecs(&mut samples, original1, Some(variants1));
        
        // Second group with system B
        let original2 = create_test_sample_with_system("3", "System B");
        let variants2 = vec![create_test_sample_with_system("4", "System B")];
        sample_place_into_vecs(&mut samples, original2, Some(variants2));
        
        // Verify system prompts are preserved per sample
        for sample in &samples.train_samples {
            let prompt_text = get_prompt_text(sample);
            if prompt_text.contains("Prompt 1") || prompt_text.contains("Prompt 2") {
                assert_eq!(sample.system, "System A", "Sample with prompt {} should have System A", prompt_text);
            } else if prompt_text.contains("Prompt 3") || prompt_text.contains("Prompt 4") {
                assert_eq!(sample.system, "System B", "Sample with prompt {} should have System B", prompt_text);
            }
        }
        
        for sample in &samples.val_samples {
            let prompt_text = get_prompt_text(sample);
            if prompt_text.contains("Prompt 1") || prompt_text.contains("Prompt 2") {
                assert_eq!(sample.system, "System A", "Sample with prompt {} should have System A", prompt_text);
            } else if prompt_text.contains("Prompt 3") || prompt_text.contains("Prompt 4") {
                assert_eq!(sample.system, "System B", "Sample with prompt {} should have System B", prompt_text);
            }
        }
    }
}
