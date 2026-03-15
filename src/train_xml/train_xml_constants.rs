// src/train_xml/train_xml_constants.rs

use serde::{Serialize, Deserialize};


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
