// src/sample/sample_token_stats_container.rs

use crate::sample::SampleTokenStats;
use crate::train_xml::TrainXMLConstantParsed;


/// Token stats container that holds pre-computed stats for each component type
/// Line breaks use the response -> SampleTokenStats
pub struct SampleTokenStatsContainer {
    response: SampleTokenStats,
    source: SampleTokenStats,
    code: SampleTokenStats,
}

impl SampleTokenStatsContainer {
    /// Create a new token stats map from constants
    pub fn new(constants: &TrainXMLConstantParsed) -> Self {
        Self {
            response: SampleTokenStats {
                weight_decay: constants.weight_decay_response,
                dropout: constants.dropout_rate_response,
                loss_scale: constants.loss_scale_response,
                gradient_scale: constants.gradient_scale_response,
                gradient_clip: constants.gradient_clip_response,
            },
            source: SampleTokenStats {
                weight_decay: constants.weight_decay_source,
                dropout: constants.dropout_rate_source,
                loss_scale: constants.loss_scale_source,
                gradient_scale: constants.gradient_scale_source,
                gradient_clip: constants.gradient_clip_source,
            },
            code: SampleTokenStats {
                weight_decay: constants.weight_decay_code,
                dropout: constants.dropout_rate_code,
                loss_scale: constants.loss_scale_code,
                gradient_scale: constants.gradient_scale_code,
                gradient_clip: constants.gradient_clip_code,
            },
        }
    }

    /// Get token stats for a given component type
    ///
    /// # Arguments
    /// * `component_type` - String slice identifying the component type:
    ///   - "response" - Text responses
    ///   - "source" - Source citations
    ///   - "code" - Code snippets  
    ///   - "line-break" - Line breaks (returns same as response)
    ///
    /// # Returns
    /// * `Option<&SampleTokenStats>` - Reference to the token stats for the component type,
    ///   or None if the component type is unknown
    pub fn get(&self, component_type: &str) -> Option<&SampleTokenStats> {
        match component_type {
            "response" | "line-break" => Some(&self.response),
            "source" => Some(&self.source),
            "code" => Some(&self.code),
            _ => None,
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_stats_map() {
        // Create test constants
        let constants = TrainXMLConstantParsed {
            warmup_steps: 100,
            val_interval: 10,
            aim_train_gb: 3.0,
            aim_infer_f16_gb: 0.9,
            learning_rate: 0.001,
            aim_loss: 0.45,

            weight_decay_response: 0.1,
            weight_decay_source: 0.01,
            weight_decay_code: 0.05,

            dropout_rate_response: 0.05,
            dropout_rate_source: 0.0,
            dropout_rate_code: 0.1,

            loss_scale_response: 1.0,
            loss_scale_source: 0.2,
            loss_scale_code: 1.0,

            gradient_scale_response: 1.0,
            gradient_scale_source: 2.0,
            gradient_scale_code: 1.2,

            gradient_clip_response: 1.0,
            gradient_clip_source: 0.1,
            gradient_clip_code: 0.7,
        };

        let stats_map = SampleTokenStatsContainer::new(&constants);

        // Test response stats
        let response_stats = stats_map.get("response").unwrap();
        assert_eq!(response_stats.weight_decay, 0.1);
        assert_eq!(response_stats.dropout, 0.05);
        assert_eq!(response_stats.loss_scale, 1.0);
        assert_eq!(response_stats.gradient_scale, 1.0);
        assert_eq!(response_stats.gradient_clip, 1.0);

        // Test source stats
        let source_stats = stats_map.get("source").unwrap();
        assert_eq!(source_stats.weight_decay, 0.01);
        assert_eq!(source_stats.dropout, 0.0);
        assert_eq!(source_stats.loss_scale, 0.2);
        assert_eq!(source_stats.gradient_scale, 2.0);
        assert_eq!(source_stats.gradient_clip, 0.1);

        // Test code stats
        let code_stats = stats_map.get("code").unwrap();
        assert_eq!(code_stats.weight_decay, 0.05);
        assert_eq!(code_stats.dropout, 0.1);
        assert_eq!(code_stats.loss_scale, 1.0);
        assert_eq!(code_stats.gradient_scale, 1.2);
        assert_eq!(code_stats.gradient_clip, 0.7);

        // Test line-break returns response stats
        let line_break_stats = stats_map.get("line-break").unwrap();
        assert_eq!(line_break_stats.weight_decay, 0.1);
        assert_eq!(line_break_stats.dropout, 0.05);
        
        // They should be the same reference
        assert!(std::ptr::eq(response_stats, line_break_stats));

        // Test unknown component type
        assert!(stats_map.get("unknown").is_none());
    }
}
