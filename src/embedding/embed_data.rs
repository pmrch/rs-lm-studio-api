use crate::prelude::*;
use super::*;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingData {
    object: String,
    data: EmbeddingResponse,
    model: String,
    usage: EmbeddingUsage
}