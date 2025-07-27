use core::fmt;
use std::str::FromStr;
use serde_json;

use crate::prelude::*;
use super::*;


#[derive(Debug)]
pub struct ParseEmbeddingDataError(String);

impl fmt::Display for ParseEmbeddingDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse EmbeddingData: {}", self.0)
    }
}

impl std::error::Error for ParseEmbeddingDataError {}


#[derive(Serialize, Deserialize, Clone)]
pub struct EmbeddingData {
    object: String,
    data: EmbeddingResponse,
    model: String,
    usage: EmbeddingUsage
}

impl FromStr for EmbeddingData {
    type Err = ParseEmbeddingDataError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_str(s)
            .map_err(|e| ParseEmbeddingDataError(e.to_string()))
    }
}