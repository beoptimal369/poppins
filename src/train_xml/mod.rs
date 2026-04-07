// src/train_xml/mod.rs

mod train_xml_parse;
mod train_xml_merge;
mod train_xml_id_maps;
mod train_xml_structs;
mod train_xml_validate;
mod train_xml_validate_ids;
mod train_xml_merge_samples;
mod train_xml_merge_phrases;
mod train_xml_merge_prompts;
mod train_xml_merge_thoughts;
mod train_xml_merge_sources;
mod train_xml_phrase_pattern;
mod train_xml_phrase_patterns;
mod train_xml_merge_constants;
mod train_xml_merge_responses;
mod train_xml_merge_beyond_scope;
mod train_xml_validate_precision;
mod train_xml_merge_code_snippets;
mod train_xml_validate_ids_sample;
mod train_xml_validate_line_breaks;
mod train_xml_merge_system_prompts;
mod train_xml_validate_ids_imports;
mod train_xml_validate_ids_sample_ids;
mod train_xml_validate_prompt_presence;
mod train_xml_validate_ids_beyond_scope;

pub use train_xml_parse::train_xml_parse;
pub use train_xml_merge::train_xml_merge;
pub use train_xml_id_maps::TrainXMLIdMaps;
pub use train_xml_validate::train_xml_validate;
pub use train_xml_validate_ids::train_xml_validate_ids;
pub use train_xml_phrase_pattern::TrainXMLPhrasePattern;
pub use train_xml_merge_samples::train_xml_merge_samples;
pub use train_xml_merge_phrases::train_xml_merge_phrases;
pub use train_xml_merge_prompts::train_xml_merge_prompts;
pub use train_xml_merge_sources::train_xml_merge_sources;
pub use train_xml_merge_thoughts::train_xml_merge_thoughts;
pub use train_xml_merge_constants::train_xml_merge_constants;
pub use train_xml_phrase_patterns::train_xml_phrase_patterns;
pub use train_xml_merge_responses::train_xml_merge_responses;
pub use train_xml_merge_beyond_scope::train_xml_merge_beyond_scope;
pub use train_xml_validate_precision::train_xml_validate_precision;
pub use train_xml_validate_ids_sample::train_xml_validate_ids_sample;
pub use train_xml_merge_code_snippets::train_xml_merge_code_snippets;
pub use train_xml_validate_line_breaks::train_xml_validate_line_breaks;
pub use train_xml_merge_system_prompts::train_xml_merge_system_prompts;
pub use train_xml_validate_ids_imports::train_xml_validate_ids_imports;
pub use train_xml_validate_ids_sample_ids::train_xml_validate_ids_sample_ids;
pub use train_xml_validate_prompt_presence::train_xml_validate_prompt_presence;
pub use train_xml_validate_ids_beyond_scope::train_xml_validate_ids_beyond_scope;


pub use train_xml_structs::{
    TrainXML,
    TrainXMLPrompts,
    TrainXMLSources,
    TrainXMLPhrases,
    TrainXMLSamples,
    TrainXMLThoughts,
    TrainXMLConstants,
    TrainXMLResponses,
    TrainXMLCodeSnippets,
    TrainXMLSamplesSystem,
    TrainXMLPhrasesPhrase,
    TrainXMLSystemPrompts,
    TrainXMLSamplesSample,
    TrainXMLSourcesSource,
    TrainXMLPromptsPrompt,
    TrainXMLConstantParsed,
    TrainXMLThoughtsThought,
    TrainXMLSamplesSampleIds,
    TrainXMLCodeSnippetsCode,
    TrainXMLResponsesResponse,
    TrainXMLSystemPromptsSystem,
    TrainXMLSamplesSampleChildren,
};

#[cfg(test)]
pub use train_xml_structs::{
    TrainXMLImports,
    TrainXMLLineBreak,
    TrainXMLSamplesCode,
    TrainXMLBeyondScope,
    TrainXMLImportsImport,
    TrainXMLSamplesPrompt,
    TrainXMLSamplesSource,
    TrainXMLSamplesThought,
    TrainXMLPhrasesVariant,
    TrainXMLSamplesResponse,
    TrainXMLBeyondScopeTopic,
    TrainXMLSamplesResponseIds,
    TrainXMLBpeRequestedTokens,
};
