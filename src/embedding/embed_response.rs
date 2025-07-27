// EmbeddingResponse represents the response from the embedding API.
// It contains the embedding vector and metadata about the response.
use crate::prelude::*;

// The response to an embedding request, 
// including the vector and its metadata
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EmbeddingResponse {
    // Type of object returned (e.g., "embedding"). Useful for 
    //distinguishing response types.
    pub object: String,

    // The embedding vector generated for the input text. This 
    //is the main result.
    pub embedding: Vec<f32>,
    
    // Index of the input in the batch. Useful when multiple 
    //inputs are embedded at once.
    pub index: i32
}

impl EmbeddingResponse {
    // Returns a clone of the embedding vector. This allows 
    //further processing without mutating the original response.
    pub fn actual_embedding(&self) -> Vec<f32> {
        self.embedding.clone()
    }
}