// src/train_xml/train_xml_structs.rs

use crate::device::Device;
use crate::sample::SampleIndent;
use serde::{Serialize, Deserialize};


#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TrainXML {
    pub imports: Option<TrainXMLImports>,
    #[serde(rename = "system-prompts")]
    pub system_prompts: Option<TrainXMLSystemPrompts>,
    pub prompts: Option<TrainXMLPrompts>,
    pub thoughts: Option<TrainXMLThoughts>,
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



// Imports:
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainXMLImports {
    /// The sequence of system elements
    pub import: Vec<TrainXMLImportsImport>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrainXMLImportsImport {
    /// Path to imported file
    #[serde(rename = "@path")]
    pub path: String,

    /// The unique system id
    #[serde(rename = "@system")]
    pub system: Option<String>,
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



// Thoughts:
#[derive(Debug, Serialize, Deserialize)]
pub struct TrainXMLThoughts {
    /// The sequence of thought elements
    pub thought: Vec<TrainXMLThoughtsThought>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrainXMLThoughtsThought {
    /// Unique identifier for this thought
    #[serde(rename = "@id")]
    pub id: String,

    /// The thought markdown content
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


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainXMLSamplesSampleIds {
    /// System pompt unique identifier
    #[serde(rename = "@system")]
    pub system: Option<String>,

    /// Prompt unique identifier
    #[serde(rename = "@prompt")]
    pub prompt: String,

    /// Thought unique identifier
    #[serde(rename = "@thought")]
    pub thought: Option<String>,

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


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainXMLSamplesSample {
    /// All children elements, preserves element order
    #[serde(rename = "$value", default)]
    pub children: Vec<TrainXMLSamplesSampleChildren>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TrainXMLSamplesSampleChildren {
    #[serde(rename = "system")]
    System(TrainXMLSamplesSystem),
    #[serde(rename = "prompt")]
    Prompt(TrainXMLSamplesPrompt),
    #[serde(rename = "thought")]
    Thought(TrainXMLSamplesThought),
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


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainXMLLineBreak {
    /// 1 or 2
    #[serde(rename = "@count")]
    pub count: u8,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainXMLSamplesResponseIds {
    /// Response unique identifier
    #[serde(rename = "@response")]
    pub response: String,

    /// Source unique identifier
    #[serde(rename = "@source")]
    pub source: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainXMLSamplesPrompt {
    /// Unique identifier for this prompt
    #[serde(rename = "@id")]
    pub id: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainXMLSamplesThought {
    /// Unique identifier for this prompt
    #[serde(rename = "@id")]
    pub id: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainXMLSamplesResponse {
    /// Unique identifier for this response
    #[serde(rename = "@id")]
    pub id: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainXMLSamplesSystem {
    /// Unique identifier for this system prompt
    #[serde(rename = "@id")]
    pub id: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainXMLSamplesSource {
    /// Unique identifier for this response
    #[serde(rename = "@id")]
    pub id: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainXMLPhrases {
    /// The sequence of phrase elements
    pub phrase: Vec<TrainXMLPhrasesPhrase>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainXMLPhrasesPhrase {
    /// The pattern to search for in the prompt (can include regex capture groups $1, $2, etc.)
    #[serde(rename = "@pattern")]
    pub pattern: String,
    
    /// The sequence of variant elements
    pub variant: Vec<TrainXMLPhrasesVariant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainXMLPhrasesVariant {
    /// The replacement value (can reference capture groups like $1, $2, etc.)
    #[serde(rename = "@value")]
    pub value: String,
}



// Constants:
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrainXMLConstants {
    #[serde(rename = "aim-train-gb", default, skip_serializing_if = "Option::is_none")]
    pub aim_train_gb: Option<f32>,

    #[serde(rename = "aim-infer-gb", default, skip_serializing_if = "Option::is_none")]
    pub aim_infer_gb: Option<f32>,

    #[serde(rename = "aim-loss", default, skip_serializing_if = "Option::is_none")]
    pub aim_loss: Option<f32>,

    #[serde(rename = "learning-rate", default, skip_serializing_if = "Option::is_none")]
    pub learning_rate: Option<f32>,

    #[serde(rename = "warmup-steps", default, skip_serializing_if = "Option::is_none")]
    pub warmup_steps: Option<usize>,

    #[serde(rename = "val-interval", default, skip_serializing_if = "Option::is_none")]
    pub val_interval: Option<usize>,

    #[serde(rename = "batch-size", default, skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<usize>,

    #[serde(rename = "mixed-precision", default, skip_serializing_if = "Option::is_none")]
    pub mixed_precision: Option<bool>,

    #[serde(rename = "gradient-accumulation-steps", default, skip_serializing_if = "Option::is_none")]
    pub gradient_accumulation_steps: Option<usize>,

    #[serde(rename = "activation-precision", default, skip_serializing_if = "Option::is_none")]
    pub activation_precision: Option<String>,

    #[serde(rename = "kv-cache-precision", default, skip_serializing_if = "Option::is_none")]
    pub kv_cache_precision: Option<String>,

    #[serde(rename = "rope-precision", default, skip_serializing_if = "Option::is_none")]
    pub rope_precision: Option<String>,

    #[serde(rename = "num-workers", default, skip_serializing_if = "Option::is_none")]
    pub num_workers: Option<usize>,

    #[serde(rename = "use-flash-attention", default, skip_serializing_if = "Option::is_none")]
    pub use_flash_attention: Option<bool>,

    #[serde(rename = "use-tensor-cores", default, skip_serializing_if = "Option::is_none")]
    pub use_tensor_cores: Option<bool>,

    #[serde(rename = "bpe-min-merge-frequency", default, skip_serializing_if = "Option::is_none")]
    pub bpe_min_merge_frequency: Option<usize>,

    #[serde(rename = "bpe-requested-tokens", default, skip_serializing_if = "Option::is_none")]
    pub bpe_requested_tokens: Option<TrainXMLBpeRequestedTokens>,

    #[serde(rename = "weight-decay-response", default, skip_serializing_if = "Option::is_none")]
    pub weight_decay_response: Option<f32>,

    #[serde(rename = "weight-decay-source", default, skip_serializing_if = "Option::is_none")]
    pub weight_decay_source: Option<f32>,

    #[serde(rename = "weight-decay-code", default, skip_serializing_if = "Option::is_none")]
    pub weight_decay_code: Option<f32>,

    #[serde(rename = "dropout-rate-response", default, skip_serializing_if = "Option::is_none")]
    pub dropout_rate_response: Option<f32>,

    #[serde(rename = "dropout-rate-source", default, skip_serializing_if = "Option::is_none")]
    pub dropout_rate_source: Option<f32>,

    #[serde(rename = "dropout-rate-code", default, skip_serializing_if = "Option::is_none")]
    pub dropout_rate_code: Option<f32>,

    #[serde(rename = "loss-scale-response", default, skip_serializing_if = "Option::is_none")]
    pub loss_scale_response: Option<f32>,

    #[serde(rename = "loss-scale-source", default, skip_serializing_if = "Option::is_none")]
    pub loss_scale_source: Option<f32>,

    #[serde(rename = "loss-scale-code", default, skip_serializing_if = "Option::is_none")]
    pub loss_scale_code: Option<f32>,

    #[serde(rename = "gradient-scale-response", default, skip_serializing_if = "Option::is_none")]
    pub gradient_scale_response: Option<f32>,

    #[serde(rename = "gradient-scale-source", default, skip_serializing_if = "Option::is_none")]
    pub gradient_scale_source: Option<f32>,

    #[serde(rename = "gradient-scale-code", default, skip_serializing_if = "Option::is_none")]
    pub gradient_scale_code: Option<f32>,

    #[serde(rename = "gradient-clip-response", default, skip_serializing_if = "Option::is_none")]
    pub gradient_clip_response: Option<f32>,

    #[serde(rename = "gradient-clip-source", default, skip_serializing_if = "Option::is_none")]
    pub gradient_clip_source: Option<f32>,

    #[serde(rename = "gradient-clip-code", default, skip_serializing_if = "Option::is_none")]
    pub gradient_clip_code: Option<f32>,
}


// BPE requested tokens container
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrainXMLBpeRequestedTokens {
    #[serde(rename = "value", default)]
    pub values: Vec<String>,
}


// Helper methods for TrainXMLConstants
impl TrainXMLConstants {
    /// Merge XML constants with device defaults
    pub fn merge_with_defaults(&self, device: &Device) -> TrainXMLConstantParsed {
        let defaults = TrainXMLConstantParsed::create(device);
        
        TrainXMLConstantParsed {
            aim_train_gb: self.aim_train_gb.unwrap_or(defaults.aim_train_gb),
            aim_infer_gb: self.aim_infer_gb.unwrap_or(defaults.aim_infer_gb),
            aim_loss: self.aim_loss.unwrap_or(defaults.aim_loss),
            learning_rate: self.learning_rate.unwrap_or(defaults.learning_rate),
            warmup_steps: self.warmup_steps.unwrap_or(defaults.warmup_steps),
            val_interval: self.val_interval.unwrap_or(defaults.val_interval),
            batch_size: self.batch_size.unwrap_or(defaults.batch_size),
            mixed_precision: self.mixed_precision.unwrap_or(defaults.mixed_precision),
            gradient_accumulation_steps: self.gradient_accumulation_steps.unwrap_or(defaults.gradient_accumulation_steps),
            activation_precision: self.activation_precision.clone().unwrap_or(defaults.activation_precision),
            kv_cache_precision: self.kv_cache_precision.clone().unwrap_or(defaults.kv_cache_precision),
            rope_precision: self.rope_precision.clone().unwrap_or(defaults.rope_precision),
            num_workers: self.num_workers.unwrap_or(defaults.num_workers),
            use_flash_attention: self.use_flash_attention.unwrap_or(defaults.use_flash_attention),
            use_tensor_cores: self.use_tensor_cores.unwrap_or(defaults.use_tensor_cores),
            bpe_min_merge_frequency: self.bpe_min_merge_frequency.unwrap_or(defaults.bpe_min_merge_frequency),
            bpe_requested_tokens: self.bpe_requested_tokens.as_ref()
                .map(|t| t.values.clone())
                .unwrap_or(defaults.bpe_requested_tokens),
            weight_decay_response: self.weight_decay_response.unwrap_or(defaults.weight_decay_response),
            weight_decay_source: self.weight_decay_source.unwrap_or(defaults.weight_decay_source),
            weight_decay_code: self.weight_decay_code.unwrap_or(defaults.weight_decay_code),
            loss_scale_response: self.loss_scale_response.unwrap_or(defaults.loss_scale_response),
            loss_scale_source: self.loss_scale_source.unwrap_or(defaults.loss_scale_source),
            loss_scale_code: self.loss_scale_code.unwrap_or(defaults.loss_scale_code),
            gradient_scale_response: self.gradient_scale_response.unwrap_or(defaults.gradient_scale_response),
            gradient_scale_source: self.gradient_scale_source.unwrap_or(defaults.gradient_scale_source),
            gradient_scale_code: self.gradient_scale_code.unwrap_or(defaults.gradient_scale_code),
            gradient_clip_response: self.gradient_clip_response.unwrap_or(defaults.gradient_clip_response),
            gradient_clip_source: self.gradient_clip_source.unwrap_or(defaults.gradient_clip_source),
            gradient_clip_code: self.gradient_clip_code.unwrap_or(defaults.gradient_clip_code),
        }
    }
}


// Parsed constants with actual values (after merging defaults)
#[derive(Debug)]
pub struct TrainXMLConstantParsed {
    pub aim_train_gb: f32,
    pub aim_infer_gb: f32,
    pub aim_loss: f32,
    pub learning_rate: f32,
    pub warmup_steps: usize,
    pub val_interval: usize,
    pub batch_size: usize,
    pub mixed_precision: bool,
    pub gradient_accumulation_steps: usize,
    pub activation_precision: String,
    pub kv_cache_precision: String,
    pub rope_precision: String,
    pub num_workers: usize,
    pub use_flash_attention: bool,
    pub use_tensor_cores: bool,

    pub bpe_min_merge_frequency: usize,
    pub bpe_requested_tokens: Vec<String>,

    pub weight_decay_response: f32,
    pub weight_decay_source: f32,
    pub weight_decay_code: f32,

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


impl TrainXMLConstantParsed {
    pub fn create(device: &Device) -> Self {
        const REFERENCE_WARMUP_STEPS: f32 = 360.0;
        const REFERENCE_EFFECTIVE_BATCH_SIZE: f32 = 32.0;

        let aim_train_gb = 7.0;
        let aim_infer_gb = 0.9;
        let batch_size = device.batch_size(aim_train_gb);
        let gradient_accumulation_steps = device.gradient_accumulation_steps();
        let current_effective_batch_size = batch_size * gradient_accumulation_steps;
        let learning_rate = 1e-3 * (current_effective_batch_size as f32 / REFERENCE_EFFECTIVE_BATCH_SIZE as f32);
        let warmup_steps = (REFERENCE_WARMUP_STEPS * (REFERENCE_EFFECTIVE_BATCH_SIZE / current_effective_batch_size as f32)) as usize;

        Self {
            batch_size,
            aim_train_gb,
            aim_infer_gb,
            warmup_steps,
            learning_rate,
            aim_loss: 0.45,
            val_interval: 10,
            num_workers: device.num_workers(),
            kv_cache_precision: "int8".to_string(),
            rope_precision: device.rope_precision(),
            mixed_precision: device.mixed_precision(),
            use_tensor_cores: device.use_tensor_cores(),
            use_flash_attention: device.use_flash_attention(),
            activation_precision: device.activation_precision(),
            gradient_accumulation_steps: device.gradient_accumulation_steps(),

            bpe_min_merge_frequency: 3,
            bpe_requested_tokens: Vec::new(),

            weight_decay_response: 0.01,
            weight_decay_source: 0.05,
            weight_decay_code: 0.02,

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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainXMLBeyondScope {
    /// The system ID this beyond-scope configuration applies to
    #[serde(rename = "@system")]
    pub system: String,

    /// Response to provide
    #[serde(rename = "@response")]
    pub response: String,

    /// Response to provide
    #[serde(rename = "@thought")]
    pub thought: Option<String>,

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
