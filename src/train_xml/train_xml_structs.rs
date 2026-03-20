// src/train_xml/train_xml_structs.rs

use regex::Regex;
use crate::sample::SampleIndent;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXML {
    pub prompts: Option<TrainXMLPrompts>,
    pub responses: Option<TrainXMLResponses>,
    pub sources: Option<TrainXMLSources>,
    #[serde(rename = "code-snippets")]
    pub code_snippets: Option<TrainXMLCodeSnippets>,
    pub samples: Option<TrainXMLSamples>,
    pub constants: Option<TrainXMLConstants>,
    pub phrases: Option<TrainXMLPhrases>,
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
    /// The sequence of prompt elements
    pub prompt: TrainXMLSamplesPrompt,

    /// All other elements, preserves element order
    #[serde(rename = "$value", default)]
    pub children: Vec<TrainXMLSamplesSampleChildren>,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum TrainXMLSamplesSampleChildren {
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

impl TrainXMLPhrasesPhrase {
    /// Compiles the pattern into a Regex
    pub fn compile_pattern(&self) -> Result<Regex, regex::Error> {
        Regex::new(&self.pattern)
    }
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
