// src/train_xml/train_xml_structs.rs

use crate::sample::SampleIndent;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXML {
    #[serde(rename = "system-prompts")]
    pub system_prompts: Option<TrainXMLSystemPrompts>,
    pub prompts: Option<TrainXMLPrompts>,
    pub responses: Option<TrainXMLResponses>,
    pub sources: Option<TrainXMLSources>,
    #[serde(rename = "code-snippets")]
    pub code_snippets: Option<TrainXMLCodeSnippets>,
    pub samples: Option<TrainXMLSamples>,
    pub constants: Option<TrainXMLConstants>,
    pub phrases: Option<TrainXMLPhrases>,
    #[serde(rename = "beyond-scope")]
    pub beyond_scope: Option<TrainXMLBeyondScope>,
}



// System Prompts:
#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLSystemPrompts {
    /// The sequence of system elements
    pub system: Vec<TrainXMLSystemPromptsSystem>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrainXMLSystemPromptsSystem {
    /// Unique identifier for this system prompt
    #[serde(rename = "@id")]
    pub id: String,

    /// The system prompt markdown content
    #[serde(rename = "$text")]
    pub content: String,
}



// Prompts:
#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLPrompts {
    /// The sequence of prompt elements
    pub prompt: Vec<TrainXMLPromptsPrompt>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrainXMLPromptsPrompt {
    /// Unique identifier for this prompt
    #[serde(rename = "@id")]
    pub id: String,

    /// The prompt markdown content
    #[serde(rename = "$text")]
    pub content: String,
}



// Responses:
#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLResponses {
    /// The sequence of response elements
    pub response: Vec<TrainXMLResponsesResponse>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrainXMLResponsesResponse {
    /// Unique identifier for this response
    #[serde(rename = "@id")]
    pub id: String,

    /// The response markdown content
    #[serde(rename = "$text")]
    pub content: String,
}



// Sources:
#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLSources {
    /// The sequence of source elements
    pub source: Vec<TrainXMLSourcesSource>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrainXMLSourcesSource {
    /// Unique identifier for this source
    #[serde(rename = "@id")]
    pub id: String,

    /// URL for this source
    #[serde(rename = "@url")]
    pub url: String,

    /// Title for this source
    #[serde(rename = "@title")]
    pub title: Option<String>
}



// CodeSnippets:
#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLCodeSnippets {
    /// The sequence of code elements
    pub code: Vec<TrainXMLCodeSnippetsCode>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrainXMLCodeSnippetsCode {
    /// Unique identifier for this code
    #[serde(rename = "@id")]
    pub id: String,

    /// Language of the code
    #[serde(rename = "@lang")]
    pub lang: String,

    /// The code content
    #[serde(rename = "$text")]
    pub content: String,
}



// Samples:
#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLSamples {
    /// The sequence of sample elements
    #[serde(rename = "sample-ids")]
    pub sample_ids: Option<Vec<TrainXMLSamplesSampleIds>>,

    /// The sequence of sample elements
    pub sample: Option<Vec<TrainXMLSamplesSample>>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLSamplesSampleIds {
    /// Prompt unique identifier
    #[serde(rename = "@prompt")]
    pub prompt: String,

    /// System pompt unique identifier
    #[serde(rename = "@system")]
    pub system: Option<String>,

    /// Response unique identifier
    #[serde(rename = "@response")]
    pub response: Option<String>,

    /// Source unique identifier
    #[serde(rename = "@source")]
    pub source: Option<String>,

    /// Code unique identifier
    #[serde(rename = "@code")]
    pub code: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLSamplesSample {
    /// All children elements, preserves element order
    #[serde(rename = "$value", default)]
    pub children: Vec<TrainXMLSamplesSampleChildren>,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum TrainXMLSamplesSampleChildren {
    #[serde(rename = "prompt")]
    Prompt(TrainXMLSamplesPrompt),
    #[serde(rename = "system")]
    System(TrainXMLSamplesSystem),
    #[serde(rename = "response")]
    Response(TrainXMLSamplesResponse),
    #[serde(rename = "source")]
    Source(TrainXMLSamplesSource),
    #[serde(rename = "code")]
    Code(TrainXMLSamplesCode),
    #[serde(rename = "response-ids")]
    ResponseIds(TrainXMLSamplesResponseIds),
    #[serde(rename = "line-break")]
    LineBreak(TrainXMLLineBreak),
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLLineBreak {
    /// 1 or 2
    #[serde(rename = "@count")]
    pub count: u8,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLSamplesResponseIds {
    /// Response unique identifier
    #[serde(rename = "@response")]
    pub response: String,

    /// Source unique identifier
    #[serde(rename = "@source")]
    pub source: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLSamplesPrompt {
    /// Unique identifier for this prompt
    #[serde(rename = "@id")]
    pub id: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLSamplesResponse {
    /// Unique identifier for this response
    #[serde(rename = "@id")]
    pub id: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLSamplesSystem {
    /// Unique identifier for this system prompt
    #[serde(rename = "@id")]
    pub id: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLSamplesSource {
    /// Unique identifier for this response
    #[serde(rename = "@id")]
    pub id: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLSamplesCode {
    /// Unique identifier for this code
    #[serde(rename = "@id")]
    pub id: String,

    /// How much to indent the code
    #[serde(rename = "@indent")]
    pub indent: Option<SampleIndent>,

    /// Is the code inline
    #[serde(rename = "@inline")]
    pub inline: Option<bool>,
}


// Phrases:
#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLPhrases {
    /// The sequence of phrase elements
    pub phrase: Vec<TrainXMLPhrasesPhrase>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLPhrasesPhrase {
    /// The pattern to search for in the prompt (can include regex capture groups $1, $2, etc.)
    #[serde(rename = "@pattern")]
    pub pattern: String,
    
    /// The sequence of variant elements
    pub variant: Vec<TrainXMLPhrasesVariant>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLPhrasesVariant {
    /// The replacement value (can reference capture groups like $1, $2, etc.)
    #[serde(rename = "@value")]
    pub value: String,
}



// Constants:
#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLConstants {
    /// The sequence of constant elements
    pub constant: Vec<TrainXMLConstantsConstant>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLConstantsConstant {
    /// The key for this constant
    #[serde(rename = "@key")]
    pub key: TrainXMLConstantsKey,

    /// The value for this constant
    #[serde(rename = "@value")]
    pub value: String,

    /// The optional delimeter for this constant
    #[serde(rename = "@delimiter")]
    pub delimiter: Option<String>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrainXMLConstantsKey {
    AimTrainGb,
    AimInferF16Gb,
    LearningRate,
    WarmupSteps,
    AimLoss,
    ValInterval,

    BpeRequestedTokens,
    BpeMinMergeFrequency,

    WeightDecayResponse,
    WeightDecaySource,
    WeightDecayCode,

    DropoutRateResponse,
    DropoutRateSource,
    DropoutRateCode,

    LossScaleResponse,
    LossScaleSource,
    LossScaleCode,

    GradientScaleResponse,
    GradientScaleSource,
    GradientScaleCode,

    GradientClipResponse,
    GradientClipSource,
    GradientClipCode,
}


#[derive(Debug)]
pub struct TrainXMLConstantParsed {
    pub warmup_steps: usize,
    pub val_interval: usize,
    pub aim_train_gb: f32,
    pub aim_infer_f16_gb: f32,
    pub learning_rate: f32,
    pub aim_loss: f32,

    pub bpe_min_merge_frequency: usize,
    pub bpe_requested_tokens: Vec<String>,

    pub weight_decay_response: f32,
    pub weight_decay_source: f32,
    pub weight_decay_code: f32,

    pub dropout_rate_response: f32,
    pub dropout_rate_source: f32,
    pub dropout_rate_code: f32,

    pub loss_scale_response: f32,
    pub loss_scale_source: f32,
    pub loss_scale_code: f32,

    pub gradient_scale_response: f32,
    pub gradient_scale_source: f32,
    pub gradient_scale_code: f32,

    pub gradient_clip_response: f32,
    pub gradient_clip_source: f32,
    pub gradient_clip_code: f32,
}


impl Default for TrainXMLConstantParsed {
    fn default() -> Self {
        Self {
            warmup_steps: 100,
            val_interval: 10,
            aim_train_gb: 3.0,
            aim_infer_f16_gb: 0.9,
            learning_rate: 1e-3,
            aim_loss: 0.45,

            bpe_min_merge_frequency: 3,
            bpe_requested_tokens: Vec::new(),

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
            gradient_clip_source: 0.0,
            gradient_clip_code: 0.7,
        }
    }
}


// Beyond Scope:
#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLBeyondScope {
    /// The system ID this beyond-scope configuration applies to
    #[serde(rename = "@system")]
    pub system: String,

    /// Response to provide
    #[serde(rename = "@response")]
    pub response: String,

    /// Additional topics as elements
    #[serde(rename = "topic", default)]
    pub topics: Vec<TrainXMLBeyondScopeTopic>,

    /// Boolean attributes for common topic categories
    #[serde(rename = "@sports", default)]
    pub sports: Option<bool>,
    #[serde(rename = "@food", default)]
    pub food: Option<bool>,
    #[serde(rename = "@movies", default)]
    pub movies: Option<bool>,
    #[serde(rename = "@history", default)]
    pub history: Option<bool>,
    #[serde(rename = "@geography", default)]
    pub geography: Option<bool>,
    #[serde(rename = "@politics", default)]
    pub politics: Option<bool>,
    #[serde(rename = "@science", default)]
    pub science: Option<bool>,
    #[serde(rename = "@health", default)]
    pub health: Option<bool>,
    #[serde(rename = "@art", default)]
    pub art: Option<bool>,
    #[serde(rename = "@music", default)]
    pub music: Option<bool>,
    #[serde(rename = "@fashion", default)]
    pub fashion: Option<bool>,
    #[serde(rename = "@travel", default)]
    pub travel: Option<bool>,
    #[serde(rename = "@pets", default)]
    pub pets: Option<bool>,
    #[serde(rename = "@cars", default)]
    pub cars: Option<bool>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrainXMLBeyondScopeTopic {
    #[serde(rename = "@value")]
    pub value: String,

    #[serde(rename = "@prefix")]
    pub prefix: String,
}
