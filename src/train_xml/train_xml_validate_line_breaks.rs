// src/train_xml/train_xml_validate_line_breaks.rs

use crate::train_xml::{TrainXML, TrainXMLSamplesSampleChildren};


/// Validates that all line break counts are either 1 or 2
pub fn train_xml_validate_line_breaks(train_xml: &TrainXML) -> Result<(), String> {
    if let Some(samples) = &train_xml.samples {
        if let Some(sample_list) = &samples.sample {
            for (sample_idx, sample) in sample_list.iter().enumerate() {
                for (child_idx, child) in sample.children.iter().enumerate() {
                    if let TrainXMLSamplesSampleChildren::LineBreak(line_break) = child {
                        if line_break.count != 1 && line_break.count != 2 {
                            return Err(format!(
                                "Invalid line break count at sample {}, child {}: count = {} (must be 1 or 2)",
                                sample_idx + 1,
                                child_idx + 1,
                                line_break.count
                            ));
                        }
                    }
                }
            }
        }
    }
    Ok(())
}



#[cfg(test)]
mod tests {
    use super::train_xml_validate_line_breaks;
    use crate::train_xml::{
        TrainXML,
        TrainXMLSamples,
        TrainXMLLineBreak,
        TrainXMLSamplesSample,
        TrainXMLSamplesSampleChildren,
    };

    #[test]
    fn test_validate_line_breaks_valid_count_1() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 1 }),
                        ],
                    },
                ]),
            }),
            ..Default::default()
        };

        let result = train_xml_validate_line_breaks(&train_xml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_line_breaks_valid_count_2() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 2 }),
                        ],
                    },
                ]),
            }),
            ..Default::default()
        };

        let result = train_xml_validate_line_breaks(&train_xml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_line_breaks_valid_multiple_line_breaks() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 1 }),
                            TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 2 }),
                            TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 1 }),
                        ],
                    },
                ]),
            }),
            ..Default::default()
        };

        let result = train_xml_validate_line_breaks(&train_xml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_line_breaks_invalid_count_0() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 0 }),
                        ],
                    },
                ]),
            }),
            ..Default::default()
        };

        let result = train_xml_validate_line_breaks(&train_xml);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("count = 0"));
    }

    #[test]
    fn test_validate_line_breaks_invalid_count_3() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 3 }),
                        ],
                    },
                ]),
            }),
            ..Default::default()
        };

        let result = train_xml_validate_line_breaks(&train_xml);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("count = 3"));
    }

    #[test]
    fn test_validate_line_breaks_invalid_count_5() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 5 }),
                        ],
                    },
                ]),
            }),
            ..Default::default()
        };

        let result = train_xml_validate_line_breaks(&train_xml);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("count = 5"));
    }

    #[test]
    fn test_validate_line_breaks_invalid_multiple_samples() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 1 }),
                        ],
                    },
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 3 }),
                        ],
                    },
                ]),
            }),
            ..Default::default()
        };

        let result = train_xml_validate_line_breaks(&train_xml);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("sample 2"));
        assert!(err.contains("count = 3"));
    }

    #[test]
    fn test_validate_line_breaks_invalid_multiple_children() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 1 }),
                            TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 2 }),
                            TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 4 }),
                        ],
                    },
                ]),
            }),
            ..Default::default()
        };

        let result = train_xml_validate_line_breaks(&train_xml);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("child 3"));
        assert!(err.contains("count = 4"));
    }

    #[test]
    fn test_validate_line_breaks_no_samples() {
        let train_xml = TrainXML {
            samples: None,
            ..Default::default()
        };

        let result = train_xml_validate_line_breaks(&train_xml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_line_breaks_empty_samples() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![]),
            }),
            ..Default::default()
        };

        let result = train_xml_validate_line_breaks(&train_xml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_line_breaks_no_line_breaks() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![],  // No line breaks
                    },
                ]),
            }),
            ..Default::default()
        };

        let result = train_xml_validate_line_breaks(&train_xml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_line_breaks_mixed_valid_and_invalid() {
        let train_xml = TrainXML {
            samples: Some(TrainXMLSamples {
                sample_ids: None,
                sample: Some(vec![
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 1 }),
                            TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 2 }),
                        ],
                    },
                    TrainXMLSamplesSample {
                        children: vec![
                            TrainXMLSamplesSampleChildren::LineBreak(TrainXMLLineBreak { count: 0 }),
                        ],
                    },
                ]),
            }),
            ..Default::default()
        };

        let result = train_xml_validate_line_breaks(&train_xml);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("sample 2"));
        assert!(err.contains("count = 0"));
    }
}
