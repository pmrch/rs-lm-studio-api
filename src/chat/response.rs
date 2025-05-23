use crate::prelude::*;
use super::{ Choice, StreamChoice, Usage, Role, Message };

// Chat response
#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
    #[serde(default)]
    pub stats: HashMap<String, serde_json::Value>,
    pub system_fingerprint: String,
}


use futures::StreamExt;
use tokio_stream::wrappers::UnboundedReceiverStream;

// Chat response stream
#[derive(Debug)]
pub struct ResponseReader {
    pub receiver: UnboundedReceiverStream<std::result::Result<StreamChoice, reqwest::Error>>,
    pub message: Message,
    pub is_ready: bool
}

impl ResponseReader {
    pub fn new(receiver: UnboundedReceiverStream<std::result::Result<StreamChoice, reqwest::Error>>) -> Self {
        Self {
            receiver,
            message: Message { role: Role::Assistant, content: str!("") },
            is_ready: false
        }
    }

    pub async fn next(&mut self) -> Option<std::result::Result<String, reqwest::Error>> {
        let result = self.receiver.next().await;

        match result {
            Some(result) => {
                match result {
                    Ok(choice) => {
                        if let Some(text) = choice.delta.content {
                            self.message.content.push_str(&text);

                            Some(Ok(text))
                        } else {
                            Some(Ok(String::new()))
                        }
                    },

                    Err(e) => Some(Err(e))
                }
            },

            _ => {
                self.is_ready = true;
                
                None
            }
        }
    }
}
