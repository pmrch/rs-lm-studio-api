/// EmbeddingModel represents supported embedding models for generating embeddings.
/// It includes predefined models and allows specifying a custom model by name.
use crate::prelude::*;

#[derive(Debug, Clone, From, Eq, PartialEq, Serialize, Deserialize)]
pub enum EmbeddingModel {
    /// Predefined model: all-MiniLM-L6. This is a commonly used 
    // model for text embeddings.
    #[serde(rename = "text-embedding-all-minilm-l6-v2-embedding")]
    AllMiniLmL6,

    /// Custom model name as String. Allows users to specify any 
    /// model available in LM Studio.
    #[from]
    Custom(String)
}

/// Now we can .to_string() a model cleanly, e.g. for CLI arguments,
/// logging, or diagnostics.
impl std::fmt::Display for EmbeddingModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AllMiniLmL6 => write!(f, "text-embedding-all-minilm-l6-v2-embedding"),
            Self::Custom(s) => write!(f, "{s}")
        }
    }
}