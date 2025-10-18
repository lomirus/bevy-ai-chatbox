use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{Model, Role, response::fields::FinishReason};

#[derive(Serialize, Deserialize, Debug)]
pub struct Chunk {
    pub id: String,
    pub choices: Vec<Choice>,
    pub created: u64,
    pub model: Model,
    pub system_fingerprint: String,
    pub object: Object,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Choice {
    pub index: u64,
    pub finish_reason: Option<FinishReason>,
    pub delta: Delta,
    pub logprobs: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Delta {
    pub content: String,
    pub reasoning_content: Option<String>,
    pub role: Option<Role>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Object {
    #[serde(rename = "chat.completion.chunk")]
    ChatCompletionChunk,
}
