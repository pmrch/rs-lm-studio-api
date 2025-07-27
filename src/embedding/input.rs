// The Input struct represents a single input message or text 
// for embedding.
// It is used to encapsulate the content that will be processed 
// by the embedding API.
use crate::prelude::*;

// A message or text to be embedded
#[derive(Debug, Clone, From, Serialize, Deserialize)]
pub struct Input {
    // The content to be embedded. This is the raw text that will 
    // be converted into an embedding vector.
    pub content: String
}
/*
The reason for making it a struct, is to make extending it 
later on much easier
*/


impl Input {
    // Create a new Input from any string-like type. This is useful for constructing inputs from various sources.
    pub fn new<S: Into<String>>(content: S) -> Self {
        Self {
            content: content.into()
        }
    }

    // Get the length of the content in characters. This can be used for validation or analytics.
    pub fn len(&self) -> usize {
        self.content.chars().count()
    }

    pub fn is_empty(&self) -> bool {
        self.content.trim().is_empty()
    }
}