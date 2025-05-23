use crate::prelude::*;
use super::Role;

// A message
#[derive(Debug, Clone, From, Serialize, Deserialize)]
#[from(String, "Self { role: Role::User, content: value.into() }")]
#[from(&str, "Self { role: Role::User, content: value.into() }")]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    // Creates a new message
    pub fn new<S: Into<String>>(role: Role, content: S) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }

    /// Возвращает длину контента в символах
    pub fn len(&self) -> usize {
        self.content.chars().count()
    }
}
