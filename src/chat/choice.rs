use crate::prelude::*;
use super::{ Message, Delta };

// Response choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub index: usize,
    pub logprobs: Option<serde_json::Value>,
    pub finish_reason: String,
    pub message: Message,
}


// Response choice stream
#[derive(Debug, Clone, Deserialize)]
pub struct StreamChoice {
    pub delta: Delta,
}
