use lm_studio_api_extended::embedding::*;

#[tokio::main]
async fn main() {
    let mut embedder = Embedding::new(None); // Uses default localhost:1234
    let req = EmbeddingRequest {
        model: EmbeddingModel::AllMiniLmL6,
        input: vec!["Rust is magic.".to_string()],
        encoding_format: Some("float".to_string()),
    };

    let res = embedder.embed(req).await.unwrap();
    println!("Embedding: {:?}", res);
    println!("Length of embedding: {}", res.len());
}
