// This module re-exports key embedding components for convenient access throughout the crate.
// It provides the main API types and structures for working with embeddings.

// Embedding logic and API client
pub mod embedding;        pub use embedding::Embedding;

// Embedding model definition, including supported and custom models
pub mod embed_model;      pub use embed_model::EmbeddingModel;

// Embedding request structure, used to send data to the embedding API
pub mod embed_request;    pub use embed_request::EmbeddingRequest;

// Input struct for embedding, representing a single message or text to embed
pub mod input;            pub use input::Input;

// Embedding response structure, representing the result returned by the embedding API
pub mod embed_response;   pub use embed_response::EmbeddingResponse;