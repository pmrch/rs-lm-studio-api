use crate::prelude::*;

#[derive(Debug, Clone, From, Eq, PartialEq, Serialize, Deserialize)]
pub enum Model {
    // gemma 2:
    #[serde(rename = "gemma-2-2b-it")]
    Gemma2_2b,
    #[serde(rename = "gemma-2-9b-it")]
    Gemma2_9b,
    #[serde(rename = "gemma-2-27b-it")]
    Gemma2_27b,
    
    // gemma 3:
    #[serde(rename = "gemma-3-1b-it-qat")]
    Gemma3_1b,
    #[serde(rename = "gemma-3-4b-it-qat")]
    Gemma3_4b,
    #[serde(rename = "gemma-3-12b-it-qat")]
    Gemma3_12b,
    #[serde(rename = "gemma-3-27b-it-qat")]
    Gemma3_27b,

    // TODO: to add more models..
    
    // custom:
    #[from]
    Custom(String)
}
