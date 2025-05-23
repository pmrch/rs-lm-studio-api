use crate::prelude::*;

// Response stream delta
#[derive(Debug, Clone, Deserialize)]
pub struct Delta {
    pub content: Option<String>,
}
