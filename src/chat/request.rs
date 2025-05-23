use crate::prelude::*;
use super::{ Model, Message };

// Chat request
#[derive(Debug, Clone, Serialize)]
pub struct Request {
    pub model: Model,
    pub messages: Vec<Message>,
    pub temperature: f32,
    pub max_tokens: i32,
    pub stream: bool,
}

impl ::std::default::Default for Request {
    fn default() -> Self {
        Self {
            model: Model::Custom("".into()),
            messages: vec![],
            temperature: 0.7,
            max_tokens: 4090,
            stream: false
        }
    }
}
