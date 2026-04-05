// src/train_xml/mod.rs

mod train_xml_parse;
mod train_xml_id_maps;
mod train_xml_structs;
mod train_xml_validate;
mod train_xml_validate_ids;
mod train_xml_phrase_pattern;
mod train_xml_constants_parse;
mod train_xml_phrase_patterns;
mod train_xml_validate_line_breaks;
mod train_xml_validate_precision;
mod train_xml_validate_prompt_presence;

pub use train_xml_parse::train_xml_parse;
pub use train_xml_id_maps::TrainXMLIdMaps;
pub use train_xml_validate::train_xml_validate;
pub use train_xml_validate_ids::train_xml_validate_ids;
pub use train_xml_phrase_pattern::TrainXMLPhrasePattern;
pub use train_xml_phrase_patterns::train_xml_phrase_patterns;
pub use train_xml_validate_precision::train_xml_validate_precision;
pub use train_xml_validate_line_breaks::train_xml_validate_line_breaks;
pub use train_xml_validate_prompt_presence::train_xml_validate_prompt_presence;
pub use train_xml_structs::{
    TrainXML,
    TrainXMLSamplesSample,
    TrainXMLSourcesSource,
    TrainXMLPromptsPrompt,
    TrainXMLConstantParsed,
    TrainXMLSamplesSampleIds,
    TrainXMLCodeSnippetsCode,
    TrainXMLResponsesResponse,
    TrainXMLSystemPromptsSystem,
    TrainXMLSamplesSampleChildren,
};

#[cfg(test)]
pub use train_xml_structs::{
    TrainXMLPhrases,
    TrainXMLPrompts,
    TrainXMLSamples,
    TrainXMLSources,
    TrainXMLConstants,
    TrainXMLLineBreak,
    TrainXMLResponses,
    TrainXMLSamplesCode,
    TrainXMLBeyondScope,
    TrainXMLCodeSnippets,
    TrainXMLSamplesSystem,
    TrainXMLSystemPrompts,
    TrainXMLPhrasesPhrase,
    TrainXMLSamplesPrompt,
    TrainXMLSamplesSource,
    TrainXMLPhrasesVariant,
    TrainXMLSamplesResponse,
    TrainXMLBeyondScopeTopic,
    TrainXMLSamplesResponseIds,
    TrainXMLBpeRequestedTokens,
};
