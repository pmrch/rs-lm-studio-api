/// EmbeddingRequest defines the structure for an 
/// embedding API request.
/// It specifies the model, input texts, and optional 
/// encoding format for the request.
use crate::{prelude::*};
use super::{ EmbeddingModel };


#[derive(Debug, Clone, Serialize)]
pub struct EmbeddingRequest {
    /// Model to use for embedding. Determines which model 
    /// will process the input.
    pub model: EmbeddingModel,

    /// Input text(s) to embed. Can be a batch of texts for efficiency.
    pub input: Vec<String>,

    /// Optional encoding format (e.g., "float"). Controls the output format 
    /// of the embedding vector.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_format: Option<String>,
}

/// Provides a default EmbeddingRequest with empty input and default 
/// encoding format.
/// This is useful for initializing requests before populating fields.
impl ::std::default::Default for EmbeddingRequest {
    fn default() -> Self {
        Self {
            model: EmbeddingModel::Custom("".into()),
            input: vec![],
            encoding_format: Some("float".to_string())
        }
    }
}