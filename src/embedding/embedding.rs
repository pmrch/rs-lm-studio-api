// This module contains the main embedding logic and API client for LM Studio's embedding endpoint.
// It provides the structures and methods for sending requests and receiving embedding vectors.
use crate::prelude::*;
use super::*;

use reqwest::{Client};


// Embedding client for LM Studio API
pub struct Embedding {
    // API endpoint URL for embedding requests
    pub url: String,

    // HTTP client used to send requests
    pub client: Client
}

impl Embedding {
    // Create a new Embedding client for a given port. 
    //This sets up the endpoint and HTTP client.
    pub fn new(url: Option<String>) -> 
    Self 
    {
        if url.is_some() {
            Self {
                url: url.unwrap().to_string(),
                client: Client::new()
            }
        } else { 
            Self {
                url: fmt!("http://127.0.0.1:1234/v1/embeddings"),
                client: Client::new()
            }
        }
    }

    // Send an embedding request and return the response from the API.
    // This method serializes the request, sends it, and parses the response.
    pub async fn send(&mut self, req: EmbeddingRequest) -> Result<EmbeddingResponse> {
        let resp = self.client.post(&self.url)
            .json(&req)
            .send()
            .await?
            .error_for_status()?
            .json::<EmbeddingResponse>()
            .await?;        

        Ok(resp)
    }
}