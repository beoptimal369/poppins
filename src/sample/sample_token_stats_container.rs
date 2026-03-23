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
    use crate::{sample::SampleTokenStatsContainer, train_xml::TrainXMLConstantParsed};

    #[test]
    fn test_token_stats_map() {
        // Create test constants
        let constants = TrainXMLConstantParsed::default();
        let stats_map = SampleTokenStatsContainer::new(&constants);

        // Test response stats
        let response_stats = stats_map.get("response").unwrap();
        assert_eq!(response_stats.weight_decay, constants.weight_decay_response);
        assert_eq!(response_stats.dropout, constants.dropout_rate_response);
        assert_eq!(response_stats.loss_scale, constants.loss_scale_response);
        assert_eq!(response_stats.gradient_scale, constants.gradient_scale_response);
        assert_eq!(response_stats.gradient_clip, constants.gradient_clip_response);

        // Test source stats
        let source_stats = stats_map.get("source").unwrap();
        assert_eq!(source_stats.weight_decay, constants.weight_decay_source);
        assert_eq!(source_stats.dropout, constants.dropout_rate_source);
        assert_eq!(source_stats.loss_scale, constants.loss_scale_source);
        assert_eq!(source_stats.gradient_scale, constants.gradient_scale_source);
        assert_eq!(source_stats.gradient_clip, constants.gradient_clip_source);

        // Test code stats
        let code_stats = stats_map.get("code").unwrap();
        assert_eq!(code_stats.weight_decay, constants.weight_decay_code);
        assert_eq!(code_stats.dropout, constants.dropout_rate_code);
        assert_eq!(code_stats.loss_scale, constants.loss_scale_code);
        assert_eq!(code_stats.gradient_scale, constants.gradient_scale_code);
        assert_eq!(code_stats.gradient_clip, constants.gradient_clip_code);

        // Test line-break returns response stats
        let line_break_stats = stats_map.get("line-break").unwrap();
        assert_eq!(line_break_stats.weight_decay, constants.weight_decay_response);
        assert_eq!(line_break_stats.dropout, constants.dropout_rate_response);
        
        // They should be the same reference
        assert!(std::ptr::eq(response_stats, line_break_stats));

        // Test unknown component type
        assert!(stats_map.get("unknown").is_none());
    }
}
