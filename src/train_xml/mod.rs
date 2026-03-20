// src/train_xml/mod.rs

mod train_xml_parse;
mod train_xml_id_maps;
mod train_xml_structs;
mod train_xml_validate;
mod train_xml_validate_ids;
mod train_xml_constants_parse;

pub use train_xml_id_maps::TrainXMLIdMaps;
pub use train_xml_parse::train_xml_parse;
pub use train_xml_validate::train_xml_validate;
pub use train_xml_validate_ids::train_xml_validate_ids;
pub use train_xml_constants_parse::train_xml_constants_parse;
pub use train_xml_structs::{
    TrainXML,
    TrainXMLConstants,
    TrainXMLConstantsKey,
    TrainXMLSamplesSample,
    TrainXMLSourcesSource,
    TrainXMLPromptsPrompt,
    TrainXMLConstantParsed,
    TrainXMLSamplesSampleIds,
    TrainXMLCodeSnippetsCode,
    TrainXMLResponsesResponse,
    TrainXMLSamplesSampleChildren,
};

#[cfg(test)]
pub use train_xml_structs::{
    TrainXMLPhrases,
    TrainXMLPrompts,
    TrainXMLSamples,
    TrainXMLSources,
    TrainXMLLineBreak,
    TrainXMLResponses,
    TrainXMLSamplesCode,
    TrainXMLCodeSnippets,
    TrainXMLPhrasesPhrase,
    TrainXMLSamplesPrompt,
    TrainXMLSamplesSource,
    TrainXMLPhrasesVariant,
    TrainXMLSamplesResponse,
    TrainXMLConstantsConstant,
    TrainXMLSamplesResponseIds,
};
