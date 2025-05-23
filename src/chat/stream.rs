use crate::prelude::*;
use super::StreamChoice;

// Response stream
#[derive(Debug, Clone, Deserialize)]
pub struct Stream {
    pub choices: Vec<StreamChoice>,
}
